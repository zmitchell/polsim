extern crate polarization;
#[macro_use] extern crate quicli;
#[macro_use] extern crate serde_derive;
extern crate toml;

use polarization::jones::{
    Angle, Beam, OpticalElement, OpticalSystem, Polarization, Polarizer,
    PolarizationRotator, Retarder, QuarterWavePlate, HalfWavePlate, Handedness,
};
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "input", short = "i")]
    input: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SystemDef {
    beam: BeamDef,
    elements: Vec<ElemDef>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BeamDef {
    polarization: PolType,
    #[serde(flatten)]
    angle: Option<AngleDef>,
    #[serde(flatten)]
    components: Option<BeamComponentDef>,
    handedness: Option<HandednessType>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ElemDef {
    element_type: ElemType,
    #[serde(flatten)]
    angle: Option<AngleDef>,
    #[serde(flatten)]
    phase: Option<PhaseDef>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ElemType {
    Polarizer,
    Rotator,
    Retarder,
    HWP,
    QWP,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum PolType {
    Linear,
    Circular,
    Elliptical,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum AngleType {
    Degrees,
    Radians,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AngleDef {
    angle: f64,
    #[serde(rename = "angle_units")]
    units: AngleType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PhaseDef {
    phase: f64,
    #[serde(rename = "phase_units")]
    units: AngleType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BeamComponentDef {
    #[serde(rename = "x_magnitude")]
    x_mag: f64,
    x_phase: f64,
    #[serde(rename = "y_magnitude")]
    y_mag: f64,
    y_phase: f64,
    phase_units: AngleType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum HandednessType {
    Left,
    Right,
}

main!(|args: Cli| {
    let file_contents = read_file(&args.input)?;
    let system: SystemDef = toml::from_str(file_contents.as_str())?;
    println!("{:#?}", system);
});
