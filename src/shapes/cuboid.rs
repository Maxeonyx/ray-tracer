use shapes::triangle;
use types::*;

#[derive(Debug)]
pub struct Cuboid {
	vertices: [V3; 8],
	open_top: bool,
}

const FACE_TRIANGLES: [[usize; 3]; 12] = [
	[0, 2, 1],
	[2, 3, 1],
	[0, 6, 4],
	[0, 2, 6],
	[0, 1, 5],
	[0, 5, 4],
	[7, 5, 4],
	[7, 4, 6],
	[7, 3, 1],
	[7, 1, 5],
	[7, 3, 2],
	[7, 2, 6],
];

impl Cuboid {
	pub fn new(center: V3, size: V3, open_top: bool) -> Cuboid {
		let mut modifiers: Vec<V3> = Vec::new();

		let postive_negative = [1 as f32, -1 as f32];

		for x_mod in &postive_negative {
			for y_mod in &postive_negative {
				for z_mod in &postive_negative {
					modifiers.push(V3 {
						x: x_mod * size.x,
						y: y_mod * size.y,
						z: z_mod * size.z,
					});
				}
			}
		}

		let mut vertices = [center; 8];
		for (index, modifier) in modifiers.into_iter().enumerate() {
			vertices[index] -= modifier / 2.0;
		}
		Cuboid { vertices, open_top }
	}

	fn triangles(&self) -> Vec<triangle::Triangle> {
		let mut triangles = vec![];

		for i in 0..12 {
			if self.open_top && (i == 0 || i == 1) {
				continue;
			}
			let indices = FACE_TRIANGLES[i];
			triangles.push(triangle::Triangle::new([
				self.vertices[indices[0]],
				self.vertices[indices[1]],
				self.vertices[indices[2]],
			]));
		}

		triangles
	}
}

pub fn normal(cuboid: &Cuboid, intersection: V3) -> V3 {
	let V3 { x, y, z } = intersection;
	if x >= y && x >= z {
		-V3 {
			x: 1.0,
			y: 0.0,
			z: 0.0,
		}
	} else if y >= x && y >= z {
		-V3 {
			x: 0.0,
			y: 1.0,
			z: 0.0,
		}
	} else {
		-V3 {
			x: 0.0,
			y: 0.0,
			z: 1.0,
		}
	}
}

pub fn intersection(cuboid: &Cuboid, ray: &Ray) -> Option<f32> {
	for triangle in cuboid.triangles() {
		match triangle::intersection(&triangle, ray) {
			None => continue,
			Some(t) => return Some(t),
		}
	}
	None
}
