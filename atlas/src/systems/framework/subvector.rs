use std::ops::{Index, IndexMut};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::vector_base::VectorBase;

pub struct Subvector<T: AtlasScalar> {
    vector: *mut na::DVector<T>,
    first_index: usize,
    num_elements: usize,
}

impl<T: AtlasScalar> Subvector<T> {
    pub fn new(vector: *mut na::DVector<T>, first_index: usize, num_elements: usize) -> Self {
        Subvector {
            vector,
            first_index,
            num_elements,
        }
    }

    pub fn value(&self) -> na::DVectorView<T> {
        let vector = unsafe { &*self.vector };
        vector.rows(self.first_index, self.num_elements)
    }

    pub fn value_mut(&mut self) -> na::DVectorViewMut<T> {
        let vector = unsafe { &mut *self.vector };
        vector.rows_mut(self.first_index, self.num_elements)
    }
}

impl<T: AtlasScalar> Index<usize> for Subvector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let vector = unsafe { &*self.vector };
        &vector[self.first_index + index]
    }
}

impl<T: AtlasScalar> IndexMut<usize> for Subvector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let vector = unsafe { &mut *self.vector };
        &mut vector[self.first_index + index]
    }
}

impl<T: AtlasScalar> VectorBase<T> for Subvector<T> {
    fn size(&self) -> usize {
        self.num_elements
    }

    fn at_index(&self, index: usize) -> &T {
        let vector = unsafe { &*self.vector };
        &vector[self.first_index + index]
    }

    fn at_index_mut(&mut self, index: usize) -> &mut T {
        let vector = unsafe { &mut *self.vector };
        &mut vector[self.first_index + index]
    }

    fn subvector(&self, start: usize, shape: usize) -> Subvector<T> {
        Subvector::<T>::new(self.vector, self.first_index + start, shape)
    }

    fn subvector_mut(&mut self, start: usize, shape: usize) -> Subvector<T> {
        Subvector::<T>::new(self.vector, self.first_index + start, shape)
    }

    fn set_at_index(&mut self, index: usize, value: T) {
        let vector = unsafe { &mut *self.vector };
        vector[self.first_index + index] = value;
    }

    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>) {
        for i in 0..self.size() {
            let vector = unsafe { &mut *self.vector };
            vector[self.first_index + i] = value[i].clone();
        }
    }

    fn set_from_vector(&mut self, value: &na::DVector<T>) {
        let vector = unsafe { &mut *self.vector };
        for i in 0..self.size() {
            vector[self.first_index + i] = value[i].clone();
        }
    }

    fn fill(&mut self, value: &T) {
        let vector = unsafe { &mut *self.vector };
        for i in 0..self.size() {
            vector[self.first_index + i] = value.clone();
        }
    }
}
