mod economy;
mod util;

use std::path::Path;
use std::fs::File;

use crate::economy::simulation::Simulation;
use crate::util::files::{Writer, Reader};

fn main() {
    /* get command line arguments from user */
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <input file path> <output file path>\n");
        std::process::exit(1);
    }

    /* create or/and open the IO files */
    let input_path  = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    
    let input_file = match File::open(&input_path) {
        Err(why) => {
            eprintln!("could not open {}: {}", input_path.display(), why);
            std::process::exit(1);
        },
        Ok(file) => file,
    };

    let output_file = match File::create(&output_path) {
        Err(why) => {
            eprintln!("could not create {}: {}", output_path.display(), why);
            std::process::exit(1);
        },
        Ok(file) => file,
    };

    /* read the input_file's content preparing the simulation */
    let mut simulation: Simulation = Reader::from_file(&input_file);

    /* perform the simulation */
    let prices = simulation.calculate_prices();
    
    /* write the new prices to output_file */
    Writer::to_file(&output_file, &simulation);
}
