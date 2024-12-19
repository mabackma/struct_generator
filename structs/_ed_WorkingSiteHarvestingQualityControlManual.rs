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
pub struct ControlLogCount {
    #[serde(flatten)]
    pub control_log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    #[serde(flatten)]
    pub measurement: MeasurementDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(flatten)]
    pub temperature: Xsinteger,
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

