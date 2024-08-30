use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::continuous_state::ContinuousState;

#[derive(Default)]
pub struct State<T: AtlasScalar> {
    continuous_state: ContinuousState<T>,
}

impl<T: AtlasScalar> State<T> {
    pub fn new(continuous_state: ContinuousState<T>) -> Self {
        State::<T> { continuous_state }
    }

    pub fn continuous_state(&self) -> &ContinuousState<T> {
        &self.continuous_state
    }

    pub fn continuous_state_mut(&mut self) -> &mut ContinuousState<T> {
        &mut self.continuous_state
    }
}
