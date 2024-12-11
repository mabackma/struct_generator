#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(flatten)]
    pub base: StoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: StumpBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingOriginType {
    #[serde(flatten)]
    pub base: SeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BranchBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrataType {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: StemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "TreeListItem", skip_serializing_if = "Option::is_none")]
    pub tree_list_item: Option<Vec<TreeListItemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: VolumeGrowthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(flatten)]
    pub base: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(flatten)]
    pub base: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: LeafBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratumType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: StratumNumberType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "Age")]
    pub age: AgeType,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: MeanHeightType,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<SeedlingOriginType>,
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
    pub data_source: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<Decimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(flatten)]
    pub base: AgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum2Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<StratumNumberType>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<MeanHeightType>,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<SeedlingOriginType>,
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
    pub data_source: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<Decimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

