use std::cell::{Ref, RefCell};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::diagram_context::DiagramContext;
use crate::systems::framework::framework_common::{
    InputPortIndex, OutputPortIndex, SubsystemIndex,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::system::{AbstractSystem, System};

#[derive(Clone)]
pub enum SystemLink<T: AtlasScalar> {
    LeafSystemLink(LeafSystemLink<T>),
    DiagramLink(DiagramLink<T>),
}

type LeafSystemLink<T: AtlasScalar> = Rc<RefCell<dyn System<T, CN = LeafContext<T>>>>;
type DiagramLink<T: AtlasScalar> = Rc<RefCell<dyn System<T, CN = DiagramContext<T>>>>;

impl<T: AtlasScalar> PartialEq for SystemLink<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SystemLink::LeafSystemLink(a), SystemLink::LeafSystemLink(b)) => {
                a.as_ptr() == b.as_ptr()
            }
            (SystemLink::DiagramLink(a), SystemLink::DiagramLink(b)) => a.as_ptr() == b.as_ptr(),
            _ => false,
        }
    }
}

impl<T: AtlasScalar> Eq for SystemLink<T> {}

impl<T: AtlasScalar> Hash for SystemLink<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = match self {
            SystemLink::LeafSystemLink(system) => (system.as_ptr() as *const ()) as usize,
            SystemLink::DiagramLink(system) => (system.as_ptr() as *const ()) as usize,
        };

        addr.hash(state);
    }
}

impl<T: AtlasScalar> SystemLink<T> {
    pub fn input_port(&self, input_port_index: InputPortIndex) -> Ref<InputPort<T>> {
        match self {
            SystemLink::LeafSystemLink(system) => {
                Ref::map(system.borrow(), |s| s.input_port(&input_port_index))
            }
            SystemLink::DiagramLink(system) => {
                Ref::map(system.borrow(), |s| s.input_port(&input_port_index))
            }
        }
    }

    pub fn allocate_input_abstract(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        match self {
            SystemLink::LeafSystemLink(system) => {
                system.borrow().allocate_input_abstract(input_port)
            }
            SystemLink::DiagramLink(system) => system.borrow().allocate_input_abstract(input_port),
        }
    }

    pub fn eval_abstract_input(
        &self,
        context: &dyn ContextBase,
        input_port_index: &InputPortIndex,
    ) -> Box<dyn AbstractValue> {
        match self {
            SystemLink::LeafSystemLink(system) => system
                .borrow()
                .eval_abstract_input(context, input_port_index),
            SystemLink::DiagramLink(system) => system
                .borrow()
                .eval_abstract_input(context, input_port_index),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SystemWeakLink<T: AtlasScalar> {
    LeafSystemWeakLink(LeafSystemWeakLink<T>),
    DiagramWeakLink(DiagramWeakLink<T>),
}

type LeafSystemWeakLink<T: AtlasScalar> = Weak<RefCell<dyn System<T, CN = LeafContext<T>>>>;
type DiagramWeakLink<T: AtlasScalar> = Weak<RefCell<dyn System<T, CN = DiagramContext<T>>>>;

impl<T: AtlasScalar> PartialEq for SystemWeakLink<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SystemWeakLink::LeafSystemWeakLink(a), SystemWeakLink::LeafSystemWeakLink(b)) => {
                a.as_ptr() == b.as_ptr()
            }
            (SystemWeakLink::DiagramWeakLink(a), SystemWeakLink::DiagramWeakLink(b)) => {
                a.as_ptr() == b.as_ptr()
            }
            _ => false,
        }
    }
}

impl<T: AtlasScalar> Eq for SystemWeakLink<T> {}

impl<T: AtlasScalar> Hash for SystemWeakLink<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = match self {
            SystemWeakLink::LeafSystemWeakLink(system) => {
                (system.upgrade().unwrap().as_ptr() as *const ()) as usize
            }
            SystemWeakLink::DiagramWeakLink(system) => {
                (system.upgrade().unwrap().as_ptr() as *const ()) as usize
            }
        };

        addr.hash(state);
    }
}

impl<T: AtlasScalar> SystemWeakLink<T> {
    pub fn as_leaf_system_weak_link(&self) -> Option<&LeafSystemWeakLink<T>> {
        match self {
            SystemWeakLink::LeafSystemWeakLink(ref leaf) => Some(leaf),
            _ => None,
        }
    }

    // DiagramWeakLink を返すメソッド
    pub fn as_diagram_weak_link(&self) -> Option<&DiagramWeakLink<T>> {
        match self {
            SystemWeakLink::DiagramWeakLink(ref diagram) => Some(diagram),
            _ => None,
        }
    }

    // pub fn input_port(&self, input_port_index: InputPortIndex) -> Ref<InputPort<T>> {
    //     match self {
    //         SystemWeakLink::LeafSystemWeakLink(system) => {
    //             // 一時的な変数にupgrade()の結果を格納
    //             let strong_system = system.upgrade().unwrap();
    //             Ref::map(strong_system.borrow(), |s| s.input_port(&input_port_index))
    //         }
    //         SystemWeakLink::DiagramWeakLink(system) => {
    //             // 一時的な変数にupgrade()の結果を格納
    //             let strong_system = system.upgrade().unwrap();
    //             Ref::map(strong_system.borrow(), |s| s.input_port(&input_port_index))
    //         }
    //     }
    // }

    pub fn allocate_input_abstract(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        match self {
            SystemWeakLink::LeafSystemWeakLink(system) => system
                .upgrade()
                .unwrap()
                .borrow()
                .allocate_input_abstract(input_port),
            SystemWeakLink::DiagramWeakLink(system) => system
                .upgrade()
                .unwrap()
                .borrow()
                .allocate_input_abstract(input_port),
        }
    }

    pub fn eval_abstract_input(
        &self,
        context: &dyn ContextBase,
        input_port_index: &InputPortIndex,
    ) -> Box<dyn AbstractValue> {
        match self {
            SystemWeakLink::LeafSystemWeakLink(system) => system
                .upgrade()
                .unwrap()
                .borrow()
                .eval_abstract_input(context, input_port_index),
            SystemWeakLink::DiagramWeakLink(system) => system
                .upgrade()
                .unwrap()
                .borrow()
                .eval_abstract_input(context, input_port_index),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputPortLocator<T: AtlasScalar> {
    pub system_weak_link: SystemWeakLink<T>,
    pub input_port_index: InputPortIndex,
}

impl<T: AtlasScalar> PartialEq for InputPortLocator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.system_weak_link == other.system_weak_link
            && self.input_port_index == other.input_port_index
    }
}

impl<T: AtlasScalar> Eq for InputPortLocator<T> {}

impl<T: AtlasScalar> Hash for InputPortLocator<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.system_weak_link.hash(state);
        self.input_port_index.hash(state);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputPortLocator<T: AtlasScalar> {
    pub system_weak_link: SystemWeakLink<T>,
    pub output_port_index: OutputPortIndex,
}

#[derive(Default)]
pub struct OwnedSystems<T: AtlasScalar> {
    pub systems: Vec<SystemLink<T>>,
}

impl<T: AtlasScalar> OwnedSystems<T> {
    pub fn push(&mut self, system: SystemLink<T>) {
        self.systems.push(system);
    }
}

#[derive(Default)]
pub struct DiagramBlueprint<T: AtlasScalar> {
    pub input_port_ids: Vec<InputPortLocator<T>>,
    pub output_port_ids: Vec<OutputPortLocator<T>>,
    pub connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    pub system_weak_links: Vec<SystemWeakLink<T>>,
    pub registered_systems: OwnedSystems<T>,
}

impl<T: AtlasScalar> DiagramBlueprint<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct Diagram<T: AtlasScalar> {
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    registered_systems: OwnedSystems<T>,
    system_index_map: HashMap<SystemLink<T>, SubsystemIndex>,
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
    //     for (index, system) in blueprint.systems.iter().enumerate() {
    //         self.system_index_map
    //             .insert(system.clone(), SubsystemIndex::new(index));
    //     }

    //     // Every system must appear exactly once.
    //     assert_eq!(self.num_subsystems(), self.registered_systems.systems.len());

    // }

    // fn export_or_connect_input(&mut self, input_port_locator: InputPortLocator<T>) {
    //     let system = input_port_locator.system;
    //     let input_port_index = input_port_locator.input_port_index;

    //     if
    // }
}
