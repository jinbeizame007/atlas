use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::state::State;
use crate::systems::framework::vector_base::VectorBase;

pub trait Context<T: AtlasScalar>: ContextBase {
    type S: State<T>;

    fn time(&self) -> &T;
    fn state(&self) -> &Self::S;
    fn state_mut(&mut self) -> &mut Self::S;
    fn init_continuous_state(&mut self, continuous_state: Box<<Self::S as State<T>>::CS>);
    fn num_continuous_states(&self) -> usize;
    fn continuous_state(&self) -> &<Self::S as State<T>>::CS;
    fn continuous_state_mut(&mut self) -> &mut <Self::S as State<T>>::CS;
    #[allow(clippy::boxed_local)]
    fn set_continuous_state(&mut self, continuous_state: Box<<Self::S as State<T>>::CS>) {
        *self.continuous_state_mut() = *continuous_state;
    }
    fn continuous_state_vector(&self) -> &dyn VectorBase<T, Output = T>;
    fn continuous_state_vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T>;
    fn as_base(&self) -> &dyn ContextBase;
    fn as_mutable_base(&mut self) -> &mut dyn ContextBase;
}
