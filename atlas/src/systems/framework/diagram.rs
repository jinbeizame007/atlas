use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::diagram_context::DiagramContext;
use crate::systems::framework::framework_common::{
    InputPortIndex, OutputPortIndex, SubsystemIndex,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::system::{AbstractSystem, System};

#[derive(Clone, Debug)]
pub enum SystemPtr<T: AtlasScalar> {
    LeafSystemPtr(*mut dyn System<T, CN = LeafContext<T>>),
    DiagramPtr(*mut dyn System<T, CN = DiagramContext<T>>),
}

impl<T: AtlasScalar> PartialEq for SystemPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SystemPtr::LeafSystemPtr(a), SystemPtr::LeafSystemPtr(b)) => *a == *b,
            (SystemPtr::DiagramPtr(a), SystemPtr::DiagramPtr(b)) => *a == *b,
            _ => false,
        }
    }
}

impl<T: AtlasScalar> Eq for SystemPtr<T> {}

impl<T: AtlasScalar> Hash for SystemPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = match self {
            SystemPtr::LeafSystemPtr(ptr) => (*ptr as *const ()) as usize,
            SystemPtr::DiagramPtr(ptr) => (*ptr as *const ()) as usize,
        };
        addr.hash(state);
    }
}

impl<T: AtlasScalar> SystemPtr<T> {
    pub fn input_port(&self, input_port_index: InputPortIndex) -> &InputPort<T> {
        match self {
            SystemPtr::LeafSystemPtr(system) => unsafe { (**system).input_port(&input_port_index) },
            SystemPtr::DiagramPtr(system) => unsafe { (**system).input_port(&input_port_index) },
        }
    }

    pub fn allocate_input_abstract(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        match self {
            SystemPtr::LeafSystemPtr(system) => unsafe {
                (**system).allocate_input_abstract(input_port)
            },
            SystemPtr::DiagramPtr(system) => unsafe {
                (**system).allocate_input_abstract(input_port)
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputPortLocator<T: AtlasScalar> {
    pub system_ptr: SystemPtr<T>,
    pub input_port_index: InputPortIndex,
}

impl<T: AtlasScalar> PartialEq for InputPortLocator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.system_ptr == other.system_ptr && self.input_port_index == other.input_port_index
    }
}

impl<T: AtlasScalar> Eq for InputPortLocator<T> {}

impl<T: AtlasScalar> Hash for InputPortLocator<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.system_ptr.hash(state);
        self.input_port_index.hash(state);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputPortLocator<T: AtlasScalar> {
    pub system_ptr: SystemPtr<T>,
    pub output_port_index: OutputPortIndex,
}

#[derive(Default)]
pub struct OwnedSystems {
    pub systems: Vec<Box<dyn AbstractSystem>>,
}

impl OwnedSystems {
    pub fn push<T: AtlasScalar, S: System<T>>(&mut self, system: Box<S>) {
        self.systems.push(system);
    }
}

#[derive(Default)]
pub struct DiagramBlueprint<T: AtlasScalar> {
    pub input_port_ids: Vec<InputPortLocator<T>>,
    pub output_port_ids: Vec<OutputPortLocator<T>>,
    pub connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    pub system_ptrs: Vec<SystemPtr<T>>,
    pub registered_systems: OwnedSystems,
}

impl<T: AtlasScalar> DiagramBlueprint<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct Diagram<T: AtlasScalar> {
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    registered_systems: OwnedSystems,
    system_index_map: HashMap<SystemPtr<T>, SubsystemIndex>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    input_port_map: HashMap<InputPortIndex, InputPortLocator<T>>,
}

impl<T: AtlasScalar> Diagram<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_subsystems(&self) -> usize {
        self.registered_systems.systems.len()
    }

    // pub fn from_blueprint(blueprint: DiagramBlueprint<T>) -> Self {
    //     let mut diagram = Self::new();
    //     diagram.initialize(blueprint);
    //     diagram
    // }

    // pub fn initialize(&mut self, blueprint: DiagramBlueprint<T>) {
    //     assert!(!blueprint.registered_systems.systems.is_empty());
    //     assert!(self.registered_systems.systems.is_empty());

    //     self.connection_map = blueprint.connection_map;
    //     self.output_port_ids = blueprint.output_port_ids;
    //     self.registered_systems = blueprint.registered_systems;

    //     // Generate a map from the System pointer to its index in the registered order.
    //     for (index, system_ptr) in blueprint.system_ptrs.iter().enumerate() {
    //         self.system_index_map
    //             .insert(system_ptr.clone(), SubsystemIndex::new(index));
    //     }

    //     // Every system must appear exactly once.
    //     assert_eq!(self.num_subsystems(), self.registered_systems.systems.len());

    // }

    // fn export_or_connect_input(&mut self, input_port_locator: InputPortLocator<T>) {
    //     let system_ptr = input_port_locator.system_ptr;
    //     let input_port_index = input_port_locator.input_port_index;

    //     if
    // }
}
