#[derive(Debug, Serialize, Deserialize)]
pub struct Hopper {
    #[serde(flatten)]
    pub hopper: HopperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperNumber {
    #[serde(flatten)]
    pub hopper_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(flatten)]
    pub hopper_type: HopperTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageVolume {
    #[serde(flatten)]
    pub average_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PilotName {
    #[serde(flatten)]
    pub pilot_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertileType {
    #[serde(flatten)]
    pub fertile_type: MaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperLocationFromGPS {
    #[serde(flatten)]
    pub hopper_location_from_g_p_s: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerName {
    #[serde(flatten)]
    pub measurer_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanVolume {
    #[serde(flatten)]
    pub mean_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerId {
    #[serde(flatten)]
    pub measurer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hoppers {
    #[serde(flatten)]
    pub hoppers: HoppersType,
}

