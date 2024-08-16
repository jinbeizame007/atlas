use std::marker::PhantomData;
use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct TypeSafeIndex<Tag> {
    index: usize,
    _marker: PhantomData<Tag>,
}

impl<Tag> TypeSafeIndex<Tag> {
    pub fn new(index: usize) -> Self {
        TypeSafeIndex {
            index,
            _marker: PhantomData,
        }
    }

    pub fn value(&self) -> usize {
        self.index
    }
}

impl<Tag, T> Index<TypeSafeIndex<Tag>> for Vec<T> {
    type Output = T;

    fn index(&self, index: TypeSafeIndex<Tag>) -> &Self::Output {
        &self[index.value()]
    }
}

impl<Tag, T> Index<&TypeSafeIndex<Tag>> for Vec<T> {
    type Output = T;

    fn index(&self, index: &TypeSafeIndex<Tag>) -> &Self::Output {
        &self[index.value()]
    }
}

impl<Tag, T> IndexMut<TypeSafeIndex<Tag>> for Vec<T> {
    fn index_mut(&mut self, index: TypeSafeIndex<Tag>) -> &mut Self::Output {
        &mut self[index.value()]
    }
}

impl<Tag, T> IndexMut<&TypeSafeIndex<Tag>> for Vec<T> {
    fn index_mut(&mut self, index: &TypeSafeIndex<Tag>) -> &mut Self::Output {
        &mut self[index.value()]
    }
}

impl<Tag> From<usize> for TypeSafeIndex<Tag> {
    fn from(value: usize) -> Self {
        TypeSafeIndex::<Tag>::new(value)
    }
}

impl<Tag> From<TypeSafeIndex<Tag>> for usize {
    fn from(value: TypeSafeIndex<Tag>) -> Self {
        value.value()
    }
}

impl<Tag> PartialEq for TypeSafeIndex<Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<Tag> PartialEq<usize> for TypeSafeIndex<Tag> {
    fn eq(&self, other: &usize) -> bool {
        self.index == *other
    }
}

impl<Tag> PartialOrd<usize> for TypeSafeIndex<Tag> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(other)
    }
}

impl<Tag> Add<usize> for TypeSafeIndex<Tag> {
    type Output = TypeSafeIndex<Tag>;

    fn add(self, other: usize) -> TypeSafeIndex<Tag> {
        TypeSafeIndex::<Tag>::new(self.index + other)
    }
}
