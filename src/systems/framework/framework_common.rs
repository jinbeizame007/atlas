use crate::common::identifier::Identifier;
use crate::common::type_safe_index::TypeSafeIndex;

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct CacheTag;

pub type CacheIndex = TypeSafeIndex<CacheTag>;

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct InputPortTag;

pub type InputPortIndex = TypeSafeIndex<InputPortTag>;

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct OutputPortTag;

pub type OutputPortIndex = TypeSafeIndex<OutputPortTag>;

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct ContinuousStateTag;

pub type ContinuousStateIndex = TypeSafeIndex<ContinuousStateTag>;

#[derive(PartialEq)]
pub enum PortDataType {
    VectorValued,
    AbstractValued,
}

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct SystemIdTag;

pub type SystemId = Identifier<SystemIdTag>;
