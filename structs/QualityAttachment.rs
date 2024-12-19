#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachmentType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<BdtString50Type>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: BdtDateType,
    #[serde(rename = "Version")]
    pub version: BdtString10Type,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString1000Type>,
    #[serde(rename = "Filename")]
    pub filename: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
}

