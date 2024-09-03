use std::collections::HashMap;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::diagram::{Diagram, InputPortLocator, OutputPortLocator, SystemPtr};
use crate::systems::framework::framework_common::{
    InputPortIndex, OutputPortIndex, SubsystemIndex,
};
use crate::systems::framework::system::System;

pub struct DiagramBuilder<T: AtlasScalar> {
    input_port_ids: Vec<InputPortLocator<T>>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    systems: Vec<SystemPtr<T>>,
    already_built: bool,
}
