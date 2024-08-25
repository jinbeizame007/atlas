use crate::common::value::AbstractValue;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::framework_common::{
    CacheIndex, InputPortIndex, OutputPortIndex, SystemId, SystemParentServiceInterface,
};
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::value_producer::ValueProducer;

#[derive(Default)]
pub struct ContextSizes {
    pub num_generalized_positions: usize,
    pub num_generalized_velocities: usize,
    pub num_misc_continuous_states: usize,
}

pub trait SystemBase {
    // Context
    // fn allocate_context(&self) -> Box<dyn ContextBase>;
    // fn do_allocate_context(&self) -> Box<dyn ContextBase>;
    fn initialize_context_base(&self, context: &mut dyn ContextBase) {
        context.set_system_id(self.get_system_id().clone());

        let cache = context.get_mutable_cache();
        for index in 0..self.num_cache_entries() {
            let cache_index = CacheIndex::new(index);
            let cache_entry = self.get_cache_entry(&cache_index);
            let cache_value = cache.create_new_cache_entry_value(cache_index);
            cache_value.set_initial_value(cache_entry.allocate());
        }

        // TODO: Add an output port ticket to the context
    }

    // Input port
    fn get_input_ports(&self) -> &Vec<Box<dyn InputPortBase>>;
    fn get_mutable_input_ports(&mut self) -> &mut Vec<Box<dyn InputPortBase>>;
    fn num_input_ports(&self) -> usize {
        self.get_input_ports().len()
    }
    fn get_input_port_base(&self, index: &InputPortIndex) -> &dyn InputPortBase {
        self.get_input_ports()[index].as_ref()
    }
    fn add_input_port(&mut self, input_port: Box<dyn InputPortBase>) {
        self.get_mutable_input_ports().push(input_port);
    }
    fn eval_abstract_input(
        &self,
        context: &dyn ContextBase,
        input_port_index: &InputPortIndex,
    ) -> Box<dyn AbstractValue> {
        if let Some(fixed_input_port_value) =
            context.get_fixed_input_port_value(input_port_index.value())
        {
            fixed_input_port_value.get_value().clone_box()
        } else {
            let parent_context_base = context.get_parent_base().unwrap();
            let mut guard = parent_context_base.lock().unwrap();
            let input_port = self.get_input_port_base(input_port_index);

            self.get_parent_service()
                .eval_connected_subsystem_input_port(&mut *guard, input_port)
        }
    }

    // Output port
    fn get_output_ports(&self) -> &Vec<Box<dyn OutputPortBase>>;
    fn get_mutable_output_ports(&mut self) -> &mut Vec<Box<dyn OutputPortBase>>;
    fn num_output_ports(&self) -> usize {
        self.get_output_ports().len()
    }
    fn get_output_port_base(&self, index: &OutputPortIndex) -> &dyn OutputPortBase {
        self.get_output_ports()[index].as_ref()
    }
    fn add_output_port(&mut self, output_port: Box<dyn OutputPortBase>) {
        self.get_mutable_output_ports().push(output_port);
    }

    // Cache entry
    fn get_cache_entries(&self) -> &Vec<Box<CacheEntry>>;
    fn get_mutable_cache_entries(&mut self) -> &mut Vec<Box<CacheEntry>>;
    fn num_cache_entries(&self) -> usize {
        self.get_cache_entries().len()
    }
    fn get_cache_entry(&self, index: &CacheIndex) -> &CacheEntry {
        self.get_cache_entries()[index].as_ref()
    }
    fn get_mutable_cache_entry(&mut self, index: &CacheIndex) -> &mut CacheEntry {
        self.get_mutable_cache_entries()[index].as_mut()
    }
    fn declare_cache_entry(&mut self, value_producer: ValueProducer) -> &CacheEntry {
        let cache_index = CacheIndex::new(self.num_cache_entries());
        let cache_entry = Box::new(CacheEntry::new(cache_index.clone(), value_producer));
        self.get_mutable_cache_entries().push(cache_entry);

        self.get_cache_entry(&cache_index)
    }

    // State
    fn num_continuous_states(&self) -> usize {
        let context_sizes = self.get_context_sizes();

        context_sizes.num_generalized_positions
            + context_sizes.num_generalized_velocities
            + context_sizes.num_misc_continuous_states
    }

    fn get_context_sizes(&self) -> &ContextSizes;
    fn get_mutable_context_sizes(&mut self) -> &mut ContextSizes;
    fn get_system_id(&self) -> &SystemId;
    fn get_parent_service(&self) -> &dyn SystemParentServiceInterface;
}
