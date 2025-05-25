use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::convert::From;

use atlas_derives::{AbstractSystem, SystemBase};

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::diagram_context::{ContextLink, DiagramContext, DiagramContextExt};
use crate::systems::framework::diagram_output_port::DiagramOutputPort;
use crate::systems::framework::framework_common::{
    CacheIndex, InputPortIndex, OutputPortIndex, SubsystemIndex, SystemId,
    SystemParentServiceInterface,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_context::LeafContext;
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

pub type LeafSystemLink<T> = Rc<RefCell<dyn System<T, CN = LeafContext<T>>>>;
pub type DiagramLink<T> = Rc<RefCell<dyn System<T, CN = DiagramContext<T>>>>;

impl<T: AtlasScalar, S> From<Rc<RefCell<S>>> for SystemLink<T>
where
    S: System<T, CN = LeafContext<T>> + 'static,
{
    fn from(system: Rc<RefCell<S>>) -> Self {
        SystemLink::LeafSystemLink(system)
    }
}

impl<T: AtlasScalar> PartialEq for SystemLink<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SystemLink::LeafSystemLink(a), SystemLink::LeafSystemLink(b)) => {
                std::ptr::addr_eq(a.as_ptr(), b.as_ptr())
            }
            (SystemLink::DiagramLink(a), SystemLink::DiagramLink(b)) => {
                std::ptr::addr_eq(a.as_ptr(), b.as_ptr())
            }
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

    pub fn input_port_mut(&mut self, input_port_index: InputPortIndex) -> RefMut<InputPort<T>> {
        match self {
            SystemLink::LeafSystemLink(system) => {
                RefMut::map(system.borrow_mut(), |s| s.input_port_mut(&input_port_index))
            }
            SystemLink::DiagramLink(system) => {
                RefMut::map(system.borrow_mut(), |s| s.input_port_mut(&input_port_index))
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

    pub fn context_sizes(&self) -> Ref<ContextSizes> {
        match self {
            SystemLink::LeafSystemLink(system) => Ref::map(system.borrow(), |s| s.context_sizes()),
            SystemLink::DiagramLink(system) => Ref::map(system.borrow(), |s| s.context_sizes()),
        }
    }

    pub fn implicit_time_derivatives_residual_size(&self) -> usize {
        match self {
            SystemLink::LeafSystemLink(system) => {
                system.borrow().implicit_time_derivatives_residual_size()
            }
            SystemLink::DiagramLink(system) => {
                system.borrow().implicit_time_derivatives_residual_size()
            }
        }
    }
}

impl<T: AtlasScalar> SystemLink<T> {
    pub fn name(&self) -> Ref<String> {
        match self {
            SystemLink::LeafSystemLink(system) => Ref::map(system.borrow(), |s| s.name()),
            SystemLink::DiagramLink(system) => Ref::map(system.borrow(), |s| s.name()),
        }
    }

    pub fn set_name(&mut self, name: String) {
        match self {
            SystemLink::LeafSystemLink(system) => system.borrow_mut().set_name(name),
            SystemLink::DiagramLink(system) => system.borrow_mut().set_name(name),
        }
    }

    pub fn output_port(
        &self,
        output_port_index: OutputPortIndex,
    ) -> Ref<dyn OutputPort<T, CN = LeafContext<T>>> {
        match self {
            SystemLink::LeafSystemLink(system) => {
                Ref::map(system.borrow(), |s| s.output_port(&output_port_index))
            }
            SystemLink::DiagramLink(_system) => {
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
            SystemLink::DiagramLink(_system) => {
                todo!()
                // Ref::map(system.borrow(), |s| s.output_port(&output_port_index))
            }
        }
    }
}

pub trait SystemLinkExt<T: AtlasScalar> {
    type CN: Context<T>;

    fn input_port(&self, input_port_index: InputPortIndex) -> Ref<InputPort<T>>;

    fn input_port_mut(&self, input_port_index: InputPortIndex) -> RefMut<InputPort<T>>;

    fn output_port(&self, output_port_index: OutputPortIndex) -> Ref<dyn OutputPort<T, CN = Self::CN>>;

    fn output_port_mut(&self, output_port_index: OutputPortIndex) -> RefMut<dyn OutputPort<T, CN = Self::CN>>;
}

impl<T: AtlasScalar, S> SystemLinkExt<T> for Rc<RefCell<S>>
where
    S: System<T, CN = LeafContext<T>>,
{
    type CN = LeafContext<T>;

    fn input_port(&self, input_port_index: InputPortIndex) -> Ref<InputPort<T>> {
        Ref::map(self.borrow(), |s| s.input_port(&input_port_index))
    }

    fn input_port_mut(&self, input_port_index: InputPortIndex) -> RefMut<InputPort<T>> {
        RefMut::map(self.borrow_mut(), |s| s.input_port_mut(&input_port_index))
    }

    fn output_port(&self, output_port_index: OutputPortIndex) -> Ref<dyn OutputPort<T, CN = Self::CN>> {
        Ref::map(self.borrow(), |s| s.output_port(&output_port_index))
    }

    fn output_port_mut(&self, output_port_index: OutputPortIndex) -> RefMut<dyn OutputPort<T, CN = Self::CN>> {
        RefMut::map(self.borrow_mut(), |s| s.output_port_mut(&output_port_index))
    }
}

impl<T: AtlasScalar> SystemLinkExt<T> for DiagramLink<T> {
    type CN = DiagramContext<T>;

    fn input_port(&self, input_port_index: InputPortIndex) -> Ref<InputPort<T>> {
        Ref::map(self.borrow(), |s| s.input_port(&input_port_index))
    }

    fn input_port_mut(&self, input_port_index: InputPortIndex) -> RefMut<InputPort<T>> {
        RefMut::map(self.borrow_mut(), |s| s.input_port_mut(&input_port_index))
    }

    fn output_port(&self, output_port_index: OutputPortIndex) -> Ref<dyn OutputPort<T, CN = Self::CN>> {
        Ref::map(self.borrow(), |s| s.output_port(&output_port_index))
    }

    fn output_port_mut(&self, output_port_index: OutputPortIndex) -> RefMut<dyn OutputPort<T, CN = Self::CN>> {
        RefMut::map(self.borrow_mut(), |s| s.output_port_mut(&output_port_index))
    }
}

#[derive(Clone, Debug)]
pub enum SystemWeakLink<T: AtlasScalar> {
    LeafSystemWeakLink(LeafSystemWeakLink<T>),
    DiagramWeakLink(DiagramWeakLink<T>),
}

type LeafSystemWeakLink<T> = Weak<RefCell<dyn System<T, CN = LeafContext<T>>>>;
type DiagramWeakLink<T> = Weak<RefCell<dyn System<T, CN = DiagramContext<T>>>>;

impl<T: AtlasScalar> PartialEq for SystemWeakLink<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SystemWeakLink::LeafSystemWeakLink(a), SystemWeakLink::LeafSystemWeakLink(b)) => {
                std::ptr::addr_eq(a.as_ptr(), b.as_ptr())
            }
            (SystemWeakLink::DiagramWeakLink(a), SystemWeakLink::DiagramWeakLink(b)) => {
                std::ptr::addr_eq(a.as_ptr(), b.as_ptr())
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
    pub fn name(&self) -> String {
        match self {
            SystemWeakLink::LeafSystemWeakLink(system) => {
                system.upgrade().unwrap().borrow().name().clone()
            }
            SystemWeakLink::DiagramWeakLink(system) => {
                system.upgrade().unwrap().borrow().name().clone()
            }
        }
    }

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
    name: String,
    input_ports: Vec<InputPort<T>>,
    output_ports: Vec<DiagramOutputPort<T>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    parent_service: Option<Weak<RefCell<dyn SystemParentServiceInterface>>>,
    implicit_time_derivatives_residual_size: Option<usize>,

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
            .map(|p| p as &dyn OutputPort<T, CN = Self::CN>)
            .collect()
    }

    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T, CN = Self::CN>> {
        self.output_ports
            .iter_mut()
            .map(|p| p as &mut dyn OutputPort<T, CN = Self::CN>)
            .collect()
    }

    fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T, CN = Self::CN> {
        &self.output_ports[index]
    }

    fn output_port_mut(
        &mut self,
        index: &OutputPortIndex,
    ) -> &mut dyn OutputPort<T, CN = Self::CN> {
        &mut self.output_ports[index]
    }

    fn system_weak_link(&self) -> SystemWeakLink<T> {
        self.system_weak_link.clone().unwrap()
    }

    fn time_derivatives_cache_index(&self) -> &CacheIndex {
        todo!()
    }

    fn allocate_context(&self) -> Rc<RefCell<Self::CN>> {
        self.do_allocate_context()
    }

    fn do_allocate_input(&self, _input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        todo!()
    }

    fn allocate_time_derivatives(&mut self) -> Box<<<Self::CN as Context<T>>::S as State<T>>::CS> {
        todo!()
    }

    fn set_default_state(&self, context: &mut Self::CN) {
        self.validate_context(context);

        for i in 0..self.num_subsystems() {
            let subcontext = context.get_context(&SubsystemIndex::new(i));
            let subsystem_link = self.registered_systems.systems[i].clone();
            match &subsystem_link {
                SystemLink::LeafSystemLink(system) => {
                    let leaf_context = subcontext.as_leaf_context().unwrap();
                    system
                        .borrow_mut()
                        .set_default_state(&mut leaf_context.borrow_mut());
                }
                SystemLink::DiagramLink(system) => {
                    let diagram_context = subcontext.as_diagram_context().unwrap();
                    system
                        .borrow_mut()
                        .set_default_state(&mut diagram_context.borrow_mut());
                }
            };
        }
    }

    fn do_calc_time_derivatives(
        &mut self,
        _context: &mut Self::CN,
        _derivatives: &mut <<Self::CN as Context<T>>::S as State<T>>::CS,
    ) {
        todo!()
    }
}

impl<T: AtlasScalar> SystemParentServiceInterface for Diagram<T> {
    fn root_system_base(&self) -> &dyn SystemBase {
        todo!()
    }

    fn eval_connected_subsystem_input_port(
        &self,
        context: &dyn ContextBase,
        input_port: &dyn InputPortBase,
    ) -> Option<Box<dyn AbstractValue>> {
        self.validate_context(context);

        let diagram_context = context
            .as_any()
            .downcast_ref::<DiagramContext<T>>()
            .expect("Context should be DiagramContext for Diagram");

        let system_weak_link = input_port
            .as_any()
            .downcast_ref::<InputPort<T>>()
            .unwrap()
            .system_weak_link();
        let id = InputPortLocator {
            system_weak_link: system_weak_link.clone(),
            input_port_index: input_port.index().clone(),
        };

        let is_exported = self.input_port_map.contains_key(&id);
        let is_connected = self.connection_map.contains_key(&id);

        if is_exported {
            Some(self.eval_abstract_input(diagram_context, self.input_port_map.get(&id).unwrap()))
        } else if is_connected {
            Some(self.eval_subsystem_output_port(
                diagram_context,
                self.connection_map.get(&id).unwrap().clone(),
            ))
        } else {
            None
        }
    }
}

impl<T: AtlasScalar> Diagram<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subsystem_index(&self, system_weak_link: &SystemWeakLink<T>) -> SubsystemIndex {
        self.system_index_map.get(system_weak_link).unwrap().clone()
    }

    pub fn do_allocate_context(&self) -> Rc<RefCell<DiagramContext<T>>> {
        let context = Rc::new(RefCell::new(DiagramContext::<T>::new(
            self.num_subsystems(),
        )));
        self.initialize_context_base(context.borrow_mut().as_mutable_base());

        for i in 0..self.num_subsystems() {
            let subsystem_link = self.registered_systems.systems[i].clone();
            let subcontext = match &subsystem_link {
                SystemLink::LeafSystemLink(system) => {
                    let leaf_context = system.borrow().allocate_context();
                    ContextLink::LeafContextLink(leaf_context)
                }
                SystemLink::DiagramLink(system) => {
                    let diagram_context = system.borrow().allocate_context();
                    ContextLink::DiagramContextLink(diagram_context)
                }
            };
            context.add_system(SubsystemIndex::new(i), subcontext);
        }

        // TODO: Add MakeState()

        // TODO: Add SubscribeDiagramCompositeTrackersToChildrens()

        // TODO: Register Subscribers and Trackers

        context
    }

    pub fn diagram_output_port(&self, index: &OutputPortIndex) -> &DiagramOutputPort<T> {
        &self.output_ports[index]
    }

    pub fn diagram_output_port_mut(
        &mut self,
        index: &OutputPortIndex,
    ) -> &mut DiagramOutputPort<T> {
        &mut self.output_ports[index]
    }

    pub fn connection_map(&self) -> &HashMap<InputPortLocator<T>, OutputPortLocator<T>> {
        &self.connection_map
    }

    pub fn num_subsystems(&self) -> usize {
        self.registered_systems.systems.len()
    }

    pub fn add_output_port(&mut self, output_port: DiagramOutputPort<T>) {
        self.output_ports.push(output_port);
    }

    pub fn create_default_context(&self) -> Rc<RefCell<DiagramContext<T>>> {
        let context = self.do_allocate_context();
        self.set_default_state(&mut context.borrow_mut());
        context
    }

    pub fn from_blueprint(blueprint: DiagramBlueprint<T>) -> Rc<RefCell<Self>> {
        let mut diagram = Rc::new(RefCell::new(Self::new()));

        unsafe {
            let diagram_weak = Rc::downgrade(&diagram);
            let diagram_weak_ptr = Weak::into_raw(diagram_weak);
            let system_weak =
                Weak::<RefCell<dyn System<T, CN = DiagramContext<T>>>>::from_raw(diagram_weak_ptr);
            diagram.borrow_mut().system_weak_link =
                Some(SystemWeakLink::DiagramWeakLink(system_weak));
        }

        diagram.initialize(blueprint);

        diagram
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
        self.add_output_port(diagram_output_port);
    }

    pub fn eval_subsystem_output_port(
        &self,
        diagram_context: &DiagramContext<T>,
        id: OutputPortLocator<T>,
    ) -> Box<dyn AbstractValue> {
        let subsystem_weak_link = id.system_weak_link.clone();
        let output_port_index = id.output_port_index.clone();
        let subsystem_index = self.subsystem_index(&subsystem_weak_link);
        let subsystem_context = diagram_context.get_context(&subsystem_index);

        match subsystem_weak_link {
            SystemWeakLink::LeafSystemWeakLink(system) => {
                let subsystem = system.upgrade().unwrap();
                let leaf_context = subsystem_context.as_leaf_context().unwrap();
                let result = subsystem
                    .borrow()
                    .output_port(&output_port_index)
                    .eval_abstract(&*leaf_context.borrow_mut());
                result
            }
            SystemWeakLink::DiagramWeakLink(system) => {
                let subsystem = system.upgrade().unwrap();
                let diagram_context = subsystem_context.as_diagram_context().unwrap();
                let result = subsystem
                    .borrow()
                    .output_port(&output_port_index)
                    .eval_abstract(&*diagram_context.borrow_mut());
                result
            }
        }
    }
}

pub trait DiagramExt<T: AtlasScalar> {
    fn input_port(&self, index: &InputPortIndex) -> Ref<InputPort<T>>;

    fn input_port_mut(&self, index: &InputPortIndex) -> RefMut<InputPort<T>>;

    fn diagram_output_port(&self, index: &OutputPortIndex) -> Ref<DiagramOutputPort<T>>;

    fn diagram_output_port_mut(&self, index: &OutputPortIndex) -> RefMut<DiagramOutputPort<T>>;

    fn initialize(&mut self, blueprint: DiagramBlueprint<T>);

    fn create_default_context(&self) -> Rc<RefCell<DiagramContext<T>>>;
}

impl<T: AtlasScalar> DiagramExt<T> for Rc<RefCell<Diagram<T>>> {
    fn input_port(&self, index: &InputPortIndex) -> Ref<InputPort<T>> {
        Ref::map(self.borrow(), |diagram| diagram.input_port(index))
    }

    fn input_port_mut(&self, index: &InputPortIndex) -> RefMut<InputPort<T>> {
        RefMut::map(self.borrow_mut(), |diagram| diagram.input_port_mut(index))
    }

    fn diagram_output_port(&self, index: &OutputPortIndex) -> Ref<DiagramOutputPort<T>> {
        Ref::map(self.borrow(), |diagram| diagram.diagram_output_port(index))
    }

    fn diagram_output_port_mut(&self, index: &OutputPortIndex) -> RefMut<DiagramOutputPort<T>> {
        RefMut::map(self.borrow_mut(), |diagram| {
            diagram.diagram_output_port_mut(index)
        })
    }

    fn initialize(&mut self, blueprint: DiagramBlueprint<T>) {
        assert!(!blueprint.registered_systems.systems.is_empty());
        assert!(self.borrow().registered_systems.systems.is_empty());

        self.borrow_mut().connection_map = blueprint.connection_map;
        self.borrow_mut().registered_systems = blueprint.registered_systems;

        // Generate a map from the System pointer to its index in the registered order.
        for (index, system) in blueprint.system_weak_links.iter().enumerate() {
            self.borrow_mut()
                .system_index_map
                .insert(system.clone(), SubsystemIndex::new(index));
        }

        // Set parent service for all subsystems
        let diagram_weak_link = Rc::downgrade(self);
        let parent_service = unsafe {
            let raw_ptr = Weak::into_raw(diagram_weak_link);
            let trait_ptr = raw_ptr as *const RefCell<dyn SystemParentServiceInterface>;
            Weak::<RefCell<dyn SystemParentServiceInterface>>::from_raw(trait_ptr)
        };

        // Set parent service for each subsystem
        for system_link in &mut self.borrow_mut().registered_systems.systems {
            match system_link {
                SystemLink::LeafSystemLink(system) => {
                    system
                        .borrow_mut()
                        .set_parent_service(parent_service.clone());
                }
                SystemLink::DiagramLink(system) => {
                    system
                        .borrow_mut()
                        .set_parent_service(parent_service.clone());
                }
            }
        }

        let mut self_borrowed_mut = self.borrow_mut();

        // Every system must appear exactly once.
        assert_eq!(
            self_borrowed_mut.num_subsystems(),
            self_borrowed_mut.registered_systems.systems.len()
        );

        // Add the inputs to the Diagram topology, and check their invariants.
        for (index, input_port_locator) in blueprint.input_port_ids.iter().enumerate() {
            self_borrowed_mut.export_or_connect_input(
                input_port_locator.clone(),
                &blueprint.input_port_names[index],
            );
        }

        for (index, output_port_locator) in blueprint.output_port_ids.iter().enumerate() {
            let subsystem_index =
                self_borrowed_mut.system_index_map[&output_port_locator.system_weak_link].clone();
            self_borrowed_mut.export_output(
                output_port_locator,
                &subsystem_index,
                &blueprint.output_port_names[index],
            );
        }
        self_borrowed_mut.output_port_ids = blueprint.output_port_ids;

        let mut residual_size = 0;
        let mut sizes = ContextSizes::default();
        for system in self_borrowed_mut.registered_systems.systems.iter() {
            sizes += &*system.context_sizes();
            residual_size += system.implicit_time_derivatives_residual_size();
        }
        self_borrowed_mut.context_sizes += &sizes;
        self_borrowed_mut.implicit_time_derivatives_residual_size = Some(residual_size);
    }

    fn create_default_context(&self) -> Rc<RefCell<DiagramContext<T>>> {
        self.borrow().create_default_context()
    }
}

#[cfg(test)]
mod tests {
    use crate::systems::framework::basic_vector::BasicVector;
    use crate::systems::framework::diagram_builder::DiagramBuilder;
    use crate::systems::primitives::adder::Adder;

    use super::*;

    #[test]
    fn test_build() {
        let mut diagram_builder = DiagramBuilder::<f64>::new();

        let adder1 = Adder::<f64>::new(2, 3);
        let adder2 = Adder::<f64>::new(2, 3);

        let mut adder1_link = diagram_builder.add_leaf_system(&adder1);
        adder1_link.set_name("adder1".to_string());
        let mut adder2_link = diagram_builder.add_leaf_system(&adder2);
        adder2_link.set_name("adder2".to_string());

        diagram_builder.export_input_port(adder1.input_port(InputPortIndex::new(0)));
        diagram_builder.export_input_port(adder1.input_port(InputPortIndex::new(1)));
        diagram_builder.export_input_port(adder2.input_port(InputPortIndex::new(0)));
        diagram_builder.export_input_port(adder2.input_port(InputPortIndex::new(1)));

        diagram_builder.export_output_port(adder1_link.output_port(OutputPortIndex::new(0)));
        diagram_builder.export_output_port(adder2_link.output_port(OutputPortIndex::new(0)));

        let diagram = diagram_builder.build();

        assert_eq!(diagram.borrow().num_subsystems(), 2);

        assert_eq!(System::<f64>::input_ports(&*diagram.borrow()).len(), 4);
        assert_eq!(System::<f64>::output_ports(&*diagram.borrow()).len(), 2);
    }

    #[test]
    fn test_connection_map() {
        let mut diagram_builder = DiagramBuilder::<f64>::new();

        let num_inputs = 2;
        let vector_size = 3;
        let adder1 = Adder::new(num_inputs, vector_size);
        let adder2 = Adder::new(num_inputs, vector_size);
        let adder3 = Adder::new(num_inputs, vector_size);

        diagram_builder.add_leaf_system(&adder1);
        diagram_builder.add_leaf_system(&adder2);
        diagram_builder.add_leaf_system(&adder3);

        diagram_builder.export_input_port(adder1.input_port(InputPortIndex::new(0)));
        diagram_builder.export_input_port(adder1.input_port(InputPortIndex::new(1)));
        diagram_builder.export_input_port(adder2.input_port(InputPortIndex::new(0)));
        diagram_builder.export_input_port(adder2.input_port(InputPortIndex::new(1)));

        diagram_builder.connect(
            adder1.output_port_mut(OutputPortIndex::new(0)),
            adder3.input_port(InputPortIndex::new(0)),
        );
        diagram_builder.connect(
            adder2.output_port_mut(OutputPortIndex::new(0)),
            adder3.input_port(InputPortIndex::new(1)),
        );

        diagram_builder.export_output_port(adder3.output_port(OutputPortIndex::new(0)));

        let diagram = diagram_builder.build();
        assert_eq!(diagram.borrow().num_subsystems(), 3);

        let diagram_context = diagram.create_default_context();

        let inputs = [
            BasicVector::<f64>::from_vec(vec![1.0, 2.0, 3.0]),
            BasicVector::<f64>::from_vec(vec![4.0, 5.0, 6.0]),
            BasicVector::<f64>::from_vec(vec![7.0, 8.0, 9.0]),
            BasicVector::<f64>::from_vec(vec![10.0, 11.0, 12.0]),
        ];

        for (i, input) in inputs.iter().enumerate() {
            diagram
                .input_port_mut(&InputPortIndex::new(i))
                .fix_value(diagram_context.borrow_mut(), input.clone());
        }

        let sum = diagram
            .diagram_output_port(&OutputPortIndex::new(0))
            .eval::<BasicVector<f64>>(diagram_context.borrow());
        let sum_expected = inputs[0].clone() + &inputs[1] + &inputs[2] + &inputs[3];
        assert_eq!(sum, sum_expected);
    }
}
