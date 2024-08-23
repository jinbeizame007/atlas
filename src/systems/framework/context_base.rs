use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::common::value::AbstractValue;
use crate::systems::framework::cache::Cache;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::SystemId;

pub trait ContextBase: Any {
    fn set_system_id(&mut self, system_id: SystemId);
    fn get_system_id(&self) -> &SystemId;
    fn get_parent_base(&self) -> Arc<Mutex<dyn ContextBase>>;
    fn get_cache(&self) -> &Cache;
    fn get_mutable_cache(&mut self) -> &mut Cache;
    fn num_input_ports(&self) -> usize;
    // TODO: implement num_output_ports()
    // fn num_output_ports(&self) -> usize;
    fn fix_input_port(&mut self, index: usize, value: &dyn AbstractValue) -> &FixedInputPortValue;
    fn get_fixed_input_port_value(&self, index: usize) -> &Option<FixedInputPortValue>;
    fn get_mutable_fixed_input_port_value(
        &mut self,
        index: usize,
    ) -> &mut Option<FixedInputPortValue>;

    fn mark_context_base_initialized(&mut self) {
        self.set_is_context_base_initialized(true);
    }
    fn set_is_context_base_initialized(&mut self, is_context_base_initialized: bool) {
        *self.get_mutable_is_context_base_initialized() = is_context_base_initialized;
    }
    fn get_mutable_is_context_base_initialized(&mut self) -> &mut bool;
}
