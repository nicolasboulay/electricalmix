
use std::{error::Error, vec};
use std::io;

use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize)]
pub struct Record {
    year:u32,
    co2_per_kwh:f64,
    euro_per_kwh:f64,
    t_per_kwh:f64,
    death_per_kwh:f64,
    m2_per_kwh:f64,
}

#[derive(Debug)]
pub struct RecordList {
    list: Vec<Record>,
}

impl RecordList {
    pub fn new(list: Vec<Record>) -> Self { 
        Self { list } 
    }
    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        dump(&self.list)
    }
}

pub fn example() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(Record {
    	year: 0,		 
    	co2_per_kwh: 10.0,
    	euro_per_kwh: 10.0,
    	t_per_kwh: 10.0,
    	death_per_kwh: 10.0,
    	m2_per_kwh: 10.0,
    })?;

    wtr.flush()?;
    Ok(())
}

pub fn dump(list:&[Record]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for r in list {
        wtr.serialize(&r)?;
    }

    wtr.flush()?;
    Ok(())
}
