use std::any::Any;
use std::cell::{Ref, RefCell};
use std::fmt::Debug;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::diagram_context::{ContextLink, DiagramContext};
use crate::systems::framework::framework_common::{
    OutputPortIndex, PortDataType, SubsystemIndex, SystemId,
};
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::port_base::PortBase;

pub struct DiagramOutputPort<T: AtlasScalar> {
    name: String,
    subsystem_weak_link: SystemWeakLink<T>,
    subsystem_index: SubsystemIndex,
    output_port_index: OutputPortIndex,
    data_type: PortDataType,
    size: usize,
}

impl<T: AtlasScalar> PortBase for DiagramOutputPort<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn data_type(&self) -> &PortDataType {
        &self.data_type
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T: AtlasScalar> OutputPortBase for DiagramOutputPort<T> {
    fn index(&self) -> &OutputPortIndex {
        &self.output_port_index
    }
}

impl<T: AtlasScalar> OutputPort<T> for DiagramOutputPort<T> {
    type CN = DiagramContext<T>;

    fn allocate(&mut self) -> Box<dyn AbstractValue> {
        self.subsystem_weak_link
            .upgrade()
            .output_port_mut(self.output_port_index.clone())
            .allocate()
    }

    fn eval_abstract(&self, context: &mut Self::CN) -> Box<dyn AbstractValue> {
        let subcontext = context.get_context(&self.subsystem_index);
        match &self.subsystem_weak_link {
            SystemWeakLink::LeafSystemWeakLink(leaf_system_weak_link) => leaf_system_weak_link
                .upgrade()
                .unwrap()
                .borrow_mut()
                .output_port_mut(&self.output_port_index)
                .eval_abstract(&mut *subcontext.as_leaf_context().unwrap().borrow_mut()),
            SystemWeakLink::DiagramWeakLink(diagram_system_weak_link) => diagram_system_weak_link
                .upgrade()
                .unwrap()
                .borrow_mut()
                .output_port_mut(&self.output_port_index)
                .eval_abstract(&mut *subcontext.as_diagram_context().unwrap().borrow_mut()),
        }
    }

    fn calc(&self, context: &mut Self::CN, value: &mut dyn AbstractValue) {
        let subcontext = context.get_context(&self.subsystem_index);
        match (&self.subsystem_weak_link, subcontext) {
            (SystemWeakLink::LeafSystemWeakLink(sys), ContextLink::LeafContextLink(ctx)) => sys
                .upgrade()
                .unwrap()
                .borrow_mut()
                .output_port_mut(&self.output_port_index)
                .calc(&mut *ctx.borrow_mut(), value),
            (SystemWeakLink::DiagramWeakLink(sys), ContextLink::DiagramContextLink(ctx)) => sys
                .upgrade()
                .unwrap()
                .borrow_mut()
                .output_port_mut(&self.output_port_index)
                .calc(&mut *ctx.borrow_mut(), value),
            _ => panic!("Mismatch between system type and context type"),
        }
    }

    fn system_weak_link(&self) -> SystemWeakLink<T> {
        self.subsystem_weak_link.clone()
    }
}

impl<T: AtlasScalar> DiagramOutputPort<T> {
    pub fn new(
        name: String,
        subsystem_weak_link: SystemWeakLink<T>,
        subsystem_index: SubsystemIndex,
        output_port_index: OutputPortIndex,
    ) -> Self {
        let data_type = subsystem_weak_link
            .upgrade()
            .output_port_mut(output_port_index.clone())
            .data_type()
            .clone();
        let size = subsystem_weak_link
            .upgrade()
            .output_port_mut(output_port_index.clone())
            .size();
        DiagramOutputPort::<T> {
            name,
            subsystem_weak_link,
            subsystem_index,
            output_port_index,
            data_type,
            size,
        }
    }

    pub fn eval<ValueType: Clone + Debug + 'static>(
        &self,
        context: &mut <Self as OutputPort<T>>::CN,
    ) -> ValueType {
        self.eval_abstract(context)
            .as_any()
            .downcast_ref::<Value<ValueType>>()
            .unwrap()
            .value()
            .clone()
    }
}
