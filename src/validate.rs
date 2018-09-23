use polarization::jones::{
    Angle, Beam, HalfWavePlate, Handedness, OpticalElement, OpticalSystem, Polarization,
    PolarizationRotator, Polarizer, QuarterWavePlate, Retarder,
};
use super::errors::*;
use super::from_toml::{AngleType, BeamDef, ElemDef, ElemType, HandednessType, PolType, SystemDef};
use num::complex::Complex;

fn validate_angle(angle: &Option<f64>, units: &Option<AngleType>) -> Result<Angle> {
    if angle.is_none() {
        return Err(ErrorKind::MissingParameter("angle".to_owned()).into());
    } else if units.is_none() {
        return Err(ErrorKind::MissingParameter("angle_units".to_owned()).into());
    }
    match units.unwrap() {
        AngleType::Degrees => return Ok(Angle::Degrees(angle.unwrap())),
        AngleType::Radians => return Ok(Angle::Radians(angle.unwrap())),
    }
}

fn validate_phase(phase: &Option<f64>, units: &Option<AngleType>) -> Result<Angle> {
    if phase.is_none() {
        return Err(ErrorKind::MissingParameter("phase".to_owned()).into());
    } else if units.is_none() {
        return Err(ErrorKind::MissingParameter("phase_units".to_owned()).into());
    }
    match units.unwrap() {
        AngleType::Degrees => return Ok(Angle::Degrees(phase.unwrap())),
        AngleType::Radians => return Ok(Angle::Radians(phase.unwrap())),
    }
}

fn validate_element(elem: &ElemDef) -> Result<OpticalElement> {
    match elem.element_type {
        ElemType::Polarizer => {
            validate_polarizer(elem).chain_err(|| "invalid polarizer definition")
        }
        ElemType::HWP => validate_hwp(elem).chain_err(|| "invalid half-wave plate definition"),
        ElemType::QWP => validate_qwp(elem).chain_err(|| "invalid quarter-wave plate definition"),
        ElemType::Retarder => validate_retarder(elem).chain_err(|| "invalid retarder definition"),
        ElemType::Rotator => {
            validate_rotator(elem).chain_err(|| "invalid polarization rotator definition")
        }
    }
}

fn validate_polarizer(elem: &ElemDef) -> Result<OpticalElement> {
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(OpticalElement::Polarizer(Polarizer::new(angle))),
    }
}

fn validate_hwp(elem: &ElemDef) -> Result<OpticalElement> {
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(OpticalElement::HalfWavePlate(HalfWavePlate::new(angle))),
    }
}

fn validate_qwp(elem: &ElemDef) -> Result<OpticalElement> {
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(OpticalElement::QuarterWavePlate(QuarterWavePlate::new(
            angle,
        ))),
    }
}

fn validate_rotator(elem: &ElemDef) -> Result<OpticalElement> {
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(OpticalElement::PolarizationRotator(
            PolarizationRotator::new(angle),
        )),
    }
}

fn validate_retarder(elem: &ElemDef) -> Result<OpticalElement> {
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    let phase_res =
        validate_phase(&elem.phase, &elem.phase_units).chain_err(|| "invalid phase definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => match phase_res {
            Err(err) => Err(err),
            Ok(phase) => Ok(OpticalElement::Retarder(Retarder::new(angle, phase))),
        },
    }
}

fn validate_beam(beam: &BeamDef) -> Result<Beam> {
    match beam.polarization {
        PolType::Linear => {
            validate_linear_pol(beam).chain_err(|| "invalid linear polarization definition")
        }
        PolType::Circular => {
            validate_circular_pol(beam).chain_err(|| "invalid circular polarization definition")
        }
        PolType::Elliptical => {
            validate_elliptical_pol(beam).chain_err(|| "invalid elliptical polarization definition")
        }
    }
}

fn validate_linear_pol(beam: &BeamDef) -> Result<Beam> {
    let angle_res =
        validate_angle(&beam.angle, &beam.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(Beam::linear(angle)),
    }
}

fn validate_handedness(h: &Option<HandednessType>) -> Result<Handedness> {
    if h.is_none() {
        return Err(ErrorKind::MissingParameter("handedness".to_owned()).into());
    }
    match h.unwrap() {
        HandednessType::Left => Ok(Handedness::Left),
        HandednessType::Right => Ok(Handedness::Right),
    }
}

fn validate_circular_pol(beam: &BeamDef) -> Result<Beam> {
    let hand_res =
        validate_handedness(&beam.handedness).chain_err(|| "invalid handedness definition");
    match hand_res {
        Err(err) => Err(err),
        Ok(h) => Ok(Beam::circular(h)),
    }
}

fn validate_x(beam: &BeamDef) -> Result<Complex<f64>> {
    if beam.x_mag.is_none() {
        return Err(ErrorKind::MissingParameter("x".to_owned()).into());
    }
    if beam.x_phase.is_none() {
        return Err(ErrorKind::MissingParameter("x_phase".to_owned()).into());
    }
    if beam.phase_units.is_none() {
        return Err(ErrorKind::MissingParameter("phase_units".to_owned()).into());
    }
    if beam.x_mag.unwrap() < 0.0 {
        return Err(ErrorKind::InvalidValue(format!(
            "invalid magnitude {}, magnitudes must be >= 0",
            beam.x_mag.unwrap()
        )).into());
    }
    let phase = match beam.phase_units.unwrap() {
        AngleType::Degrees => beam.x_phase.unwrap().to_radians(),
        AngleType::Radians => beam.x_phase.unwrap(),
    };
    Ok(Complex::from_polar(&beam.x_mag.unwrap(), &phase))
}

fn validate_y(beam: &BeamDef) -> Result<Complex<f64>> {
    if beam.y_mag.is_none() {
        return Err(ErrorKind::MissingParameter("y".to_owned()).into());
    }
    if beam.y_phase.is_none() {
        return Err(ErrorKind::MissingParameter("y_phase".to_owned()).into());
    }
    if beam.phase_units.is_none() {
        return Err(ErrorKind::MissingParameter("phase_units".to_owned()).into());
    }
    if beam.y_mag.unwrap() < 0.0 {
        return Err(ErrorKind::InvalidValue(format!(
            "invalid magnitude {}, magnitudes must be >= 0",
            beam.y_mag.unwrap()
        )).into());
    }
    let phase = match beam.phase_units.unwrap() {
        AngleType::Degrees => beam.y_phase.unwrap().to_radians(),
        AngleType::Radians => beam.y_phase.unwrap(),
    };
    Ok(Complex::from_polar(&beam.y_mag.unwrap(), &phase))
}

fn validate_elliptical_pol(beam: &BeamDef) -> Result<Beam> {
    let x_res = validate_x(&beam).chain_err(|| "invalid x component definition");
    let y_res = validate_y(&beam).chain_err(|| "invalid y component definition");
    match x_res {
        Err(err) => Err(err),
        Ok(x) => match y_res {
            Err(err) => Err(err),
            Ok(y) => Ok(Beam::new(x, y)),
        },
    }
}

pub fn validate_system(sys: &SystemDef) -> Result<OpticalSystem> {
    let beam_res = validate_beam(&sys.beam).chain_err(|| "invalid beam definition");
    let elements_res: Vec<Result<OpticalElement>> = sys
        .elements
        .iter()
        .map(|elem| validate_element(elem).chain_err(|| "invalid element definition"))
        .collect();
    let mut elements: Vec<OpticalElement> = Vec::new();
    for elem in elements_res.into_iter() {
        match elem {
            Ok(elem) => elements.push(elem),
            Err(err) => return Err(err),
        }
    }
    match beam_res {
        Err(err) => Err(err),
        Ok(beam) => Ok(OpticalSystem::new().with_beam(beam).with_elements(elements)),
    }
}

#[cfg(test)]
mod test {
    //
}
