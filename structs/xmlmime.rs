#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: XshexBinary,
}

