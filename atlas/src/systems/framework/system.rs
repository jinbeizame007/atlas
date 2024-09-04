use std::any::Any;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::context_base::ContextBase;
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

pub trait AbstractSystem: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait System<T: AtlasScalar>: AbstractSystem + SystemBase
where
    Self: 'static,
{
    type CN: Context<T>;

    // Getters and setters without default implementations
    fn input_ports(&self) -> Vec<&InputPort<T>>;
    fn input_ports_mut(&mut self) -> Vec<&mut InputPort<T>>;
    fn input_port(&self, index: &InputPortIndex) -> &InputPort<T>;
    fn input_port_mut(&mut self, index: &InputPortIndex) -> &mut InputPort<T>;
    fn add_input_port(&mut self, input_port: InputPort<T>);
    fn output_ports(&self) -> Vec<&dyn OutputPort<T, CN = Self::CN>>;
    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T, CN = Self::CN>>;
    fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T, CN = Self::CN>;
    fn output_port_mut(&mut self, index: &OutputPortIndex)
        -> &mut dyn OutputPort<T, CN = Self::CN>;

    // Resource allocation and initializaion
    fn allocate_context(&self) -> Box<Self::CN>;
    // fn allocate_context(&mut self) -> Box<dyn Context<T>> {
    //     self.do_allocate_context().as_ref().
    // }
    fn declare_input_port(&mut self, data_type: PortDataType, size: usize) -> &InputPort<T> {
        let input_port_index = InputPortIndex::new(self.num_input_ports());
        let eval = {
            let cloned_input_port_index = input_port_index.clone();
            let self_ptr: *mut Self = self;
            Box::new(move |context_base: &mut dyn ContextBase| unsafe {
                (*self_ptr).eval_abstract_input(context_base, &cloned_input_port_index)
            })
        };
        let alloc = {
            let cloned_input_port_index = input_port_index.clone();
            let self_ptr: *mut Self = self;
            Box::new(move || unsafe {
                (*self_ptr)
                    .allocate_input_abstract((*self_ptr).input_port(&cloned_input_port_index))
            })
        };
        let input_port = InputPort::<T>::new(
            self.system_id().clone(),
            input_port_index.clone(),
            data_type,
            size,
            eval,
            alloc,
        );
        self.add_input_port(input_port);

        self.input_port(&input_port_index)
    }

    fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue>;
    fn allocate_input_vector(&mut self, input_port: &InputPort<T>) -> BasicVector<T> {
        assert!(*input_port.data_type() == PortDataType::VectorValued);
        let self_input_port_base = self.input_port_base(input_port.index());
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
    fn allocate_input_abstract(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        self.do_allocate_input(input_port)
    }
    fn allocate_time_derivatives(&mut self) -> Box<<<Self::CN as Context<T>>::S as State<T>>::CS>;
    fn create_default_context(&mut self) -> Box<Self::CN> {
        let mut context = self.allocate_context();
        self.set_default_context(context.as_mut());
        context
    }

    // TODO: Consider inputting &dyn Context<T> and &mut State<T>
    fn set_default_state(&self, context: &mut Self::CN);
    fn set_default_context(&mut self, context: &mut Self::CN) {
        self.set_default_state(context);
    }

    // Cached evaluations
    fn eval_time_derivatives<'a>(
        &mut self,
        context: &'a mut Self::CN,
    ) -> &'a <<Self::CN as Context<T>>::S as State<T>>::CS {
        let cache_entry = self.time_derivatives_cache_entry();

        cache_entry.eval(context.as_mutable_base())
    }
    fn time_derivatives_cache_entry(&self) -> &CacheEntry {
        self.cache_entry(self.time_derivatives_cache_index())
    }
    fn time_derivatives_cache_index(&self) -> &CacheIndex;

    // Calculations
    fn calc_time_derivatives(
        &mut self,
        context: &mut Self::CN,
        derivatives: Option<&mut <<Self::CN as Context<T>>::S as State<T>>::CS>,
    ) {
        self.validate_context(context.as_base());
        self.do_calc_time_derivatives(context, derivatives.unwrap());
    }

    fn do_calc_time_derivatives(
        &mut self,
        _context: &mut Self::CN,
        derivatives: &mut <<Self::CN as Context<T>>::S as State<T>>::CS,
    ) {
        // This default implementation is only valid for Systems with no continuous
        // state. Other Systems must override this method!
        assert!(derivatives.size() == 0);
    }
}
