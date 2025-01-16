use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TreeStrataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: CddDistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StratumOrigin {
    #[serde(flatten)]
    pub stratum_origin: CoSeedlingOriginType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStratum {
    #[serde(flatten)]
    pub tree_stratum: TreeStratum2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingOriginType {
    #[serde(flatten)]
    pub base: CoSeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(flatten)]
    pub base: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: CoSawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrataType {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: CoStratumNumberType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "Age")]
    pub age: AgeType,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: CoMeanHeightType,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<CoSeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: CoVolumeGrowthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: CoStemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: CoStumpBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(flatten)]
    pub base: CoStoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoMeanHeightType>,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<CoSeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(flatten)]
    pub base: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: CoSawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: CoPulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "TreeListItem", skip_serializing_if = "Option::is_none")]
    pub tree_list_item: Option<Vec<TliTreeListItemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: CoBranchBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: CoLeafBiomassType,
}

