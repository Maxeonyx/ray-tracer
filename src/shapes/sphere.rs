use cgmath::prelude::*;
use types::*;
use util::*;

#[derive(Debug)]
pub struct Sphere {
	pub center: V3,
	pub radius: f32,
}
pub fn normal(sphere: &Sphere, intersection: V3) -> V3 {
	(intersection - sphere.center).normalize()
}

pub fn intersection(sphere: &Sphere, ray: &Ray) -> Option<f32> {
	// quadratic polynomial from analytic solution
	let shared_term = ray.origin - sphere.center;
	let a = ray.direction.dot(ray.direction);
	let b = 2.0 * ray.direction.dot(shared_term);
	let c = shared_term.dot(shared_term) - sphere.radius;

	// solve for t (distance along ray) and choose closest root that is greater than 0
	match solve_quadratic(a, b, c) {
		QuadraticRoot::None => None,
		QuadraticRoot::One(t) => if t > EPSILON {
			Some(t)
		} else {
			None
		},
		QuadraticRoot::Two(t1, t2) => if t1 < EPSILON && t2 < EPSILON {
			None
		} else if t2 < EPSILON {
			Some(t1)
		} else if t1 < EPSILON {
			Some(t2)
		} else {
			if t1 < t2 {
				Some(t1)
			} else {
				Some(t2)
			}
		},
	}
}
