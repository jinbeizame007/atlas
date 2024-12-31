use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;

pub trait AbstractValue: Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn set_from(&mut self, abstract_value: &dyn AbstractValue);
    fn clone_box(&self) -> Box<dyn AbstractValue>;
    fn type_id(&self) -> TypeId;
}

impl Default for Box<dyn AbstractValue> {
    fn default() -> Self {
        Box::new(Value::<i64>::new(0))
    }
}

impl dyn AbstractValue {
    pub fn get_value<T: 'static + Clone + Debug>(&self) -> &T {
        self.cast::<T>().value()
    }

    pub fn get_value_mut<T: 'static + Clone + Debug>(&mut self) -> &mut T {
        self.cast_mut::<T>().value_mut()
    }

    pub fn set_value<T: 'static + Clone + Debug>(&mut self, value: T) {
        self.cast_mut::<T>().set_value(value);
    }

    fn cast<T: 'static + Clone + Debug>(&self) -> &Value<T> {
        self.as_any().downcast_ref::<Value<T>>().unwrap()
    }

    fn cast_mut<T: 'static + Clone + Debug>(&mut self) -> &mut Value<T> {
        self.as_any_mut().downcast_mut::<Value<T>>().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Value<T: Clone + Debug> {
    value: T,
}

impl<T: 'static + Clone + Debug> Value<T> {
    pub fn new(value: T) -> Self {
        Value { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T: 'static + Clone + Debug> AbstractValue for Value<T> {
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
            .value();
        self.set_value(value.clone());
    }

    fn clone_box(&self) -> Box<dyn AbstractValue> {
        Box::new(self.clone())
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
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
        assert_eq!(
            22,
            *value.as_any().downcast_ref::<Value<i64>>().unwrap().value()
        );
    }

    #[test]
    fn test_set_from() {
        let value1 = Value::<i64>::new(11);
        let mut abstract_value1 = Box::new(value1) as Box<dyn AbstractValue>;
        let value2 = Value::<i64>::new(22);
        let abstract_value2 = Box::new(value2) as Box<dyn AbstractValue>;
        abstract_value1.set_from(abstract_value2.as_ref());
        assert_eq!(22, *abstract_value1.get_value::<i64>());

        abstract_value1.set_value(33_i64);
        assert_eq!(33, *abstract_value1.get_value::<i64>());
    }
}
