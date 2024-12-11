#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPulpwoodVolume {
    #[serde(flatten)]
    pub total_pulpwood_volume: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpStemCount {
    #[serde(flatten)]
    pub stump_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesData {
    #[serde(flatten)]
    pub tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: NotEmptyTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalSawlogVolume {
    #[serde(flatten)]
    pub total_sawlog_volume: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: AgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVolume {
    #[serde(flatten)]
    pub total_volume: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOutsideSpot {
    #[serde(flatten)]
    pub amount_outside_spot: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TreeStrata2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: AmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountInSpot {
    #[serde(flatten)]
    pub amount_in_spot: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: TreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate {
    #[serde(flatten)]
    pub tree_stand_data_date: TreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingBeginningCode {
    #[serde(flatten)]
    pub seedling_beginning_code: SeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpMeanDiameter {
    #[serde(flatten)]
    pub stump_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummary {
    #[serde(flatten)]
    pub tree_species_summary: TreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary {
    #[serde(flatten)]
    pub tree_stand_summary: TreeStandSummary2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: PositiveInteger6digitsType,
}

