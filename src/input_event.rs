use serde::{Serialize, Deserialize};
use std::error::Error;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod my_date_format;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventList {
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(with = "my_date_format")]
    pub date: chrono::NaiveDate,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    SetDailyConsumption { gwh: f64 },    
    CreatePowerplant { kind : String, number : u32, 
        #[serde(with = "my_date_format")]    
        first_cornerstone: chrono::NaiveDate   
    },
}

pub fn from_str(s: &str) -> Result<EventList, Box<dyn Error>> {
    let deserialized: EventList = serde_json::from_str(s)?;

    Ok(deserialized)
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<EventList, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let e = serde_json::from_reader(reader)?;

    Ok(e)
}

#[cfg(test)]
mod tests {
    use crate::input_event::*;

    #[test]
    fn basic() {
        let s:String = serde_json::json!({"daily_consumption_in_gwh" : 1000.0}).to_string();
        let e: EventList = from_str(&s).unwrap();
        assert_eq!(e.daily_consumption_in_gwh, 1000.0);
    }
}