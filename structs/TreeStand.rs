#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReductionType {
    #[serde(rename = "StumpStemCount", skip_serializing_if = "Option::is_none")]
    pub stump_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StumpMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stump_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: CoDateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: CoInventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TstTreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TreeStrata>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TssTreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TreeStandSummary>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingDataType {
    #[serde(rename = "SeedlingStratum")]
    pub seedling_stratum: Vec<SeedlingStratumType>,
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
pub struct TreeStandDataMomentType {
    #[serde(flatten)]
    pub base: CoTreeStandDataMomentType,
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
pub struct AlternativeIdentifierType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummaryType {
    #[serde(rename = "TreeSpeciesData")]
    pub tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupsType {
    #[serde(rename = "SpareTreeGroup")]
    pub spare_tree_group: SpareTreeGroupType,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate2Type {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: CoDateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: CoInventoryMethodType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<TstTreeStrata2Type>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TssTreeStandSummary2Type>,
    #[serde(rename = "SdTrees", skip_serializing_if = "Option::is_none")]
    pub sd_trees: Option<Trees>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

