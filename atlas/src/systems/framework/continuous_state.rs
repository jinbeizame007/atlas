extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::subvector::Subvector;
use crate::systems::framework::vector_base::VectorBase;

#[derive(Default)]
pub struct ContinuousState<T: AtlasScalar> {
    state: Box<dyn VectorBase<T, Output = T>>,
    num_q: usize,
    num_v: usize,
    num_z: usize,
    system_id: SystemId,
}

impl<T: AtlasScalar> ContinuousState<T> {
    pub fn new(
        state: Box<dyn VectorBase<T, Output = T>>,
        num_q: usize,
        num_v: usize,
        num_z: usize,
    ) -> Self {
        ContinuousState::<T> {
            state,
            num_q,
            num_v,
            num_z,
            system_id: SystemId::new(0),
        }
    }

    pub fn size(&self) -> usize {
        self.state.size()
    }

    pub fn system_id(&self) -> &SystemId {
        &self.system_id
    }

    pub fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    pub fn set_from_vector(&mut self, value: &na::DVector<T>) {
        self.state.set_from_vector(value);
    }

    pub fn vector(&self) -> &dyn VectorBase<T, Output = T> {
        self.state.as_ref()
    }

    pub fn vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T> {
        self.state.as_mut()
    }

    pub fn generalized_position_mut(&mut self) -> Subvector<T> {
        self.state.subvector_mut(0, self.num_q)
    }

    pub fn generalized_velocity_mut(&mut self) -> Subvector<T> {
        self.state.subvector_mut(self.num_q, self.num_v)
    }

    pub fn misc_continuous_state_mut(&mut self) -> Subvector<T> {
        self.state
            .subvector_mut(self.num_q + self.num_v, self.num_z)
    }
}
