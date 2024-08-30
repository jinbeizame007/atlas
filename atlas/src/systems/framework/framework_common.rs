use std::cmp::PartialEq;

use crate::common::identifier::Identifier;
use crate::common::type_safe_index::TypeSafeIndex;
use crate::common::value::AbstractValue;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::system_base::SystemBase;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct CacheTag;

pub type CacheIndex = TypeSafeIndex<CacheTag>;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct InputPortTag;

pub type InputPortIndex = TypeSafeIndex<InputPortTag>;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct OutputPortTag;

pub type OutputPortIndex = TypeSafeIndex<OutputPortTag>;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ContinuousStateTag;

pub type ContinuousStateIndex = TypeSafeIndex<ContinuousStateTag>;

#[derive(Clone, Debug, PartialEq)]
pub enum PortDataType {
    VectorValued,
    AbstractValued,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemIdTag;

pub type SystemId = Identifier<SystemIdTag>;

pub trait SystemParentServiceInterface {
    fn root_system_base(&self) -> &dyn SystemBase;
    fn eval_connected_subsystem_input_port(
        &self,
        context: &mut dyn ContextBase,
        input_port: &dyn InputPortBase,
    ) -> Box<dyn AbstractValue>;
}
