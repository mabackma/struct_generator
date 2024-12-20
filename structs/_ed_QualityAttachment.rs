#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachment {
    #[serde(flatten)]
    pub quality_attachment: QualityAttachmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(flatten)]
    pub version: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Xsbase64Binary,
}

