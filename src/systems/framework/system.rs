use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::{InputPortIndex, OutputPortIndex};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::state::State;
use crate::systems::framework::system_base::SystemBase;

pub trait System<'a, T: Add + PartialEq + Clone + Debug + Zero>: SystemBase {
    // Resource allocation and initializaion
    fn allocate_context(&mut self) -> Box<dyn Context<T>>;
    fn allocate_input_vector(&mut self, input_port: &InputPort<'a, T>) -> &BasicVector<T>;
    fn allocate_input_abstract(&mut self, input_port: &InputPort<'a, T>) -> &'a dyn AbstractValue;
    fn allocate_time_derivatives(&mut self) -> &ContinuousState<T>;
    fn create_default_context(&mut self) -> Box<dyn Context<T>>;
    fn set_default_state(&mut self, context: &dyn Context<T>, state: &State<T>);
    fn set_default_context(&mut self, context: &dyn Context<T>);

    // Cached evaluations
    fn eval_time_derivatives(&mut self, context: &mut dyn Context<T>) -> &ContinuousState<T>;
    fn get_time_derivatives_cache_entry(&self) -> &CacheEntry;

    // Calculations
    fn calc_time_derivatives(
        &mut self,
        context: &mut dyn Context<T>,
        derivatives: &mut ContinuousState<T>,
    );

    // Utility methods
    fn get_input_port(&self, index: &InputPortIndex) -> &InputPort<'a, T>;
    fn get_output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<'a, T>;
    fn num_input_ports(&self) -> usize;
    fn num_output_ports(&self) -> usize;
}
