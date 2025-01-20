use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: StandInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: AreaDecreaseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureRestrictionEnds {
    #[serde(flatten)]
    pub silviculture_restriction_ends: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StandBasicDataDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    pub base: String,
}

