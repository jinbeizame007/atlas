use std::marker::PhantomData;

#[derive(Clone)]
pub struct Identifier<Tag> {
    value: usize,
    _marker: PhantomData<Tag>,
}

impl<Tag> Identifier<Tag> {
    pub fn get_value(&self) -> &usize {
        &self.value
    }
}
