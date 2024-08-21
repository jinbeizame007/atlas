use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::port_base::PortBase;

pub trait OutputPortBase: PortBase {
    fn get_index(&self) -> &OutputPortIndex;
}
