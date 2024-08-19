use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::input_port_base::PortDataType;

pub trait OutputPortBase {
    fn get_index(&self) -> &OutputPortIndex;
    fn get_data_type(&self) -> &PortDataType;
    fn size(&self) -> usize;
}
