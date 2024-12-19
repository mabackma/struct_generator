#[derive(Debug, Serialize, Deserialize)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListItem {
    #[serde(flatten)]
    pub fee_base_list_item: FeebaseListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseList {
    #[serde(flatten)]
    pub fee_base_list: FeeBaseListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: YesNoType,
}

