mod zero_real_functions;
use rust_decimal::prelude::*;
use zero_real_functions::{
	base::calculate,
	datatypes::{approximation::Approximation, interval::Interval, order::Order},
};

fn main() {
	// Example using Newton's method to approximate:
	//let mut approximations = Vec::new();
	//let mut calculated_functions = Vec::new();
	//let mut errors = Vec::new();

	//approximations.push(Approximation::new(0.5));
	//errors.push(None);

	//let interval = Interval::new(0.0, 1.0);
	//calculate::newton_aproximate_method(interval, &mut calculated_functions, &mut approximations, &mut errors, 0.00001);

	//println!("K      X       Fx       Dx       E");
	//for i in 0..approximations.len() {
	//	println!(
	//		"{} {:.4} {:.4} {:.4} {}",
	//		i + 1,
	//		approximations[i].get_value(),
	//		calculated_functions[i].fx,
	//		calculated_functions[i].dx,
	//		match errors[i] {
	//			Some(error) => format!("{:.4}", error),
	//			None => "-------".to_string(),
	//		}
	//	);
	//}
	// End of example.

	// Example using Newton's cotes method:
	let interval = Interval::new(1.0, 5.0);
	let value_of_a = || -> f64 { return (15.0 * calculate::cubic_root(5.0) - 3.0) / 4.0 };
	println!("Exactly value of A {:.6}", value_of_a());

	println!();

	println!("Newton cotes order of 2:");
	let newton_cotes_order_of_2 = calculate::newton_cotes_method(Order::Two, interval.clone());
	println!("{:.6}", newton_cotes_order_of_2);
	println!("Relative error {:.6}", calculate::error::relative(value_of_a(), newton_cotes_order_of_2));

	println!();

	println!("Newton cotes order of 4:");
	let newton_cotes_order_of_4 = calculate::newton_cotes_method(Order::Four, interval.clone());
	println!("{:.4}", newton_cotes_order_of_4);
	println!("Relative error {:.6}", calculate::error::relative(value_of_a(), newton_cotes_order_of_4));
	// End of example.
}
