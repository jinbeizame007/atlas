use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::basic_vector::BasicVector;

// https://en.wikipedia.org/wiki/Prototype_pattern
pub struct ModelValues {
    values: Vec<Option<Box<dyn AbstractValue>>>,
}

impl ModelValues {
    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn add_model(&mut self, index: usize, model_value: Box<dyn AbstractValue>) {
        if index >= self.size() {
            self.values.resize_with(index, Default::default)
        }
        self.values[index] = Some(model_value);
    }

    pub fn add_vector_model<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static>(
        &mut self,
        index: usize,
        model_vector: BasicVector<T>,
    ) {
        self.add_model(index, Box::new(Value::<BasicVector<T>>::new(model_vector)));
    }

    pub fn clone_model(&self, index: usize) -> Option<Box<dyn AbstractValue>> {
        self.values[index].clone()
    }

    pub fn clone_all_models(&self) -> Vec<Option<Box<dyn AbstractValue>>> {
        self.values.to_vec()
    }

    pub fn clone_vector_model<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static>(
        &self,
        index: usize,
    ) -> Option<BasicVector<T>> {
        if self.clone_model(index).is_some() {
            self.clone_model(index).map(|abstract_value| {
                abstract_value
                    .as_ref()
                    .as_any()
                    .downcast_ref::<BasicVector<T>>()
                    .unwrap()
                    .clone()
            })
        } else {
            None
        }
    }
}
