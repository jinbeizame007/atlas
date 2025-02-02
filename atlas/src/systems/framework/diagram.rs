use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use atlas_derives::{AbstractSystem, LeafSystem, System, SystemBase};

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::diagram_context::DiagramContext;
use crate::systems::framework::diagram_output_port::DiagramOutputPort;
use crate::systems::framework::framework_common::{
    CacheIndex, InputPortIndex, OutputPortIndex, SubsystemIndex, SystemId,
    SystemParentServiceInterface,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::leaf_system::LeafSystem;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::system::{AbstractSystem, System};
use crate::systems::framework::system_base::ContextSizes;
use crate::systems::framework::system_base::SystemBase;

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

impl<T: AtlasScalar> SystemLink<T> {
    pub fn output_port(
        &self,
        output_port_index: OutputPortIndex,
    ) -> Ref<dyn OutputPort<T, CN = LeafContext<T>>> {
        match self {
            SystemLink::LeafSystemLink(system) => {
                Ref::map(system.borrow(), |s| s.output_port(&output_port_index))
            }
            SystemLink::DiagramLink(system) => {
                todo!()
                // Ref::map(system.borrow(), |s| s.output_port(&output_port_index))
            }
        }
    }

    pub fn output_port_mut(
        &mut self,
        output_port_index: OutputPortIndex,
    ) -> RefMut<dyn OutputPort<T, CN = LeafContext<T>>> {
        match self {
            SystemLink::LeafSystemLink(system) => RefMut::map(system.borrow_mut(), |s| {
                s.output_port_mut(&output_port_index)
            }),
            SystemLink::DiagramLink(system) => {
                todo!()
                // Ref::map(system.borrow(), |s| s.output_port(&output_port_index))
            }
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
    pub fn upgrade(&self) -> SystemLink<T> {
        match self {
            SystemWeakLink::LeafSystemWeakLink(system) => {
                SystemLink::LeafSystemLink(system.upgrade().unwrap())
            }
            SystemWeakLink::DiagramWeakLink(system) => {
                SystemLink::DiagramLink(system.upgrade().unwrap())
            }
        }
    }

    pub fn as_leaf_system_weak_link(&self) -> Option<&LeafSystemWeakLink<T>> {
        match self {
            SystemWeakLink::LeafSystemWeakLink(ref leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn as_diagram_weak_link(&self) -> Option<&DiagramWeakLink<T>> {
        match self {
            SystemWeakLink::DiagramWeakLink(ref diagram) => Some(diagram),
            _ => None,
        }
    }

    pub fn has_input_port(&self, name: &str) -> bool {
        match self {
            SystemWeakLink::LeafSystemWeakLink(system) => {
                system.upgrade().unwrap().borrow().has_input_port(name)
            }
            SystemWeakLink::DiagramWeakLink(system) => {
                system.upgrade().unwrap().borrow().has_input_port(name)
            }
        }
    }

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

// /* ~~~ OutputPort Links ~~~ */
// type LeafOutputPortLink<T: AtlasScalar> = Rc<RefCell<dyn OutputPort<T, CN = LeafContext<T>>>>;
// type DiagramOutputPortLink<T: AtlasScalar> = Rc<RefCell<dyn OutputPort<T, CN = DiagramContext<T>>>>;

// #[derive(Clone)]
// pub enum OutputPortLink<T: AtlasScalar> {
//     LeafOutputPortLink(LeafOutputPortLink<T>),
//     DiagramOutputPortLink(DiagramOutputPortLink<T>),
// }

// impl<T: AtlasScalar> OutputPortLink<T> {
//     pub fn eval_abstract(&self, context: &mut dyn Context<T>) -> Box<dyn AbstractValue> {
//         match self {
//             OutputPortLink::LeafOutputPortLink(port) => port.borrow().eval_abstract(context),
//             OutputPortLink::DiagramOutputPortLink(port) => port.borrow().eval_abstract(context),
//         }
//     }

//     pub fn allocate(&self) -> Box<dyn AbstractValue> {
//         match self {
//             OutputPortLink::LeafOutputPortLink(port) => port.borrow().allocate(),
//             OutputPortLink::DiagramOutputPortLink(port) => port.borrow().allocate(),
//         }
//     }

//     pub fn calc(&self, context: &mut dyn Context<T>, value: &mut dyn AbstractValue) {
//         match self {
//             OutputPortLink::LeafOutputPortLink(port) => port.borrow().calc(context, value),
//             OutputPortLink::DiagramOutputPortLink(port) => port.borrow().calc(context, value),
//         }
//     }

//     pub fn system_weak_link(&self) -> SystemWeakLink<T> {
//         match self {
//             OutputPortLink::LeafOutputPortLink(port) => port.borrow().system_weak_link(),
//             OutputPortLink::DiagramOutputPortLink(port) => port.borrow().system_weak_link(),
//         }
//     }
// }

// type LeafOutputPortWeakLink<T: AtlasScalar> = Weak<RefCell<dyn OutputPort<T, CN = LeafContext<T>>>>;
// type DiagramOutputPortWeakLink<T: AtlasScalar> =
//     Weak<RefCell<dyn OutputPort<T, CN = DiagramContext<T>>>>;

// #[derive(Clone)]
// pub enum OutputPortWeakLink<T: AtlasScalar> {
//     LeafOutputPortWeakLink(LeafOutputPortWeakLink<T>),
//     DiagramOutputPortWeakLink(DiagramOutputPortWeakLink<T>),
// }

// impl<T: AtlasScalar> OutputPortWeakLink<T> {
//     pub fn eval_abstract(&self, context: &mut dyn Context<T>) -> Box<dyn AbstractValue> {
//         match self {
//             OutputPortWeakLink::LeafOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().eval_abstract(context)
//             }
//             OutputPortWeakLink::DiagramOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().eval_abstract(context)
//             }
//         }
//     }

//     pub fn allocate(&self) -> Box<dyn AbstractValue> {
//         match self {
//             OutputPortWeakLink::LeafOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().allocate()
//             }
//             OutputPortWeakLink::DiagramOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().allocate()
//             }
//         }
//     }

//     pub fn calc(&self, context: &mut dyn Context<T>, value: &mut dyn AbstractValue) {
//         match self {
//             OutputPortWeakLink::LeafOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().calc(context, value)
//             }
//             OutputPortWeakLink::DiagramOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().calc(context, value)
//             }
//         }
//     }

//     pub fn system_weak_link(&self) -> SystemWeakLink<T> {
//         match self {
//             OutputPortWeakLink::LeafOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().system_weak_link()
//             }
//             OutputPortWeakLink::DiagramOutputPortWeakLink(port) => {
//                 port.upgrade().unwrap().borrow().system_weak_link()
//             }
//         }
//     }
// }

/* ~~~ Input/Output Port Locators ~~~ */

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
    pub input_port_names: Vec<String>,
    pub output_port_ids: Vec<OutputPortLocator<T>>,
    pub output_port_names: Vec<String>,
    pub connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    pub system_weak_links: Vec<SystemWeakLink<T>>,
    pub registered_systems: OwnedSystems<T>,
}

impl<T: AtlasScalar> DiagramBlueprint<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, SystemBase, AbstractSystem)]
pub struct Diagram<T: AtlasScalar> {
    // SystemBase
    input_ports: Vec<InputPort<T>>,
    output_ports: Vec<Box<DiagramOutputPort<T>>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    parent_service: Option<Box<dyn SystemParentServiceInterface>>,

    // System
    system_weak_link: Option<SystemWeakLink<T>>,

    // Diagram
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    registered_systems: OwnedSystems<T>,
    system_index_map: HashMap<SystemWeakLink<T>, SubsystemIndex>,
    input_port_map: HashMap<InputPortLocator<T>, InputPortIndex>,
    output_port_ids: Vec<OutputPortLocator<T>>,
}

impl<T: AtlasScalar> System<T> for Diagram<T> {
    type CN = DiagramContext<T>;

    fn input_ports(&self) -> Vec<&InputPort<T>> {
        self.input_ports.iter().collect()
    }

    fn input_ports_mut(&mut self) -> Vec<&mut InputPort<T>> {
        self.input_ports.iter_mut().collect()
    }

    fn input_port(&self, index: &InputPortIndex) -> &InputPort<T> {
        &self.input_ports[index]
    }

    fn input_port_mut(&mut self, index: &InputPortIndex) -> &mut InputPort<T> {
        &mut self.input_ports[index]
    }

    fn add_input_port(&mut self, input_port: InputPort<T>) {
        self.input_ports.push(input_port);
    }

    fn output_ports(&self) -> Vec<&dyn OutputPort<T, CN = Self::CN>> {
        self.output_ports
            .iter()
            .map(|p| p.as_ref() as &dyn OutputPort<T, CN = Self::CN>)
            .collect()
    }

    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T, CN = Self::CN>> {
        self.output_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn OutputPort<T, CN = Self::CN>)
            .collect()
    }

    fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T, CN = Self::CN> {
        self.output_ports[index].as_ref()
    }

    fn output_port_mut(
        &mut self,
        index: &OutputPortIndex,
    ) -> &mut dyn OutputPort<T, CN = Self::CN> {
        self.output_ports[index].as_mut()
    }

    fn system_weak_link(&self) -> SystemWeakLink<T> {
        self.system_weak_link.clone().unwrap()
    }

    fn time_derivatives_cache_index(&self) -> &CacheIndex {
        todo!()
    }

    fn allocate_context(&self) -> Box<Self::CN> {
        todo!()
    }

    fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        todo!()
    }

    fn allocate_time_derivatives(&mut self) -> Box<<<Self::CN as Context<T>>::S as State<T>>::CS> {
        todo!()
    }

    fn set_default_state(&self, context: &mut Self::CN) {
        todo!()
    }

    fn do_calc_time_derivatives(
        &mut self,
        context: &mut Self::CN,
        derivatives: &mut <<Self::CN as Context<T>>::S as State<T>>::CS,
    ) {
        todo!()
    }
}

impl<T: AtlasScalar> Diagram<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_subsystems(&self) -> usize {
        self.registered_systems.systems.len()
    }

    pub fn add_output_port(&mut self, output_port: Box<DiagramOutputPort<T>>) {
        self.output_ports.push(output_port);
    }

    pub fn from_blueprint(blueprint: DiagramBlueprint<T>) -> Self {
        let mut diagram = Self::new();
        diagram.initialize(blueprint);
        diagram
    }

    pub fn initialize(&mut self, blueprint: DiagramBlueprint<T>) {
        assert!(!blueprint.registered_systems.systems.is_empty());
        assert!(self.registered_systems.systems.is_empty());

        self.connection_map = blueprint.connection_map;
        self.registered_systems = blueprint.registered_systems;

        // Generate a map from the System pointer to its index in the registered order.
        for (index, system) in blueprint.system_weak_links.iter().enumerate() {
            self.system_index_map
                .insert(system.clone(), SubsystemIndex::new(index));
        }

        // Every system must appear exactly once.
        assert_eq!(self.num_subsystems(), self.registered_systems.systems.len());

        // Add the inputs to the Diagram topology, and check their invariants.
        for (index, input_port_locator) in blueprint.input_port_ids.iter().enumerate() {
            self.export_or_connect_input(
                input_port_locator.clone(),
                &blueprint.input_port_names[index],
            );
        }

        for (index, output_port_locator) in blueprint.output_port_ids.iter().enumerate() {
            let subsystem_index =
                self.system_index_map[&output_port_locator.system_weak_link].clone();
            self.export_output(
                output_port_locator,
                &subsystem_index,
                &blueprint.output_port_names[index],
            );
        }
        self.output_port_ids = blueprint.output_port_ids;

        // TODO: Set implicit time derivatives residual size.
    }

    fn export_or_connect_input(&mut self, input_port_locator: InputPortLocator<T>, name: &str) {
        if !input_port_locator.system_weak_link.has_input_port(name) {
            self.input_port_map.insert(
                input_port_locator.clone(),
                input_port_locator.input_port_index,
            );
        } else {
            let subsystem_link = input_port_locator.system_weak_link.upgrade();
            let subsystem_input_port =
                subsystem_link.input_port(input_port_locator.input_port_index.clone());
            let new_port = self.declare_input_port(
                name.to_string(),
                subsystem_input_port.data_type().clone(),
                subsystem_input_port.size(),
            );
            let input_port_index = new_port.index().clone();
            self.input_port_map
                .insert(input_port_locator.clone(), input_port_index);
        }
    }

    fn export_output(
        &mut self,
        output_port_locator: &OutputPortLocator<T>,
        subsystem_index: &SubsystemIndex,
        name: &str,
    ) {
        let subsystem_weak_link = output_port_locator.system_weak_link.clone();
        let diagram_output_port = DiagramOutputPort::new(
            name.to_string(),
            subsystem_weak_link.clone(),
            subsystem_index.clone(),
            output_port_locator.output_port_index.clone(),
        );
        self.add_output_port(Box::new(diagram_output_port));
    }
}
