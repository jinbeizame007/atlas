use crate::common::value::AbstractValue;
use crate::systems::framework::cache::Cache;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::SystemId;

pub trait ContextBase {
    fn get_system_id(&self) -> &SystemId;
    fn get_cache(&self) -> &Cache;
    fn get_mutable_cache(&mut self) -> &mut Cache;
    fn num_input_ports(&self) -> usize;
    fn num_output_ports(&self) -> usize;
    fn fix_input_port(index: usize, value: &dyn AbstractValue) -> &FixedInputPortValue;
}
