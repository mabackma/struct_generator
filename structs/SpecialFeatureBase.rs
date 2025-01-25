use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: CoFeatureAdditionalCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: FeatureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: EndDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: StartDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: BufferDistanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: FeatureAdditionalInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: CoString1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: CoFeatureTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: ValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: CoFeatureCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierTypeType {
    #[serde(rename = "IdentifierTypeType")]
    pub co_identifier_type_type: CoIdentifierTypeType,
    #[serde(rename = "SpecialFeatureIdentifierExtensionType")]
    pub co_special_feature_identifier_extension_type: CoSpecialFeatureIdentifierExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionsType {
    #[serde(rename = "UsingRestriction")]
    pub using_restriction: Vec<UsingRestrictionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "Explanation")]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: CoDateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionType {
    #[serde(rename = "RestrictionType")]
    pub co_restriction_type: CoRestrictionType,
    #[serde(rename = "RestrictionCode")]
    pub co_restriction_code: CoRestrictionCode,
    #[serde(rename = "RestrictionStartDate", skip_serializing_if = "Option::is_none")]
    pub restriction_start_date: Option<StartDateType>,
    #[serde(rename = "RestrictionEndDate", skip_serializing_if = "Option::is_none")]
    pub restriction_end_date: Option<EndDateType>,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "RestrictionDescription", skip_serializing_if = "Option::is_none")]
    pub restriction_description: Option<CoString1500Type>,
    #[serde(rename = "RestrictionOutOfObject", skip_serializing_if = "Option::is_none")]
    pub restriction_out_of_object: Option<CoYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    #[serde(flatten)]
    pub base: CoIdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistanceType {
    #[serde(flatten)]
    pub base: CoDecimal4And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: CoDateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationType {
    pub base: String,
}

