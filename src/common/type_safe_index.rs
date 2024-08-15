use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct TypeSafeIndex<Tag> {
    index: i64,
    _marker: PhantomData<Tag>,
}

impl<Tag> TypeSafeIndex<Tag> {
    pub fn new(index: i64) -> Self {
        TypeSafeIndex {
            index,
            _marker: PhantomData,
        }
    }

    pub fn value(&self) -> i64 {
        self.index
    }
}

impl<Tag> PartialEq for TypeSafeIndex<Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
