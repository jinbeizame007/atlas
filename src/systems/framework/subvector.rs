use std::ops::{Index, IndexMut};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::vector_base::VectorBase;

// TODO: implement immutable Subvector
pub struct Subvector<'a, T: AtlasScalar> {
    vector: na::DVectorViewMut<'a, T>,
}

impl<'a, T: AtlasScalar> Subvector<'a, T> {
    pub fn new(vector: na::DVectorViewMut<'a, T>) -> Self {
        Subvector { vector }
    }
}

impl<'a, T: AtlasScalar> Index<usize> for Subvector<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}

impl<'a, T: AtlasScalar> IndexMut<usize> for Subvector<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}

impl<'a, T: AtlasScalar> VectorBase<T> for Subvector<'a, T> {
    fn size(&self) -> usize {
        self.vector.len()
    }

    fn get_at_index(&self, index: usize) -> &T {
        &self.vector[index]
    }

    fn get_mut_at_index(&mut self, index: usize) -> &mut T {
        &mut self.vector[index]
    }

    fn get_mutable_subvector<'b>(&'b mut self, start: usize, shape: usize) -> Subvector<'b, T> {
        Subvector::<'b, T>::new(self.vector.rows_mut(start, shape))
    }

    fn set_at_index(&mut self, index: usize, value: T) {
        self.vector[index] = value;
    }

    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>) {
        for i in 0..self.size() {
            self.vector[i] = value[i].clone();
        }
    }

    fn set_from_vector(&mut self, value: &na::DVector<T>) {
        for i in 0..self.size() {
            self.vector[i] = value[i].clone();
        }
    }

    fn fill(&mut self, value: &T) {
        for i in 0..self.size() {
            self.vector[i] = value.clone();
        }
    }
}
