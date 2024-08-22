use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::{
    CacheIndex, InputPortIndex, OutputPortIndex, PortDataType,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::system_base::SystemBase;

pub trait System<'a, T: Add + PartialEq + Clone + Debug + Zero + 'static>: SystemBase {
    // Resource allocation and initializaion
    fn allocate_context(&mut self) -> Box<dyn Context<T>>;
    // fn allocate_context(&mut self) -> Box<dyn Context<T>> {
    //     self.do_allocate_context().as_ref().
    // }
    fn do_allocate_input(&mut self, input_port: &InputPort<'a, T>) -> Box<dyn AbstractValue>;
    fn allocate_input_vector(&mut self, input_port: &InputPort<'a, T>) -> BasicVector<T> {
        assert!(*input_port.get_data_type() == PortDataType::VectorValued);
        let self_input_port_base = self.get_input_port_base(input_port.get_index());
        assert!(std::ptr::eq(
            self_input_port_base,
            input_port as &dyn InputPortBase
        ));
        let value = self.do_allocate_input(input_port);

        value
            .as_any()
            .downcast_ref::<BasicVector<T>>()
            .unwrap()
            .clone()
    }
    fn allocate_input_abstract(&mut self, input_port: &InputPort<'a, T>) -> Box<dyn AbstractValue> {
        self.do_allocate_input(input_port)
    }
    fn allocate_time_derivatives(&mut self) -> &ContinuousState<T>;
    fn create_default_context(&mut self) -> Box<dyn Context<T>> {
        let mut context = self.allocate_context();
        self.set_default_context(context.as_mut());
        context
    }
    fn set_default_state(&self, state: &State<T>);
    fn set_default_context(&mut self, context: &mut dyn Context<T>) {
        self.set_default_state(context.get_mutable_state());
    }

    // Cached evaluations
    fn eval_time_derivatives(&mut self, context: &'a mut dyn Context<T>) -> &'a ContinuousState<T> {
        let cache_entry = self.get_time_derivatives_cache_entry();

        cache_entry.eval(context.as_mutable_base())
    }
    fn get_time_derivatives_cache_entry(&self) -> &CacheEntry {
        self.get_cache_entry(self.get_time_derivatives_cache_index())
    }
    fn get_time_derivatives_cache_index(&self) -> &CacheIndex;

    // Calculations
    fn calc_time_derivatives(
        &mut self,
        context: &mut dyn Context<T>,
        derivatives: &mut ContinuousState<T>,
    );

    // Utility methods
    fn get_input_ports(&self) -> &Vec<InputPort<'a, T>>;
    fn get_mutable_input_ports(&mut self) -> &mut Vec<InputPort<'a, T>>;
    fn get_input_port(&self, index: &InputPortIndex) -> &InputPort<'a, T>;
    fn get_output_ports(&self) -> &Vec<Box<dyn OutputPort<'a, T>>>;
    fn get_mutable_output_ports(&mut self) -> &mut Vec<Box<dyn OutputPort<'a, T>>>;
    fn get_output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<'a, T>;
}
