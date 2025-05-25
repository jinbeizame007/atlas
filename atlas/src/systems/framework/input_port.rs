use std::any::Any;
use std::fmt::Debug;
use std::ops::DerefMut;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::{InputPortIndex, PortDataType, SystemId};
use crate::systems::framework::input_port_base::{EvalAbstractCallback, InputPortBase};
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::value_producer::AllocateCallback;

pub struct InputPort<T: AtlasScalar> {
    name: String,
    system_weak_link: SystemWeakLink<T>,
    _system_id: SystemId,
    index: InputPortIndex,
    data_type: PortDataType,
    size: usize,
    eval: Box<EvalAbstractCallback>,
    alloc: Box<AllocateCallback>,
}

impl<T: AtlasScalar> PortBase for InputPort<T> {
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

impl<T: AtlasScalar> InputPortBase for InputPort<T> {
    fn index(&self) -> &InputPortIndex {
        &self.index
    }

    fn allocate(&mut self) -> Box<dyn AbstractValue> {
        (self.alloc)()
    }
}

impl<T: AtlasScalar> InputPort<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        system_weak_link: SystemWeakLink<T>,
        _system_id: SystemId,
        index: InputPortIndex,
        data_type: PortDataType,
        size: usize,
        eval: Box<EvalAbstractCallback>,
        alloc: Box<AllocateCallback>,
    ) -> Self {
        InputPort {
            name,
            system_weak_link,
            _system_id,
            index,
            data_type,
            size,
            eval,
            alloc,
        }
    }

    pub fn data_type(&self) -> &PortDataType {
        &self.data_type
    }

    pub fn system_weak_link(&self) -> &SystemWeakLink<T> {
        &self.system_weak_link
    }

    pub fn index(&self) -> &InputPortIndex {
        &self.index
    }

    pub fn set_eval(&mut self, eval: Box<EvalAbstractCallback>) {
        self.eval = eval;
    }

    pub fn eval<S: State<T> + 'static, ValueType: Clone + Debug + 'static>(
        &self,
        context: &dyn Context<T, S = S>,
    ) -> ValueType {
        let context_base = context.as_base();
        let abstract_value = (self.eval)(context_base);
        self.port_eval_cast::<ValueType>(abstract_value.as_ref())
    }

    fn port_eval_cast<ValueType: Clone + Debug + 'static>(
        &self,
        abstract_value: &dyn AbstractValue,
    ) -> ValueType {
        abstract_value
            .as_any()
            .downcast_ref::<Value<ValueType>>()
            .unwrap()
            .value()
            .clone()
    }

    pub fn set_alloc(&mut self, alloc: Box<AllocateCallback>) {
        self.alloc = alloc;
    }

    pub fn fix_value<CN, S, ValueType: Clone + Debug + 'static>(
        &self,
        mut context: CN,
        value: ValueType,
    ) -> Option<FixedInputPortValue>
    where
        CN: DerefMut,
        CN::Target: Context<T, S = S>,
    {
        let context = &mut *context;
        let abstract_value = Value::<ValueType>::new(value);

        if context
            .fix_input_port(self.index().value(), &abstract_value)
            .is_some()
        {
            Some(FixedInputPortValue::new(Box::new(abstract_value)))
        } else {
            None
        }
    }
}
