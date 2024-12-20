#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManualType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "InfoText")]
    pub info_text: BdtString200Type,
    #[serde(rename = "Measurements")]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDataType {
    #[serde(rename = "MeasurementId")]
    pub measurement_id: BdtPositiveIntegerType,
    #[serde(rename = "Measurer")]
    pub measurer: BdtString50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "SelectionType")]
    pub selection_type: BdtString10Type,
    #[serde(rename = "Temperature")]
    pub temperature: Xsinteger,
    #[serde(rename = "ProductKey")]
    pub product_key: WctERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: BdtPositiveInteger5digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: BdtPositiveInteger5digitsType,
    #[serde(rename = "LogCount")]
    pub log_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "ControlLogCount")]
    pub control_log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementsType {
    #[serde(rename = "Measurement")]
    pub measurement: Vec<MeasurementDataType>,
}

