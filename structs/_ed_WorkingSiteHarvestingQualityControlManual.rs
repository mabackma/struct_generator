#[derive(Debug, Serialize, Deserialize)]
pub struct LogVolume {
    #[serde(flatten)]
    pub log_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogCount {
    #[serde(flatten)]
    pub control_log_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementId {
    #[serde(flatten)]
    pub measurement_id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManual {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    #[serde(flatten)]
    pub measurement: MeasurementDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogLength {
    #[serde(flatten)]
    pub control_log_length: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogCount {
    #[serde(flatten)]
    pub log_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurements {
    #[serde(flatten)]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogLength {
    #[serde(flatten)]
    pub log_length: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(flatten)]
    pub temperature: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectionType {
    #[serde(flatten)]
    pub selection_type: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductKey {
    #[serde(flatten)]
    pub product_key: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogVolume {
    #[serde(flatten)]
    pub control_log_volume: Decimal3FractionDigitsType,
}

