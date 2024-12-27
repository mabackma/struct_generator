#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFile {
    #[serde(flatten)]
    pub stanford_file: StanfordFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: BdtString100Type,
}

