use polarization::jones::{
    Angle, Beam, OpticalElement, OpticalSystem, Polarization, Polarizer,
    PolarizationRotator, Retarder, QuarterWavePlate, HalfWavePlate, Handedness,
};
use super::common::{Result};
use super::from_toml::{
    AngleType, BeamDef, ElemDef, ElemType, HandednessType, PolType, SystemDef,
};

macro_rules! fields_valid {
    ($x:ident, $($field:ident),+) => {
        let mut valid = true;
        $(
            if $x.$field.is_none() {
                valid = false;
            }
        )*
        return valid
    }
}
