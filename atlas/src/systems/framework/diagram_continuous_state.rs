extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::SystemId;

#[derive(Default)]
pub struct DiagramContinuousState<T: AtlasScalar> {
    substates: Vec<*mut ContinuousState<T>>,
    system_id: SystemId,
}

impl<T: AtlasScalar> DiagramContinuousState<T> {
    pub fn new(substates: Vec<*mut ContinuousState<T>>) -> Self {
        DiagramContinuousState::<T> {
            substates,
            system_id: SystemId::default(),
        }
    }

    pub fn num_substates(&self) -> usize {
        self.substates.len()
    }

    pub fn substate(&self, index: usize) -> &ContinuousState<T> {
        unsafe { &*self.substates[index] }
    }

    pub fn substate_mut(&mut self, index: usize) -> &mut ContinuousState<T> {
        unsafe { &mut *self.substates[index] }
    }

    pub fn system_id(&self) -> &SystemId {
        &self.system_id
    }

    pub fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }
}
