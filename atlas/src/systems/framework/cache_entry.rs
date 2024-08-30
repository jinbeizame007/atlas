use crate::common::value::AbstractValue;
use crate::systems::framework::cache::CacheEntryValue;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::framework_common::CacheIndex;
use crate::systems::framework::value_producer::ValueProducer;

pub struct CacheEntry {
    cache_index: CacheIndex,
    value_producer: ValueProducer,
}

impl CacheEntry {
    pub fn new(cache_index: CacheIndex, value_producer: ValueProducer) -> Self {
        CacheEntry {
            cache_index,
            value_producer,
        }
    }

    pub fn allocate(&self) -> Box<dyn AbstractValue> {
        self.value_producer.allocate()
    }

    pub fn calc(&self, context: &mut dyn ContextBase, value: &mut dyn AbstractValue) {
        self.value_producer.calc(context, value)
    }

    pub fn eval<'a, ValueType: 'static>(&self, context: &'a mut dyn ContextBase) -> &'a ValueType {
        let abstract_value = self.eval_abstract(context);
        abstract_value.as_any().downcast_ref::<ValueType>().unwrap()
    }

    pub fn eval_abstract<'a>(&self, context: &'a mut dyn ContextBase) -> &'a dyn AbstractValue {
        if self.cache_entry_value(context).needs_recomputation() {
            self.update_value(context)
        }
        self.cache_entry_value(context).abstract_value()
    }

    fn update_value(&self, context: &mut dyn ContextBase) {
        let mut value = {
            let mutable_cache_value = self.cache_mut_entry_value(context);
            mutable_cache_value.abstract_value_mut().clone_box()
        };
        self.calc(context, value.as_mut());

        let mutable_abstract_value = self.cache_mut_entry_value(context).abstract_value_mut();
        mutable_abstract_value.set_from(value.as_ref());
    }

    pub fn cache_entry_value<'a>(&self, context: &'a dyn ContextBase) -> &'a CacheEntryValue {
        context.cache().cache_entry_value(&self.cache_index)
    }

    pub fn cache_mut_entry_value<'a>(
        &self,
        context: &'a mut dyn ContextBase,
    ) -> &'a mut CacheEntryValue {
        context.cache_mut().cache_mut_entry_value(&self.cache_index)
    }

    pub fn cache_index(&self) -> &CacheIndex {
        &self.cache_index
    }
}
