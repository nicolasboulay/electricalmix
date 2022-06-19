use std::process;

mod csv;
mod powerplant;

fn main() {
    if let Err(err) = csv::example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    if let Err(err) = powerplant::example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}