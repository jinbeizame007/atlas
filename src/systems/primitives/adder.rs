extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::InputPortIndex;
use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::framework_common::{
    CacheIndex, PortDataType, SystemId, SystemParentServiceInterface,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_output_port::LeafOutputPort;
use crate::systems::framework::leaf_system::LeafSystem;
use crate::systems::framework::model_values::ModelValues;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::system::System;
use crate::systems::framework::system_base::{ContextSizes, SystemBase};
use crate::systems::framework::vector_base::VectorBase;

pub struct Adder<T: AtlasScalar> {
    #[allow(clippy::box_collection)]
    input_ports: Box<Vec<InputPort<T>>>,
    output_ports: Vec<Box<LeafOutputPort<T>>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    parent_service: Option<Box<dyn SystemParentServiceInterface>>,
    time_derivatives_cache_index: CacheIndex,
    model_input_values: ModelValues,
    model_continuous_state_vector: BasicVector<T>,
}

impl<T: AtlasScalar> SystemBase for Adder<T> {
    fn input_ports(&self) -> Vec<&dyn InputPortBase> {
        self.input_ports
            .iter()
            .map(|p| p as &dyn InputPortBase)
            .collect()
    }

    fn input_ports_mut(&mut self) -> Vec<&mut dyn InputPortBase> {
        self.input_ports
            .iter_mut()
            .map(|p| p as &mut dyn InputPortBase)
            .collect()
    }

    fn output_ports(&self) -> Vec<&dyn OutputPortBase> {
        self.output_ports
            .iter()
            .map(|p| p.as_ref() as &dyn OutputPortBase)
            .collect()
    }

    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPortBase> {
        self.output_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn OutputPortBase)
            .collect()
    }

    // fn add_input_port(&mut self, input_port: Box<InputPort<T>>) {
    //     self.input_ports.push(input_port);
    // }

    fn cache_entries(&self) -> &Vec<CacheEntry> {
        &self.cache_entries
    }

    fn cache_mut_entries(&mut self) -> &mut Vec<CacheEntry> {
        &mut self.cache_entries
    }

    fn context_sizes(&self) -> &ContextSizes {
        &self.context_sizes
    }

    fn context_sizes_mut(&mut self) -> &mut ContextSizes {
        &mut self.context_sizes
    }

    fn system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn parent_service(&self) -> Option<&dyn SystemParentServiceInterface> {
        self.parent_service.as_ref().map(|p| p.as_ref())
    }
}

impl<T: AtlasScalar> System<T> for Adder<T> {
    fn input_ports(&self) -> Vec<&InputPort<T>> {
        self.input_ports.iter().collect()
    }

    fn input_ports_mut(&mut self) -> Vec<&mut InputPort<T>> {
        self.input_ports.iter_mut().collect()
    }

    fn input_port(&self, index: &InputPortIndex) -> &InputPort<T> {
        &self.input_ports[index]
    }

    fn input_port_mut(&mut self, index: &InputPortIndex) -> &mut InputPort<T> {
        &mut self.input_ports[index]
    }

    fn add_input_port(&mut self, input_port: InputPort<T>) {
        self.input_ports.push(input_port);
    }

    fn output_ports(&self) -> Vec<&dyn OutputPort<T>> {
        self.output_ports
            .iter()
            .map(|p| p.as_ref() as &dyn OutputPort<T>)
            .collect()
    }

    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T>> {
        self.output_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn OutputPort<T>)
            .collect()
    }

    fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T> {
        self.output_ports[index].as_ref()
    }

    fn output_port_mut(&mut self, index: &OutputPortIndex) -> &mut dyn OutputPort<T> {
        self.output_ports[index].as_mut()
    }

    fn time_derivatives_cache_index(&self) -> &CacheIndex {
        &self.time_derivatives_cache_index
    }

    fn allocate_context(&self) -> Box<dyn Context<T>> {
        LeafSystem::<T>::allocate_context(self)
    }

    fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        LeafSystem::<T>::do_allocate_input(self, input_port)
    }

    fn allocate_time_derivatives(&mut self) -> ContinuousState<T> {
        LeafSystem::<T>::allocate_time_derivatives(self)
    }

    fn set_default_state(&self, context: &mut dyn Context<T>) {
        LeafSystem::<T>::set_default_state(self, context)
    }
}

impl<T: AtlasScalar> LeafSystem<T> for Adder<T> {
    fn model_input_values(&self) -> &ModelValues {
        &self.model_input_values
    }

    fn model_input_values_mut(&mut self) -> &mut ModelValues {
        &mut self.model_input_values
    }

    fn model_continuous_state_vector(&self) -> &BasicVector<T> {
        &self.model_continuous_state_vector
    }

    fn model_continuous_state_vector_mut(&mut self) -> &mut BasicVector<T> {
        &mut self.model_continuous_state_vector
    }

    fn leaf_output_port(&self, output_port_index: &OutputPortIndex) -> &LeafOutputPort<T> {
        &self.output_ports[output_port_index]
    }

    fn leaf_output_port_mut(
        &mut self,
        output_port_index: &OutputPortIndex,
    ) -> &mut LeafOutputPort<T> {
        &mut self.output_ports[output_port_index]
    }

    fn add_output_port(&mut self, output_port: Box<LeafOutputPort<T>>) {
        self.output_ports.push(output_port);
    }
}

impl<T: AtlasScalar> Adder<T> {
    pub fn new(num_inputs: usize, size: usize) -> Self {
        let mut adder = Self {
            input_ports: Box::<Vec<InputPort<T>>>::default(),
            output_ports: vec![],
            cache_entries: vec![],
            context_sizes: ContextSizes::default(),
            system_id: SystemId::new(0),
            parent_service: None,
            time_derivatives_cache_index: CacheIndex::new(0),
            model_input_values: ModelValues::default(),
            model_continuous_state_vector: BasicVector::<T>::zeros(0),
        };

        let calc = {
            let input_ports_calc = adder.input_ports.as_ref() as *const Vec<InputPort<T>>;
            Box::new(
                move |context: &mut dyn Context<T>, sum: &mut BasicVector<T>| unsafe {
                    Adder::<T>::calc_sum(&(*input_ports_calc), context, sum)
                },
            )
        };

        for _ in 0..num_inputs {
            adder.declare_input_port(PortDataType::VectorValued, size);
        }
        adder.declare_vector_output_port(size, calc);

        adder
    }

    fn calc_sum(
        input_ports: &[InputPort<T>],
        context: &mut dyn Context<T>,
        sum: &mut BasicVector<T>,
    ) {
        VectorBase::fill(sum, &T::zero());
        for input_port in input_ports.iter() {
            *sum += input_port.eval::<BasicVector<T>>(context);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let adder = Adder::<f64>::new(2, 3);
        assert_eq!(System::<f64>::input_ports(&adder).len(), 2);
        assert_eq!(System::<f64>::output_ports(&adder).len(), 1);
    }

    #[test]
    fn test_create_default_context() {
        let mut adder = Adder::<f64>::new(2, 3);
        let _context = adder.create_default_context();
    }

    #[test]
    fn test_fix_input_port_values() {
        let mut adder = Adder::<f64>::new(2, 3);
        let mut context = adder.create_default_context();

        adder.input_port_mut(&InputPortIndex::new(0)).fix_value(
            context.as_mut(),
            BasicVector::<f64>::from_vec(vec![1.0, 2.0, 3.0]),
        );
        adder.input_port_mut(&InputPortIndex::new(1)).fix_value(
            context.as_mut(),
            BasicVector::<f64>::from_vec(vec![0.5, 1.2, 0.3]),
        );

        let sum = adder
            .leaf_output_port(&OutputPortIndex::new(0))
            .eval::<BasicVector<f64>>(context.as_mut());
        assert_eq!(
            *sum.value(),
            na::DVector::<f64>::from_vec(vec![1.5, 3.2, 3.3])
        );
    }
}
