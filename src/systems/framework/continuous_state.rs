use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::subvector::Subvector;
use crate::systems::framework::vector_base::VectorBase;

#[derive(Default)]
pub struct ContinuousState<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> {
    state: Box<dyn VectorBase<T, Output = T>>,
    num_q: usize,
    num_v: usize,
    num_z: usize,
    system_id: SystemId,
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> ContinuousState<T> {
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

    pub fn get_system_id(&self) -> &SystemId {
        &self.system_id
    }

    pub fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    pub fn set_from_vector(&mut self, value: &na::DVector<T>) {
        self.state.set_from_vector(value);
    }

    pub fn get_vector(&self) -> &dyn VectorBase<T, Output = T> {
        self.state.as_ref()
    }

    pub fn get_mutable_vector(&mut self) -> &mut dyn VectorBase<T, Output = T> {
        self.state.as_mut()
    }

    pub fn get_mutable_generalized_position(&mut self) -> Subvector<T> {
        self.state.get_mutable_subvector(0, self.num_q)
    }

    pub fn get_mutable_generalized_velocity(&mut self) -> Subvector<T> {
        self.state.get_mutable_subvector(self.num_q, self.num_v)
    }

    pub fn get_mutable_misc_continuous_state(&mut self) -> Subvector<T> {
        self.state
            .get_mutable_subvector(self.num_q + self.num_v, self.num_z)
    }
}
