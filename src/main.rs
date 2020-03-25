extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    let minimum_pop: u64 = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(arg) => arg.parse()?,
    };

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for result in rdr.deserialize() {
        let record: Record = result?;

        if record.population.map_or(false, |pop| pop >= minimum_pop) {
            wtr.serialize(record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
