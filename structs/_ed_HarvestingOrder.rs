#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrder {
    #[serde(flatten)]
    pub harvesting_order: HarvestingOrderType,
}

