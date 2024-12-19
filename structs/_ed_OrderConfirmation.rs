#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_status: OrderStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmation {
    #[serde(flatten)]
    pub order_confirmation: OrderConfirmationType,
}

