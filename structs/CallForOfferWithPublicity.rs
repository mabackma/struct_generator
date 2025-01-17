use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWithPublicity {
    #[serde(flatten)]
    pub call_for_offer_with_publicity: CallForOfferWithPublicityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicityOrganizations {
    #[serde(flatten)]
    pub publicity_organizations: OOrganizationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicity {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: Vec<CfoCallForOffer>,
    #[serde(rename = "Publicity")]
    pub publicity: PublicityType,
    #[serde(rename = "PublicityOrganizations", skip_serializing_if = "Option::is_none")]
    pub publicity_organizations: Option<OrganizationsType>,
}

