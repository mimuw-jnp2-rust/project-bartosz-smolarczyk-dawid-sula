mod economy;
mod util;

use std::path::Path;

use crate::economy::simulation::Simulation;

fn main() {
    /* get command line arguments from user */
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <input file path> <output file path>\n");
        std::process::exit(1);
    }

    /* convert input string into file path */
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    /* check if files exist */
    assert!(input_path.is_file());

    /* load the simulation */
    let mut simulation = match Simulation::read_from_file(&input_path) {
        Err(why) => {
            eprintln!("could not open {}: {}", input_path.display(), why);
            std::process::exit(1);
        }
        Ok(result) => result,
    };

    simulation.run();
    simulation.plot(output_path.to_str().unwrap()).unwrap();
}
