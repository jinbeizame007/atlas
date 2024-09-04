use std::collections::{HashMap, HashSet};

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::diagram::{
    Diagram, InputPortLocator, OutputPortLocator, OwnedSystems, SystemPtr,
};
use crate::systems::framework::diagram_context::DiagramContext;
use crate::systems::framework::framework_common::{
    InputPortIndex, OutputPortIndex, SubsystemIndex,
};
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::system::System;

#[derive(Default)]
pub struct DiagramBuilder<T: AtlasScalar> {
    input_port_ids: Vec<InputPortLocator<T>>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    systems: HashSet<SystemPtr<T>>,
    owned_systems: OwnedSystems,
    already_built: bool,
}

impl<T: AtlasScalar> DiagramBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_leaf_system<S>(&mut self, mut system: Box<S>) -> SystemPtr<T>
    where
        S: System<T, CN = LeafContext<T>>,
        T: AtlasScalar,
    {
        let system_ptr =
            SystemPtr::LeafSystemPtr(system.as_mut() as *mut dyn System<T, CN = LeafContext<T>>);

        self.owned_systems.push(system);

        system_ptr
    }

    pub fn add_diagram<S>(&mut self, mut system: Box<S>) -> SystemPtr<T>
    where
        S: System<T, CN = DiagramContext<T>>,
        T: AtlasScalar,
    {
        let system_ptr =
            SystemPtr::DiagramPtr(system.as_mut() as *mut dyn System<T, CN = DiagramContext<T>>);

        self.owned_systems.push(system);

        system_ptr
    }
}
