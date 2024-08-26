use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::state::State;
use crate::systems::framework::vector_base::VectorBase;

pub trait Context<T: AtlasScalar>: ContextBase {
    fn get_time(&self) -> &T;
    fn get_state(&self) -> &State<T>;
    fn get_mutable_state(&mut self) -> &mut State<T>;
    fn init_continuous_state(&mut self, continuous_state: ContinuousState<T>);
    fn num_continuous_states(&self) -> usize;
    fn get_continuous_state(&self) -> &ContinuousState<T>;
    fn get_mutable_continuous_state(&mut self) -> &mut ContinuousState<T>;
    fn set_continuous_state(&mut self, continuous_state: ContinuousState<T>) {
        *self.get_mutable_continuous_state() = continuous_state;
    }
    fn get_continuous_state_vector(&self) -> &dyn VectorBase<T, Output = T>;
    fn as_base(&self) -> &dyn ContextBase;
    fn as_mutable_base(&mut self) -> &mut dyn ContextBase;
}
