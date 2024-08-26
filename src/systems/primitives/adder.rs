use num_traits::identities::Zero;
use std::fmt::Debug;
use std::ops::Add;

use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::framework_common::{SystemId, SystemParentServiceInterface};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_output_port::LeafOutputPort;
use crate::systems::framework::leaf_system::LeafSystem;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::system_base::{ContextSizes, SystemBase};

pub struct Adder<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> {
    num_inputs: usize,
    size: usize,
    input_ports: Vec<Box<InputPort<T>>>,
    output_ports: Vec<Box<LeafOutputPort<T>>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    parent_service: Option<Box<dyn SystemParentServiceInterface>>,
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> Adder<T> {
    pub fn new(num_inputs: usize, size: usize) -> Self {
        Self {
            num_inputs,
            size,
            input_ports: vec![],
            output_ports: vec![],
            cache_entries: vec![],
            context_sizes: ContextSizes::default(),
            system_id: SystemId::new(0),
            parent_service: None,
        }
    }

    // fn calc_sum(&self, context: &dyn Context<T>, output: &mut BasicVector<T>) {
    //     for input_port_index in 0..self.num_{
    //         let input = context.get_input_port_value(i);
    //         output.add_inplace(input);
    //     }
    // }
}
