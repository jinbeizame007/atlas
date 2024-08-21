use crate::common::identifier::Identifier;
use crate::common::type_safe_index::TypeSafeIndex;

#[allow(dead_code)]
#[derive(Clone)]
pub struct CacheTag;

pub type CacheIndex = TypeSafeIndex<CacheTag>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct InputPortTag;

pub type InputPortIndex = TypeSafeIndex<InputPortTag>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct OutputPortTag;

pub type OutputPortIndex = TypeSafeIndex<OutputPortTag>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct ContinuousStateTag;

pub type ContinuousStateIndex = TypeSafeIndex<ContinuousStateTag>;

pub enum PortDataType {
    VectorValued,
    AbstractValued,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SystemIdTag;

pub type SystemId = Identifier<SystemIdTag>;
