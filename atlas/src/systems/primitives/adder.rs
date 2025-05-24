use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use atlas_derives::{AbstractSystem, LeafSystem, System, SystemBase};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::framework_common::InputPortIndex;
use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::framework_common::{
    CacheIndex, PortDataType, SystemId, SystemParentServiceInterface,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::leaf_output_port::LeafOutputPort;
use crate::systems::framework::leaf_state::LeafState;
use crate::systems::framework::leaf_system::LeafSystem;
use crate::systems::framework::model_values::ModelValues;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::system::{AbstractSystem, System};
use crate::systems::framework::system_base::ContextSizes;
use crate::systems::framework::system_base::SystemBase;
use crate::systems::framework::vector_base::VectorBase;

#[derive(LeafSystem, AbstractSystem, System, SystemBase)]
pub struct Adder<T: AtlasScalar> {
    name: String,
    input_ports: Vec<InputPort<T>>,
    output_ports: Vec<Box<LeafOutputPort<T>>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    system_weak_link: Option<SystemWeakLink<T>>,
    parent_service: Option<Weak<RefCell<dyn SystemParentServiceInterface>>>,
    time_derivatives_cache_index: CacheIndex,
    model_input_values: ModelValues,
    model_continuous_state_vector: BasicVector<T>,
    implicit_time_derivatives_residual_size: Option<usize>,
}

impl<T: AtlasScalar> Adder<T> {
    pub fn new(num_inputs: usize, size: usize) -> Rc<RefCell<Self>> {
        let adder = Rc::new(RefCell::new(Self {
            name: "adder".to_string(),
            input_ports: vec![],
            output_ports: vec![],
            cache_entries: vec![],
            context_sizes: ContextSizes::default(),
            system_id: SystemId::new(0),
            system_weak_link: None,
            parent_service: None,
            time_derivatives_cache_index: CacheIndex::new(0),
            model_input_values: ModelValues::default(),
            model_continuous_state_vector: BasicVector::<T>::zeros(0),
            implicit_time_derivatives_residual_size: None,
        }));
        // adder.borrow_mut().system_weak_link =
        //     SystemWeakLink::LeafSystemWeakLink(Rc::downgrade(&adder));

        unsafe {
            let adder_weak = Rc::downgrade(&adder);
            let adder_weak_ptr = Weak::into_raw(adder_weak);
            let system_weak =
                Weak::<RefCell<dyn System<T, CN = LeafContext<T>>>>::from_raw(adder_weak_ptr);
            adder.borrow_mut().system_weak_link =
                Some(SystemWeakLink::LeafSystemWeakLink(system_weak));
        }

        let calc = {
            let weak_adder = Rc::downgrade(&adder);
            Box::new(
                move |context: &mut LeafContext<T>, sum: &mut BasicVector<T>| {
                    let adder = weak_adder.upgrade().unwrap();
                    adder.borrow().calc_sum(context, sum);
                },
            )
        };

        for i in 0..num_inputs {
            adder.borrow_mut().declare_input_port(
                format!("input_{}", i),
                PortDataType::VectorValued,
                size,
            );
        }
        adder
            .borrow_mut()
            .declare_vector_output_port("sum".to_string(), size, calc);

        adder
    }

    fn calc_sum(&self, context: &mut <Self as System<T>>::CN, sum: &mut BasicVector<T>) {
        VectorBase::fill(sum, &T::zero());
        for input_port in self.input_ports.iter() {
            *sum += input_port.eval::<LeafState<T>, BasicVector<T>>(context);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let adder = Adder::<f64>::new(2, 3);
        assert_eq!(adder.borrow().input_ports.len(), 2);
        assert_eq!(adder.borrow().output_ports.len(), 1);
    }

    #[test]
    fn test_create_default_context() {
        let adder = Adder::<f64>::new(2, 3);
        let _context = adder.borrow_mut().create_default_context();
    }

    #[test]
    fn test_fix_input_port_values() {
        let adder = Adder::<f64>::new(2, 3);
        let mut context = adder.borrow_mut().create_default_context();

        adder
            .borrow_mut()
            .input_port_mut(&InputPortIndex::new(0))
            .fix_value(
                &mut *context.borrow_mut(),
                BasicVector::<f64>::from_vec(vec![1.0, 2.0, 3.0]),
            );
        adder
            .borrow_mut()
            .input_port_mut(&InputPortIndex::new(1))
            .fix_value(
                &mut *context.borrow_mut(),
                BasicVector::<f64>::from_vec(vec![0.5, 1.2, 0.3]),
            );

        let sum = adder
            .borrow()
            .leaf_output_port(&OutputPortIndex::new(0))
            .eval::<BasicVector<f64>>(&mut *context.borrow_mut());
        assert_eq!(
            *sum.value(),
            na::DVector::<f64>::from_vec(vec![1.5, 3.2, 3.3])
        );
    }
}
