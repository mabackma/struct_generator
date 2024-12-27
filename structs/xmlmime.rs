#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(flatten)]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(flatten)]
    pub base: Vec<u8>,
}

