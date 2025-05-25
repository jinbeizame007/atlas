extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::SystemId;
use crate::systems::framework::leaf_continuous_state::LeafContinuousState;
use crate::systems::framework::vector_base::VectorBase;

pub enum ContinuousStatePtr<T: AtlasScalar> {
    LeafContinuousStatePtr(*mut LeafContinuousState<T>),
    DiagramContinuousStatePtr(*mut DiagramContinuousState<T>),
}

impl<T: AtlasScalar> ContinuousStatePtr<T> {
    #[allow(dead_code)]
    fn num_q(&self) -> usize {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().num_q()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().num_q()
            },
        }
    }

    #[allow(dead_code)]
    fn num_v(&self) -> usize {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().num_v()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().num_v()
            },
        }
    }

    #[allow(dead_code)]
    fn num_z(&self) -> usize {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().num_z()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().num_z()
            },
        }
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().size()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().size()
            },
        }
    }

    #[allow(dead_code)]
    fn system_id(&self) -> &SystemId {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().system_id()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().system_id()
            },
        }
    }

    #[allow(dead_code)]
    fn set_system_id(&mut self, system_id: SystemId) {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_mut().unwrap().set_system_id(system_id)
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_mut().unwrap().set_system_id(system_id)
            },
        }
    }

    #[allow(dead_code)]
    fn set_from_vector(&mut self, value: &na::DVector<T>) {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_mut().unwrap().set_from_vector(value)
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_mut().unwrap().set_from_vector(value)
            },
        }
    }

    #[allow(dead_code)]
    fn vector(&self) -> &dyn VectorBase<T, Output = T> {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().vector()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_ref().unwrap().vector()
            },
        }
    }

    #[allow(dead_code)]
    fn vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T> {
        match self {
            ContinuousStatePtr::LeafContinuousStatePtr(ptr) => unsafe {
                ptr.as_mut().unwrap().vector_mut()
            },
            ContinuousStatePtr::DiagramContinuousStatePtr(ptr) => unsafe {
                ptr.as_mut().unwrap().vector_mut()
            },
        }
    }
}

#[derive(Default)]
pub struct DiagramContinuousState<T: AtlasScalar> {
    #[allow(dead_code)]
    state: Box<dyn VectorBase<T, Output = T>>,
    #[allow(dead_code)]
    substates: Vec<ContinuousStatePtr<T>>,
    num_q: usize,
    num_v: usize,
    num_z: usize,
    system_id: SystemId,
}

impl<T: AtlasScalar> DiagramContinuousState<T> {
    pub fn new(_substates: Vec<ContinuousStatePtr<T>>) -> Self {
        todo!()
        // let state = Box::new(BasicVector
        // DiagramContinuousState::<T> {
        //     substates,
        //     state:
        //     num_q: 0,
        //     num_v: 0,
        //     num_z: 0,
        //     system_id: SystemId::default(),
        // }
    }

    #[allow(dead_code)]
    fn num_substates(&self) -> usize {
        self.substates.len()
    }

    #[allow(dead_code)]
    fn substate(&self, index: usize) -> &ContinuousStatePtr<T> {
        &self.substates[index]
    }

    #[allow(dead_code)]
    fn substate_mut(&mut self, index: usize) -> &mut ContinuousStatePtr<T> {
        &mut self.substates[index]
    }
}

impl<T: AtlasScalar> ContinuousState<T> for DiagramContinuousState<T> {
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
        self.num_q + self.num_v + self.num_z
    }

    fn system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    fn set_from_vector(&mut self, _value: &na::DVector<T>) {
        todo!()
    }

    fn vector(&self) -> &dyn VectorBase<T, Output = T> {
        todo!()
    }

    fn vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T> {
        todo!()
    }
}
