use std::fmt::Debug;

use num_traits::NumAssign;

pub trait AtlasScalar: NumAssign + Clone + Debug + Default + 'static {}

impl AtlasScalar for f64 {}
