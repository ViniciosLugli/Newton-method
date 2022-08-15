mod zero_real_functions;

use zero_real_functions::{
	base::calculate,
	datatypes::{approximation::Approximation, interval::Interval},
};

fn main() {
	let mut approximations = Vec::new();
	let mut calculated_functions = Vec::new();
	let mut errors = Vec::new();

	approximations.push(Approximation::new(0.5));
	errors.push(None);

	let interval = Interval::new(0.0, 1.0);
	calculate::newton_method(interval, &mut calculated_functions, &mut approximations, &mut errors, 0.00001);

	println!("K      X       Fx       Dx       E");
	for i in 0..approximations.len() {
		println!(
			"{} {:.6} {:.6} {:.6} {}",
			i + 1,
			approximations[i].get_value(),
			calculated_functions[i].fx,
			calculated_functions[i].dx,
			match errors[i] {
				Some(error) => format!("{:.6}", error),
				None => "-------".to_string(),
			}
		);
	}
}
