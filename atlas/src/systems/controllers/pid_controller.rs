use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use atlas_derives::{AbstractSystem, LeafSystem, SystemBase};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::continuous_state::ContinuousState;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::framework_common::InputPortIndex;
use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::framework_common::{
    CacheIndex, SystemId, SystemParentServiceInterface,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::leaf_continuous_state::LeafContinuousState;
use crate::systems::framework::leaf_output_port::LeafOutputPort;
use crate::systems::framework::leaf_state::LeafState;
use crate::systems::framework::leaf_system::LeafSystem;
use crate::systems::framework::model_values::ModelValues;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::system::{AbstractSystem, System};
use crate::systems::framework::system_base::ContextSizes;
use crate::systems::framework::system_base::SystemBase;

#[derive(SystemBase, AbstractSystem, LeafSystem)]
pub struct PIDController<T: AtlasScalar> {
    kp: na::DVector<T>,
    ki: na::DVector<T>,
    kd: na::DVector<T>,
    input_port_index_state: InputPortIndex,
    input_port_index_desired_state: InputPortIndex,
    output_port_index_control: OutputPortIndex,
    num_controlled_q: usize,
    #[allow(clippy::box_collection)]
    name: String,
    system_weak_link: Option<SystemWeakLink<T>>,
    input_ports: Vec<InputPort<T>>,
    output_ports: Vec<Box<LeafOutputPort<T>>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    parent_service: Option<Box<dyn SystemParentServiceInterface>>,
    time_derivatives_cache_index: CacheIndex,
    model_input_values: ModelValues,
    model_continuous_state_vector: BasicVector<T>,
    implicit_time_derivatives_residual_size: Option<usize>,
}

impl<T: AtlasScalar> PIDController<T> {
    pub fn new(kp: na::DVector<T>, ki: na::DVector<T>, kd: na::DVector<T>) -> Rc<RefCell<Self>> {
        let num_controlled_q = kp.len();

        let mut pid_controller = Rc::new(RefCell::new(Self {
            name: "pid_controller".to_string(),
            kp,
            ki,
            kd,
            input_port_index_state: InputPortIndex::default(),
            input_port_index_desired_state: InputPortIndex::default(),
            output_port_index_control: OutputPortIndex::default(),
            num_controlled_q,
            system_weak_link: None,
            input_ports: vec![],
            output_ports: vec![],
            cache_entries: vec![],
            context_sizes: ContextSizes::default(),
            system_id: SystemId::new(0),
            parent_service: None,
            time_derivatives_cache_index: CacheIndex::new(0),
            model_input_values: ModelValues::default(),
            model_continuous_state_vector: BasicVector::<T>::zeros(0),
            implicit_time_derivatives_residual_size: None,
        }));

        unsafe {
            let pid_controller_weak = Rc::downgrade(&pid_controller);
            let pid_controller_weak_ptr = Weak::into_raw(pid_controller_weak);
            let system_weak = Weak::<RefCell<dyn System<T, CN = LeafContext<T>>>>::from_raw(
                pid_controller_weak_ptr,
            );
            pid_controller.borrow_mut().system_weak_link =
                Some(SystemWeakLink::LeafSystemWeakLink(system_weak));
        }

        pid_controller
            .borrow_mut()
            .declare_continuous_state(num_controlled_q, 0, 0);

        let calc = {
            let pid_controller_weak = Rc::downgrade(&pid_controller);
            Box::new(
                move |context: &mut LeafContext<T>, control: &mut BasicVector<T>| {
                    let pid_controller = pid_controller_weak.upgrade().unwrap();
                    pid_controller.borrow().calc_control(context, control);
                },
            )
        };

        pid_controller.borrow_mut().output_port_index_control = pid_controller
            .borrow_mut()
            .declare_vector_output_port("control".to_string(), num_controlled_q, calc)
            .index()
            .clone();

        pid_controller.borrow_mut().input_port_index_state = pid_controller
            .borrow_mut()
            .declare_vector_input_port("estimated_state".to_string(), num_controlled_q * 2)
            .index()
            .clone();

        pid_controller.borrow_mut().input_port_index_desired_state = pid_controller
            .borrow_mut()
            .declare_vector_input_port("desired_state".to_string(), num_controlled_q * 2)
            .index()
            .clone();

        pid_controller
    }

    pub fn do_calc_time_derivatives(
        &mut self,
        context: &mut LeafContext<T>,
        derivatives: &mut LeafContinuousState<T>,
    ) {
        let state = self
            .input_port_estimated_state()
            .eval::<LeafState<T>, BasicVector<T>>(context);
        let desired_state = self
            .input_port_desired_state()
            .eval::<LeafState<T>, BasicVector<T>>(context);

        let derivatives_vector = derivatives.vector_mut();
        let controlled_state_diff = &desired_state - &state;
        derivatives_vector.set_from_vector(controlled_state_diff.value());
    }

    pub fn calc_control(&self, context: &mut LeafContext<T>, control: &mut BasicVector<T>) {
        let state = self
            .input_port_estimated_state()
            .eval::<LeafState<T>, BasicVector<T>>(context);
        let desired_state = self
            .input_port_desired_state()
            .eval::<LeafState<T>, BasicVector<T>>(context);

        let controlled_state_diff = &desired_state - &state;

        let integrated_controlled_state_diff =
            context.continuous_state_mut().generalized_position_mut();

        let control_vector = self
            .kp
            .component_mul(&controlled_state_diff.value().rows(0, self.num_controlled_q))
            + self
                .ki
                .component_mul(integrated_controlled_state_diff.value())
            + self.kd.component_mul(
                &controlled_state_diff
                    .value()
                    .rows(self.num_controlled_q, self.num_controlled_q),
            );
        control.set_value(&control_vector);
    }

    pub fn set_integral_value(&self, context: &mut LeafContext<T>, value: &na::DVector<T>) {
        let integrated_controlled_state_diff = context.continuous_state_vector_mut();
        integrated_controlled_state_diff.set_from_vector(value)
    }

    fn input_port_estimated_state(&self) -> &InputPort<T> {
        &self.input_ports[&self.input_port_index_state]
    }

    fn input_port_desired_state(&self) -> &InputPort<T> {
        &self.input_ports[&self.input_port_index_desired_state]
    }

    fn output_port_control(&self) -> &LeafOutputPort<T> {
        &self.output_ports[&self.output_port_index_control]
    }
}

impl<T: AtlasScalar> System<T> for PIDController<T> {
    type CN = LeafContext<T>;

    fn input_ports(&self) -> Vec<&InputPort<T>> {
        self.input_ports.iter().collect()
    }

    fn input_ports_mut(&mut self) -> Vec<&mut InputPort<T>> {
        self.input_ports.iter_mut().collect()
    }

    fn input_port(&self, index: &InputPortIndex) -> &InputPort<T> {
        &self.input_ports[index]
    }

    fn input_port_mut(&mut self, index: &InputPortIndex) -> &mut InputPort<T> {
        &mut self.input_ports[index]
    }

    fn add_input_port(&mut self, input_port: InputPort<T>) {
        self.input_ports.push(input_port);
    }

    fn output_ports(&self) -> Vec<&dyn OutputPort<T, CN = Self::CN>> {
        self.output_ports
            .iter()
            .map(|p| p.as_ref() as &dyn OutputPort<T, CN = Self::CN>)
            .collect()
    }

    fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T, CN = Self::CN>> {
        self.output_ports
            .iter_mut()
            .map(|p| p.as_mut() as &mut dyn OutputPort<T, CN = Self::CN>)
            .collect()
    }

    fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T, CN = Self::CN> {
        self.output_ports[index].as_ref()
    }

    fn output_port_mut(
        &mut self,
        index: &OutputPortIndex,
    ) -> &mut dyn OutputPort<T, CN = Self::CN> {
        self.output_ports[index].as_mut()
    }

    fn system_weak_link(&self) -> SystemWeakLink<T> {
        self.system_weak_link.clone().unwrap()
    }

    fn time_derivatives_cache_index(&self) -> &CacheIndex {
        &self.time_derivatives_cache_index
    }

    fn allocate_context(&self) -> Rc<RefCell<Self::CN>> {
        LeafSystem::<T>::allocate_context(self)
    }

    fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
        LeafSystem::<T>::do_allocate_input(self, input_port)
    }

    fn allocate_time_derivatives(&mut self) -> Box<<<Self::CN as Context<T>>::S as State<T>>::CS> {
        LeafSystem::<T>::allocate_time_derivatives(self)
    }

    fn set_default_state(&self, context: &mut Self::CN) {
        LeafSystem::<T>::set_default_state(self, context)
    }

    fn do_calc_time_derivatives(
        &mut self,
        context: &mut Self::CN,
        derivatives: &mut <<Self::CN as Context<T>>::S as State<T>>::CS,
    ) {
        PIDController::<T>::do_calc_time_derivatives(self, context, derivatives)
    }
}
