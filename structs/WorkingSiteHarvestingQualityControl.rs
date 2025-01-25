use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogKey {
    #[serde(flatten)]
    pub log_key: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationDescription {
    #[serde(flatten)]
    pub diameter_calibration_description: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationDate {
    #[serde(flatten)]
    pub length_calibration_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stem {
    #[serde(flatten)]
    pub stem: StemDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibration {
    #[serde(flatten)]
    pub length_calibration: LengthCalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaliperApplication {
    #[serde(flatten)]
    pub caliper_application: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibration {
    #[serde(flatten)]
    pub diameter_calibration: DiameterCalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDiameterClass {
    #[serde(flatten)]
    pub log_diameter_class: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteHarvestingQualityControl {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomControlStemRejectedReason {
    #[serde(flatten)]
    pub random_control_stem_rejected_reason: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalibrationUseLog {
    #[serde(flatten)]
    pub calibration_use_log: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationReason {
    #[serde(flatten)]
    pub diameter_calibration_reason: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemId {
    #[serde(flatten)]
    pub stem_id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDiameter {
    #[serde(flatten)]
    pub log_diameter: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationDescription {
    #[serde(flatten)]
    pub length_calibration_description: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationAdjustment {
    #[serde(flatten)]
    pub length_calibration_adjustment: BdtInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlLogDiameter {
    #[serde(flatten)]
    pub control_log_diameter: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogLengthClass {
    #[serde(flatten)]
    pub log_length_class: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationReason {
    #[serde(flatten)]
    pub length_calibration_reason: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationAdjustmentButtLog {
    #[serde(flatten)]
    pub diameter_calibration_adjustment_butt_log: BdtInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMeasurements {
    #[serde(flatten)]
    pub log_measurements: LogMeasurementsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    #[serde(flatten)]
    pub log: LogDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationAdjustment {
    #[serde(flatten)]
    pub diameter_calibration_adjustment: BdtInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaliperId {
    #[serde(flatten)]
    pub caliper_id: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCoordinates {
    #[serde(flatten)]
    pub stem_coordinates: GdtPointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationDate {
    #[serde(flatten)]
    pub diameter_calibration_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Caliper {
    #[serde(flatten)]
    pub caliper: CaliperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementsType {
    #[serde(rename = "LogDiameter")]
    pub log_diameter: PositiveInteger3digitsType,
    #[serde(rename = "ControlLogDiameter")]
    pub control_log_diameter: PositiveInteger3digitsType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperType {
    #[serde(rename = "CaliperId", skip_serializing_if = "Option::is_none")]
    pub caliper_id: Option<String200Type>,
    #[serde(rename = "CaliperApplication", skip_serializing_if = "Option::is_none")]
    pub caliper_application: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemSelectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDataType {
    #[serde(rename = "LogKey")]
    pub log_key: String10Type,
    #[serde(rename = "ProductKey")]
    pub product_key: ERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: PositiveInteger4digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: PositiveInteger4digitsType,
    #[serde(rename = "CalibrationUseLog", skip_serializing_if = "Option::is_none")]
    pub calibration_use_log: Option<YesNoType>,
    #[serde(rename = "LogDiameterClass", skip_serializing_if = "Option::is_none")]
    pub log_diameter_class: Option<PositiveInteger3digitsType>,
    #[serde(rename = "LogLengthClass", skip_serializing_if = "Option::is_none")]
    pub log_length_class: Option<PositiveInteger4digitsType>,
    #[serde(rename = "LogMeasurements")]
    pub log_measurements: Vec<LogMeasurementsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "Stem")]
    pub stem: Vec<StemDataType>,
    #[serde(rename = "LengthCalibration", skip_serializing_if = "Option::is_none")]
    pub length_calibration: Option<Vec<LengthCalibrationType>>,
    #[serde(rename = "DiameterCalibration", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration: Option<Vec<DiameterCalibrationType>>,
    #[serde(rename = "Caliper", skip_serializing_if = "Option::is_none")]
    pub caliper: Option<CaliperType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDataType {
    #[serde(rename = "StemId")]
    pub stem_id: PositiveIntegerType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<HarvestingStemTypeType>,
    #[serde(rename = "SelectionType")]
    pub selection_type: ControlStemSelectionType,
    #[serde(rename = "RandomControlStemRejectedReason", skip_serializing_if = "Option::is_none")]
    pub random_control_stem_rejected_reason: Option<String100Type>,
    #[serde(rename = "StemCoordinates", skip_serializing_if = "Option::is_none")]
    pub stem_coordinates: Option<PointGeometryType>,
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "DiameterCalibrationDate")]
    pub diameter_calibration_date: TimeStampType,
    #[serde(rename = "DiameterCalibrationReason")]
    pub diameter_calibration_reason: String200Type,
    #[serde(rename = "DiameterCalibrationDescription")]
    pub diameter_calibration_description: String200Type,
    #[serde(rename = "DiameterCalibrationAdjustment")]
    pub diameter_calibration_adjustment: Integer3digitsType,
    #[serde(rename = "DiameterCalibrationAdjustmentButtLog", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration_adjustment_butt_log: Option<Integer3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "LengthCalibrationDate")]
    pub length_calibration_date: TimeStampType,
    #[serde(rename = "LengthCalibrationReason")]
    pub length_calibration_reason: String200Type,
    #[serde(rename = "LengthCalibrationDescription")]
    pub length_calibration_description: String200Type,
    #[serde(rename = "LengthCalibrationAdjustment")]
    pub length_calibration_adjustment: Integer3digitsType,
}

