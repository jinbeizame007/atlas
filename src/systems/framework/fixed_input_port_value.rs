use crate::common::value::AbstractValue;

pub struct FixedInputPortValue {
    value: Box<dyn AbstractValue>,
}

impl FixedInputPortValue {
    pub fn get_value(&mut self) -> &mut dyn AbstractValue {
        self.value.as_mut()
    }
}
