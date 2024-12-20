#[derive(Debug, Serialize, Deserialize)]
pub struct locator {
    #[serde(flatten)]
    pub locator: XlinklocatorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arc {
    #[serde(flatten)]
    pub arc: XlinkarcType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct title {
    #[serde(flatten)]
    pub title: XlinktitleEltType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resource {
    #[serde(flatten)]
    pub resource: XlinkresourceType,
}

