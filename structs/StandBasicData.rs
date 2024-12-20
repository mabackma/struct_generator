#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataType {
    #[serde(rename = "SoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseCompactStandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<StbStandNumberType>,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data_date: Option<StbStandBasicDataDateType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataGroup {
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<MainGroup>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageState>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataType {
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataGroup {
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<StandQuality>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClass>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<MainTreeSpecies>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionGroup {
    #[serde(rename = "SilvicultureRestriction")]
    pub silviculture_restriction: SilvicultureRestriction,
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<SilvicultureRestrictionEnds>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionsType {
    #[serde(rename = "SilvicultureRestrictionDetails")]
    pub silviculture_restriction_details: Vec<SilvicultureRestrictionDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionsMainGroup {
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction: Option<CuttingRestriction>,
    #[serde(rename = "CuttingRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction_ends: Option<CuttingRestrictionEnds>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionMainGroup {
    #[serde(rename = "SilvicultureRestrictions", skip_serializing_if = "Option::is_none")]
    pub silviculture_restrictions: Option<SilvicultureRestrictions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactStandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StbStandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetailsType {
    #[serde(rename = "SilvicultureRestrictionGroup")]
    pub silviculture_restriction_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "SoilDataGroup")]
    pub soil_data_group: String,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: String,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StbStandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataGroup {
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageState>,
    #[serde(rename = "MainGroup")]
    pub main_group: MainGroup,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
}

