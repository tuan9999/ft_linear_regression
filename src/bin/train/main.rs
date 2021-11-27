extern crate csv;

use std::error::Error;
use csv::Reader;

fn main() {
	let f = example();

	match f {
		Err(e) => {
			println!("file not found \n{:?}",e);   //handled error
		}
		_ => {},
	 }
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("/Users/tuanperera/Documents/42/linear_regression/src/data/data.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}