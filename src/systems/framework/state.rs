use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::systems::framework::continuous_state::ContinuousState;

pub struct State<T: Add + PartialEq + Clone + Debug + Zero + 'static> {
    continuous_state: ContinuousState<T>,
}

impl<T: Add + PartialEq + Clone + Debug + Zero> State<T> {
    pub fn new(continuous_state: ContinuousState<T>) -> Self {
        State::<T> { continuous_state }
    }

    pub fn get_continuous_state(&self) -> &ContinuousState<T> {
        &self.continuous_state
    }

    pub fn get_mutable_continuous_state(&mut self) -> &mut ContinuousState<T> {
        &mut self.continuous_state
    }
}
