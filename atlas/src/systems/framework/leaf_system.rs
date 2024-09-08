use std::fmt::Debug;

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::{AbstractValue, Value};
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::framework_common::{
    ContinuousStateIndex, OutputPortIndex, PortDataType,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::leaf_continuous_state::LeafContinuousState;
use crate::systems::framework::leaf_output_port::LeafOutputPort;
use crate::systems::framework::model_values::ModelValues;
use crate::systems::framework::port_base::PortBase;
use crate::systems::framework::system::System;
use crate::systems::framework::value_producer::{AllocateCallback, ValueProducer};

pub trait LeafSystem<T: AtlasScalar>: System<T, CN = LeafContext<T>> {
    // Getters and setters without default implementations
    fn model_input_values(&self) -> &ModelValues;
    fn model_input_values_mut(&mut self) -> &mut ModelValues;
    fn model_continuous_state_vector(&self) -> &BasicVector<T>;
    fn model_continuous_state_vector_mut(&mut self) -> &mut BasicVector<T>;
    fn leaf_output_port(&self, output_port_index: &OutputPortIndex) -> &LeafOutputPort<T>;
    fn leaf_output_port_mut(
        &mut self,
        output_port_index: &OutputPortIndex,
    ) -> &mut LeafOutputPort<T>;
    fn add_output_port(&mut self, output_port: Box<LeafOutputPort<T>>);

    fn allocate_context(&self) -> Box<LeafContext<T>> {
        self.do_allocate_context()
    }

    fn do_allocate_context(&self) -> Box<LeafContext<T>> {
        let mut context = self.do_make_leaf_context();
        self.initialize_context_base(context.as_mutable_base());

        Box::new(context)
    }

    fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        if let Some(model_result) = self
            .model_input_values()
            .clone_model(input_port.index().value())
        {
            model_result
        } else if *input_port.data_type() == PortDataType::VectorValued {
            Box::new(Value::<BasicVector<T>>::new(BasicVector::<T>::zeros(
                input_port.size(),
            )))
        } else {
            panic!("a System with abstract input ports must pass a model_value to declare_abstract_input_port()");
        }
    }

    fn do_make_leaf_context(&self) -> LeafContext<T> {
        LeafContext::<T>::default()
    }

    fn allocate_time_derivatives(&mut self) -> Box<LeafContinuousState<T>> {
        self.allocate_continuous_state()
    }
    fn allocate_continuous_state(&self) -> Box<LeafContinuousState<T>> {
        let context_sizes = self.context_sizes();
        let mut continuous_state = Box::new(LeafContinuousState::<T>::new(
            Box::new(self.model_continuous_state_vector().clone()),
            context_sizes.num_generalized_positions,
            context_sizes.num_generalized_velocities,
            context_sizes.num_misc_continuous_states,
        ));
        continuous_state.set_system_id(self.system_id().clone());

        continuous_state
    }

    fn set_model_continuous_state_vector(&mut self, model_continuous_state_vector: BasicVector<T>) {
        *self.model_continuous_state_vector_mut() = model_continuous_state_vector;
    }

    fn set_default_state(&self, context: &mut Self::CN) {
        self.validate_context(context.as_base());

        let continuous_state = context.continuous_state_mut();
        continuous_state.set_from_vector(self.model_continuous_state_vector().value());
    }

    fn declare_continuous_state(
        &mut self,
        num_q: usize,
        num_v: usize,
        num_z: usize,
    ) -> ContinuousStateIndex {
        let n = num_q + num_v + num_z;
        let model_continuous_state_vector =
            BasicVector::<T>::new(na::DVector::<T>::from_element(n, T::default()));
        self.set_model_continuous_state_vector(model_continuous_state_vector);

        let context_sizes = self.context_sizes_mut();
        context_sizes.num_generalized_positions = num_q;
        context_sizes.num_generalized_velocities = num_v;
        context_sizes.num_misc_continuous_states = num_z;

        ContinuousStateIndex::new(0)
    }

    // Declare input port
    fn declare_vector_input_port(&mut self, size: usize) -> &InputPort<T>
    where
        Self: Sized,
    {
        let model_vector = BasicVector::<T>::zeros(size);
        let input_port_index = self.num_input_ports();
        self.model_input_values_mut()
            .add_vector_model(input_port_index, model_vector);

        self.declare_input_port(PortDataType::VectorValued, size)
    }
    fn declare_abstract_input_port(&mut self, model_value: &dyn AbstractValue) -> &InputPort<T>
    where
        Self: Sized,
    {
        let next_input_port_index = self.num_input_ports();
        let model_input_values = self.model_input_values_mut();
        model_input_values.add_model(next_input_port_index, model_value.clone_box());

        self.declare_input_port(PortDataType::AbstractValued, 0)
    }

    // Declare output port
    #[allow(clippy::type_complexity)]
    fn declare_vector_output_port(
        &mut self,
        size: usize,
        calc: Box<dyn Fn(&mut Self::CN, &mut BasicVector<T>)>,
    ) -> &LeafOutputPort<T> {
        let model_vector = BasicVector::<T>::zeros(size);
        self.create_vector_leaf_output_port(size, Self::make_allocate_callback(model_vector), calc)
    }

    // fn make_allocate_callback<OutputType: Clone + Debug + 'static>(
    fn make_allocate_callback<OutputType: Clone + Debug + 'static>(
        model_value: OutputType,
    ) -> Box<AllocateCallback> {
        Box::new(move || {
            Box::new(Value::<OutputType>::new(model_value.clone())) as Box<dyn AbstractValue>
        })
    }

    #[allow(clippy::type_complexity)]
    fn declare_abstract_output_port(
        &mut self,
        alloc: Box<AllocateCallback>,
        calc: Box<dyn Fn(&mut Self::CN, &mut dyn AbstractValue)>,
    ) -> &LeafOutputPort<T> {
        let calc_ = Box::new(
            move |context_base: &mut dyn ContextBase, abstract_value: &mut dyn AbstractValue| {
                let leaf_context = context_base
                    .as_any_mut()
                    .downcast_mut::<Self::CN>()
                    .unwrap();
                // let context = leaf_context.as_context_mut();
                (calc)(leaf_context, abstract_value)
            },
        );
        let value_producer = ValueProducer::new(alloc, calc_);

        self.create_abstract_leaf_output_port(value_producer)
    }

    #[allow(clippy::type_complexity)]
    fn create_vector_leaf_output_port(
        &mut self,
        fixed_size: usize,
        alloc: Box<AllocateCallback>,
        calc: Box<dyn Fn(&mut Self::CN, &mut BasicVector<T>)>,
    ) -> &LeafOutputPort<T> {
        let cache_calc = Box::new(
            move |context_base: &mut dyn ContextBase, abstract_value: &mut dyn AbstractValue| {
                let leaf_context = context_base
                    .as_any_mut()
                    .downcast_mut::<Self::CN>()
                    .unwrap();
                // let context = leaf_context.as_context_mut();
                let basic_vector = abstract_value
                    .as_any_mut()
                    .downcast_mut::<Value<BasicVector<T>>>()
                    .unwrap()
                    .value_mut();

                (calc)(leaf_context, basic_vector)
            },
        );
        let value_producer = ValueProducer::new(alloc, cache_calc);

        self.create_cached_leaf_output_port(Some(fixed_size), value_producer)
    }

    fn create_abstract_leaf_output_port(
        &mut self,
        value_producer: ValueProducer,
    ) -> &LeafOutputPort<T> {
        self.create_cached_leaf_output_port(None, value_producer)
    }

    fn create_cached_leaf_output_port(
        &mut self,
        fixed_size: Option<usize>,
        value_producer: ValueProducer,
    ) -> &LeafOutputPort<T> {
        let output_port_index = OutputPortIndex::new(self.num_output_ports());
        let _system_id = self.system_id().clone();
        let cache_entry = self.declare_cache_entry(value_producer);
        let cache_entry_ptr: *const CacheEntry = cache_entry;
        let output_port = if let Some(size) = fixed_size {
            let data_type = PortDataType::VectorValued;
            Box::new(LeafOutputPort::<T>::new(
                self.system_ptr(),
                _system_id,
                output_port_index.clone(),
                data_type,
                size,
                cache_entry_ptr,
            ))
        } else {
            let data_type = PortDataType::AbstractValued;
            let size = 0;
            Box::new(LeafOutputPort::<T>::new(
                self.system_ptr(),
                _system_id,
                output_port_index.clone(),
                data_type,
                size,
                cache_entry_ptr,
            ))
        };
        self.add_output_port(output_port);

        self.leaf_output_port(&output_port_index)
    }
}
