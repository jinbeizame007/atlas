use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::leaf_continuous_state::LeafContinuousState;
use crate::systems::framework::state::State;

#[derive(Default)]
pub struct LeafState<T: AtlasScalar> {
    continuous_state: Box<LeafContinuousState<T>>,
}

impl<T: AtlasScalar> LeafState<T> {
    pub fn new(continuous_state: Box<LeafContinuousState<T>>) -> Self {
        Self { continuous_state }
    }
}

impl<T: AtlasScalar> State<T> for LeafState<T> {
    type CS = LeafContinuousState<T>;

    fn continuous_state(&self) -> &Self::CS {
        self.continuous_state.as_ref()
    }

    fn continuous_state_mut(&mut self) -> &mut Self::CS {
        self.continuous_state.as_mut()
    }
}
