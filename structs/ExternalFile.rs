#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "ProductUserId", skip_serializing_if = "Option::is_none")]
    pub product_user_id: Option<BdtString50Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "FileFormat")]
    pub file_format: BdtString5Type,
    #[serde(rename = "Label")]
    pub label: BdtString100Type,
    #[serde(rename = "DocumentClass", skip_serializing_if = "Option::is_none")]
    pub document_class: Option<BdtDocumentClassType>,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

