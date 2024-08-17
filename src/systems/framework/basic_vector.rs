use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, Index, IndexMut};

// use nalgebra::zero;
use num_traits::identities::Zero;
extern crate nalgebra as na;

use crate::systems::framework::vector_base::VectorBase;

pub struct BasicVector<T: Add + PartialEq + Clone + Debug + Zero> {
    values: na::DVector<T>,
}

impl<T: Add + PartialEq + Clone + Debug + Zero> BasicVector<T> {
    pub fn new(values: na::DVector<T>) -> Self {
        BasicVector::<T> { values }
    }

    pub fn get_value(&self) -> &na::DVector<T> {
        &self.values
    }

    pub fn get_mutable_value(&mut self) -> &mut na::DVector<T> {
        &mut self.values
    }

    pub fn set_value(&mut self, value: &na::DVector<T>) {
        self.values = (*value).clone();
    }
}

impl<T: Add + PartialEq + Clone + Debug + Zero> Index<usize> for BasicVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T: Add + PartialEq + Clone + Debug + Zero> IndexMut<usize> for BasicVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T: Add + PartialEq + Clone + Debug + Zero> VectorBase<T> for BasicVector<T> {
    fn size(&self) -> usize {
        self.values.len()
    }

    fn copy_to_vector(&self) -> nalgebra::DVector<T> {
        self.values.clone()
    }

    fn get_at_index(&self, index: usize) -> &T {
        &self.values[index]
    }

    fn get_mut_at_index(&mut self, index: usize) -> &mut T {
        &mut self.values[index]
    }

    fn set_at_index(&mut self, index: usize, value: T) {
        self.values[index] = value
    }

    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>) {
        for i in 0..self.size() {
            self.values[i] = (*value.get_at_index(i)).clone();
        }
    }

    fn set_from_vector(&mut self, value: &na::DVector<T>) {
        self.values = value.clone()
    }

    fn fill(&mut self, value: &T) {
        for i in 0..self.values.len() {
            self.values[i] = value.clone();
        }
    }
}
