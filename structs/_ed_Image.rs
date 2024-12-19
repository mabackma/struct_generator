#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(flatten)]
    pub position: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubCategory {
    #[serde(flatten)]
    pub sub_category: ImageSubCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photographer {
    #[serde(flatten)]
    pub photographer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(flatten)]
    pub category: ImageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(flatten)]
    pub map_symbol_type: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDate {
    #[serde(flatten)]
    pub image_date: TimeStampType,
}

