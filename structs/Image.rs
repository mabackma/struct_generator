#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringImageType {
    #[serde(flatten)]
    pub base: ImageBaseType,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<BdtServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<BdtImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<BdtImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<BdtFeatureCodeType>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<GdtPointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<BdtString50Type>,
    #[serde(rename = "ImageDate")]
    pub image_date: BdtTimeStampType,
    #[serde(rename = "Filename")]
    pub filename: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<BdtServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<BdtImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<BdtImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<BdtFeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<WctERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<BdtString20Type>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<GdtPointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<BdtString50Type>,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "ImageDate", skip_serializing_if = "Option::is_none")]
    pub image_date: Option<BdtTimeStampType>,
    #[serde(rename = "Filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<BdtString100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<base64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageType {
    #[serde(flatten)]
    pub base: ImageBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "Category")]
    pub category: BdtImageCategoryType,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<BdtImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<BdtFeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<WctERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<BdtString20Type>,
    #[serde(rename = "Position")]
    pub position: GdtPointGeometryType,
    #[serde(rename = "InfoText")]
    pub info_text: BdtString200Type,
    #[serde(rename = "Photographer")]
    pub photographer: BdtString50Type,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "ImageDate")]
    pub image_date: BdtTimeStampType,
    #[serde(rename = "Filename")]
    pub filename: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

