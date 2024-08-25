use std::any::Any;

use crate::systems::framework::framework_common::PortDataType;

pub trait PortBase: Any {
    fn get_data_type(&self) -> &PortDataType;
    fn size(&self) -> usize;
}
