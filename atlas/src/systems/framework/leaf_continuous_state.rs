extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::vector_base::VectorBase;

#[derive(Default)]
pub struct LeafContinuousState<T: AtlasScalar> {
    state: Box<dyn VectorBase<T, Output = T>>,
    num_q: usize,
    num_v: usize,
    num_z: usize,
    system_id: SystemId,
}

impl<T: AtlasScalar> LeafContinuousState<T> {
    pub fn new(
        state: Box<dyn VectorBase<T, Output = T>>,
        num_q: usize,
        num_v: usize,
        num_z: usize,
    ) -> Self {
        LeafContinuousState::<T> {
            state,
            num_q,
            num_v,
            num_z,
            system_id: SystemId::new(0),
        }
    }
}

impl<T: AtlasScalar> ContinuousState<T> for LeafContinuousState<T> {
    fn num_q(&self) -> usize {
        self.num_q
    }

    fn num_v(&self) -> usize {
        self.num_v
    }

    fn num_z(&self) -> usize {
        self.num_z
    }

    fn size(&self) -> usize {
        self.state.size()
    }

    fn system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    fn set_from_vector(&mut self, value: &na::DVector<T>) {
        self.state.set_from_vector(value);
    }

    fn vector(&self) -> &dyn VectorBase<T, Output = T> {
        self.state.as_ref()
    }

    fn vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T> {
        self.state.as_mut()
    }
}
