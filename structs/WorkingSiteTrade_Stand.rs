#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroupType {
    #[serde(flatten)]
    pub base: CoMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SfBasicFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassType {
    #[serde(flatten)]
    pub base: CoDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifierType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfoType {
    #[serde(rename = "CuttingAreaPreclearingNeed", skip_serializing_if = "Option::is_none")]
    pub cutting_area_preclearing_need: Option<CuttingAreaPreclearingNeedType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "StoutTimberClassifier", skip_serializing_if = "Option::is_none")]
    pub stout_timber_classifier: Option<StoutTimberClassifierType>,
    #[serde(rename = "LoggingAccessibility", skip_serializing_if = "Option::is_none")]
    pub logging_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<CoStemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeedType {
    #[serde(flatten)]
    pub base: YesNoSellerResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    #[serde(flatten)]
    pub base: StbStandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: StbAreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(flatten)]
    pub base: StbIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "MainGroup")]
    pub main_group: CoMainGroupType,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageStateType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<DitchingYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClassType>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<StandQualityType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<MainTreeSpeciesType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction: Option<CuttingRestrictionType>,
    #[serde(rename = "CuttingRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction_ends: Option<CoDateType>,
    #[serde(rename = "SilvicultureRestriction", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction: Option<CoSilvicultureRestrictionType>,
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<CoDateType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaDecreaseType>,
    #[serde(rename = "GdtPolygonGeometry")]
    pub gdt_polygon_geometry: String,
    #[serde(rename = "GdtMultiPolygonGeometry")]
    pub gdt_multi_polygon_geometry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionType {
    #[serde(flatten)]
    pub base: CoCuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(flatten)]
    pub base: StbStandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfoType {
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<CoStemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClassType {
    #[serde(flatten)]
    pub base: CoBearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerResponsible {
    #[serde(flatten)]
    pub base: CoSellerResponsible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroupType {
    #[serde(flatten)]
    pub base: CoSubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@realEstateid")]
    pub real_estateid: Xsstring,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataType,
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<String>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "StandWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub stand_wood_trade_info: Option<StandWoodTradeInfoType>,
    #[serde(rename = "StandSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub stand_silviculture_info: Option<StandSilvicultureInfoType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: StbAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoSellerResponsibleType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageStateType {
    #[serde(flatten)]
    pub base: CoDrainageStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClassType {
    #[serde(flatten)]
    pub base: CoFertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQualityType {
    #[serde(flatten)]
    pub base: CoStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    #[serde(flatten)]
    pub base: StbStandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTypeType {
    #[serde(flatten)]
    pub base: CoSoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: StbStandBasicDataDateType,
}

