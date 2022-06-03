mod economy;
mod util;

use std::path::Path;

use crate::economy::simulation::{SimulationBuilder, Simulation};

fn main() {
    /* get command line arguments from user */
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <input file path> <output file path>\n");
        std::process::exit(1);
    }

    /* convert input strings into file paths */
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    /* load the simulation */
    let simulation_builder = match SimulationBuilder::read_from_file(&input_path) {
        Err(why) => {
            eprintln!("could not open {}: {}", input_path.display(), why);
            std::process::exit(1);
        }
        Ok(file) => file,
    };

    let mut simulation: Simulation = simulation_builder.build();

    let prices = simulation.run();

    println!("{:#?}", prices);
}
