use std::cell::RefCell;
use std::rc::Rc;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::cache::Cache;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::diagram_state::DiagramState;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::{SubsystemIndex, SystemId};
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::state::State;
use crate::systems::framework::vector_base::VectorBase;

pub enum ContextPtr<T: AtlasScalar> {
    LeafContextPtr(*mut LeafContext<T>),
    DiagramContextPtr(*mut DiagramContext<T>),
}

impl<T: AtlasScalar> ContextPtr<T> {
    pub fn as_leaf_context(&mut self) -> &mut LeafContext<T> {
        match self {
            ContextPtr::LeafContextPtr(ptr) => unsafe { &mut **ptr },
            ContextPtr::DiagramContextPtr(ptr) => {
                todo!()
            }
        }
    }
}

#[derive(Default)]
pub struct DiagramContext<T: AtlasScalar> {
    system_id: SystemId,
    parent: Option<Rc<RefCell<dyn ContextBase>>>,
    cache: Cache,
    time: T,
    state: DiagramState<T>,
    input_port_values: Vec<Option<FixedInputPortValue>>,
    is_context_base_initialized: bool,
    contexts: Vec<ContextPtr<T>>,
}

impl<T: AtlasScalar> ContextBase for DiagramContext<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn set_system_id(&mut self, system_id: SystemId) {
        self.system_id = system_id;
    }

    fn system_id(&self) -> &SystemId {
        &self.system_id
    }

    fn parent_base(&self) -> &Option<Rc<RefCell<dyn ContextBase>>> {
        &self.parent
    }

    fn parent_base_mut(&mut self) -> &mut Option<Rc<RefCell<dyn ContextBase>>> {
        &mut self.parent
    }

    fn cache(&self) -> &Cache {
        &self.cache
    }

    fn cache_mut(&mut self) -> &mut Cache {
        &mut self.cache
    }

    fn input_port_values(&mut self) -> &mut Vec<Option<FixedInputPortValue>> {
        &mut self.input_port_values
    }

    fn num_input_ports(&self) -> usize {
        self.input_port_values.len()
    }

    fn fix_input_port(
        &mut self,
        index: usize,
        value: &dyn AbstractValue,
    ) -> Option<&FixedInputPortValue> {
        self.input_port_values[index] = Some(FixedInputPortValue::new(value.clone_box()));

        self.fixed_input_port_value(index)
    }

    fn fixed_input_port_value(&self, index: usize) -> Option<&FixedInputPortValue> {
        self.input_port_values[index].as_ref()
    }

    fn fixed_input_port_value_mut(&mut self, index: usize) -> Option<&mut FixedInputPortValue> {
        self.input_port_values[index].as_mut()
    }

    fn is_context_base_initialized_mut(&mut self) -> &mut bool {
        &mut self.is_context_base_initialized
    }
}

impl<T: AtlasScalar> Context<T> for DiagramContext<T> {
    type S = DiagramState<T>;

    fn time(&self) -> &T {
        &self.time
    }

    fn state(&self) -> &Self::S {
        &self.state
    }

    fn state_mut(&mut self) -> &mut Self::S {
        &mut self.state
    }

    fn init_continuous_state(&mut self, continuous_state: Box<<Self::S as State<T>>::CS>) {
        self.set_continuous_state(continuous_state);
    }

    fn num_continuous_states(&self) -> usize {
        self.state.continuous_state().size()
    }

    fn continuous_state(&self) -> &<Self::S as State<T>>::CS {
        self.state.continuous_state()
    }

    fn continuous_state_mut(&mut self) -> &mut <Self::S as State<T>>::CS {
        self.state.continuous_state_mut()
    }

    fn continuous_state_vector(&self) -> &dyn VectorBase<T, Output = T> {
        self.state.continuous_state().vector()
    }

    fn continuous_state_vector_mut(&mut self) -> &mut dyn VectorBase<T, Output = T> {
        self.state.continuous_state_mut().vector_mut()
    }

    fn as_base(&self) -> &dyn ContextBase {
        self
    }

    fn as_mutable_base(&mut self) -> &mut dyn ContextBase {
        self
    }
}

impl<T: AtlasScalar> DiagramContext<T> {
    pub fn add_system<CN: Context<T>>(&mut self, index: SubsystemIndex, context: &mut CN) {}

    pub fn get_subcontext(&mut self, subsystem_index: &SubsystemIndex) -> &mut LeafContext<T> {
        self.contexts[subsystem_index].as_leaf_context()
    }
}
