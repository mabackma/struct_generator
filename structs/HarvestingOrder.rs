#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDataType {
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsType {
    #[serde(rename = "Attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerArea")]
    pub service_buyer_area: BdtString20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<BdtString50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: BdtString20Type,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<WctERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<BdtString50Type>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<BdtString50Type>,
    #[serde(rename = "CodeGroup")]
    pub code_group: BdtAssortmentGroupType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
    #[serde(rename = "BeginDate")]
    pub begin_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
}

