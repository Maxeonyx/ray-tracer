use cgmath::prelude::*;

use solvers::*;
use types::*;

#[derive(Debug, Clone)]
pub enum Surface {
	Diffuse,
	// Portion of light diffuse vs. reflected
	//Reflective(f32),
	// Portion of light diffuse vs. refracted and refractive index
	//Refractive(f32, f32),
	// Portion of light diffuse vs. (reflected/refracted), portion of light reflected vs. refracted, refractive index
	//ReflectiveAndRefractive(f32, f32, f32),
	// Texture ID
	//Textured(u8),
}

#[derive(Debug)]
pub struct Light {
	pub position: V3,
	pub brightness: f32,
}

#[derive(Debug)]
pub struct Object2 {
	pub position: V3,
	pub color: Color,
	pub surface: Surface,
	pub shape: Shape,
}

#[derive(Debug)]
pub enum Shape {
	// Sphere has radius
	Sphere(f32),
	// Cylinder has height and radius
	//Cylinder(f32, f32),
	// Cone has height and radius
	//Cone(f32, f32),
}

impl Object2 {
	pub fn closest_intersection(&self, ray: &Ray) -> Option<f32> {
		match self.shape {
			Shape::Sphere(radius) => sphere::intersection(self.position, radius, ray),
		}
	}
	pub fn normal(&self, intersection: V3) -> V3 {
		match self.shape {
			Shape::Sphere(_radius) => sphere::normal(self.position, intersection),
		}
	}
}

mod sphere {
	use super::*;

	pub fn normal(center: V3, intersection: V3) -> V3 {
		(intersection - center).normalize()
	}

	pub fn intersection(center: V3, radius: f32, ray: &Ray) -> Option<f32> {
		// quadratic polynomial from analytic solution
		let shared_term = ray.origin - center;
		let a = ray.direction.dot(ray.direction);
		let b = 2.0 * ray.direction.dot(shared_term);
		let c = shared_term.dot(shared_term) - radius;

		// solve for t (distance along ray) and choose closest root that is greater than 0
		match solve_quadratic(a, b, c) {
			QuadraticRoot::None => None,
			QuadraticRoot::One(t) => if t > 0.001 {
				Some(t)
			} else {
				None
			},
			QuadraticRoot::Two(t1, t2) => if t1 < 0.001 && t2 < 0.001 {
				None
			} else if t2 < 0.001 {
				Some(t1)
			} else if t1 < 0.001 {
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
}
