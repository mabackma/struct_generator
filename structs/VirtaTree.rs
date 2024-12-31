use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeCount {
    #[serde(flatten)]
    pub tree_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageClass {
    #[serde(flatten)]
    pub damage_class: VirtaDamageClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubDiameter {
    #[serde(flatten)]
    pub stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status5 {
    #[serde(flatten)]
    pub status5: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkQuality {
    #[serde(flatten)]
    pub work_quality: VirtaWorkQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDiameter {
    #[serde(flatten)]
    pub tree_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeHeight {
    #[serde(flatten)]
    pub tree_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    #[serde(flatten)]
    pub base: VirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "Status5")]
    pub status5: ChangeStateType,
    #[serde(rename = "TreeNumber")]
    pub tree_number: String,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeDiameter", skip_serializing_if = "Option::is_none")]
    pub tree_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeHeight", skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "TreeCount", skip_serializing_if = "Option::is_none")]
    pub tree_count: Option<StemCountType>,
    #[serde(rename = "WorkQuality", skip_serializing_if = "Option::is_none")]
    pub work_quality: Option<VirtaWorkQualityType>,
    #[serde(rename = "DamageClass", skip_serializing_if = "Option::is_none")]
    pub damage_class: Option<VirtaDamageClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    #[serde(flatten)]
    pub base: VirtaWorkQualityType,
}

