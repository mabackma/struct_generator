#[derive(Debug, Serialize, Deserialize)]
pub struct locator {
    #[serde(flatten)]
    pub locator: locatorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arc {
    #[serde(flatten)]
    pub arc: arcType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resource {
    #[serde(flatten)]
    pub resource: resourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct title {
    #[serde(flatten)]
    pub title: titleEltType,
}

