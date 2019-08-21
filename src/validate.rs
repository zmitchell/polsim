use super::errors::*;
use super::from_toml::{AngleType, BeamDef, ElemDef, ElemType, HandednessType, PolType, SystemDef};
use num::complex::Complex;
use polarization::jones::{
    Angle, Beam, HalfWavePlate, Handedness, OpticalElement, OpticalSystem, PolarizationRotator,
    Polarizer, QuarterWavePlate, Retarder,
};

macro_rules! error_on_extra_params {
    ($base:ident, $( $field:ident ),+ $(,)*) => {$(
        if $base.$field.is_some() {
            return Err(ErrorKind::ExtraParameter(stringify!($field).to_string()).into())
        }
    )*}
}

macro_rules! error_on_missing_params {
    ($base:ident, $( $field:ident ),+ $(,)*) => {$(
        if $base.$field.is_none() {
            return Err(ErrorKind::MissingParameter(stringify!($field).to_string()).into())
        }
    )*}
}

macro_rules! error_if_none {
    ($( $x:ident ),* $(,)*) => {$(
        if $x.is_none() {
            return Err(ErrorKind::MissingParameter(stringify!($x).to_string()).into())
        }
    )*}
}

fn validate_angle(angle: &Option<f64>, units: &Option<AngleType>) -> Result<Angle> {
    error_if_none!(angle, units);
    match units.unwrap() {
        AngleType::Degrees => Ok(Angle::Degrees(angle.unwrap())),
        AngleType::Radians => Ok(Angle::Radians(angle.unwrap())),
    }
}

fn validate_phase(phase: &Option<f64>, units: &Option<AngleType>) -> Result<Angle> {
    error_if_none!(phase, units);
    match units.unwrap() {
        AngleType::Degrees => Ok(Angle::Degrees(phase.unwrap())),
        AngleType::Radians => Ok(Angle::Radians(phase.unwrap())),
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
    if elem.element_type != ElemType::Polarizer {
        return Err(ErrorKind::WrongElementType(format!(
            "Expected to validate element type Polarizer, found {:#?} instead",
            elem.element_type
        ))
        .into());
    }
    error_on_extra_params!(elem, phase, phase_units);
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(OpticalElement::Polarizer(Polarizer::new(angle))),
    }
}

fn validate_hwp(elem: &ElemDef) -> Result<OpticalElement> {
    if elem.element_type != ElemType::HWP {
        return Err(ErrorKind::WrongElementType(format!(
            "Expected to validate element type HWP, found {:#?} instead",
            elem.element_type
        ))
        .into());
    }
    error_on_extra_params!(elem, phase, phase_units);
    let angle_res =
        validate_angle(&elem.angle, &elem.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(OpticalElement::HalfWavePlate(HalfWavePlate::new(angle))),
    }
}

fn validate_qwp(elem: &ElemDef) -> Result<OpticalElement> {
    if elem.element_type != ElemType::QWP {
        return Err(ErrorKind::WrongElementType(format!(
            "Expected to validate element type QWP, found {:#?} instead",
            elem.element_type
        ))
        .into());
    }
    error_on_extra_params!(elem, phase, phase_units);
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
    if elem.element_type != ElemType::Rotator {
        return Err(ErrorKind::WrongElementType(format!(
            "Expected to validate element type Rotator, found {:#?} instead",
            elem.element_type
        ))
        .into());
    }
    error_on_extra_params!(elem, phase, phase_units);
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
    if elem.element_type != ElemType::Retarder {
        return Err(ErrorKind::WrongElementType(format!(
            "Expected to validate element type Retarder, found {:#?} instead",
            elem.element_type
        ))
        .into());
    }
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
    if beam.polarization != PolType::Linear {
        return Err(ErrorKind::WrongPolarizationType(format!(
            "expected to validate beam with polarization Linear, found {:#?} instead",
            beam.polarization
        ))
        .into());
    }
    error_on_extra_params!(
        beam,
        x_mag,
        x_phase,
        y_mag,
        y_phase,
        phase_units,
        handedness
    );
    let angle_res =
        validate_angle(&beam.angle, &beam.angle_units).chain_err(|| "invalid angle definition");
    match angle_res {
        Err(err) => Err(err),
        Ok(angle) => Ok(Beam::linear(angle)),
    }
}

fn validate_handedness(h: &Option<HandednessType>) -> Result<Handedness> {
    error_if_none!(h);
    match h.unwrap() {
        HandednessType::Left => Ok(Handedness::Left),
        HandednessType::Right => Ok(Handedness::Right),
    }
}

fn validate_circular_pol(beam: &BeamDef) -> Result<Beam> {
    if beam.polarization != PolType::Circular {
        return Err(ErrorKind::WrongPolarizationType(format!(
            "expected to validate beam with polarization Circular, found {:#?} instead",
            beam.polarization
        ))
        .into());
    }
    error_on_extra_params!(
        beam,
        x_mag,
        x_phase,
        y_mag,
        y_phase,
        phase_units,
        angle,
        angle_units
    );
    let hand_res =
        validate_handedness(&beam.handedness).chain_err(|| "invalid handedness definition");
    match hand_res {
        Err(err) => Err(err),
        Ok(h) => Ok(Beam::circular(h)),
    }
}

fn validate_x(beam: &BeamDef) -> Result<Complex<f64>> {
    error_on_missing_params!(beam, x_mag, x_phase, phase_units);
    if beam.x_mag.unwrap() < 0.0 {
        return Err(ErrorKind::InvalidValue(format!(
            "invalid magnitude {}, magnitudes must be >= 0",
            beam.x_mag.unwrap()
        ))
        .into());
    }
    let phase = match beam.phase_units.unwrap() {
        AngleType::Degrees => beam.x_phase.unwrap().to_radians(),
        AngleType::Radians => beam.x_phase.unwrap(),
    };
    Ok(Complex::from_polar(&beam.x_mag.unwrap(), &phase))
}

fn validate_y(beam: &BeamDef) -> Result<Complex<f64>> {
    error_on_missing_params!(beam, y_mag, y_phase, phase_units);
    if beam.y_mag.unwrap() < 0.0 {
        return Err(ErrorKind::InvalidValue(format!(
            "invalid magnitude {}, magnitudes must be >= 0",
            beam.y_mag.unwrap()
        ))
        .into());
    }
    let phase = match beam.phase_units.unwrap() {
        AngleType::Degrees => beam.y_phase.unwrap().to_radians(),
        AngleType::Radians => beam.y_phase.unwrap(),
    };
    Ok(Complex::from_polar(&beam.y_mag.unwrap(), &phase))
}

fn validate_elliptical_pol(beam: &BeamDef) -> Result<Beam> {
    if beam.polarization != PolType::Elliptical {
        return Err(ErrorKind::WrongPolarizationType(format!(
            "expected to validate beam with polarization Elliptical, found {:#?} instead",
            beam.polarization
        ))
        .into());
    }
    error_on_extra_params!(beam, angle, angle_units, handedness);
    let x_res = validate_x(&beam).chain_err(|| "invalid x component definition");
    let y_res = validate_y(&beam).chain_err(|| "invalid y component definition");
    match x_res {
        Err(err) => Err(err),
        Ok(x) => match y_res {
            Err(err) => Err(err),
            Ok(y) => {
                let norm = (x.norm_sqr() + y.norm_sqr()).sqrt();
                Ok(Beam::new(x / norm, y / norm))
            }
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
    for elem in elements_res {
        match elem {
            Ok(elem) => elements.push(elem),
            Err(err) => return Err(err),
        }
    }
    match beam_res {
        Err(err) => Err(err),
        Ok(beam) => Ok(OpticalSystem::new().add_beam(beam).add_elements(elements)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::{self, prelude::*};

    fn arb_element_type() -> BoxedStrategy<ElemType> {
        prop_oneof![
            Just(ElemType::Polarizer),
            Just(ElemType::HWP),
            Just(ElemType::QWP),
            Just(ElemType::Rotator),
            Just(ElemType::Retarder),
        ]
        .boxed()
    }

    fn arb_angle_type() -> BoxedStrategy<AngleType> {
        prop_oneof![Just(AngleType::Degrees), Just(AngleType::Radians),].boxed()
    }

    fn arb_optional_angle_type() -> BoxedStrategy<Option<AngleType>> {
        proptest::option::of(arb_angle_type()).boxed()
    }

    fn float_is_well_behaved(x: f64) -> bool {
        let not_nan = !x.is_nan();
        let not_too_small = x > -1e12;
        let not_too_big = x < 1e12;
        return not_nan && not_too_small && not_too_big;
    }

    fn arb_optional_float() -> BoxedStrategy<Option<f64>> {
        let well_behaved_floats =
            any::<f64>().prop_filter("float is not well-behaved", |x| float_is_well_behaved(*x));
        proptest::option::of(well_behaved_floats).boxed()
    }

    fn arb_element_def() -> BoxedStrategy<ElemDef> {
        (
            arb_element_type(),
            arb_optional_float(),
            arb_optional_angle_type(),
            arb_optional_float(),
            arb_optional_angle_type(),
        )
            .prop_map(|(typ, ph, ph_units, ang, ang_units)| ElemDef {
                element_type: typ,
                phase: ph,
                phase_units: ph_units,
                angle: ang,
                angle_units: ang_units,
            })
            .boxed()
    }

    fn arb_polarization_type() -> BoxedStrategy<PolType> {
        prop_oneof![
            Just(PolType::Linear),
            Just(PolType::Circular),
            Just(PolType::Elliptical),
        ]
        .boxed()
    }

    fn arb_optional_handedness() -> BoxedStrategy<Option<HandednessType>> {
        let arb_handedness = prop_oneof![Just(HandednessType::Left), Just(HandednessType::Right),];
        proptest::option::of(arb_handedness).boxed()
    }

    fn arb_beam_def() -> BoxedStrategy<BeamDef> {
        (
            arb_polarization_type(),
            arb_optional_float(),
            arb_optional_angle_type(),
            arb_optional_float(),
            arb_optional_float(),
            arb_optional_float(),
            arb_optional_float(),
            arb_optional_angle_type(),
            arb_optional_handedness(),
        )
            .prop_map(|(typ, a, au, xm, xp, ym, yp, pu, h)| BeamDef {
                polarization: typ,
                angle: a,
                angle_units: au,
                x_mag: xm,
                x_phase: xp,
                y_mag: ym,
                y_phase: yp,
                phase_units: pu,
                handedness: h,
            })
            .boxed()
    }

    macro_rules! assert_angles_approx_eq {
        ($x:expr, $y:expr) => {
            match $x {
                Angle::Degrees(x_deg) => match $y {
                    Angle::Degrees(y_deg) => {
                        assert_approx_eq!(x_deg, y_deg);
                    }
                    Angle::Radians(y_rad) => {
                        assert_approx_eq!(x_deg, y_rad.to_degrees());
                    }
                },
                Angle::Radians(x_rad) => match $y {
                    Angle::Degrees(y_deg) => {
                        assert_approx_eq!(x_rad, y_deg.to_radians());
                    }
                    Angle::Radians(y_rad) => {
                        assert_approx_eq!(x_rad, y_rad);
                    }
                },
            }
        };
    }

    #[test]
    fn test_validate_angle_rejects_partial_def() {
        let good_value: Option<f64> = Some(0.0);
        let good_units: Option<AngleType> = Some(AngleType::Degrees);
        assert!(validate_angle(&good_value, &None).is_err());
        assert!(validate_angle(&None, &good_units).is_err());
    }

    proptest! {
        #[test]
        fn test_validate_angle_returns_correct_angle(theta in 0_f64..90_f64) {
            let correct_angle = Angle::Degrees(theta);
            let from_validation = validate_angle(&Some(theta), &Some(AngleType::Degrees)).unwrap();
            assert_angles_approx_eq!(correct_angle, from_validation);
        }
    }

    #[test]
    fn test_validate_phase_rejects_partial_def() {
        let good_value: Option<f64> = Some(0.0);
        let good_units: Option<AngleType> = Some(AngleType::Degrees);
        assert!(validate_phase(&good_value, &None).is_err());
        assert!(validate_phase(&None, &good_units).is_err());
    }

    proptest! {
        #[test]
        fn test_validate_phase_returns_correct_phase(theta in 0_f64..90_f64) {
            let correct_phase = Angle::Degrees(theta);
            let from_validation = validate_phase(&Some(theta), &Some(AngleType::Degrees)).unwrap();
            assert_angles_approx_eq!(correct_phase, from_validation);
        }
    }

    proptest! {
        #[test]
        fn test_validate_polarizer_rejects_invalid_def(def in arb_element_def()) {
            let correct_type = def.element_type == ElemType::Polarizer;
            let valid_angle_def = def.angle.is_some() && def.angle_units.is_some();
            let no_phase = def.phase.is_none() && def.phase_units.is_none();
            let valid_def = correct_type && valid_angle_def && no_phase;
            if valid_def {
                prop_assert!(validate_polarizer(&def).is_ok());
            } else {
                prop_assert!(validate_polarizer(&def).is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn test_validate_hwp_rejects_invalid_def(def in arb_element_def()) {
            let correct_type = def.element_type == ElemType::HWP;
            let valid_angle_def = def.angle.is_some() && def.angle_units.is_some();
            let no_phase = def.phase.is_none() && def.phase_units.is_none();
            let valid_def = correct_type && valid_angle_def && no_phase;
            if valid_def {
                prop_assert!(validate_hwp(&def).is_ok());
            } else {
                prop_assert!(validate_hwp(&def).is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn test_validate_qwp_rejects_invalid_def(def in arb_element_def()) {
            let correct_type = def.element_type == ElemType::QWP;
            let valid_angle_def = def.angle.is_some() && def.angle_units.is_some();
            let no_phase = def.phase.is_none() && def.phase_units.is_none();
            let valid_def = correct_type && valid_angle_def && no_phase;
            if valid_def {
                prop_assert!(validate_qwp(&def).is_ok());
            } else {
                prop_assert!(validate_qwp(&def).is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn test_validate_rotator_rejects_invalid_def(def in arb_element_def()) {
            let correct_type = def.element_type == ElemType::Rotator;
            let valid_angle_def = def.angle.is_some() && def.angle_units.is_some();
            let no_phase = def.phase.is_none() && def.phase_units.is_none();
            let valid_def = correct_type && valid_angle_def && no_phase;
            if valid_def {
                prop_assert!(validate_rotator(&def).is_ok());
            } else {
                prop_assert!(validate_rotator(&def).is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn test_validate_linear_pol_rejects_invalid_def(def in arb_beam_def()) {
            let has_correct_pol = def.polarization == PolType::Linear;
            let has_angle_def = def.angle.is_some() && def.angle_units.is_some();
            let no_x_def = def.x_mag.is_none() && def.x_phase.is_none();
            let no_y_def = def.y_mag.is_none() && def.y_phase.is_none();
            let no_phase_units = def.phase_units.is_none();
            let no_handedness = def.handedness.is_none();
            let valid_def = has_correct_pol
                && has_angle_def
                && no_x_def
                && no_y_def
                && no_phase_units
                && no_handedness;
            if valid_def {
                prop_assert!(validate_linear_pol(&def).is_ok());
            } else {
                prop_assert!(validate_linear_pol(&def).is_err());
            }
        }

        #[test]
        fn test_validate_circular_pol_rejects_invalid_def(def in arb_beam_def()) {
            let has_correct_pol = def.polarization == PolType::Circular;
            let has_handedness = def.handedness.is_some();
            let no_x_def = def.x_mag.is_none() && def.x_phase.is_none();
            let no_y_def = def.y_mag.is_none() && def.y_phase.is_none();
            let no_phase_units = def.phase_units.is_none();
            let no_angle = def.angle.is_none() && def.angle_units.is_none();
            let valid_def = has_correct_pol
                && has_handedness
                && no_x_def
                && no_y_def
                && no_phase_units
                && no_angle;
            if valid_def {
                prop_assert!(validate_circular_pol(&def).is_ok());
            } else {
                prop_assert!(validate_circular_pol(&def).is_err());
            }
        }

        #[test]
        fn test_validate_elliptical_pol_rejects_invalid_def(def in arb_beam_def()) {
            let has_correct_pol = def.polarization == PolType::Elliptical;
            let has_x = def.x_mag.is_some() && def.x_phase.is_some();
            let has_y = def.y_mag.is_some() && def.y_phase.is_some();
            let has_valid_magnitudes = match def.x_mag {
                Some(x) if x > 0.0 => match def.y_mag {
                    Some(y) if y > 0.0 => true,
                    _ => false,
                },
                _ => false
            };
            let has_phase_units = def.phase_units.is_some();
            let no_angle_def = def.angle.is_none() && def.angle_units.is_none();
            let no_handedness = def.handedness.is_none();
            let valid_def = has_correct_pol
                && has_x
                && has_y
                && has_valid_magnitudes
                && has_phase_units
                && no_angle_def
                && no_handedness;
            if valid_def {
                prop_assert!(validate_elliptical_pol(&def).is_ok());
            } else {
                prop_assert!(validate_elliptical_pol(&def).is_err());
            }
        }
    }

}
