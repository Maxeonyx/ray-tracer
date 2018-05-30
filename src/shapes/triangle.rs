use cgmath::prelude::*;
use types::*;

#[derive(Debug)]
pub struct Triangle {
	vertices: [V3; 3],
}

impl Triangle {
	pub fn new(vertices: [V3; 3]) -> Triangle {
		Triangle { vertices }
	}
	pub fn vertices(&self) -> &[V3; 3] {
		&self.vertices
	}
}

pub fn normal(triangle: &Triangle, _intersection: V3) -> V3 {
	let vertex = triangle.vertices();

	(vertex[0] - vertex[1])
		.cross(vertex[0] - vertex[2])
		.normalize()
}

pub fn intersection(triangle: &Triangle, ray: &Ray) -> Option<f32> {
	// Möller–Trumbore ray-triangle intersection algorithm
	let vertex = triangle.vertices();

	let edge_1 = vertex[1] - vertex[0];
	let edge_2 = vertex[2] - vertex[0];
	let h = ray.direction.cross(edge_2);
	let a = edge_1.dot(h);
	if a > -EPSILON && a < EPSILON {
		return None;
	}
	let f = 1.0 / a;
	let s = ray.origin - vertex[0];
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
