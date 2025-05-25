use crate::common::value::AbstractValue;
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

    pub fn calc(&self, context: &dyn ContextBase, value: &mut dyn AbstractValue) {
        self.value_producer.calc(context, value)
    }

    pub fn eval<ValueType>(&self, context: &mut dyn ContextBase) -> ValueType
    where
        ValueType: Clone + 'static,
    {
        let abstract_value = self.eval_abstract(context);
        abstract_value
            .as_any()
            .downcast_ref::<ValueType>()
            .unwrap()
            .clone()
    }

    pub fn eval_abstract(&self, context: &dyn ContextBase) -> Box<dyn AbstractValue> {
        if context
            .cache()
            .borrow()
            .cache_entry_value(&self.cache_index)
            .needs_recomputation()
        {
            self.update_value(context)
        }
        context
            .cache()
            .borrow()
            .cache_entry_value(&self.cache_index)
            .abstract_value()
            .clone_box()
    }

    fn update_value(&self, context: &dyn ContextBase) {
        let mut value = {
            let mut cache = context.cache().borrow_mut();
            cache
                .cache_mut_entry_value(&self.cache_index)
                .abstract_value_mut()
                .clone_box()
        };
        self.calc(context, value.as_mut());

        {
            let mut cache = context.cache().borrow_mut();
            cache
                .cache_mut_entry_value(&self.cache_index)
                .abstract_value_mut()
                .set_from(value.as_ref());
        }
    }

    pub fn cache_index(&self) -> &CacheIndex {
        &self.cache_index
    }
}
