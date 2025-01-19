use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Count {
    #[serde(flatten)]
    pub count: CountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: StemDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: CoSawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: CoPulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(rename = "HeightType")]
    pub co_height_type: CoHeightType,
    #[serde(rename = "EmptyStringType")]
    pub co_empty_string_type: CoEmptyStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(flatten)]
    pub base: CoStoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(flatten)]
    pub base: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionType {
    #[serde(rename = "Tree")]
    pub tree: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: CoSawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "TreeNumber")]
    pub tree_number: TreeNumberType,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: CoTreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "Count")]
    pub count: CountType,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<DiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<HeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClassType {
    #[serde(flatten)]
    pub base: CoTreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

