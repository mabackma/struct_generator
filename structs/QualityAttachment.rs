use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(flatten)]
    pub version: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityAttachment {
    #[serde(flatten)]
    pub quality_attachment: QualityAttachmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachmentType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<String50Type>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: DateType,
    #[serde(rename = "Version")]
    pub version: String10Type,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

