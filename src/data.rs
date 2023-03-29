use std::fs::File;

use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DataRecord {
	pub km: f64,
	pub price: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataSet {
	pub data: Vec<DataRecord>,
}

impl DataSet {
	pub fn new(data: Vec<DataRecord>) -> Self {
		Self { data }
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn get_max_value(&self, is_km: bool) -> f64 {
		let mut max = 0.0;
		for record in self.data.clone() {
			if is_km && record.km > max {
				max = record.km;
			} else if record.price > max {
				max = record.price;
			}
		}
		tracing::info!("IS KM: {is_km} Max value: {max}");
		max
	}

	pub fn get_min_value(&self, is_km: bool) -> f64 {
		let mut min = 0.0;
		for record in self.data.clone() {
			if is_km && record.km < min {
				min = record.km;
			} else if record.price < min {
				min = record.price;
			}
		}
		tracing::info!("IS KM: {is_km} Min value: {min}");
		min
	}

	pub fn get_vector_of_tuples(&self) -> Vec<(f64, f64)> {
		let mut vector = Vec::new();
		for record in self.data.clone() {
			vector.push((record.km, record.price));
		}
		vector
	}
}

impl IntoIterator for DataSet {
	type Item = DataRecord;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.data.into_iter()
	}
}

pub fn get_vector_of_data_records(mut rdr: Reader<File>) -> Result<Vec<DataRecord>, ()> {
	let mut data = Vec::new();
	for result in rdr.deserialize() {
		let record: DataRecord = result.map_err(|e| tracing::error!("Error deserializing record: {e}"))?;
		data.push(record);
	}
	Ok(data)
}
