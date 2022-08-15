use super::datatypes;

pub mod calculate {
	use crate::zero_real_functions::datatypes::calculated_function;

	use super::datatypes::approximation::Approximation;
	use super::datatypes::calculated_function::CalculatedFunction;
	use super::datatypes::interval::Interval;

	pub fn function(x: f64) -> f64 {
		x - x.cos().powf(2.0)
	}

	pub fn derived(x: f64) -> f64 {
		1.0 + (x * 2.0).sin()
	}

	pub fn relative_error(approximation: f64, last_approximation: f64) -> f64 {
		((last_approximation - approximation) / approximation).abs()
	}

	pub fn next_approximation(xk: f64) -> (Approximation, CalculatedFunction) {
		let function_of_xk = self::function(xk);
		let derived_of_xk = self::derived(xk);

		let next_xk = xk - (function_of_xk / derived_of_xk);

		(Approximation::new(next_xk), CalculatedFunction::new(function_of_xk, derived_of_xk))
	}

	pub fn newton_method(
		interval: Interval,
		calculated_functions: &mut Vec<CalculatedFunction>,
		approximations: &mut Vec<Approximation>,
		errors: &mut Vec<f64>,
	) {
		loop {
			let (next_approximation, next_calculated_function) = self::next_approximation(approximations[approximations.len() - 1].get_value());
			approximations.push(next_approximation);
			calculated_functions.push(next_calculated_function);
			errors.push(self::relative_error(
				approximations[approximations.len() - 1].get_value(),
				approximations[approximations.len() - 2].get_value(),
			));

			if errors[errors.len() - 1] < 0.00001 {
				calculated_functions.push(CalculatedFunction::new(
					self::function(approximations[approximations.len() - 1].get_value()),
					self::derived(approximations[approximations.len() - 1].get_value()),
				));
				break;
			}
		}
	}
}

#[cfg(test)]
mod base_tests {
	use crate::zero_real_functions::datatypes::calculated_function;

	use super::*;
	use datatypes::approximation::Approximation;
	use datatypes::calculated_function::CalculatedFunction;
	use datatypes::interval::Interval;

	#[test]
	fn test_calculate_function() {
		assert_eq!(calculate::function(0.0), -1.0);
		assert_eq!(calculate::function(25.0), 24.017516985753943);
	}

	#[test]
	fn test_calculate_derived() {
		assert_eq!(calculate::derived(0.0), 1.0);
		assert_eq!(calculate::derived(25.0), 1.2623748537039288);
	}

	#[test]
	fn test_calculate_relative_error() {
		assert_eq!(calculate::relative_error(25.0, 20.0), 0.2);
	}

	#[test]
	fn test_calculate_next_approximation() {
		assert_eq!(calculate::next_approximation(0.0).0.get_value(), 1.0);
		assert_eq!(calculate::next_approximation(25.0).0.get_value(), 5.974338236155255);
	}
}
