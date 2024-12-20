#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SfBasicFeature1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType1 {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@realEstateId")]
    pub real_estate_id: Xsstring,
    #[serde(rename = "@parcelId")]
    pub parcel_id: Xsstring,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataWithGeometryType,
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<String>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "OpPlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub op_planned_operation_chains: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<String>,
    #[serde(rename = "GdtPolygonGeometry")]
    pub gdt_polygon_geometry: String,
    #[serde(rename = "GdtMultiPolygonGeometry")]
    pub gdt_multi_polygon_geometry: String,
}

