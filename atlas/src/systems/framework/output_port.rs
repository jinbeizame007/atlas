use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::context::Context;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::output_port_base::OutputPortBase;

pub trait OutputPort<T: AtlasScalar>: OutputPortBase {
    type CN: Context<T>;

    // fn eval<ValueType: Clone + Debug + 'static>(&self, context: &mut <Self as OutputPort<T>>::CN) -> ValueType;
    fn eval_abstract(&self, context: &Self::CN) -> Box<dyn AbstractValue>;
    fn allocate(&mut self) -> Box<dyn AbstractValue>;
    fn calc(&self, context: &mut Self::CN, value: &mut dyn AbstractValue);
    fn system_weak_link(&self) -> SystemWeakLink<T>;
}
