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
use polsim::report::{basic_report, table_report};
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "polsim",
    about = "Simulate polarization with Jones calculus."
)]
struct Cli {
    #[structopt(
        raw(required = r#"true"#),
        value_name = "FILE",
        help = "Input file defining the optical system."
    )]
    input: String,
    #[structopt(
        short = "p",
        long = "pretty",
        raw(takes_value = r#"false"#),
        help = "Pretty-print the output as a table."
    )]
    pretty: bool
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
            match beam.intensity() {
                Ok(_) => {
                    if args.pretty {
                        table_report(beam);
                    } else {
                        basic_report(beam);
                    }
                },
                Err(err) => eprintln!("error: {}", err),
            }
        },
        Err(err) => eprintln!("error: {}", err),
    }
});
