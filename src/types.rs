use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub const CELLS_WIDE: usize = 100;
pub const CELLS_HIGH: usize = 100;

pub type Color = [f32; 4];

pub type Cells = Arc<Vec<Mutex<Color>>>;
