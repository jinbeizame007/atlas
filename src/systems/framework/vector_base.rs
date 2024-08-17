use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, Index, IndexMut};

use num_traits::identities::Zero;
extern crate nalgebra as na;

use crate::systems::framework::subvector::Subvector;

pub trait VectorBase<T: Add + PartialEq + Clone + Debug + Zero, Output = T>:
    Index<usize> + IndexMut<usize>
{
    fn size(&self) -> usize;
    fn get_at_index(&self, index: usize) -> &T;
    fn get_mut_at_index(&mut self, index: usize) -> &mut T;
    fn get_mutable_subvector(&mut self, start: usize, shape: usize) -> Subvector<T>;
    fn set_at_index(&mut self, index: usize, value: T);
    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>);
    fn set_from_vector(&mut self, value: &na::DVector<T>);
    fn fill(&mut self, value: &T);
}
