use crate::common::value::AbstractValue;
use crate::systems::framework::context_base::ContextBase;

pub type AllocateCallback = dyn Fn() -> Box<dyn AbstractValue>;
pub type CalcCallback = dyn Fn(&dyn ContextBase, &mut dyn AbstractValue);

pub struct ValueProducer {
    allocate_: Box<AllocateCallback>,
    calc_: Box<CalcCallback>,
}

impl ValueProducer {
    pub fn new(allocate_: Box<AllocateCallback>, calc_: Box<CalcCallback>) -> Self {
        ValueProducer { allocate_, calc_ }
    }

    pub fn allocate(&self) -> Box<dyn AbstractValue> {
        (self.allocate_)()
    }

    pub fn calc(&self, context: &dyn ContextBase, value: &mut dyn AbstractValue) {
        (self.calc_)(context, value)
    }
}
