use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::framework_common::CacheIndex;

#[derive(Clone, PartialEq)]
enum Flag {
    ReadyToUse,
    ValueIsOutOfDate,
}

#[derive(Clone)]
pub struct CacheEntryValue {
    cache_index: CacheIndex,
    value: Box<dyn AbstractValue>,
    flag: Flag,
}

impl CacheEntryValue {
    pub fn cache_index(&self) -> &CacheIndex {
        &self.cache_index
    }

    pub fn get_abstract_value(&mut self) -> &mut dyn AbstractValue {
        self.value.as_mut()
    }

    pub fn mark_up_to_date(&mut self) {
        self.flag = Flag::ReadyToUse;
    }

    pub fn mark_out_of_date(&mut self) {
        self.flag = Flag::ValueIsOutOfDate;
    }

    pub fn needs_recomputation(&self) -> bool {
        self.flag == Flag::ValueIsOutOfDate
    }
}

impl Default for CacheEntryValue {
    fn default() -> Self {
        CacheEntryValue {
            cache_index: CacheIndex::new(0),
            value: Box::new(Value::<usize>::new(0)),
            flag: Flag::ValueIsOutOfDate,
        }
    }
}

pub struct Cache {
    store: Vec<CacheEntryValue>,
}

impl Cache {
    pub fn cache_size(&self) -> usize {
        self.store.len()
    }

    pub fn create_new_cache_entry_value(&mut self, cache_index: CacheIndex) {
        if cache_index >= self.cache_size() {
            self.store
                .resize_with(cache_index.value() + 1, Default::default)
        }

        let cache_entry_value = CacheEntryValue {
            cache_index: cache_index.clone(),
            ..Default::default()
        };
        self.store[cache_index] = cache_entry_value;
    }

    pub fn get_cache_entry_value(&self, cache_index: CacheIndex) -> Option<&CacheEntryValue> {
        self.store.get(cache_index.value())
    }

    pub fn get_mutable_cache_entry_value(
        &mut self,
        cache_index: CacheIndex,
    ) -> Option<&mut CacheEntryValue> {
        self.store.get_mut(cache_index.value())
    }
}
