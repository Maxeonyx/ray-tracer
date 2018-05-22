pub fn solve_quadratic(a: f32, b: f32, c: f32) -> impl Iterator<Item = f32> {
	let discriminant = b.powf(2.0) - 4.0 * a * c;

	if discriminant <= -0.0001 {
		// no real roots
		vec![].into_iter()
	} else if discriminant.abs() < 0.0001 {
		// only 1 root
		vec![(-b + discriminant.sqrt()) / 4.0 * a].into_iter()
	} else {
		// two real roots
		vec![
			(-b + discriminant.sqrt()) / 4.0 * a,
			(-b - discriminant.sqrt()) / 4.0 * a,
		].into_iter()
	}
}
