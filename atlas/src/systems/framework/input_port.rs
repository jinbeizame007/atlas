use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::context::Context;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::{InputPortIndex, PortDataType, SystemId};
use crate::systems::framework::input_port_base::{EvalAbstractCallback, InputPortBase};
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::value_producer::AllocateCallback;

pub struct InputPort<T: AtlasScalar> {
    _system_id: SystemId,
    index: InputPortIndex,
    data_type: PortDataType,
    size: usize,
    eval: Box<EvalAbstractCallback>,
    alloc: Box<AllocateCallback>,
    _phantom: PhantomData<T>,
}

impl<T: AtlasScalar> PortBase for InputPort<T> {
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
    pub fn new(
        _system_id: SystemId,
        index: InputPortIndex,
        data_type: PortDataType,
        size: usize,
        eval: Box<EvalAbstractCallback>,
        alloc: Box<AllocateCallback>,
    ) -> Self {
        InputPort::<T> {
            _system_id,
            index,
            data_type,
            size,
            eval,
            alloc,
            _phantom: PhantomData::<T>,
        }
    }

    pub fn set_eval(&mut self, eval: Box<EvalAbstractCallback>) {
        self.eval = eval;
    }

    pub fn eval<S: State<T> + 'static, ValueType: Clone + Debug + 'static>(
        &self,
        context: &mut dyn Context<T, S = S>,
    ) -> ValueType {
        let context_base = context.as_mutable_base();
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

    pub fn fix_value<'a, ValueType: Clone + Debug + 'static>(
        &self,
        context: &'a mut impl Context<T>,
        value: ValueType,
    ) -> Option<&'a FixedInputPortValue> {
        let abstract_value = Value::<ValueType>::new(value);

        context.fix_input_port(self.index().value(), &abstract_value)
    }
}
