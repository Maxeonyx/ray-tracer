use cgmath::Vector3;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
pub const EPSILON: f32 = 0.001;

pub const CELLS_WIDE: usize = 50;
pub const CELLS_HIGH: usize = 50;

pub type Color = V3;

#[derive(Clone)]
pub struct Cells {
	pub data: Arc<Vec<Mutex<Color>>>,
}

pub type V3 = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
	pub origin: V3,
	pub direction: V3,
}
