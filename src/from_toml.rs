#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SystemDef {
    pub beam: BeamDef,
    pub elements: Vec<ElemDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BeamDef {
    pub polarization: PolType,
    pub angle: Option<f64>,
    pub angle_units: Option<AngleType>,
    pub x_mag: Option<f64>,
    pub x_phase: Option<f64>,
    pub y_mag: Option<f64>,
    pub y_phase: Option<f64>,
    pub phase_units: Option<AngleType>,
    pub handedness: Option<HandednessType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElemDef {
    pub element_type: ElemType,
    pub phase: Option<f64>,
    pub phase_units: Option<AngleType>,
    pub angle: Option<f64>,
    pub angle_units: Option<AngleType>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ElemType {
    Polarizer,
    Rotator,
    Retarder,
    HWP,
    QWP,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PolType {
    Linear,
    Circular,
    Elliptical,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AngleType {
    Degrees,
    Radians,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HandednessType {
    Left,
    Right,
}
