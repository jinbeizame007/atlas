use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::context::Context;
use crate::systems::framework::output_port_base::OutputPortBase;

pub trait OutputPort<T: Add + PartialEq + Clone + Debug + Zero>: OutputPortBase {
    // fn eval<ValueType: Clone + 'static>(&self, context: &mut dyn Context<T>) -> ValueType;
    fn eval_abstract(&self, context: &mut dyn Context<T>) -> Box<dyn AbstractValue>;
    fn allocate(&mut self) -> Box<dyn AbstractValue>;
    fn calc(&self, context: &mut dyn Context<T>, value: &mut dyn AbstractValue);
}
