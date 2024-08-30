use std::sync::{Arc, Mutex};

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::cache::Cache;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::state::State;
use crate::systems::framework::vector_base::VectorBase;

#[derive(Default)]
pub struct LeafContext<T: AtlasScalar> {
    system_id: SystemId,
    parent: Option<Arc<Mutex<dyn ContextBase>>>,
    cache: Cache,
    time: T,
    state: State<T>,
    input_port_values: Vec<Option<FixedInputPortValue>>,
    is_context_base_initialized: bool,
}

impl<T: AtlasScalar> ContextBase for LeafContext<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    fn get_system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn get_parent_base(&self) -> Option<Arc<Mutex<dyn ContextBase>>> {
        self.parent.clone()
    }

    fn get_cache(&self) -> &Cache {
        &self.cache
    }

    fn get_mutable_cache(&mut self) -> &mut Cache {
        &mut self.cache
    }

    fn get_input_port_values(&mut self) -> &mut Vec<Option<FixedInputPortValue>> {
        &mut self.input_port_values
    }

    fn num_input_ports(&self) -> usize {
        self.input_port_values.len()
    }

    fn fix_input_port(
        &mut self,
        index: usize,
        value: &dyn AbstractValue,
    ) -> Option<&FixedInputPortValue> {
        self.input_port_values[index] = Some(FixedInputPortValue::new(value.clone_box()));

        self.get_fixed_input_port_value(index)
    }

    fn get_fixed_input_port_value(&self, index: usize) -> Option<&FixedInputPortValue> {
        self.input_port_values[index].as_ref()
    }

    fn get_mutable_fixed_input_port_value(
        &mut self,
        index: usize,
    ) -> Option<&mut FixedInputPortValue> {
        self.input_port_values[index].as_mut()
    }

    fn get_mutable_is_context_base_initialized(&mut self) -> &mut bool {
        &mut self.is_context_base_initialized
    }
}

impl<T: AtlasScalar> Context<T> for LeafContext<T> {
    fn get_time(&self) -> &T {
        &self.time
    }

    fn get_state(&self) -> &State<T> {
        &self.state
    }

    fn get_mutable_state(&mut self) -> &mut State<T> {
        &mut self.state
    }

    fn init_continuous_state(&mut self, continuous_state: ContinuousState<T>) {
        self.set_continuous_state(continuous_state);
    }

    fn num_continuous_states(&self) -> usize {
        self.state.get_continuous_state().size()
    }

    fn get_continuous_state(&self) -> &ContinuousState<T> {
        self.state.get_continuous_state()
    }

    fn get_mutable_continuous_state(&mut self) -> &mut ContinuousState<T> {
        self.state.get_mutable_continuous_state()
    }

    fn get_continuous_state_vector(&self) -> &dyn VectorBase<T, Output = T> {
        self.state.get_continuous_state().get_vector()
    }

    fn as_base(&self) -> &dyn ContextBase {
        self
    }

    fn as_mutable_base(&mut self) -> &mut dyn ContextBase {
        self
    }
}
