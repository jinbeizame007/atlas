extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::subvector::Subvector;
use crate::systems::framework::vector_base::VectorBase;

pub trait ContinuousState<T: AtlasScalar>: Default {
    fn size(&self) -> usize;
    fn system_id(&self) -> &SystemId;
    fn set_system_id(&mut self, system_id: SystemId);

    fn num_q(&self) -> usize;
    fn num_v(&self) -> usize;
    fn num_z(&self) -> usize;
    fn vector(&self) -> &dyn VectorBase<T, Output = T>;
    fn vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T>;
    fn set_from_vector(&mut self, value: &na::DVector<T>) {
        self.vector_mut().set_from_vector(value);
    }

    fn generalized_position_mut(&mut self) -> Subvector<T> {
        let num_q = self.num_q();
        self.vector_mut().subvector_mut(0, num_q)
    }
    fn generalized_velocity_mut(&mut self) -> Subvector<T> {
        let num_q = self.num_q();
        let num_v = self.num_v();
        self.vector_mut().subvector_mut(num_q, num_v)
    }
    fn misc_continuous_state_mut(&mut self) -> Subvector<T> {
        let num_q = self.num_q();
        let num_v = self.num_v();
        let num_z = self.num_z();
        self.vector_mut().subvector_mut(num_q + num_v, num_z)
    }
}
