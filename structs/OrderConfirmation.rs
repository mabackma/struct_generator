#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<String20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "OrderStatus")]
    pub order_status: OrderStatusType,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
}

