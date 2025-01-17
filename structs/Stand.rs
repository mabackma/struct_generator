use serde::{Serialize, Deserialize};
use chrono::*;

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
    pub ts_tree_stand_data: Option<TsTreeStandData>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
    #[serde(rename = "PlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub op_planned_operation_chains: Option<OpPlannedOperationChains>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaDecreaseType>,
    #[serde(rename = "PolygonGeometry")]
    pub gdt_polygon_geometry: GdtPolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub gdt_multi_polygon_geometry: GdtMultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SfBasicFeature1Type>,
}

