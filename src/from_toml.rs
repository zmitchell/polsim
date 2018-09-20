#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SystemDef {
    beam: BeamDef,
    elements: Vec<ElemDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BeamDef {
    polarization: PolType,
    angle: Option<f64>,
    angle_units: Option<AngleType>,
    x: Option<f64>,
    x_phase: Option<f64>,
    y: Option<f64>,
    y_phase: Option<f64>,
    phase_units: Option<AngleType>,
    handedness: Option<HandednessType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElemDef {
    element_type: ElemType,
    phase: Option<f64>,
    phase_units: Option<AngleType>,
    angle: Option<f64>,
    angle_units: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ElemType {
    Polarizer,
    Rotator,
    Retarder,
    HWP,
    QWP,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PolType {
    Linear,
    Circular,
    Elliptical,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AngleType {
    Degrees,
    Radians,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HandednessType {
    Left,
    Right,
}

