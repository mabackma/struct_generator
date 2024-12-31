use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWithPublicityType {
    #[serde(flatten)]
    pub call_for_offer_with_publicity_type: CallForOfferWithPublicity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicityOrganizations {
    #[serde(flatten)]
    pub publicity_organizations: OrganizationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicity {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: Vec<CallForOffer>,
    #[serde(rename = "Publicity")]
    pub publicity: PublicityType,
    #[serde(rename = "PublicityOrganizations", skip_serializing_if = "Option::is_none")]
    pub publicity_organizations: Option<OrganizationsType>,
}

