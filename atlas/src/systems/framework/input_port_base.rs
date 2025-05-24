use crate::common::value::AbstractValue;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::framework_common::InputPortIndex;
use crate::systems::framework::port_base::PortBase;

pub type EvalAbstractCallback = dyn Fn(&dyn ContextBase) -> Box<dyn AbstractValue>;

pub trait InputPortBase: PortBase {
    fn index(&self) -> &InputPortIndex;
    fn allocate(&mut self) -> Box<dyn AbstractValue>;
}
