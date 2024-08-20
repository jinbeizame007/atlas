use crate::systems::framework::framework_common::{OutputPortIndex, PortDataType};

pub trait OutputPortBase {
    fn get_index(&self) -> &OutputPortIndex;
    fn get_data_type(&self) -> &PortDataType;
    fn size(&self) -> usize;
}
