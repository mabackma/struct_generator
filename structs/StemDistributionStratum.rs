use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDistributionStrata {
    #[serde(flatten)]
    pub stem_distribution_strata: StemDistributionStrataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDistributionStratum {
    #[serde(flatten)]
    pub stem_distribution_stratum: StemDistributionStratumType,
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
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrataType {
    #[serde(rename = "StemDistributionStratum")]
    pub stem_distribution_stratum: Vec<StemDistributionStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "DistributionModelGroup")]
    pub cdd_distribution_model_group: CddDistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

