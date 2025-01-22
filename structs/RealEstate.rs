use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: BaseRealEstatesType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: RealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateOwner {
    #[serde(flatten)]
    pub real_estate_owner: CiContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateOwners {
    #[serde(flatten)]
    pub real_estate_owners: RealEstateOwnersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseRealEstates {
    #[serde(flatten)]
    pub base_real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType2 {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwnersType {
    #[serde(rename = "RealEstateOwner")]
    pub real_estate_owner: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType,
    #[serde(rename = "RealEstateOwners", skip_serializing_if = "Option::is_none")]
    pub real_estate_owners: Option<RealEstateOwnersType>,
    #[serde(rename = "Parcels", skip_serializing_if = "Option::is_none")]
    pub parcels: Option<Parcels>,
}

