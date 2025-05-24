use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::framework_common::{
    CacheIndex, InputPortIndex, OutputPortIndex, PortDataType,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::system_base::SystemBase;

pub trait AbstractSystem: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait System<T: AtlasScalar>: AbstractSystem + SystemBase
where
    Self: 'static,
{
    type CN: Context<T>;

    // Getters and setters without default implementations
    fn input_ports(&self) -> Vec<&InputPort<T>>;
    fn input_ports_mut(&mut self) -> Vec<&mut InputPort<T>>;
    fn input_port(&self, index: &InputPortIndex) -> &InputPort<T>;
    fn input_port_mut(&mut self, index: &InputPortIndex) -> &mut InputPort<T>;
    fn add_input_port(&mut self, input_port: InputPort<T>);
    fn has_input_port(&self, name: &str) -> bool {
        <Self as System<T>>::input_ports(self)
            .iter()
            .any(|ip| ip.name() == name)
    }
    fn output_ports(&self) -> Vec<&dyn OutputPort<T, CN = Self::CN>>;
    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T, CN = Self::CN>>;
    fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T, CN = Self::CN>;
    fn output_port_mut(&mut self, index: &OutputPortIndex)
        -> &mut dyn OutputPort<T, CN = Self::CN>;
    fn system_weak_link(&self) -> SystemWeakLink<T>;

    // Resource allocation and initializaion
    fn allocate_context(&self) -> Rc<RefCell<Self::CN>>;
    // fn allocate_context(&mut self) -> Box<dyn Context<T>> {
    //     self.do_allocate_context().as_ref().
    // }
    fn declare_input_port(
        &mut self,
        name: String,
        data_type: PortDataType,
        size: usize,
    ) -> &InputPort<T> {
        let input_port_index = InputPortIndex::new(self.num_input_ports());
        let eval = {
            let cloned_input_port_index = input_port_index.clone();
            let system_weak_link = self.system_weak_link();
            Box::new(move |context_base: &dyn ContextBase| {
                system_weak_link.eval_abstract_input(context_base, &cloned_input_port_index)
            })
        };
        let alloc = {
            let cloned_input_port_index = input_port_index.clone();
            let system_weak_link = self.system_weak_link();
            Box::new(move || {
                let leaf_system_weak_link = system_weak_link.as_leaf_system_weak_link().unwrap();
                let system_rc = leaf_system_weak_link.upgrade().unwrap();
                let system = system_rc.borrow();
                let input_port = system.input_port(&cloned_input_port_index);
                system.allocate_input_abstract(&input_port)
            })
        };
        let input_port = {
            let system_weak_link = self.system_weak_link();
            InputPort::<T>::new(
                name,
                system_weak_link,
                self.system_id().clone(),
                input_port_index.clone(),
                data_type,
                size,
                eval,
                alloc,
            )
        };
        self.add_input_port(input_port);

        self.input_port(&input_port_index)
    }

    fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue>;
    fn allocate_input_vector(&mut self, input_port: &InputPort<T>) -> BasicVector<T> {
        assert!(*input_port.data_type() == PortDataType::VectorValued);
        let self_input_port_base = self.input_port_base(input_port.index());
        assert!(std::ptr::eq(
            self_input_port_base,
            input_port as &dyn InputPortBase
        ));
        let value = self.do_allocate_input(input_port);

        value
            .as_any()
            .downcast_ref::<BasicVector<T>>()
            .unwrap()
            .clone()
    }
    fn allocate_input_abstract(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        self.do_allocate_input(input_port)
    }
    fn allocate_time_derivatives(&mut self) -> Box<<<Self::CN as Context<T>>::S as State<T>>::CS>;
    fn create_default_context(&mut self) -> Rc<RefCell<Self::CN>> {
        let context = self.allocate_context();
        self.set_default_context(&mut context.borrow_mut());
        context
    }

    // TODO: Consider inputting &dyn Context<T> and &mut State<T>
    fn set_default_state(&self, context: &mut Self::CN);
    fn set_default_context(&mut self, context: &mut Self::CN) {
        self.set_default_state(context);
    }

    // Cached evaluations
    fn eval_time_derivatives<'a>(
        &mut self,
        context: &'a mut Self::CN,
    ) -> &'a <<Self::CN as Context<T>>::S as State<T>>::CS {
        let cache_entry = self.time_derivatives_cache_entry();

        cache_entry.eval(context.as_mutable_base())
    }
    fn time_derivatives_cache_entry(&self) -> &CacheEntry {
        self.cache_entry(self.time_derivatives_cache_index())
    }
    fn time_derivatives_cache_index(&self) -> &CacheIndex;

    // Calculations
    fn calc_time_derivatives(
        &mut self,
        context: &mut Self::CN,
        derivatives: Option<&mut <<Self::CN as Context<T>>::S as State<T>>::CS>,
    ) {
        self.validate_context(context.as_base());
        self.do_calc_time_derivatives(context, derivatives.unwrap());
    }

    fn do_calc_time_derivatives(
        &mut self,
        _context: &mut Self::CN,
        derivatives: &mut <<Self::CN as Context<T>>::S as State<T>>::CS,
    ) {
        // This default implementation is only valid for Systems with no continuous
        // state. Other Systems must override this method!
        assert!(derivatives.size() == 0);
    }
}
