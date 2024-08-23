use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::context::Context;
use crate::systems::framework::fixed_input_port_value::FixedInputPortValue;
use crate::systems::framework::framework_common::{InputPortIndex, PortDataType, SystemId};
use crate::systems::framework::input_port_base::{EvalAbstractCallback, InputPortBase};
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::value_producer::AllocateCallback;

pub struct InputPort<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> {
    // TODO: Consider add a reference to the system
    // system: &'a dyn System<'a, T>,
    _system_id: SystemId,
    index: InputPortIndex,
    data_type: PortDataType,
    size: usize,
    eval: Box<EvalAbstractCallback>,
    alloc: Box<AllocateCallback>,
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> PortBase for InputPort<T> {
    fn get_data_type(&self) -> &PortDataType {
        &self.data_type
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> InputPortBase for InputPort<T> {
    fn get_index(&self) -> &InputPortIndex {
        &self.index
    }

    fn allocate(&mut self) -> Box<dyn AbstractValue> {
        (self.alloc)()
    }
}

impl<T: Add + PartialEq + Clone + Debug + Default + Zero + 'static> InputPort<T> {
    pub fn new(
        _system_id: SystemId,
        index: InputPortIndex,
        data_type: PortDataType,
        size: usize,
        eval: Box<EvalAbstractCallback>,
        alloc: Box<AllocateCallback>,
    ) -> Self {
        InputPort::<T> {
            // system,
            _system_id,
            index,
            data_type,
            size,
            eval,
            alloc,
        }
    }

    pub fn eval<ValueType: Clone + 'static>(&mut self, context: &dyn Context<T>) -> ValueType {
        let context_base = context.as_mutable_base();
        let abstract_value = (self.eval)(context_base);
        self.port_eval_cast::<ValueType>(abstract_value.as_ref())
    }

    fn port_eval_cast<ValueType: Clone + 'static>(
        &self,
        abstract_value: &dyn AbstractValue,
    ) -> ValueType {
        abstract_value
            .as_any()
            .downcast_ref::<ValueType>()
            .unwrap()
            .clone()
    }

    pub fn fix_value<ValueType: Clone + 'static>(
        &self,
        context: &mut dyn Context<T>,
        value: ValueType,
    ) -> &FixedInputPortValue {
        let abstract_value = Box::new(Value::<ValueType>::new(value)) as Box<dyn AbstractValue>;
        context.fix_input_port(self.get_index().value(), abstract_value.as_ref())
    }
}
