#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibration {
    #[serde(flatten)]
    pub length_calibration: LengthCalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurements {
    #[serde(flatten)]
    pub log_measurements: LogMeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationDescription {
    #[serde(flatten)]
    pub length_calibration_description: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stem {
    #[serde(flatten)]
    pub stem: StemDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemId {
    #[serde(flatten)]
    pub stem_id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationDate {
    #[serde(flatten)]
    pub diameter_calibration_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustmentButtLog {
    #[serde(flatten)]
    pub diameter_calibration_adjustment_butt_log: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperApplication {
    #[serde(flatten)]
    pub caliper_application: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibration {
    #[serde(flatten)]
    pub diameter_calibration: DiameterCalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogKey {
    #[serde(flatten)]
    pub log_key: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControl {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationUseLog {
    #[serde(flatten)]
    pub calibration_use_log: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCoordinates {
    #[serde(flatten)]
    pub stem_coordinates: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationReason {
    #[serde(flatten)]
    pub length_calibration_reason: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationReason {
    #[serde(flatten)]
    pub diameter_calibration_reason: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationDate {
    #[serde(flatten)]
    pub length_calibration_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationDescription {
    #[serde(flatten)]
    pub diameter_calibration_description: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameterClass {
    #[serde(flatten)]
    pub log_diameter_class: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameter {
    #[serde(flatten)]
    pub log_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogDiameter {
    #[serde(flatten)]
    pub control_log_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caliper {
    #[serde(flatten)]
    pub caliper: CaliperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogLengthClass {
    #[serde(flatten)]
    pub log_length_class: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomControlStemRejectedReason {
    #[serde(flatten)]
    pub random_control_stem_rejected_reason: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperId {
    #[serde(flatten)]
    pub caliper_id: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustment {
    #[serde(flatten)]
    pub diameter_calibration_adjustment: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationAdjustment {
    #[serde(flatten)]
    pub length_calibration_adjustment: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    #[serde(flatten)]
    pub log: LogDataType,
}

