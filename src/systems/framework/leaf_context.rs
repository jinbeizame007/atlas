use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

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

pub struct LeafContext<T: Add + PartialEq + Clone + Debug + Zero> {
    system_id: SystemId,
    cache: Cache,
    time: T,
    state: State<T>,
    input_port_values: Vec<FixedInputPortValue>,
}

impl<T: Add + PartialEq + Clone + Debug + Zero> ContextBase for LeafContext<T> {
    fn get_system_id(&self) -> &SystemId {
        &self.system_id
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
        self.input_port_values[index] = FixedInputPortValue::new(value.clone_box());
        &self.input_port_values[index]
    }

    fn get_fixed_input_port_value(&self, index: usize) -> &FixedInputPortValue {
        &self.input_port_values[index]
    }

    fn get_mutable_fixed_input_port_value(&mut self, index: usize) -> &mut FixedInputPortValue {
        &mut self.input_port_values[index]
    }
}

impl<T: Add + PartialEq + Clone + Debug + Zero> Context<T> for LeafContext<T> {
    fn get_time(&self) -> &T {
        &self.time
    }

    fn get_state(&self) -> &State<T> {
        &self.state
    }

    fn num_continuous_states(&self) -> usize {
        self.state.get_continuous_state().size()
    }

    fn get_continuous_state(&self) -> &ContinuousState<T> {
        self.state.get_continuous_state()
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
