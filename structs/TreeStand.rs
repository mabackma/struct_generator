use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesData {
    #[serde(flatten)]
    pub tree_species_data: TreeSpeciesDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingBeginningCode {
    #[serde(flatten)]
    pub seedling_beginning_code: CoSeedlingOriginType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalVolume {
    #[serde(flatten)]
    pub total_volume: CoVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalPulpwoodVolume {
    #[serde(flatten)]
    pub total_pulpwood_volume: CoPulpWoodVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalSawlogVolume {
    #[serde(flatten)]
    pub total_sawlog_volume: CoSawLogVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: CoSawLogPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpMeanDiameter {
    #[serde(flatten)]
    pub stump_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: CoAmountUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesSummary {
    #[serde(flatten)]
    pub tree_species_summary: TreeSpeciesSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandDataDate {
    #[serde(flatten)]
    pub tree_stand_data_date: TreeStandDataDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpStemCount {
    #[serde(flatten)]
    pub stump_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountInSpot {
    #[serde(flatten)]
    pub amount_in_spot: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountOutsideSpot {
    #[serde(flatten)]
    pub amount_outside_spot: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataMomentType {
    #[serde(flatten)]
    pub base: CoTreeStandDataMomentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummaryType {
    #[serde(rename = "TreeSpeciesData")]
    pub tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingDataType {
    #[serde(rename = "SeedlingStratum")]
    pub seedling_stratum: Vec<SeedlingStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: NotEmptyTreeSpeciesType,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "SawlogPercent", skip_serializing_if = "Option::is_none")]
    pub sawlog_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "TotalSawlogVolume", skip_serializing_if = "Option::is_none")]
    pub total_sawlog_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "TotalPulpwoodVolume", skip_serializing_if = "Option::is_none")]
    pub total_pulpwood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TotalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<CoVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReductionType {
    #[serde(rename = "StumpStemCount", skip_serializing_if = "Option::is_none")]
    pub stump_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StumpMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stump_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotEmptyTreeSpeciesType {
    #[serde(rename = "TreeSpeciesType")]
    pub co_tree_species_type: CoTreeSpeciesType,
    #[serde(rename = "ExtraTreeSpeciesType")]
    pub co_extra_tree_species_type: CoExtraTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TstTreeStrata>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DtsDeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TssTreeStandSummary>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<SdStemDistribution>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<SdsStemDistributionStrata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate2Type {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<TreeStrata2Type>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DtsDeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TreeStandSummary2Type>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub sd_trees: Option<SdTrees>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<SdStemDistribution>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<SdsStemDistributionStrata>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupsType {
    #[serde(rename = "SpareTreeGroup")]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratumType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "SeedlingBeginningCode", skip_serializing_if = "Option::is_none")]
    pub seedling_beginning_code: Option<CoSeedlingOriginType>,
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountInSpot", skip_serializing_if = "Option::is_none")]
    pub amount_in_spot: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountOutsideSpot", skip_serializing_if = "Option::is_none")]
    pub amount_outside_spot: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountUnit", skip_serializing_if = "Option::is_none")]
    pub amount_unit: Option<CoAmountUnitType>,
}

