#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginDate {
    #[serde(flatten)]
    pub begin_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(flatten)]
    pub attachments: AttachmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(flatten)]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(flatten)]
    pub attachment: AttachmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrder {
    #[serde(flatten)]
    pub silviculture_order: SilvicultureOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: BdtString20Type,
}

