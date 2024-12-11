#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(flatten)]
    pub header: HeaderType,
}

