use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct Identifier<Tag: Default> {
    value: usize,
    _marker: PhantomData<Tag>,
}

impl<Tag: Default> Identifier<Tag> {
    pub fn new(value: usize) -> Self {
        Identifier::<Tag> {
            value,
            _marker: PhantomData::<Tag>,
        }
    }
    pub fn get_value(&self) -> &usize {
        &self.value
    }
}
