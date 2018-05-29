use cgmath::Vector3;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
pub const EPSILON: f32 = 0.001;

pub const CELLS_WIDE: usize = 800;
pub const CELLS_HIGH: usize = 800;

pub const ANTIALIASING_DIV: u32 = 4;

pub type Color = V3;

#[derive(Clone)]
pub struct Cells {
	pub data: Arc<Vec<Mutex<Color>>>,
}

impl Cells {
	pub fn to_vec(self) -> Vec<f32> {
		let mut v = Vec::with_capacity(self.data.len());
		for cell in self.data.iter() {
			let vector_cell = *cell.lock().unwrap();
			v.push(vector_cell.x);
			v.push(vector_cell.y);
			v.push(vector_cell.z);
			v.push(1.0);
		}
		v
	}
}

pub type V3 = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
	pub origin: V3,
	pub direction: V3,
}
