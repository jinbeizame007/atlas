use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, Index, IndexMut};

extern crate nalgebra as na;

pub trait VectorBase<T: Add + PartialEq + Clone + Debug, Output = T>:
    Index<usize> + IndexMut<usize>
{
    fn size(&self) -> usize;
    fn copy_to_vector(&self) -> na::DVector<T>;
    fn get_at_index(&self, index: usize) -> &T;
    fn get_mut_at_index(&mut self, index: usize) -> &mut T;
    fn set_at_index(&mut self, index: usize, value: T);
    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>);
    fn set_from_vector(&mut self, value: &na::DVector<T>);
    fn fill(&mut self, value: &T);
}
