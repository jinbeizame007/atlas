use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::continuous_state::ContinuousState;

pub trait State<T: AtlasScalar> {
    type CS: ContinuousState<T>;

    fn continuous_state(&self) -> &Self::CS;

    fn continuous_state_mut(&mut self) -> &mut Self::CS;
}
