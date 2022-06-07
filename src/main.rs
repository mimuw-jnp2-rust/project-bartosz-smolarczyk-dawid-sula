mod economy;
mod util;

use std::{fs::File, path::Path};

use crate::economy::simulation::{Simulation, SimulationBuilder};

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
        Ok(result) => result,
    };

    println!("{:#?}", simulation_builder);
    let mut simulation = simulation_builder.build();
    println!("{:#?}", simulation);
    simulation.plot(&args[2]);

    // let output_file = match File::create(&output_path) {
    //     Err(why) => {
    //         eprintln!("could not create {}: {}", output_path.display(), why);
    //         std::process::exit(1);
    //     }
    //     Ok(file) => file,
    // };
}
