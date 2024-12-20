#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightType {
    #[serde(rename = "UsingRightExists")]
    pub using_right_exists: CoYesNoNotKnownType,
    #[serde(rename = "UsingRightExaminedDate")]
    pub using_right_examined_date: CoDateType,
    #[serde(rename = "UsingRightCompensation")]
    pub using_right_compensation: CoYesNoNotKnownType,
    #[serde(rename = "UsingRightCompensationAmount", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_amount: Option<CoDecimal7And2Type>,
    #[serde(rename = "UsingRightCompensationDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_description: Option<CoString1500Type>,
    #[serde(rename = "UsingRightCompensationResponsible")]
    pub using_right_compensation_responsible: CoUsingRightResponsibleType,
    #[serde(rename = "UsingRightDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(flatten)]
    pub base: SfIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "GdtSimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "GdtAlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureDataGroup {
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: CoForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<CoDateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TsTreeStandDataType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
}

