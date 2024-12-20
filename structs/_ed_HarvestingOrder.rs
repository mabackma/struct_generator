#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrder {
    #[serde(flatten)]
    pub harvesting_order: HarvestingOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: BdtAssortmentGroupType,
}

