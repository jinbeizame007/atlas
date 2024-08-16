use std::any::Any;

pub trait AbstractValue {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn AbstractValue>;
}

#[derive(Clone)]
pub struct Value<T: Clone> {
    value: T,
}

impl<T: 'static + Clone> Value<T> {
    pub fn new(value: T) -> Self {
        Value { value }
    }
}

impl<T: 'static + Clone> AbstractValue for Value<T> {
    fn as_any(&self) -> &dyn Any {
        &self.value
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut self.value
    }

    fn clone_box(&self) -> Box<dyn AbstractValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AbstractValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let value = Value::<i64>::new(22);
        assert_eq!(22, *value.as_any().downcast_ref::<i64>().unwrap());
    }
}
