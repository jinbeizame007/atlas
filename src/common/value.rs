use std::any::Any;

pub trait AbstractValue {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn set_from(&mut self, abstract_value: &dyn AbstractValue);
    fn clone_box(&self) -> Box<dyn AbstractValue>;
}

impl Default for Box<dyn AbstractValue> {
    fn default() -> Self {
        Box::new(Value::<i64>::new(0))
    }
}

#[derive(Clone)]
pub struct Value<T: Clone> {
    value: T,
}

impl<T: 'static + Clone> Value<T> {
    pub fn new(value: T) -> Self {
        Value { value }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: &T) {
        self.value = value.clone();
    }
}

impl<T: 'static + Clone> AbstractValue for Value<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_from(&mut self, abstract_value: &dyn AbstractValue) {
        let value = abstract_value
            .as_any()
            .downcast_ref::<Value<T>>()
            .unwrap()
            .get_value();
        self.set_value(value);
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

    #[test]
    fn test_set_from() {
        let value1 = Value::<i64>::new(11);
        let mut abstract_value1 = Box::new(value1) as Box<dyn AbstractValue>;
        let value2 = Value::<i64>::new(22);
        let abstract_value2 = Box::new(value2) as Box<dyn AbstractValue>;
        abstract_value1.set_from(abstract_value2.as_ref());
        assert_eq!(
            22,
            *abstract_value1
                .as_any()
                .downcast_ref::<Value<i64>>()
                .unwrap()
                .get_value()
        )
    }
}
