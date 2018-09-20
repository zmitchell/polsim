#[macro_use] extern crate quicli;
extern crate toml;
extern crate polsim;

use quicli::prelude::*;
use polsim::from_toml::SystemDef;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "input", short = "i")]
    input: String,
}

main!(|args: Cli| {
    let file_contents = read_file(&args.input)?;
    let system: SystemDef = toml::from_str(file_contents.as_str())?;
    println!("{:#?}", system);
});
