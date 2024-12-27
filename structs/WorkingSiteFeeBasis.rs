#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "StandNumber")]
    pub stand_number: Vec<BdtString20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListType {
    #[serde(rename = "FeeBaseListItem")]
    pub fee_base_list_item: Vec<FeebaseListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebaseListItemType {
    #[serde(rename = "Id")]
    pub id: BdtPositiveIntegerType,
    #[serde(rename = "FeeValue")]
    pub fee_value: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasisType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "FeeBasis")]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<BdtWorkCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: BdtString10Type,
    #[serde(rename = "FeeListId", skip_serializing_if = "Option::is_none")]
    pub fee_list_id: Option<BdtPositiveIntegerType>,
    #[serde(rename = "FeeYesNo", skip_serializing_if = "Option::is_none")]
    pub fee_yes_no: Option<BdtYesNoType>,
    #[serde(rename = "FeeValue", skip_serializing_if = "Option::is_none")]
    pub fee_value: Option<BdtString10Type>,
    #[serde(rename = "FeeAssortment", skip_serializing_if = "Option::is_none")]
    pub fee_assortment: Option<BdtString50Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString1000Type>,
    #[serde(rename = "NeedToCheck", skip_serializing_if = "Option::is_none")]
    pub need_to_check: Option<BdtYesNoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "FeeBaseList", skip_serializing_if = "Option::is_none")]
    pub fee_base_list: Option<FeeBaseListType>,
}

