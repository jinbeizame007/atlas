use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;
use std::sync::{Arc, Mutex};

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::cache::Cache;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::state::State;
use crate::systems::framework::vector_base::VectorBase;

// #[derive(Default)]
pub struct LeafContext<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> {
    system_id: SystemId,
    parent: Arc<Mutex<dyn ContextBase>>,
    cache: Cache,
    time: T,
    state: State<T>,
    input_port_values: Vec<Option<FixedInputPortValue>>,
    is_context_base_initialized: bool,
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> ContextBase for LeafContext<T> {
    fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    fn get_system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn get_parent_base(&self) -> Arc<Mutex<dyn ContextBase>> {
        self.parent.clone()
    }

    fn get_cache(&self) -> &Cache {
        &self.cache
    }

    fn get_mutable_cache(&mut self) -> &mut Cache {
        &mut self.cache
    }

    fn num_input_ports(&self) -> usize {
        self.input_port_values.len()
    }

    fn fix_input_port(&mut self, index: usize, value: &dyn AbstractValue) -> &FixedInputPortValue {
        self.input_port_values[index] = Some(FixedInputPortValue::new(value.clone_box()));

        &self.input_port_values[index].unwrap()
    }

    fn get_fixed_input_port_value(&self, index: usize) -> &Option<FixedInputPortValue> {
        &self.input_port_values[index]
    }

    fn get_mutable_fixed_input_port_value(
        &mut self,
        index: usize,
    ) -> &mut Option<FixedInputPortValue> {
        &mut self.input_port_values[index]
    }

    fn get_mutable_is_context_base_initialized(&mut self) -> &mut bool {
        &mut self.is_context_base_initialized
    }
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> Context<T> for LeafContext<T> {
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
