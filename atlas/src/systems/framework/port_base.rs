use std::any::Any;

use crate::systems::framework::framework_common::PortDataType;

pub trait PortBase: Any {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn data_type(&self) -> &PortDataType;
    fn size(&self) -> usize;
}
