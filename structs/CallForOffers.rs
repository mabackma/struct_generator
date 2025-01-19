use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOffers {
    #[serde(flatten)]
    pub call_for_offers: CallForOffersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: Vec<CfoCallForOffer>,
}

