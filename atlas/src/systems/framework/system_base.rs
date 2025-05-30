use std::{
    cell::RefCell,
    ops::{Add, AddAssign},
    rc::Weak,
};

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

impl Add for ContextSizes {
    type Output = ContextSizes;

    fn add(self, other: ContextSizes) -> ContextSizes {
        ContextSizes {
            num_generalized_positions: self.num_generalized_positions
                + other.num_generalized_positions,
            num_generalized_velocities: self.num_generalized_velocities
                + other.num_generalized_velocities,
            num_misc_continuous_states: self.num_misc_continuous_states
                + other.num_misc_continuous_states,
        }
    }
}

impl AddAssign<&ContextSizes> for ContextSizes {
    fn add_assign(&mut self, rhs: &ContextSizes) {
        self.num_generalized_positions += rhs.num_generalized_positions;
        self.num_generalized_velocities += rhs.num_generalized_velocities;
        self.num_misc_continuous_states += rhs.num_misc_continuous_states;
    }
}

impl AddAssign<&ContextSizes> for &mut ContextSizes {
    fn add_assign(&mut self, rhs: &ContextSizes) {
        self.num_generalized_positions += rhs.num_generalized_positions;
        self.num_generalized_velocities += rhs.num_generalized_velocities;
        self.num_misc_continuous_states += rhs.num_misc_continuous_states;
    }
}

pub trait SystemBase {
    // Getters and setters without default implementations
    fn set_name(&mut self, name: String);
    fn name(&self) -> &String;
    fn input_ports(&self) -> Vec<&dyn InputPortBase>;
    fn input_ports_mut(&mut self) -> Vec<&mut dyn InputPortBase>;
    fn output_ports(&self) -> Vec<&dyn OutputPortBase>;
    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPortBase>;
    fn cache_entries(&self) -> &Vec<CacheEntry>;
    fn cache_entries_mut(&mut self) -> &mut Vec<CacheEntry>;
    fn context_sizes(&self) -> &ContextSizes;
    fn context_sizes_mut(&mut self) -> &mut ContextSizes;
    fn system_id(&self) -> &SystemId;
    fn parent_service(&self) -> Option<Weak<RefCell<dyn SystemParentServiceInterface>>>;
    fn set_parent_service(
        &mut self,
        parent_service: Weak<RefCell<dyn SystemParentServiceInterface>>,
    );

    // Context
    // fn allocate_context(&self) -> Box<dyn ContextBase>;
    // fn do_allocate_context(&self) -> Box<dyn ContextBase>;

    fn set_implicit_time_derivatives_residual_size(&mut self, size: usize);

    fn initialize_context_base(&self, context: &mut dyn ContextBase) {
        context.set_system_id(self.system_id().clone());

        self.create_source_trackers(context);

        let mut cache = context.cache().borrow_mut();
        for index in 0..self.num_cache_entries() {
            let cache_index = CacheIndex::new(index);
            let cache_entry = self.cache_entry(&cache_index);
            let cache_value = cache.create_new_cache_entry_value(cache_index);
            cache_value.set_initial_value(cache_entry.allocate());
        }

        // TODO: Add an output port ticket to the context
        // for output_port in self.output_ports() {
        //     context.add_output_port(output_port.index());
        // }
    }

    fn create_source_trackers(&self, context: &mut dyn ContextBase) {
        for input_port in self.input_ports() {
            context.add_input_port(input_port.index());
        }
    }

    // Input port
    fn num_input_ports(&self) -> usize {
        self.input_ports().len()
    }
    fn input_port_base(&self, index: &InputPortIndex) -> &dyn InputPortBase {
        self.input_ports()[index]
    }
    // fn add_input_port(&mut self, input_port: Box<dyn InputPortBase>);
    // fn add_input_port(&mut self, input_port: Box<dyn InputPortBase>) {
    //     self.input_ports_mut().push(input_port);
    // }
    fn eval_abstract_input(
        &self,
        context: &dyn ContextBase,
        input_port_index: &InputPortIndex,
    ) -> Box<dyn AbstractValue> {
        if let Some(fixed_input_port_value) =
            context.fixed_input_port_value(input_port_index.value())
        {
            fixed_input_port_value.value().clone_box()
        } else {
            let parent_context_base = context.parent_base().clone().unwrap();
            let guard = parent_context_base.borrow();
            let input_port = self.input_port_base(input_port_index);

            self.parent_service()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .eval_connected_subsystem_input_port(&*guard, input_port)
                .unwrap()
        }
    }

    // Output port
    fn num_output_ports(&self) -> usize {
        self.output_ports().len()
    }
    fn output_port_base(&self, index: &OutputPortIndex) -> &dyn OutputPortBase {
        self.output_ports()[index]
    }
    // fn add_output_port(&mut self, output_port: Box<dyn OutputPortBase>) {
    //     self.output_ports_mut().push(output_port);
    // }

    // Cache entry
    fn num_cache_entries(&self) -> usize {
        self.cache_entries().len()
    }
    fn cache_entry(&self, index: &CacheIndex) -> &CacheEntry {
        &self.cache_entries()[index]
    }
    fn cache_mut_entry(&mut self, index: &CacheIndex) -> &mut CacheEntry {
        &mut self.cache_entries_mut()[index]
    }
    fn declare_cache_entry(&mut self, value_producer: ValueProducer) -> &CacheEntry {
        let cache_index = CacheIndex::new(self.num_cache_entries());
        let cache_entry = CacheEntry::new(cache_index.clone(), value_producer);
        self.cache_entries_mut().push(cache_entry);

        self.cache_entry(&cache_index)
    }

    // State
    fn num_continuous_states(&self) -> usize {
        let context_sizes = self.context_sizes();

        context_sizes.num_generalized_positions
            + context_sizes.num_generalized_velocities
            + context_sizes.num_misc_continuous_states
    }

    fn implicit_time_derivatives_residual_size(&self) -> usize;

    fn validate_context(&self, context: &dyn ContextBase) {
        assert!(*context.system_id() == *self.system_id())
    }
}
