use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::framework_common::{
    CacheIndex, InputPortIndex, OutputPortIndex, SystemId,
};
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::output_port_base::OutputPortBase;

use super::value_producer::ValueProducer;

pub trait SystemBase {
    fn allocate_context(&self) -> dyn ContextBase;
    fn get_input_port_base(&self, index: InputPortIndex) -> &dyn InputPortBase;
    fn num_input_ports(&self) -> usize;
    fn get_output_port_base(&self, index: OutputPortIndex) -> &dyn OutputPortBase;
    fn num_output_ports(&self) -> usize;
    fn num_cache_entries(&self) -> usize;
    fn get_cache_entry(&self, index: &CacheIndex) -> &CacheEntry;
    fn get_mutable_cache_entry(&mut self, index: &CacheIndex) -> &mut CacheEntry;
    fn num_continuous_state(&self) -> usize;
    fn add_input_port(&mut self, input_port: Box<dyn InputPortBase>);
    fn add_output_port(&mut self, output_port: Box<dyn OutputPortBase>);
    fn get_system_id(&self) -> SystemId;
    fn declare_cache_entry(&mut self, value_producer: ValueProducer);
}
