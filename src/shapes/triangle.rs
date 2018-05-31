use cgmath::prelude::*;
use types::*;

#[derive(Debug)]
pub struct Triangle {
	vertices: [V3; 3],
	uv: Option<[V2; 3]>,
}

impl Triangle {
	pub fn new(vertices: [V3; 3]) -> Triangle {
		Triangle { vertices, uv: None }
	}
	pub fn new_with_uv(vertices: [V3; 3], uv: [V2; 3]) -> Triangle {
		Triangle {
			vertices,
			uv: Some(uv),
		}
	}
	pub fn vertices(&self) -> &[V3; 3] {
		&self.vertices
	}
	pub fn uv(&self) -> &Option<[V2; 3]> {
		&self.uv
	}
}

pub fn normal(triangle: &Triangle) -> V3 {
	let vertex = triangle.vertices();

	let normal = (vertex[0] - vertex[1])
		.cross(vertex[0] - vertex[2])
		.normalize();

	normal
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

pub fn get_texture_coord(triangle: &Triangle, intersection: V3) -> V2 {
	let [vertex_1, vertex_2, vertex_3] = triangle.vertices();

	let uv = match triangle.uv() {
		None => return V2 { x: 0.0, y: 0.0 },
		Some(uv) => uv,
	};

	let f1 = vertex_1 - intersection;
	let f2 = vertex_2 - intersection;
	let f3 = vertex_3 - intersection;
	// calculate the areas and factors (order of parameters doesn't matter):
	let area = (vertex_1 - vertex_2).cross(vertex_1 - vertex_3).magnitude(); // main triangle area a
	let area_1 = f2.cross(f3).magnitude() / area; // p1's triangle area / a
	let area_2 = f3.cross(f1).magnitude() / area; // p2's triangle area / a
	let area_3 = f1.cross(f2).magnitude() / area; // p3's triangle area / a
											   // find the uv corresponding to point f (uv1/uv2/uv3 are associated to p1/p2/p3):
	uv[0] * area_1 + uv[1] * area_2 + uv[2] * area_3
}
