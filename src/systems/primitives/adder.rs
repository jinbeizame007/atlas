use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::InputPortIndex;
use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::framework_common::{
    CacheIndex, SystemId, SystemParentServiceInterface,
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

pub struct Adder<T: AtlasScalar> {
    num_inputs: usize,
    size: usize,
    input_ports: Vec<Box<InputPort<T>>>,
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
    fn get_input_ports(&self) -> Vec<&dyn InputPortBase> {
        self.input_ports
            .iter()
            .map(|p| p.as_ref() as &dyn InputPortBase)
            .collect()
    }

    fn get_mutable_input_ports(&mut self) -> Vec<&mut dyn InputPortBase> {
        self.input_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn InputPortBase)
            .collect()
    }

    fn get_output_ports(&self) -> Vec<&dyn OutputPortBase> {
        self.output_ports
            .iter()
            .map(|p| p.as_ref() as &dyn OutputPortBase)
            .collect()
    }

    fn get_mutable_output_ports(&mut self) -> Vec<&mut dyn OutputPortBase> {
        self.output_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn OutputPortBase)
            .collect()
    }

    // fn add_input_port(&mut self, input_port: Box<InputPort<T>>) {
    //     self.input_ports.push(input_port);
    // }

    fn get_cache_entries(&self) -> &Vec<CacheEntry> {
        &self.cache_entries
    }

    fn get_mutable_cache_entries(&mut self) -> &mut Vec<CacheEntry> {
        &mut self.cache_entries
    }

    fn get_context_sizes(&self) -> &ContextSizes {
        &self.context_sizes
    }

    fn get_mutable_context_sizes(&mut self) -> &mut ContextSizes {
        &mut self.context_sizes
    }

    fn get_system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn get_parent_service(&self) -> Option<&dyn SystemParentServiceInterface> {
        self.parent_service.as_ref().map(|p| p.as_ref())
    }
}

impl<T: AtlasScalar> System<T> for Adder<T> {
    fn get_input_ports(&self) -> Vec<&InputPort<T>> {
        self.input_ports.iter().map(|p| p.as_ref()).collect()
    }

    fn get_mutable_input_ports(&mut self) -> Vec<&mut InputPort<T>> {
        self.input_ports.iter_mut().map(|p| p.as_mut()).collect()
    }

    fn get_input_port(&self, index: &InputPortIndex) -> &InputPort<T> {
        self.input_ports[index].as_ref()
    }

    fn get_mutable_input_port(&mut self, index: &InputPortIndex) -> &mut InputPort<T> {
        self.input_ports[index].as_mut()
    }

    fn add_input_port(&mut self, input_port: Box<InputPort<T>>) {
        self.input_ports.push(input_port);
    }

    fn get_output_ports(&self) -> Vec<&dyn OutputPort<T>> {
        self.output_ports
            .iter()
            .map(|p| p.as_ref() as &dyn OutputPort<T>)
            .collect()
    }

    fn get_mutable_output_ports(&mut self) -> Vec<&mut dyn OutputPort<T>> {
        self.output_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn OutputPort<T>)
            .collect()
    }

    fn get_output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T> {
        self.output_ports[index].as_ref()
    }

    fn get_mutable_output_port(&mut self, index: &OutputPortIndex) -> &mut dyn OutputPort<T> {
        self.output_ports[index].as_mut()
    }

    fn get_time_derivatives_cache_index(&self) -> &CacheIndex {
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
    fn get_model_input_values(&self) -> &ModelValues {
        &self.model_input_values
    }

    fn get_mutable_model_input_values(&mut self) -> &mut ModelValues {
        &mut self.model_input_values
    }

    fn get_model_continuous_state_vector(&self) -> &BasicVector<T> {
        &self.model_continuous_state_vector
    }

    fn get_mutable_model_continuous_state_vector(&mut self) -> &mut BasicVector<T> {
        &mut self.model_continuous_state_vector
    }

    fn get_leaf_output_port(&self, output_port_index: &OutputPortIndex) -> &LeafOutputPort<T> {
        &self.output_ports[output_port_index]
    }

    fn add_output_port(&mut self, output_port: Box<LeafOutputPort<T>>) {
        self.output_ports.push(output_port);
    }
}

impl<T: AtlasScalar> Adder<T> {
    pub fn new(num_inputs: usize, size: usize) -> Self {
        Self {
            num_inputs,
            size,
            input_ports: vec![],
            output_ports: vec![],
            cache_entries: vec![],
            context_sizes: ContextSizes::default(),
            system_id: SystemId::new(0),
            parent_service: None,
            time_derivatives_cache_index: CacheIndex::new(0),
            model_input_values: ModelValues::default(),
            model_continuous_state_vector: BasicVector::<T>::zeros(0),
        }
    }

    // fn calc_sum(&self, context: &mut dyn Context<T>, sum: &mut BasicVector<T>) {
    //     let mut sum_vector = sum.get_mutable_value();
    //     for input_port in self.input_ports.iter() {
    //         sum_vector = sum_vector
    //             + input_port
    //                 .eval::<BasicVector<T>>(context)
    //                 .get_value()
    //                 .clone();
    //     }
    // }
}
