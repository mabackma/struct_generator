#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    #[serde(flatten)]
    pub base: string,
}

