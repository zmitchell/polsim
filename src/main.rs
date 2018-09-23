extern crate error_chain;
#[macro_use]
extern crate quicli;
extern crate polarization;
extern crate polsim;
extern crate toml;

use polarization::jones::JonesVector;
use polsim::errors::ResultExt;
use polsim::from_toml::SystemDef;
use polsim::validate;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "input", short = "i")]
    input: String,
}

main!(|args: Cli| {
    let file_contents = read_file(&args.input)?;
    let sys_def: SystemDef = toml::from_str(file_contents.as_str())?;
    let system = validate::validate_system(&sys_def).chain_err(|| "invalid system definition");
    if let Err(err) = system {
        eprintln!("error: {}", err);
        for e in err.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }
        std::process::exit(1);
    }
    let system = system.unwrap();
    let final_beam = system.propagate();
    match final_beam {
        Ok(beam) => {
            let intensity = beam.intensity();
            match intensity {
                Ok(int) => println!("intensity: {:e}", int),
                Err(err) => eprintln!("error: {}", err),
            }
        }
        Err(err) => eprintln!("error: {}", err),
    }
});
