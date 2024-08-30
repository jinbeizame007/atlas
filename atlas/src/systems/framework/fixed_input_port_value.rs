use crate::common::value::AbstractValue;

pub struct FixedInputPortValue {
    value: Box<dyn AbstractValue>,
}

impl FixedInputPortValue {
    pub fn new(value: Box<dyn AbstractValue>) -> Self {
        FixedInputPortValue { value }
    }

    pub fn value(&self) -> &dyn AbstractValue {
        self.value.as_ref()
    }

    pub fn value_mut(&mut self) -> &mut dyn AbstractValue {
        self.value.as_mut()
    }
}
