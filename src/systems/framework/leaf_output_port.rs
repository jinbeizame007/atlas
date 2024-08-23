use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::framework_common::{OutputPortIndex, PortDataType, SystemId};
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::port_base::PortBase;

pub struct LeafOutputPort<'a, T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> {
    _system_id: SystemId,
    index: OutputPortIndex,
    data_type: PortDataType,
    size: usize,
    cache_entry: &'a CacheEntry,
}

impl<'a, T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> PortBase
    for LeafOutputPort<'a, T>
{
    fn get_data_type(&self) -> &PortDataType {
        &self.data_type
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> OutputPortBase
    for LeafOutputPort<'a, T>
{
    fn get_index(&self) -> &OutputPortIndex {
        &self.index
    }
}

impl<'a, T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> OutputPort<T>
    for LeafOutputPort<'a, T>
{
    fn allocate(&mut self) -> Box<dyn AbstractValue> {
        self.cache_entry.allocate()
    }

    fn eval_abstract(&self, context: &mut dyn Context<T>) -> Box<dyn AbstractValue> {
        self.cache_entry
            .eval_abstract(context.as_mutable_base())
            .clone_box()
    }

    fn calc(&self, context: &dyn Context<T>, value: &mut dyn AbstractValue) {
        self.cache_entry.calc(context.as_base(), value)
    }
}

impl<'a, T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> LeafOutputPort<'a, T> {
    pub fn new(
        _system_id: SystemId,
        index: OutputPortIndex,
        data_type: PortDataType,
        size: usize,
        cache_entry: &'a CacheEntry,
    ) -> Self {
        LeafOutputPort::<T> {
            _system_id,
            index,
            data_type,
            size,
            cache_entry,
        }
    }

    pub fn eval<ValueType: Clone + 'static>(&self, context: &mut dyn Context<T>) -> ValueType {
        self.eval_abstract(context)
            .as_any()
            .downcast_ref::<ValueType>()
            .unwrap()
            .clone()
    }

    pub fn cache_entry(&self) -> &'a CacheEntry {
        self.cache_entry
    }
}
