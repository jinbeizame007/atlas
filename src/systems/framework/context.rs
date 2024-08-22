use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::state::State;
use crate::systems::framework::vector_base::VectorBase;

pub trait Context<T: Add + PartialEq + Clone + Debug + Zero>: ContextBase {
    fn get_time(&self) -> &T;
    fn get_state(&self) -> &State<T>;
    fn get_mutable_state(&mut self) -> &mut State<T>;
    fn num_continuous_states(&self) -> usize;
    fn get_continuous_state(&self) -> &ContinuousState<T>;
    fn get_continuous_state_vector(&self) -> &dyn VectorBase<T, Output = T>;
    fn as_base(&self) -> &dyn ContextBase;
    fn as_mutable_base(&mut self) -> &mut dyn ContextBase;
}
