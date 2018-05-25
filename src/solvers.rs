pub enum QuadraticRoot {
	None,
	One(f32),
	Two(f32, f32),
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticRoot {
	let discriminant = b * b - 4.0 * a * c;

	if discriminant <= -0.000001 {
		// no real roots
		QuadraticRoot::None
	} else if discriminant.abs() < 0.000001 {
		// only 1 root
		QuadraticRoot::One((-b + discriminant.sqrt()) / 2.0 * a)
	} else {
		// two real roots
		QuadraticRoot::Two(
			(-b + discriminant.sqrt()) / 2.0 * a,
			(-b - discriminant.sqrt()) / 2.0 * a,
		)
	}
}
