#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: BdtDecimal1FractionDigitType,
    #[serde(rename = "Orientation")]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString100Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: BdtString20Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "ControlReferenceMass")]
    pub control_reference_mass: BdtDecimal1FractionDigitType,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "ScaleData")]
    pub scale_data: Vec<ScaleDataType>,
    #[serde(rename = "Calibration", skip_serializing_if = "Option::is_none")]
    pub calibration: Option<Vec<CalibrationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientationType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationType {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: BdtTimeStampType,
    #[serde(rename = "CalibrationAdjustment")]
    pub calibration_adjustment: BdtPositiveInteger3digitsType,
}

