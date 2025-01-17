use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
--pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Calibration {
    #[serde(flatten)]
    pub calibration: CalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Orientation {
    #[serde(flatten)]
    pub orientation: OrientationType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CalibrationAdjustment {
    #[serde(flatten)]
    pub calibration_adjustment: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CalibrationDate {
    #[serde(flatten)]
    pub calibration_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ScaleData {
    #[serde(flatten)]
    pub scale_data: ScaleDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ScaledMass {
    #[serde(flatten)]
    pub scaled_mass: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ControlReferenceMass {
    #[serde(flatten)]
    pub control_reference_mass: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationType {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: TimeStampType,
    #[serde(rename = "CalibrationAdjustment")]
    pub calibration_adjustment: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: Decimal1FractionDigitType,
    #[serde(rename = "Orientation")]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: String20Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "ControlReferenceMass")]
    pub control_reference_mass: Decimal1FractionDigitType,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "ScaleData")]
    pub scale_data: Vec<ScaleDataType>,
    #[serde(rename = "Calibration", skip_serializing_if = "Option::is_none")]
    pub calibration: Option<Vec<CalibrationType>>,
}

