use crate::common::type_safe_index::TypeSafeIndex;

#[allow(dead_code)]
#[derive(Clone)]
pub struct CacheTag;

pub type CacheIndex = TypeSafeIndex<CacheTag>;
