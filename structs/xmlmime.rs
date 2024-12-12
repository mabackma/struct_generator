#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@contentType")]
    pub content_type: String,
    #[serde(flatten)]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@contentType")]
    pub content_type: String,
    #[serde(flatten)]
    pub base: Vec<u8>,
}

