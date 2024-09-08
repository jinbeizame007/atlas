use num_traits::NumAssign;
use std::fmt::Debug;

pub trait AtlasScalar: NumAssign + Clone + Debug + Default + 'static {}

impl AtlasScalar for f64 {}
