use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementId {
    #[serde(flatten)]
    pub measurement_id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogCount {
    #[serde(flatten)]
    pub log_count: BdtPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    #[serde(flatten)]
    pub temperature: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements {
    #[serde(flatten)]
    pub measurements: MeasurementsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteHarvestingQualityControlManual {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlLogCount {
    #[serde(flatten)]
    pub control_log_count: BdtPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {
    #[serde(flatten)]
    pub measurement: MeasurementDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDataType {
    #[serde(rename = "MeasurementId")]
    pub measurement_id: PositiveIntegerType,
    #[serde(rename = "Measurer")]
    pub measurer: String50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "SelectionType")]
    pub selection_type: String10Type,
    #[serde(rename = "Temperature")]
    pub temperature: i32,
    #[serde(rename = "ProductKey")]
    pub product_key: ERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: PositiveInteger5digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: PositiveInteger5digitsType,
    #[serde(rename = "LogCount")]
    pub log_count: PositiveInteger2digitsType,
    #[serde(rename = "ControlLogCount")]
    pub control_log_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManualType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "InfoText")]
    pub info_text: String200Type,
    #[serde(rename = "Measurements")]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementsType {
    #[serde(rename = "Measurement")]
    pub measurement: Vec<MeasurementDataType>,
}

