use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomMINLength {
    #[serde(flatten)]
    pub random_m_i_n_length: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomControlStemMeasurementMode {
    #[serde(flatten)]
    pub random_control_stem_measurement_mode: RandomControlStemMeasurementModeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlMeasurementDefinition {
    #[serde(flatten)]
    pub control_measurement_definition: ControlMeasurementDefinitionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomEndTime {
    #[serde(flatten)]
    pub random_end_time: Xsdtime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomFrequency {
    #[serde(flatten)]
    pub random_frequency: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomStartTime {
    #[serde(flatten)]
    pub random_start_time: Xsdtime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomControlStemSelection {
    #[serde(flatten)]
    pub random_control_stem_selection: RandomControlStemSelectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlStemSettings {
    #[serde(flatten)]
    pub control_stem_settings: ControlStemSettingsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingQualityControl {
    #[serde(flatten)]
    pub harvesting_quality_control: HarvestingQualityControlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaliperID {
    #[serde(flatten)]
    pub caliper_i_d: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomMINDBH {
    #[serde(flatten)]
    pub random_m_i_n_d_b_h: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingQualityControlHeader {
    #[serde(flatten)]
    pub harvesting_quality_control_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalibrationValues {
    #[serde(flatten)]
    pub calibration_values: CalibrationValuesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationAdjustmentButtLog {
    #[serde(flatten)]
    pub length_calibration_adjustment_butt_log: LengthCalibrationAdjustmentButtLogType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlStemInfo {
    #[serde(flatten)]
    pub control_stem_info: ControlStemInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomFrequencyCategory {
    #[serde(flatten)]
    pub random_frequency_category: RandomFrequencyCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomSettingDateTime {
    #[serde(flatten)]
    pub random_setting_date_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomLogNotification {
    #[serde(flatten)]
    pub random_log_notification: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlValues {
    #[serde(flatten)]
    pub control_values: ControlValuesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemSettingsType {
    #[serde(rename = "RandomFrequency")]
    pub random_frequency: u32,
    #[serde(rename = "RandomFrequencyCategory")]
    pub random_frequency_category: RandomFrequencyCategoryType,
    #[serde(rename = "RandomMINDBH")]
    pub random_m_i_n_d_b_h: u32,
    #[serde(rename = "RandomMINLength")]
    pub random_m_i_n_length: u32,
    #[serde(rename = "RandomLogNotification")]
    pub random_log_notification: u32,
    #[serde(rename = "RandomStartTime")]
    pub random_start_time: NaiveTime,
    #[serde(rename = "RandomEndTime")]
    pub random_end_time: NaiveTime,
    #[serde(rename = "RandomSettingDateTime")]
    pub random_setting_date_time: StanForD2010DateTimeType,
    #[serde(rename = "SpeciesGroupUserID")]
    pub species_group_user_i_d: UserIDType,
    #[serde(rename = "ProcessingCategory")]
    pub processing_category: ProcessingCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlMeasurementDefinitionType {
    #[serde(rename = "Measurer")]
    pub measurer: ContactInformationType,
    #[serde(rename = "CaliperApplication")]
    pub caliper_application: String,
    #[serde(rename = "CaliperID")]
    pub caliper_i_d: String,
    #[serde(rename = "logMeasurementCategory")]
    pub log_measurement_category: LogMeasurementCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationReasonType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonCalibrationValuesType {
    #[serde(rename = "SpeciesGroupUserID")]
    pub species_group_user_i_d: UserIDType,
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: StanForD2010DateTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlValuesType {
    #[serde(rename = "ProductDefinition")]
    pub product_definition: Vec<ProductDefinitionMachineHarvestingQualityControlType>,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: Vec<ObjectDefinitionMachineType>,
    #[serde(rename = "ControlStemSettings")]
    pub control_stem_settings: Vec<ControlStemSettingsType>,
    #[serde(rename = "Stem")]
    pub stem: Vec<StemHarvestingQualityControlType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingQualityControlType {
    #[serde(rename = "HarvestingQualityControlHeader")]
    pub harvesting_quality_control_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineHarvestingQualityControlType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationAdjustmentType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "lengthCalibrationPosition")]
    pub length_calibration_position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogHarvestingQualityControlType {
    #[serde(flatten)]
    pub base: LogType,
    #[serde(rename = "LogMeasurement")]
    pub log_measurement: Vec<LogMeasurementHarvestingQualityControlType>,
    #[serde(rename = "CalibrationUseLog", skip_serializing_if = "Option::is_none")]
    pub calibration_use_log: Option<bool>,
    #[serde(rename = "LogLengthClass", skip_serializing_if = "Option::is_none")]
    pub log_length_class: Option<u32>,
    #[serde(rename = "LogDiameterClass", skip_serializing_if = "Option::is_none")]
    pub log_diameter_class: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationValuesType {
    #[serde(rename = "DiameterCalibration", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration: Option<Vec<DiameterCalibrationType>>,
    #[serde(rename = "LengthCalibration", skip_serializing_if = "Option::is_none")]
    pub length_calibration: Option<Vec<LengthCalibrationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationAdjustmentButtLogType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "lengthCalibrationPosition")]
    pub length_calibration_position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomFrequencyCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationType {
    #[serde(flatten)]
    pub base: CommonCalibrationValuesType,
    #[serde(rename = "DiameterCalibrationReason")]
    pub diameter_calibration_reason: CalibrationReasonType,
    #[serde(rename = "DiameterCalibrationDescription")]
    pub diameter_calibration_description: String,
    #[serde(rename = "DiameterCalibrationAdjustment")]
    pub diameter_calibration_adjustment: Vec<DiameterCalibrationAdjustmentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifiedProductDefinitionMachineHarvestingQualityControlType {
    #[serde(flatten)]
    pub base: ProductDefinitionWithCommonDetailsType,
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: UserIDType,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogDiameterType {
    #[serde(flatten)]
    pub base: XsdnonNegativeInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemInfoType {
    #[serde(rename = "RandomControlStemRejectedReason")]
    pub random_control_stem_rejected_reason: RandomControlStemRejectedReasonType,
    #[serde(rename = "RandomControlStemMeasurementMode", skip_serializing_if = "Option::is_none")]
    pub random_control_stem_measurement_mode: Option<RandomControlStemMeasurementModeType>,
    #[serde(rename = "RandomControlStemSelection")]
    pub random_control_stem_selection: RandomControlStemSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleTreeProcessedStemHarvestingQualityControlType {
    #[serde(rename = "DBH")]
    pub dbh: u32,
    #[serde(rename = "ReferenceDiameter")]
    pub reference_diameter: ReferenceDiameterType,
    #[serde(rename = "StemGrade")]
    pub stem_grade: Vec<StemGradeType>,
    #[serde(rename = "StemDiameters", skip_serializing_if = "Option::is_none")]
    pub stem_diameters: Option<StemDiametersType>,
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogHarvestingQualityControlType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameterHarvestingQualityControlType {
    #[serde(flatten)]
    pub base: XsdnonNegativeInteger,
    #[serde(rename = "logDiameterCategory")]
    pub log_diameter_category: LogDiameterCategoryType,
    #[serde(rename = "diameterMeasurementCategory")]
    pub diameter_measurement_category: DiameterMeasurementCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineHarvestingQualityControlType {
    #[serde(flatten)]
    pub base: MachineWithHeadType,
    #[serde(rename = "SpeciesGroupDefinition")]
    pub species_group_definition: Vec<SpeciesGroupDefinitionWithDetailsAndKeyType>,
    #[serde(rename = "ControlValues")]
    pub control_values: ControlValuesType,
    #[serde(rename = "CalibrationValues", skip_serializing_if = "Option::is_none")]
    pub calibration_values: Option<CalibrationValuesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingQualityControlMessageCategory {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementHarvestingQualityControlType {
    #[serde(rename = "LogDiameter", skip_serializing_if = "Option::is_none")]
    pub log_diameter: Option<Vec<LogDiameterHarvestingQualityControlType>>,
    #[serde(rename = "LogLength", skip_serializing_if = "Option::is_none")]
    pub log_length: Option<u32>,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: StanForD2010DateTimeType,
    #[serde(rename = "ControlLogDiameter", skip_serializing_if = "Option::is_none")]
    pub control_log_diameter: Option<Vec<ControlLogDiameterType>>,
    #[serde(rename = "logMeasurementCategory")]
    pub log_measurement_category: LogMeasurementCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomControlStemMeasurementModeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomControlStemRejectedReasonType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionMachineHarvestingQualityControlType {
    #[serde(rename = "ProductKey")]
    pub product_key: u32,
    #[serde(rename = "ClassifiedProductDefinition")]
    pub classified_product_definition: ClassifiedProductDefinitionMachineHarvestingQualityControlType,
    #[serde(rename = "UnclassifiedProductDefinition")]
    pub unclassified_product_definition: UnclassifiedProductDefinitionMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationType {
    #[serde(flatten)]
    pub base: CommonCalibrationValuesType,
    #[serde(rename = "LengthCalibrationReason")]
    pub length_calibration_reason: CalibrationReasonType,
    #[serde(rename = "LengthCalibrationDescription")]
    pub length_calibration_description: String,
    #[serde(rename = "LengthCalibrationAdjustment")]
    pub length_calibration_adjustment: Vec<LengthCalibrationAdjustmentType>,
    #[serde(rename = "LengthCalibrationAdjustmentButtLog", skip_serializing_if = "Option::is_none")]
    pub length_calibration_adjustment_butt_log: Option<LengthCalibrationAdjustmentButtLogType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemHarvestingQualityControlType {
    #[serde(flatten)]
    pub base: StemType,
    #[serde(rename = "ControlMeasurementDefinition", skip_serializing_if = "Option::is_none")]
    pub control_measurement_definition: Option<Vec<ControlMeasurementDefinitionType>>,
    #[serde(rename = "ControlStemInfo")]
    pub control_stem_info: ControlStemInfoType,
    #[serde(rename = "SingleTreeProcessedStem")]
    pub single_tree_processed_stem: SingleTreeProcessedStemHarvestingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogDiameterCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomControlStemSelectionType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustmentType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "diameterCalibrationPosition")]
    pub diameter_calibration_position: u32,
}

