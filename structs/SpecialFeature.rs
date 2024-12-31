use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: YesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: Decimal7And2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: YesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: FeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<FeatureDataGroup>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<DateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "GdtSimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: SimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(flatten)]
    pub base: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<YesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: FeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<DateType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "GdtAlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: AlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<YesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: FeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<DateType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureDataGroup {
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightType {
    #[serde(rename = "UsingRightExists")]
    pub using_right_exists: YesNoNotKnownType,
    #[serde(rename = "UsingRightExaminedDate")]
    pub using_right_examined_date: DateType,
    #[serde(rename = "UsingRightCompensation")]
    pub using_right_compensation: YesNoNotKnownType,
    #[serde(rename = "UsingRightCompensationAmount", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_amount: Option<Decimal7And2Type>,
    #[serde(rename = "UsingRightCompensationDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_description: Option<String1500Type>,
    #[serde(rename = "UsingRightCompensationResponsible")]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
    #[serde(rename = "UsingRightDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_description: Option<String1500Type>,
}

