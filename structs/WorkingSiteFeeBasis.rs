use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    #[serde(flatten)]
    pub id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBaseList {
    #[serde(flatten)]
    pub fee_base_list: FeeBaseListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBaseListItem {
    #[serde(flatten)]
    pub fee_base_list_item: FeebaseListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "StandNumber")]
    pub stand_number: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListType {
    #[serde(rename = "FeeBaseListItem")]
    pub fee_base_list_item: Vec<FeebaseListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebaseListItemType {
    #[serde(rename = "Id")]
    pub id: PositiveIntegerType,
    #[serde(rename = "FeeValue")]
    pub fee_value: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasisType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "FeeBasis")]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: String10Type,
    #[serde(rename = "FeeListId", skip_serializing_if = "Option::is_none")]
    pub fee_list_id: Option<PositiveIntegerType>,
    #[serde(rename = "FeeYesNo", skip_serializing_if = "Option::is_none")]
    pub fee_yes_no: Option<YesNoType>,
    #[serde(rename = "FeeValue", skip_serializing_if = "Option::is_none")]
    pub fee_value: Option<String10Type>,
    #[serde(rename = "FeeAssortment", skip_serializing_if = "Option::is_none")]
    pub fee_assortment: Option<String50Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "NeedToCheck", skip_serializing_if = "Option::is_none")]
    pub need_to_check: Option<YesNoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "FeeBaseList", skip_serializing_if = "Option::is_none")]
    pub fee_base_list: Option<FeeBaseListType>,
}

