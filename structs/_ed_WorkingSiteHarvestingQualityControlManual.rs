#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogCount {
    #[serde(flatten)]
    pub control_log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: BdtMeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductKey {
    #[serde(flatten)]
    pub product_key: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogLength {
    #[serde(flatten)]
    pub log_length: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(flatten)]
    pub temperature: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectionType {
    #[serde(flatten)]
    pub selection_type: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogVolume {
    #[serde(flatten)]
    pub log_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurements {
    #[serde(flatten)]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManual {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogVolume {
    #[serde(flatten)]
    pub control_log_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogLength {
    #[serde(flatten)]
    pub control_log_length: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementId {
    #[serde(flatten)]
    pub measurement_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogCount {
    #[serde(flatten)]
    pub log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    #[serde(flatten)]
    pub measurement: MeasurementDataType,
}

