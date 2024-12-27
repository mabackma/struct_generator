#[derive(Debug, Serialize, Deserialize)]
pub struct ScaledMass {
    #[serde(flatten)]
    pub scaled_mass: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orientation {
    #[serde(flatten)]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationDate {
    #[serde(flatten)]
    pub calibration_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationAdjustment {
    #[serde(flatten)]
    pub calibration_adjustment: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleData {
    #[serde(flatten)]
    pub scale_data: ScaleDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calibration {
    #[serde(flatten)]
    pub calibration: CalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceMass {
    #[serde(flatten)]
    pub control_reference_mass: BdtDecimal1FractionDigitType,
}

