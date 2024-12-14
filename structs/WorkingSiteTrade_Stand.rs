#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "MainGroup")]
    pub main_group: MainGroupType,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageStateType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<DitchingYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<YearType>,
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
    pub cutting_restriction_ends: Option<DateType>,
    #[serde(rename = "SilvicultureRestriction", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction: Option<SilvicultureRestrictionType>,
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<DateType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSourceType>,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaDecreaseType>,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometryType,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClassType {
    #[serde(flatten)]
    pub base: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: StandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoSellerResponsibleType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifierType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibilityType {
    #[serde(flatten)]
    pub base: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    #[serde(flatten)]
    pub base: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(flatten)]
    pub base: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYearType {
    #[serde(flatten)]
    pub base: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@realEstateid")]
    pub real_estateid: String,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataType,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<OperationsType>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "StandWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub stand_wood_trade_info: Option<StandWoodTradeInfoType>,
    #[serde(rename = "StandSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub stand_silviculture_info: Option<StandSilvicultureInfoType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionType {
    #[serde(flatten)]
    pub base: CuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeedType {
    #[serde(flatten)]
    pub base: YesNoSellerResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibilityType {
    #[serde(flatten)]
    pub base: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerResponsible {
    #[serde(flatten)]
    pub base: SellerResponsible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroupType {
    #[serde(flatten)]
    pub base: SubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClassType {
    #[serde(flatten)]
    pub base: BearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    #[serde(flatten)]
    pub base: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTypeType {
    #[serde(flatten)]
    pub base: SoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(flatten)]
    pub base: StandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<BasicFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: AreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroupType {
    #[serde(flatten)]
    pub base: MainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassType {
    #[serde(flatten)]
    pub base: DevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageStateType {
    #[serde(flatten)]
    pub base: DrainageStateType,
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
    pub logging_accessibility: Option<HarvestingAccessibilityType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<HarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<BasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<StemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityType {
    #[serde(flatten)]
    pub base: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceType {
    #[serde(flatten)]
    pub base: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpeciesType {
    #[serde(flatten)]
    pub base: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQualityType {
    #[serde(flatten)]
    pub base: StandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfoType {
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<HarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<BasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<StemCountType>,
}
