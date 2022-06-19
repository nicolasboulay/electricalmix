use std::process;

mod csv;
mod powerplant;
mod input_event;
use clap::Parser;

/// electric mix simulator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[clap(short, long, value_parser)]
    input_file_name: String,
}


fn main() {

    let args = Args::parse();

    println!("Input file : {}", args.input_file_name);
    
    let e = input_event::from_file(args.input_file_name).unwrap();

    println!("Input events : {:?}", e);

    if let Err(err) = csv::example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    if let Err(err) = powerplant::example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}