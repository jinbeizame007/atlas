use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use atlas_derives::{AbstractSystem, LeafSystem, System, SystemBase};

extern crate nalgebra as na;

use crate::common::atlas_scalar::AtlasScalar;
use crate::common::value::AbstractValue;
use crate::systems::framework::basic_vector::BasicVector;
use crate::systems::framework::cache_entry::CacheEntry;
use crate::systems::framework::context::Context;
use crate::systems::framework::diagram::SystemWeakLink;
use crate::systems::framework::framework_common::InputPortIndex;
use crate::systems::framework::framework_common::OutputPortIndex;
use crate::systems::framework::framework_common::{
    CacheIndex, SystemId, SystemParentServiceInterface,
};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::input_port_base::InputPortBase;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::leaf_output_port::LeafOutputPort;
use crate::systems::framework::leaf_system::LeafSystem;
use crate::systems::framework::model_values::ModelValues;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::output_port_base::OutputPortBase;
use crate::systems::framework::state::State;
use crate::systems::framework::system::{AbstractSystem, System};
use crate::systems::framework::system_base::ContextSizes;
use crate::systems::framework::system_base::SystemBase;

// TODO: implement
#[derive(LeafSystem, System, AbstractSystem, SystemBase)]
pub struct AffineSystem<T: AtlasScalar> {
    name: String,
    a: na::DMatrix<T>,
    b: na::DMatrix<T>,
    f0: na::DVector<T>,
    c: na::DVector<T>,
    d: na::DMatrix<T>,
    y0: na::DVector<T>,
    time_period: f64,
    #[allow(clippy::box_collection)]
    input_ports: Vec<InputPort<T>>,
    output_ports: Vec<Box<LeafOutputPort<T>>>,
    cache_entries: Vec<CacheEntry>,
    context_sizes: ContextSizes,
    system_id: SystemId,
    system_weak_link: Option<SystemWeakLink<T>>,
    parent_service: Option<Box<dyn SystemParentServiceInterface>>,
    time_derivatives_cache_index: CacheIndex,
    model_input_values: ModelValues,
    model_continuous_state_vector: BasicVector<T>,
    implicit_time_derivatives_residual_size: Option<usize>,
}

impl<T: AtlasScalar> AffineSystem<T> {
    pub fn new(
        a: na::DMatrix<T>,
        b: na::DMatrix<T>,
        f0: na::DVector<T>,
        c: na::DVector<T>,
        d: na::DMatrix<T>,
        y0: na::DVector<T>,
        time_period: f64,
    ) -> Rc<RefCell<Self>> {
        let num_states = calc_num_states(&a, &b, &f0, &c);
        let num_inputs = calc_num_inputs(&b, &d);
        let num_outputs = calc_num_outputs(&c, &d, &y0);
        assert!(num_states > 0);
        assert!(num_inputs > 0);
        assert!(num_outputs > 0);

        let affine_system = Rc::new(RefCell::new(Self {
            name: "affine_system".to_string(),
            a,
            b,
            f0,
            c,
            d,
            y0,
            time_period,
            input_ports: vec![],
            output_ports: vec![],
            cache_entries: vec![],
            context_sizes: ContextSizes::default(),
            system_id: SystemId::new(0),
            system_weak_link: None,
            parent_service: None,
            time_derivatives_cache_index: CacheIndex::new(0),
            model_input_values: ModelValues::default(),
            model_continuous_state_vector: BasicVector::<T>::zeros(0),
            implicit_time_derivatives_residual_size: None,
        }));

        affine_system
    }
}

fn calc_num_states<T: AtlasScalar>(
    a: &na::DMatrix<T>,
    b: &na::DMatrix<T>,
    f0: &na::DVector<T>,
    c: &na::DVector<T>,
) -> usize {
    let mut num_states = 0;

    if !a.is_empty() {
        num_states = a.nrows();
        assert_eq!(a.nrows(), a.ncols());
    }
    if !b.is_empty() {
        if num_states > 0 {
            assert_eq!(b.nrows(), b.ncols());
        } else {
            num_states = b.nrows();
        }
    }
    if !f0.is_empty() {
        if num_states > 0 {
            assert_eq!(f0.len(), num_states);
        } else {
            num_states = f0.len();
        }
    }
    if !c.is_empty() {
        if num_states > 0 {
            assert_eq!(c.ncols(), num_states);
        } else {
            num_states = c.ncols();
        }
    }

    num_states
}

fn calc_num_inputs<T: AtlasScalar>(b: &na::DMatrix<T>, d: &na::DMatrix<T>) -> usize {
    let mut num_inputs = 0;

    if !b.is_empty() {
        num_inputs = b.ncols();
    }
    if !d.is_empty() {
        if num_inputs > 0 {
            assert_eq!(d.nrows(), d.ncols());
        } else {
            num_inputs = d.ncols();
        }
    }

    num_inputs
}

fn calc_num_outputs<T: AtlasScalar>(
    c: &na::DVector<T>,
    d: &na::DMatrix<T>,
    y0: &na::DVector<T>,
) -> usize {
    let mut num_outputs = 0;
    if !c.is_empty() {
        num_outputs = c.nrows();
    }
    if !d.is_empty() {
        if num_outputs > 0 {
            assert_eq!(d.nrows(), d.ncols());
        } else {
            num_outputs = d.nrows();
        }
    }
    if !y0.is_empty() {
        if num_outputs > 0 {
            assert_eq!(y0.len(), num_outputs);
        } else {
            num_outputs = y0.len();
        }
    }

    num_outputs
}
