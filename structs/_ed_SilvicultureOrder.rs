#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(flatten)]
    pub data: XshexBinary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrder {
    #[serde(flatten)]
    pub silviculture_order: SilvicultureOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(flatten)]
    pub attachments: AttachmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginDate {
    #[serde(flatten)]
    pub begin_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(flatten)]
    pub attachment: AttachmentDataType,
}

