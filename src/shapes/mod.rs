use cgmath::prelude::*;

use types::*;
use util::*;

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
	pub color: Color,
	pub surface: Surface,
	pub shape: Shape,
}

#[derive(Debug)]
pub struct Sphere {
	pub center: V3,
	pub radius: f32,
}

#[derive(Debug)]
pub struct Triangle {
	pub vertex_1: V3,
	pub vertex_2: V3,
	pub vertex_3: V3,
}

#[derive(Debug)]
pub enum Shape {
	Triangle(Triangle),
	// Sphere has radius
	Sphere(Sphere),
	// Cylinder has height and radius
	//Cylinder(f32, f32),
	// Cone has height and radius
	//Cone(f32, f32),
}

impl Object2 {
	pub fn closest_intersection(&self, ray: &Ray) -> Option<f32> {
		match self.shape {
			Shape::Sphere(ref sphere) => sphere::intersection(&sphere, ray),
			Shape::Triangle(ref triangle) => triangle::intersection(&triangle, ray),
		}
	}
	pub fn normal(&self, intersection: V3) -> V3 {
		match self.shape {
			Shape::Sphere(ref sphere) => sphere::normal(&sphere, intersection),
			Shape::Triangle(ref triangle) => triangle::normal(&triangle, intersection),
		}
	}
}

mod triangle {
	use super::*;

	pub fn normal(triangle: &Triangle, intersection: V3) -> V3 {
		// TODO might be backwards
		(triangle.vertex_1 - triangle.vertex_2)
			.cross(triangle.vertex_1 - triangle.vertex_3)
			.normalize()
	}

	pub fn intersection(triangle: &Triangle, ray: &Ray) -> Option<f32> {
		// Möller–Trumbore ray-triangle intersection algorithm

		let edge_1 = triangle.vertex_2 - triangle.vertex_1;
		let edge_2 = triangle.vertex_3 - triangle.vertex_1;
		let h = ray.direction.cross(edge_2);
		let a = edge_1.dot(h);
		if a > -EPSILON && a < EPSILON {
			return None;
		}
		let f = 1.0 / a;
		let s = ray.origin - triangle.vertex_1;
		let u = f * s.dot(h);
		if u < 0.0 || u > 1.0 {
			return None;
		}
		let q = s.cross(edge_1);
		let v = f * ray.direction.dot(q);
		if v < 0.0 || u + v > 1.0 {
			return None;
		}
		// At this stage we can compute t to find out where the intersection point is on the line.
		let t = f * edge_2.dot(q);
		if t > EPSILON {
			return Some(t);
		} else {
			// This means that there is a line intersection but not a ray intersection.
			return None;
		}
	}
}

mod sphere {
	use super::*;

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
}
