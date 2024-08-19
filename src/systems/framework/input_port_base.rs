use crate::common::value::AbstractValue;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::framework_common::InputPortIndex;

pub type EvalAbstractCallback = dyn Fn(&dyn AbstractValue, &dyn ContextBase);

pub enum PortDataType {
    VectorValued,
    AbstractValued,
}

pub trait InputPortBase {
    fn get_index(&self) -> &InputPortIndex;
    fn allocate(&mut self) -> &dyn AbstractValue;
    fn get_data_type(&self) -> &PortDataType;
    fn size(&self) -> usize;
}
