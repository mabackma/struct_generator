#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: MeanHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: MeanAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary {
    #[serde(flatten)]
    pub tree_stand_summary: TreeStandSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanStumpDiameter {
    #[serde(flatten)]
    pub mean_stump_diameter: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: DevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: TreeSpeciesType,
}

