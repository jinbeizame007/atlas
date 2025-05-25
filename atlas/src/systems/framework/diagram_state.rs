extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::diagram_continuous_state::ContinuousStatePtr;
use crate::systems::framework::leaf_state::LeafState;
use crate::systems::framework::state::State;

use super::diagram_continuous_state::DiagramContinuousState;
use super::leaf_continuous_state::LeafContinuousState;

pub enum StatePtr<T: AtlasScalar> {
    LeafStatePtr(*mut LeafState<T>),
    LeafDiagramPtr(*mut DiagramState<T>),
}

impl<T: AtlasScalar> StatePtr<T> {
    #[allow(dead_code)]
    fn continuous_state_ptr(&self) -> ContinuousStatePtr<T> {
        match self {
            StatePtr::LeafStatePtr(ptr) => unsafe {
                ContinuousStatePtr::LeafContinuousStatePtr(
                    ptr.as_mut().unwrap().continuous_state_mut() as *mut LeafContinuousState<T>,
                )
            },
            StatePtr::LeafDiagramPtr(ptr) => unsafe {
                ContinuousStatePtr::DiagramContinuousStatePtr(
                    ptr.as_mut().unwrap().continuous_state_mut() as *mut DiagramContinuousState<T>,
                )
            },
        }
    }
}

#[derive(Default)]
pub struct DiagramState<T: AtlasScalar> {
    substates: Vec<Option<StatePtr<T>>>,
    is_finalized: bool,
}

impl<T: AtlasScalar> State<T> for DiagramState<T> {
    type CS = DiagramContinuousState<T>;

    fn continuous_state(&self) -> &Self::CS {
        todo!()
    }

    fn continuous_state_mut(&mut self) -> &mut Self::CS {
        todo!()
    }
}

impl<T: AtlasScalar> DiagramState<T> {
    pub fn new(_size: usize) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn num_substates(&self) -> usize {
        self.substates.len()
    }

    pub fn set_substate(&mut self, index: usize, substate: StatePtr<T>) {
        self.substates[index] = Some(substate);
    }

    pub fn substate_ptr(&self, index: usize) -> &StatePtr<T> {
        self.substates[index].as_ref().unwrap()
    }

    pub fn substate_ptr_mut(&mut self, index: usize) -> &mut StatePtr<T> {
        self.substates[index].as_mut().unwrap()
    }

    pub fn finalize(&mut self) {
        self.is_finalized = true;
        assert!(self.substates.iter().all(|x| x.is_some()));
    }
}
