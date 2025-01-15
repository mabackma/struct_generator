use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetExtraInfo {
    #[serde(flatten)]
    pub target_extra_info: VirtaExtraInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status2 {
    #[serde(flatten)]
    pub status2: CoChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetNumber {
    #[serde(flatten)]
    pub target_number: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstablishedPartNumber {
    #[serde(flatten)]
    pub established_part_number: VirtaPartNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetParts {
    #[serde(flatten)]
    pub target_parts: TargetPartsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetAnnouncedAmount {
    #[serde(flatten)]
    pub target_announced_amount: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetId {
    #[serde(flatten)]
    pub target_id: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasePartNumber {
    #[serde(flatten)]
    pub base_part_number: VirtaPartNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatAdvertisement {
    #[serde(flatten)]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatAdvertisementType {
    #[serde(flatten)]
    pub base: CoVirtaHabitatAdvertisementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetType {
    #[serde(rename = "Status2")]
    pub status2: CoChangeStateType,
    #[serde(rename = "TargetId")]
    pub target_id: String,
    #[serde(rename = "TargetNumber")]
    pub target_number: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
    #[serde(rename = "BasePartNumber")]
    pub base_part_number: VirtaPartNumberType,
    #[serde(rename = "EstablishedPartNumber")]
    pub established_part_number: VirtaPartNumberType,
    #[serde(rename = "TargetAnnouncedAmount")]
    pub target_announced_amount: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "HabitatAdvertisement")]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
    #[serde(rename = "TargetExtraInfo")]
    pub target_extra_info: VirtaExtraInfoType,
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<Polygon>,
    #[serde(rename = "TargetParts", skip_serializing_if = "Option::is_none")]
    pub target_parts: Option<TargetPartsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartsType {
    #[serde(rename = "TargetPart")]
    pub tp_target_part: Vec<TargetPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPartNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExtraInfoType {
    pub base: String,
}

