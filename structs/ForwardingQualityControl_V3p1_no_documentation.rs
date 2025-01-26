use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleCalibrationAdjustment {
    #[serde(flatten)]
    pub scale_calibration_adjustment: Xsdinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingQualityControlHeader {
    #[serde(flatten)]
    pub forwarding_quality_control_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleDefinition {
    #[serde(flatten)]
    pub scale_definition: ScaleDefinitionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingQualityControl {
    #[serde(flatten)]
    pub forwarding_quality_control: ForwardingQualityControlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleControlKey {
    #[serde(flatten)]
    pub scale_control_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleCalibrationDate {
    #[serde(flatten)]
    pub scale_calibration_date: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleCalibration {
    #[serde(flatten)]
    pub scale_calibration: ScaleCalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleControl {
    #[serde(flatten)]
    pub scale_control: ScaleControlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleControlDate {
    #[serde(flatten)]
    pub scale_control_date: StanForD2010DateTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleCalibrationType {
    #[serde(rename = "ScaleKey")]
    pub scale_key: u32,
    #[serde(rename = "ScaleControlKey")]
    pub scale_control_key: u32,
    #[serde(rename = "ScaleCalibrationDate")]
    pub scale_calibration_date: StanForD2010DateTimeType,
    #[serde(rename = "ScaleCalibrationAdjustment")]
    pub scale_calibration_adjustment: i32,
    #[serde(rename = "scaleWorkCategory")]
    pub scale_work_category: ScaleWorkCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientationType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleControlCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleWorkCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineForwardingQualityControlType {
    #[serde(flatten)]
    pub base: MachineType,
    #[serde(rename = "ObjectDefinition", skip_serializing_if = "Option::is_none")]
    pub object_definition: Option<Vec<ObjectDefinitionMachineType>>,
    #[serde(rename = "ScaleDefinition")]
    pub scale_definition: Vec<ScaleDefinitionType>,
    #[serde(rename = "DeliveryDefinition", skip_serializing_if = "Option::is_none")]
    pub delivery_definition: Option<Vec<DeliveryDefinitionWithDetailsType>>,
    #[serde(rename = "ScaleControl", skip_serializing_if = "Option::is_none")]
    pub scale_control: Option<Vec<ScaleControlType>>,
    #[serde(rename = "ScaleCalibration", skip_serializing_if = "Option::is_none")]
    pub scale_calibration: Option<Vec<ScaleCalibrationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingQualityControlType {
    #[serde(rename = "ForwardingQualityControlHeader")]
    pub forwarding_quality_control_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineForwardingQualityControlType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleControlType {
    #[serde(rename = "ScaleControlKey")]
    pub scale_control_key: u32,
    #[serde(rename = "ScaleKey")]
    pub scale_key: u32,
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "DeliveryKey", skip_serializing_if = "Option::is_none")]
    pub delivery_key: Option<u32>,
    #[serde(rename = "ObjectKey", skip_serializing_if = "Option::is_none")]
    pub object_key: Option<u32>,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "ScaleControlDate")]
    pub scale_control_date: StanForD2010DateTimeType,
    #[serde(rename = "ControlReferenceMass")]
    pub control_reference_mass: u32,
    #[serde(rename = "ScaleData")]
    pub scale_data: Vec<ScaleDataType>,
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<OrientationType>,
    #[serde(rename = "scaleWorkCategory")]
    pub scale_work_category: ScaleWorkCategoryType,
    #[serde(rename = "scaleControlCategory")]
    pub scale_control_category: ScaleControlCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: u32,
}

