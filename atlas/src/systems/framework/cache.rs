use crate::common::value::AbstractValue;
use crate::systems::framework::framework_common::CacheIndex;

#[derive(Clone, Debug, Default, PartialEq)]
enum Flag {
    #[default]
    ValueIsOutOfDate,
    ReadyToUse,
}

#[derive(Clone, Debug, Default)]
pub struct CacheEntryValue {
    cache_index: CacheIndex,
    value: Box<dyn AbstractValue>,
    flag: Flag,
}

impl CacheEntryValue {
    pub fn cache_index(&self) -> &CacheIndex {
        &self.cache_index
    }

    pub fn set_initial_value(&mut self, initial_value: Box<dyn AbstractValue>) {
        self.value = initial_value;
    }

    pub fn abstract_value(&self) -> &dyn AbstractValue {
        self.value.as_ref()
    }

    pub fn abstract_value_mut(&mut self) -> &mut dyn AbstractValue {
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

#[derive(Default)]
pub struct Cache {
    store: Vec<CacheEntryValue>,
}

impl Cache {
    pub fn cache_size(&self) -> usize {
        self.store.len()
    }

    pub fn create_new_cache_entry_value(
        &mut self,
        cache_index: CacheIndex,
    ) -> &mut CacheEntryValue {
        if cache_index >= self.cache_size() {
            self.store
                .resize_with(cache_index.value() + 1, Default::default)
        }

        let cache_entry_value = CacheEntryValue {
            cache_index: cache_index.clone(),
            ..Default::default()
        };
        self.store[cache_index.value()] = cache_entry_value;

        self.cache_mut_entry_value(&cache_index)
    }

    pub fn cache_entry_value(&self, cache_index: &CacheIndex) -> &CacheEntryValue {
        &self.store[cache_index]
    }

    pub fn cache_mut_entry_value(&mut self, cache_index: &CacheIndex) -> &mut CacheEntryValue {
        &mut self.store[cache_index.value()]
    }
}
