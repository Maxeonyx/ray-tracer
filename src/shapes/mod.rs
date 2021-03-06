use types::*;

mod sphere;
mod triangle;

pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

#[derive(Debug, Clone, Copy)]
pub enum Surface {
	Diffuse,
	// Portion of light diffuse vs. reflected
	Reflective(f32),
	// Portion of light diffuse vs. refracted and refractive index
	//Refractive(f32, f32),
	// Portion of light diffuse vs. (reflected/refracted), portion of light reflected vs. refracted, refractive index
	//ReflectiveAndRefractive(f32, f32, f32),
	// Texture ID
	Textured(usize),
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
	pub shininess: f32,
}

#[derive(Debug)]
pub enum Shape {
	Triangle(Triangle),
	Sphere(Sphere),
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
			Shape::Triangle(ref triangle) => triangle::normal(&triangle),
		}
	}
	pub fn get_texture_coord(&self, intersection: V3) -> V2 {
		match self.shape {
			Shape::Sphere(ref sphere) => V2 { x: 0.0, y: 0.0 },
			Shape::Triangle(ref triangle) => triangle::get_texture_coord(&triangle, intersection),
		}
	}
}
