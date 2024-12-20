#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseList {
    #[serde(flatten)]
    pub fee_base_list: FeeBaseListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListItem {
    #[serde(flatten)]
    pub fee_base_list_item: FeebaseListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(flatten)]
    pub id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: BdtPositiveIntegerType,
}

