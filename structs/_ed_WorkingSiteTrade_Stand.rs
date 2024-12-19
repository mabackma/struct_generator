#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeed {
    #[serde(flatten)]
    pub cutting_area_preclearing_need: CuttingAreaPreclearingNeedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfo {
    #[serde(flatten)]
    pub stand_silviculture_info: StandSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: DitchingYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfo {
    #[serde(flatten)]
    pub stand_wood_trade_info: StandWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifier {
    #[serde(flatten)]
    pub stout_timber_classifier: StoutTimberClassifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedStemCount {
    #[serde(flatten)]
    pub planned_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandLocatedOnIsland {
    #[serde(flatten)]
    pub stand_located_on_island: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: CoSilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: CoSubGroupType,
}

