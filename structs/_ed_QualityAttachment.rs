#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachment {
    #[serde(flatten)]
    pub quality_attachment: QualityAttachmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(flatten)]
    pub version: BdtString10Type,
}

