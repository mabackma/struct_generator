#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceMass {
    #[serde(flatten)]
    pub control_reference_mass: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orientation {
    #[serde(flatten)]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationDate {
    #[serde(flatten)]
    pub calibration_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleData {
    #[serde(flatten)]
    pub scale_data: ScaleDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationAdjustment {
    #[serde(flatten)]
    pub calibration_adjustment: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calibration {
    #[serde(flatten)]
    pub calibration: CalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaledMass {
    #[serde(flatten)]
    pub scaled_mass: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: String100Type,
}

