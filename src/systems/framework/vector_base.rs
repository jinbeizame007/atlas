use std::ops::{Index, IndexMut};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::subvector::Subvector;

pub trait VectorBase<T: AtlasScalar, Output = T>: Index<usize> + IndexMut<usize> {
    fn size(&self) -> usize;
    fn at_index(&self, index: usize) -> &T;
    fn at_index_mut(&mut self, index: usize) -> &mut T;
    fn subvector_mut(&mut self, start: usize, shape: usize) -> Subvector<T>;
    fn set_at_index(&mut self, index: usize, value: T);
    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>);
    fn set_from_vector(&mut self, value: &na::DVector<T>);
    fn fill(&mut self, value: &T);
}

impl<T: AtlasScalar> Default for Box<dyn VectorBase<T, Output = T>> {
    fn default() -> Self {
        Box::new(BasicVector::new(na::DVector::<T>::from_element(
            1,
            T::default(),
        )))
    }
}
