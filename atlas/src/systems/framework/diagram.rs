use std::collections::HashMap;

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::framework_common::{
    InputPortIndex, OutputPortIndex, SubsystemIndex,
};
use crate::systems::framework::system::System;

use super::leaf_context::LeafContext;

#[derive(Clone)]
pub enum SystemPtr<T: AtlasScalar> {
    LeafSystemPtr(*mut dyn System<T, CN = LeafContext<T>>),
    DiagramPtr(*mut Diagram<T>),
}

#[derive(Clone)]
pub struct InputPortLocator<T: AtlasScalar> {
    pub system_ptr: SystemPtr<T>,
    pub input_port_index: InputPortIndex,
}

#[derive(Clone)]
pub struct OutputPortLocator<T: AtlasScalar> {
    pub system_ptr: SystemPtr<T>,
    pub input_port_index: OutputPortIndex,
}

struct Blueprint<T: AtlasScalar> {
    input_port_ids: Vec<InputPortLocator<T>>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
}

pub struct Diagram<T: AtlasScalar> {
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    system_index_map: HashMap<SystemPtr<T>, SubsystemIndex>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    input_port_map: HashMap<InputPortIndex, InputPortLocator<T>>,
}
