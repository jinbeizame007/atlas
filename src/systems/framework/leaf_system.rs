use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::context::Context;
use crate::systems::framework::framework_common::ContinuousStateIndex;
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::system::System;

pub trait LeafSystem<'a, T: Add + PartialEq + Clone + Debug + Zero>: System<'a, T> {
    fn allocate_context(&mut self) -> LeafContext<T>;
    fn declate_continuous_state(
        &mut self,
        num_q: usize,
        num_v: usize,
        num_z: usize,
    ) -> &ContinuousStateIndex;

    // Declare input port
    fn declare_vector_input_port(&mut self, size: usize) -> &InputPort<'a, T>;
    fn declare_abstract_input_port(&mut self, model_value: &dyn AbstractValue);

    // Declare output port
    #[allow(clippy::type_complexity)]
    fn declare_vector_output_port(
        &mut self,
        size: usize,
        calc: Box<dyn Fn(&dyn System<'a, T>, &mut dyn Context<T>, &mut BasicVector<T>)>,
    );
    #[allow(clippy::type_complexity)]
    fn declare_abstract_output_port<OutputType>(
        &mut self,
        calc: Box<dyn Fn(&dyn System<'a, T>, &mut dyn Context<T>, &mut OutputType)>,
    );
}
