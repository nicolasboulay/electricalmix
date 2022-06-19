use std::error::Error;
use std::io;

use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Powerplant {
    construction_year:u32,
    construction_duration_year: f64,
    co2_per_kwh:f64,
    max_power_kw:f64,
    earth_surface_m2:f64,
    death_rates_per_twh:f64,
    ressouces_use_ton:f64,
    controlable: bool,
}

pub fn sort_co2(p:&mut [Powerplant]) -> &[Powerplant] {
    p.sort_unstable_by(|a, b| b.co2_per_kwh.cmp(&a.co2_per_kwh))
}

pub fn example() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(Powerplant {
        construction_year: 0,
        construction_duration_year: 10.0,
        co2_per_kwh: 6.0,
        max_power_kw: 1600.0,
        earth_surface_m2: 100000.0,
        death_rates_per_twh: 0.07,
        ressouces_use_ton: 100000.0,
        controlable: true,     
    })?;

    wtr.flush()?;
    Ok(())
}

pub fn dump(r:&[Powerplant]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(&r[0])?;

    wtr.flush()?;
    Ok(())
}
