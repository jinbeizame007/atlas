use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

use num_traits::identities::Zero;

pub trait AtlasScalar: Add + PartialEq + Clone + Debug + Default + Zero + 'static {}
