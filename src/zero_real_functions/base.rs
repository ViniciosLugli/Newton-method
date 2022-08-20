use super::datatypes;

pub mod calculate {
	use super::datatypes::approximation::Approximation;
	use super::datatypes::calculated_functions::CalculatedFunction;
	use super::datatypes::interval::Interval;
	use super::datatypes::order::Order;

	pub fn function(x: f64) -> f64 {
		x - x.cos().powf(2.0)
	}

	pub fn derived(x: f64) -> f64 {
		1.0 + (x * 2.0).sin()
	}

	pub fn cubic_root(x: f64) -> f64 {
		x.powf(1.0 / 3.0)
	}

	pub mod error {
		pub fn relative(value: f64, estimative: f64) -> f64 {
			((estimative - value) / value).abs()
		}
	}

	pub fn next_approximation(xk: f64) -> (Approximation, CalculatedFunction) {
		let function_of_xk = self::function(xk);
		let derived_of_xk = self::derived(xk);

		let next_xk = xk - (function_of_xk / derived_of_xk);

		(Approximation::new(next_xk), CalculatedFunction::new(function_of_xk, derived_of_xk))
	}

	pub fn newton_aproximate_method(
		_interval: Interval,
		calculated_functions: &mut Vec<CalculatedFunction>,
		approximations: &mut Vec<Approximation>,
		errors: &mut Vec<Option<f64>>,
		precision: f64,
	) {
		loop {
			if approximations.len() > 1 {
				errors.push(Some(self::error::relative(
					approximations[approximations.len() - 1].get_value(),
					approximations[approximations.len() - 2].get_value(),
				)));
			}

			let (next_approximation, next_calculated_function) = self::next_approximation(approximations[approximations.len() - 1].get_value());
			approximations.push(next_approximation);
			calculated_functions.push(next_calculated_function);

			if errors[errors.len() - 1].is_some() && errors[errors.len() - 1].unwrap() < precision {
				break;
			}
		}
	}

	pub fn newton_cotes_method(order: Order, interval: Interval) -> f64 {
		let h = interval.get_difference() / order.value();

		let mut result = 0.0;
		let mut buffer = 0.0;
		for i in 0..order.value() as usize + 1 {
			buffer = interval.get_left_border() + (i as f64 * h);
			println!("{:.4} {:.4} {:.4} {:.4}", i, buffer as i32, self::cubic_root(buffer), order.constant()[i] / order.divisor());
			result += self::cubic_root(buffer) * (order.constant()[i] / order.divisor());
		}

		return interval.get_difference() * result;
	}
}

#[cfg(test)]
mod base_tests {
	use super::*;

	#[test]
	fn test_calculate_function() {
		assert_eq!(calculate::function(0.0), -1.0);
		assert_eq!(calculate::function(25.0), 24.017516985753943);
	}

	#[test]
	fn test_calculate_derived() {
		assert_eq!(calculate::derived(0.0), 1.0);
		assert_eq!(calculate::derived(25.0), 0.7376251462960712);
	}

	#[test]
	fn test_calculate_error_relative() {
		assert_eq!(calculate::error::relative(25.0, 20.0), 0.2);
	}

	#[test]
	fn test_calculate_next_approximation() {
		assert_eq!(calculate::next_approximation(0.0).0.get_value(), 1.0);
		assert_eq!(calculate::next_approximation(25.0).0.get_value(), -7.560599521797876);
	}
}
