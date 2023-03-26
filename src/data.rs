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
