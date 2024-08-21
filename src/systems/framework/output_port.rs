use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::context::Context;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::system::System;

pub trait OutputPort<'a, T: Add + PartialEq + Clone + Debug + Zero>: OutputPortBase {
    fn eval<ValueType: Clone + 'static>(&self, context: &dyn Context<T>) -> ValueType;
    fn allocate(&mut self) -> Box<dyn AbstractValue>;
    fn calc(&self, context: &dyn Context<T>, value: &mut dyn AbstractValue);
    fn get_system(&self) -> &'a dyn System<T>;
}
