use std::path::Path;

use crate::{data::DataSet, utils::save_contents_to_file};

pub struct LinearRegression {
	pub data: DataSet,
	pub m: usize,
	alpha: f64,
	theta_zero: f64,
	theta_one: f64,
	gradient_descent_iterations: usize,
}

impl LinearRegression {
	pub fn new(data: DataSet, alpha: Option<f64>, gradient_descent_iterations: usize) -> Self {
		Self {
			m: data.len(),
			data: data,
			alpha: if let Some(alpha) = alpha {
				alpha
			} else {
				0.01
			},
			theta_zero: 0.0,
			theta_one: 0.0,
			gradient_descent_iterations,
		}
	}

	pub fn train(&mut self) {
		self.gradient_descent();
	}

	fn hypothesis(&self, x: f64) -> f64 {
		self.theta_zero + self.theta_one * x
	}

	fn cost(&mut self, is_theta_zero: bool) -> f64 {
		let mut sum = 0.0;
		for record in self.data.clone() {
			let (x, y) = (record.km, record.price);
			let x_hypothesis = self.hypothesis(x);
			if is_theta_zero {
				sum += x_hypothesis - y;
			} else {
				sum += (x_hypothesis - y) * x;
			}
		}
		(1.0 / (2.0 * self.m as f64)) * sum
	}

	fn gradient_descent(&mut self) {
		let mut theta_zero = self.theta_zero;
		let mut theta_one = self.theta_one;
		for i in 0..self.gradient_descent_iterations {
			let temp_zero = theta_zero - self.alpha * self.cost(true);
			let temp_one = theta_one - self.alpha * self.cost(false);
			theta_zero = temp_zero;
			theta_one = temp_one;
			tracing::info!("iteration: {i}\ntheta_zero: {}\ntheta_one: {}", theta_zero, theta_one);
		}
		self.theta_zero = theta_zero;
		self.theta_one = theta_one;
		tracing::info!("theta_zero_final_value: {}\ntheta_one_final_value: {}", self.theta_zero, self.theta_one)
	}

	pub fn save_thetas_to_file(&self, path: &Path) -> Result<(), ()> {
		let file_contents = format!("theta_zero: {}\ntheta_one: {}", self.theta_zero, self.theta_one);
		save_contents_to_file(&path, &file_contents)?;
		Ok(())
	}

	pub fn set_thetas(&mut self, theta_zero: f64, theta_one: f64) {
		self.theta_zero = theta_zero;
		self.theta_one = theta_one;
	}

	pub fn predict(&self, x: f64) -> f64 {
		self.hypothesis(x)
	}
}