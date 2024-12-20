#[derive(Debug, Serialize, Deserialize)]
pub struct FileFormat {
    #[serde(flatten)]
    pub file_format: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFile {
    #[serde(flatten)]
    pub external_file: ExternalFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(flatten)]
    pub label: BdtString100Type,
}

