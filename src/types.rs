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
	pub data: Arc<Vec<Cell>>,
}

pub struct Cell {
	inner: Mutex<Color>,
}

impl Cell {
	pub fn new(color: Color) -> Cell {
		Cell {
			inner: Mutex::new(color),
		}
	}

	pub fn get_content(&self) -> Color {
		*self.inner.lock().unwrap()
	}

	pub fn set_content(&self, color: Color) {
		let mut guard = self.inner.lock().unwrap();
		*guard = color;
	}
}

impl Cells {
	pub fn to_vec(self) -> Vec<f32> {
		let mut v = Vec::with_capacity(self.data.len());
		for cell in self.data.iter() {
			let vector_cell = cell.get_content();
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
