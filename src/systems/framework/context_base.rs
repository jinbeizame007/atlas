use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::common::value::AbstractValue;
use crate::systems::framework::cache::Cache;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::{InputPortIndex, SystemId};
use crate::systems::framework::output_port_base::OutputPortBase;

pub trait ContextBase: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn set_system_id(&mut self, system_id: SystemId);
    fn get_system_id(&self) -> &SystemId;
    fn get_parent_base(&self) -> Option<Arc<Mutex<dyn ContextBase>>>;
    fn get_cache(&self) -> &Cache;
    fn get_mutable_cache(&mut self) -> &mut Cache;

    fn get_input_port_values(&mut self) -> &mut Vec<Option<FixedInputPortValue>>;
    fn num_input_ports(&self) -> usize;
    // TODO: implement num_output_ports()
    // fn num_output_ports(&self) -> usize;
    fn add_input_port(&mut self, input_port_index: &InputPortIndex) {
        assert!(*input_port_index == self.num_input_ports());
        self.get_input_port_values().push(None);
    }
    fn fix_input_port(
        &mut self,
        index: usize,
        value: &dyn AbstractValue,
    ) -> Option<&FixedInputPortValue>;
    fn get_fixed_input_port_value(&self, index: usize) -> Option<&FixedInputPortValue>;
    fn get_mutable_fixed_input_port_value(
        &mut self,
        index: usize,
    ) -> Option<&mut FixedInputPortValue>;

    // fn add_output_port(&mut self, output_port_index: &OutputPortIndex) {
    //     // assert!(*output_port_index == self.num_output_ports());

    // }

    fn mark_context_base_initialized(&mut self) {
        self.set_is_context_base_initialized(true);
    }
    fn set_is_context_base_initialized(&mut self, is_context_base_initialized: bool) {
        *self.get_mutable_is_context_base_initialized() = is_context_base_initialized;
    }
    fn get_mutable_is_context_base_initialized(&mut self) -> &mut bool;
}
