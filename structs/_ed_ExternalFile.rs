#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(flatten)]
    pub label: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Xsbase64Binary,
}

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

