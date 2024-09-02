use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::state::State;

#[derive(Default)]
pub struct DiagramState<T: AtlasScalar> {
    substates: Vec<Option<*mut State<T>>>,
    is_finalized: bool,
}

impl<T: AtlasScalar> DiagramState<T> {
    pub fn new(size: usize) -> Self {
        Self {
            substates: vec![None; size],
            is_finalized: false,
        }
    }

    fn num_substates(&self) -> usize {
        self.substates.len()
    }

    pub fn set_substate(&mut self, index: usize, substate: &mut State<T>) {
        self.substates[index] = Some(substate as *mut State<T>);
    }

    pub fn substate(&self, index: usize) -> &State<T> {
        unsafe { &*self.substates[index].unwrap() }
    }

    pub fn substate_mut(&mut self, index: usize) -> &mut State<T> {
        unsafe { &mut *self.substates[index].unwrap() }
    }

    pub fn finalize(&mut self) {
        self.is_finalized = true;
        assert!(self.substates.iter().all(|x| x.is_some()));
    }
}
