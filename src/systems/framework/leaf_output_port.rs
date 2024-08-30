use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::framework_common::{OutputPortIndex, PortDataType, SystemId};
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::port_base::PortBase;

pub struct LeafOutputPort<T: AtlasScalar> {
    _system_id: SystemId,
    index: OutputPortIndex,
    data_type: PortDataType,
    size: usize,
    cache_entry: *const CacheEntry,
    _phantom: PhantomData<T>,
}

impl<T: AtlasScalar> PortBase for LeafOutputPort<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_data_type(&self) -> &PortDataType {
        &self.data_type
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T: AtlasScalar> OutputPortBase for LeafOutputPort<T> {
    fn get_index(&self) -> &OutputPortIndex {
        &self.index
    }
}

impl<T: AtlasScalar> OutputPort<T> for LeafOutputPort<T> {
    fn allocate(&mut self) -> Box<dyn AbstractValue> {
        self.cache_entry().allocate()
    }

    fn eval_abstract(&self, context: &mut dyn Context<T>) -> Box<dyn AbstractValue> {
        self.cache_entry()
            .eval_abstract(context.as_mutable_base())
            .clone_box()
    }

    fn calc(&self, context: &mut dyn Context<T>, value: &mut dyn AbstractValue) {
        self.cache_entry().calc(context.as_mutable_base(), value)
    }
}

impl<T: AtlasScalar> LeafOutputPort<T> {
    pub fn new(
        _system_id: SystemId,
        index: OutputPortIndex,
        data_type: PortDataType,
        size: usize,
        cache_entry: *const CacheEntry,
    ) -> Self {
        LeafOutputPort::<T> {
            _system_id,
            index,
            data_type,
            size,
            cache_entry,
            _phantom: PhantomData::<T>,
        }
    }

    pub fn eval<ValueType: Clone + Debug + 'static>(
        &self,
        context: &mut dyn Context<T>,
    ) -> ValueType {
        self.eval_abstract(context)
            .as_any()
            .downcast_ref::<Value<ValueType>>()
            .unwrap()
            .get_value()
            .clone()
    }

    pub fn cache_entry(&self) -> &CacheEntry {
        unsafe { &(*self.cache_entry) }
    }
}
