#[derive(Debug, Serialize, Deserialize)]
pub struct StandLocatedOnIsland {
    #[serde(flatten)]
    pub stand_located_on_island: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeed {
    #[serde(flatten)]
    pub cutting_area_preclearing_need: CuttingAreaPreclearingNeedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: HarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: HarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: DitchingYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: SubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: BasicFeature2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifier {
    #[serde(flatten)]
    pub stout_timber_classifier: StoutTimberClassifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfo {
    #[serde(flatten)]
    pub stand_wood_trade_info: StandWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfo {
    #[serde(flatten)]
    pub stand_silviculture_info: StandSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: DrainageStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: MainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedStemCount {
    #[serde(flatten)]
    pub planned_stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: StandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: SilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

