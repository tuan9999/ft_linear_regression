use crate::data::DataSet;

pub struct LinearRegression {
	pub data: DataSet,
	pub m: usize,
}

impl LinearRegression {
	pub fn new(data: DataSet) -> Self {
		Self {
			m: data.len(),
			data: data,
		}
	}

	pub fn train() {}

	fn hypothesis(&self, x: f64, theta_zero: f64, theta_one: f64) -> f64 {
		theta_zero + theta_one * x
	}

	fn cost(&mut self) -> f64 {
		let mut sum = 0.0;
		for record in self.data.clone() {
			let (x, y) = (record.km, record.price);
			let x_hypothesis = self.hypothesis(x, 1.0, 1.0);
			sum += x_hypothesis - y;
		}
		(1.0 / (2.0 * self.m as f64)) * sum
	}

	fn gradient_descent(&mut self) {}
}