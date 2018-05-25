use cgmath::Vector3;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub const CELLS_WIDE: usize = 50;
pub const CELLS_HIGH: usize = 50;

pub type Color = V3;

pub type Cells = Arc<Vec<Mutex<Color>>>;

pub type V3 = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
	pub origin: V3,
	pub direction: V3,
}
