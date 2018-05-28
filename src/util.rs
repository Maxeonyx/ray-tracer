use cgmath::prelude::*;
use types::*;

pub enum QuadraticRoot {
	None,
	One(f32),
	Two(f32, f32),
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticRoot {
	let discriminant = b * b - 4.0 * a * c;

	if discriminant <= -EPSILON {
		// no real roots
		QuadraticRoot::None
	} else if discriminant.abs() < EPSILON {
		// only 1 root
		QuadraticRoot::One((-b + discriminant.sqrt()) / 2.0 * a)
	} else {
		// two real roots
		QuadraticRoot::Two(
			(-b + discriminant.sqrt()) / 2.0 * a,
			(-b - discriminant.sqrt()) / 2.0 * a,
		)
	}
}

pub trait V3Extensions {
	fn reflect(self, V3) -> V3;
}

impl V3Extensions for V3 {
	fn reflect(self, normal: V3) -> V3 {
		self - 2.0 * normal.dot(self) * normal
	}
}
