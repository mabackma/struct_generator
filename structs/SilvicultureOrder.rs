#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsType {
    #[serde(rename = "Attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDataType {
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeInfoType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Amount")]
    pub amount: BdtDecimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<BdtString20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<BdtString50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: BdtString20Type,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "BeginDate")]
    pub begin_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

