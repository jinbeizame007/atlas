use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::context::Context;
use crate::systems::framework::output_port_base::OutputPortBase;

pub trait OutputPort<T: AtlasScalar>: OutputPortBase {
    // fn eval<ValueType: Clone + 'static>(&self, context: &mut dyn Context<T>) -> ValueType;
    fn eval_abstract(&self, context: &mut dyn Context<T>) -> Box<dyn AbstractValue>;
    fn allocate(&mut self) -> Box<dyn AbstractValue>;
    fn calc(&self, context: &mut dyn Context<T>, value: &mut dyn AbstractValue);
}
