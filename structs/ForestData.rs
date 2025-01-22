use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyData {
    #[serde(flatten)]
    pub forest_property_data: ForestPropertyDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<ReRealEstates>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<StStands>,
}

