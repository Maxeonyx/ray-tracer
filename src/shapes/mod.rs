use types::*;

mod cuboid;
mod sphere;
mod triangle;

pub use self::cuboid::Cuboid;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

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
pub enum Shape {
	Triangle(Triangle),
	Sphere(Sphere),
	Cuboid(Cuboid),
}

impl Object2 {
	pub fn closest_intersection(&self, ray: &Ray) -> Option<f32> {
		match self.shape {
			Shape::Sphere(ref sphere) => sphere::intersection(&sphere, ray),
			Shape::Triangle(ref triangle) => triangle::intersection(&triangle, ray),
			Shape::Cuboid(ref cuboid) => cuboid::intersection(&cuboid, ray),
		}
	}
	pub fn normal(&self, intersection: V3) -> V3 {
		match self.shape {
			Shape::Sphere(ref sphere) => sphere::normal(&sphere, intersection),
			Shape::Triangle(ref triangle) => triangle::normal(&triangle, intersection),
			Shape::Cuboid(ref cuboid) => cuboid::normal(&cuboid, intersection),
		}
	}
}
