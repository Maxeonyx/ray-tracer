use cgmath::Vector3;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub const CELLS_WIDE: usize = 30;
pub const CELLS_HIGH: usize = 30;

#[derive(Clone, Copy)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

pub type Cells = Arc<Vec<Mutex<Color>>>;

pub type V3 = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
	pub origin: V3,
	pub direction: V3,
}
