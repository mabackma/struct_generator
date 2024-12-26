#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(flatten)]
    pub map_symbol_type: BdtFeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photographer {
    #[serde(flatten)]
    pub photographer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: ImageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(flatten)]
    pub position: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(flatten)]
    pub category: BdtImageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDate {
    #[serde(flatten)]
    pub image_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubCategory {
    #[serde(flatten)]
    pub sub_category: BdtImageSubCategoryType,
}

