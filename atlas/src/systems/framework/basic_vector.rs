use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::subvector::Subvector;
use crate::systems::framework::vector_base::VectorBase;

#[derive(Clone, Debug)]
pub struct BasicVector<T: AtlasScalar> {
    values: na::DVector<T>,
}

impl<T: AtlasScalar> BasicVector<T> {
    pub fn new(values: na::DVector<T>) -> Self {
        BasicVector::<T> { values }
    }

    pub fn from_vec(values: Vec<T>) -> Self {
        BasicVector::<T>::new(na::DVector::<T>::from_vec(values))
    }

    pub fn zeros(size: usize) -> BasicVector<T> {
        BasicVector::<T>::new(na::DVector::<T>::zeros(size))
    }

    pub fn value(&self) -> &na::DVector<T> {
        &self.values
    }

    pub fn value_mut(&mut self) -> &mut na::DVector<T> {
        &mut self.values
    }

    pub fn set_value(&mut self, value: &na::DVector<T>) {
        self.values = (*value).clone();
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

// NumOps

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

// NumAssignOps

impl<T: AtlasScalar> AddAssign for BasicVector<T> {
    fn add_assign(&mut self, rhs: BasicVector<T>) {
        self.values += rhs.values;
    }
}

impl<T: AtlasScalar> AddAssign<&BasicVector<T>> for BasicVector<T> {
    fn add_assign(&mut self, rhs: &BasicVector<T>) {
        self.values += &rhs.values;
    }
}

impl<T: AtlasScalar> SubAssign for BasicVector<T> {
    fn sub_assign(&mut self, rhs: BasicVector<T>) {
        self.values -= rhs.values;
    }
}

impl<T: AtlasScalar> SubAssign<&BasicVector<T>> for BasicVector<T> {
    fn sub_assign(&mut self, rhs: &BasicVector<T>) {
        self.values -= &rhs.values;
    }
}

impl<T: AtlasScalar> MulAssign for BasicVector<T> {
    fn mul_assign(&mut self, rhs: BasicVector<T>) {
        self.values *= rhs.values;
    }
}

impl<T: AtlasScalar> MulAssign<&BasicVector<T>> for BasicVector<T> {
    fn mul_assign(&mut self, rhs: &BasicVector<T>) {
        self.values *= &rhs.values;
    }
}

// impl<T: AtlasScalar> DivAssign for BasicVector<T> {
//     fn div_assign(&mut self, rhs: BasicVector<T>) {
//         self.values /= rhs.values;
//     }
// }

// impl<T: AtlasScalar> DivAssign<&BasicVector<T>> for BasicVector<T> {
//     fn div_assign(&mut self, rhs: &BasicVector<T>) {
//         self.values /= &rhs.values;
//     }
// }

impl<T: AtlasScalar> VectorBase<T> for BasicVector<T> {
    fn size(&self) -> usize {
        self.values.len()
    }

    fn at_index(&self, index: usize) -> &T {
        &self.values[index]
    }

    fn at_index_mut(&mut self, index: usize) -> &mut T {
        &mut self.values[index]
    }

    fn subvector(&self, start: usize, shape: usize) -> Subvector<T> {
        Subvector::<T>::new(
            &self.values as *const na::DVector<T> as *mut na::DVector<T>,
            start,
            shape,
        )
    }

    fn subvector_mut(&mut self, start: usize, shape: usize) -> Subvector<T> {
        Subvector::<T>::new(&mut self.values, start, shape)
    }

    fn set_at_index(&mut self, index: usize, value: T) {
        self.values[index] = value
    }

    fn set_from(&mut self, value: &dyn VectorBase<T, Output = T>) {
        for i in 0..self.size() {
            self.values[i] = (*value.at_index(i)).clone();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_ops() {
        let a = BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![1.0, 2.0]));
        let b = BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![3.0, 4.0]));

        assert_eq!(
            &a + &b,
            BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![4.0, 6.0]))
        );
        assert_eq!(
            &a - &b,
            BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![-2.0, -2.0]))
        );
        // assert_eq!(
        //     &a * &b,
        //     BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![3.0, 8.0]))
        // );
        // assert_eq!(
        //     &a / &b,
        //     BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![1.0 / 3.0, 0.5]))
        // );
    }

    #[test]
    fn test_num_assign_ops() {
        let mut a = BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![1.0, 2.0]));
        let b = BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![3.0, 4.0]));

        a += &b;
        assert_eq!(
            a,
            BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![4.0, 6.0]))
        );

        a = BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![1.0, 2.0]));
        a -= &b;
        assert_eq!(
            a,
            BasicVector::<f64>::new(na::DVector::<f64>::from_vec(vec![-2.0, -2.0]))
        );
    }
}
