use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: CoString1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: CoUsingRightResponsibleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: CoYesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: CoDecimal7And2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: CoString1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: CoForestDepotAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: CoYesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

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
pub struct FeatureDataGroup {
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<SfUsingRestrictions>,
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
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<CoDataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<FeatureDataGroup>,
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
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: GdtAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "AlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: GdtAlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: GdtSimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<SfUsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<CoDataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(flatten)]
    pub base: SfIdentifiersType,
}

