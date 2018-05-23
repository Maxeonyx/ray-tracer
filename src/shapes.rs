use cgmath::num_traits::abs;
use cgmath::*;

use solvers::*;
use types::*;

pub trait Object {
	fn intersections(&self, ray: Ray) -> Vec<f32>;
}

pub struct Sphere {
	center: V3,
	radius: f32,
}

impl Object for Sphere {
	fn intersections(&self, ray: Ray) -> Vec<f32> {
		// quadratic polynomial from analytic solution
		let shared_term = ray.origin - self.center;
		let a = ray.direction.dot(ray.direction);
		let b = -2.0 * ray.direction.dot(shared_term);
		let c = shared_term.dot(shared_term) - self.radius;

		// solve for t (distance along ray)
		solve_quadratic(a, b, c)
			.filter(|t| t.abs() > 0.001)
			.collect()
	}
}
