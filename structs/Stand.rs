#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<BasicFeature1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<String>,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType1 {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@realEstateId")]
    pub real_estate_id: String,
    #[serde(rename = "@parcelId")]
    pub parcel_id: String,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataWithGeometryType,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<String>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "PlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub planned_operation_chains: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

