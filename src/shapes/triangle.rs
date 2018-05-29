use cgmath::prelude::*;
use types::*;

#[derive(Debug)]
pub struct Triangle {
	pub vertex_1: V3,
	pub vertex_2: V3,
	pub vertex_3: V3,
}

pub fn normal(triangle: &Triangle, _intersection: V3) -> V3 {
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
