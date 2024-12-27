#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmation {
    #[serde(flatten)]
    pub order_confirmation: OrderConfirmationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_status: BdtOrderStatusType,
}

