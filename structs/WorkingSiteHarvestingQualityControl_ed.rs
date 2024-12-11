#[derive(Debug, Serialize, Deserialize)]
pub struct LogLength {
    #[serde(flatten)]
    pub log_length: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationDate {
    #[serde(flatten)]
    pub length_calibration_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibration {
    #[serde(flatten)]
    pub diameter_calibration: DiameterCalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibration {
    #[serde(flatten)]
    pub length_calibration: LengthCalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperApplication {
    #[serde(flatten)]
    pub caliper_application: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemId {
    #[serde(flatten)]
    pub stem_id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationUseLog {
    #[serde(flatten)]
    pub calibration_use_log: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stem {
    #[serde(flatten)]
    pub stem: StemDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControl {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogVolume {
    #[serde(flatten)]
    pub control_log_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameter {
    #[serde(flatten)]
    pub log_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustmentButtLog {
    #[serde(flatten)]
    pub diameter_calibration_adjustment_butt_log: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: HarvestingStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectionType {
    #[serde(flatten)]
    pub selection_type: ControlStemSelectionType,
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
pub struct ProductKey {
    #[serde(flatten)]
    pub product_key: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogLength {
    #[serde(flatten)]
    pub control_log_length: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: StanfordTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogDiameter {
    #[serde(flatten)]
    pub control_log_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationDate {
    #[serde(flatten)]
    pub diameter_calibration_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustment {
    #[serde(flatten)]
    pub diameter_calibration_adjustment: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationDescription {
    #[serde(flatten)]
    pub diameter_calibration_description: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    #[serde(flatten)]
    pub log: LogDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationAdjustment {
    #[serde(flatten)]
    pub length_calibration_adjustment: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationReason {
    #[serde(flatten)]
    pub diameter_calibration_reason: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCoordinates {
    #[serde(flatten)]
    pub stem_coordinates: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: MeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationReason {
    #[serde(flatten)]
    pub length_calibration_reason: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameterClass {
    #[serde(flatten)]
    pub log_diameter_class: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogVolume {
    #[serde(flatten)]
    pub log_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurements {
    #[serde(flatten)]
    pub log_measurements: LogMeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caliper {
    #[serde(flatten)]
    pub caliper: CaliperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogKey {
    #[serde(flatten)]
    pub log_key: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationDescription {
    #[serde(flatten)]
    pub length_calibration_description: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperId {
    #[serde(flatten)]
    pub caliper_id: String200Type,
}

