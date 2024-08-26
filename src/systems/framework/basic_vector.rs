use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, Div, Index, IndexMut, Mul, Rem, Sub};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::subvector::Subvector;
use crate::systems::framework::vector_base::VectorBase;

#[derive(Clone)]
pub struct BasicVector<T: AtlasScalar> {
    values: na::DVector<T>,
}

impl<T: AtlasScalar> BasicVector<T> {
    pub fn new(values: na::DVector<T>) -> Self {
        BasicVector::<T> { values }
    }

    pub fn zeros(size: usize) -> BasicVector<T> {
        BasicVector::<T>::new(na::DVector::<T>::zeros(size))
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

impl<T: AtlasScalar> Debug for BasicVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values)
    }
}

impl<T: AtlasScalar> Index<usize> for BasicVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T: AtlasScalar> IndexMut<usize> for BasicVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T: AtlasScalar> PartialEq for BasicVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T: AtlasScalar> Add<&BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn add(self, rhs: &BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values.clone() + rhs.values.clone())
    }
}

impl<T: AtlasScalar> Add<BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn add(self, rhs: BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values.clone() + rhs.values.clone())
    }
}

impl<T: AtlasScalar> Add<&BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn add(self, rhs: &BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values + rhs.values.clone())
    }
}

impl<T: AtlasScalar> Add<BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn add(self, rhs: BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values + rhs.values)
    }
}

impl<T: AtlasScalar> Sub<&BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn sub(self, rhs: &BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values.clone() - rhs.values.clone())
    }
}

impl<T: AtlasScalar> Sub<BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn sub(self, rhs: BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values - rhs.values)
    }
}

impl<T: AtlasScalar> Sub<BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn sub(self, rhs: BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values.clone() - rhs.values)
    }
}

impl<T: AtlasScalar> Mul<&BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn mul(self, rhs: &BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values.clone() * rhs.values.clone())
    }
}

impl<T: AtlasScalar> Mul<BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn mul(self, rhs: BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values.clone() * rhs.values)
    }
}

impl<T: AtlasScalar> Mul<&BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn mul(self, rhs: &BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values * rhs.values.clone())
    }
}

impl<T: AtlasScalar> Mul<BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn mul(self, rhs: BasicVector<T>) -> Self::Output {
        BasicVector::<T>::new(self.values * rhs.values)
    }
}

impl<T: AtlasScalar> Div<&BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn div(self, rhs: &BasicVector<T>) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        let result = self.clone().values.zip_map(&rhs.values, |a, b| a / b);
        BasicVector::<T>::new(result)
    }
}

impl<T: AtlasScalar> Div<BasicVector<T>> for &BasicVector<T> {
    type Output = BasicVector<T>;

    fn div(self, rhs: BasicVector<T>) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        let result = self.clone().values.zip_map(&rhs.values, |a, b| a / b);
        BasicVector::<T>::new(result)
    }
}

impl<T: AtlasScalar> Div<&BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn div(self, rhs: &BasicVector<T>) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        let result = self.values.zip_map(&rhs.values, |a, b| a / b);
        BasicVector::<T>::new(result)
    }
}

impl<T: AtlasScalar> Div<BasicVector<T>> for BasicVector<T> {
    type Output = BasicVector<T>;

    fn div(self, rhs: BasicVector<T>) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        let result = self.values.zip_map(&rhs.values, |a, b| a / b);
        BasicVector::<T>::new(result)
    }
}

impl<T: AtlasScalar> VectorBase<T> for BasicVector<T> {
    fn size(&self) -> usize {
        self.values.len()
    }

    fn get_at_index(&self, index: usize) -> &T {
        &self.values[index]
    }

    fn get_mut_at_index(&mut self, index: usize) -> &mut T {
        &mut self.values[index]
    }

    fn get_mutable_subvector<'a>(&'a mut self, start: usize, shape: usize) -> Subvector<'a, T> {
        Subvector::<'a, T>::new(self.get_mutable_value().rows_mut(start, shape))
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
