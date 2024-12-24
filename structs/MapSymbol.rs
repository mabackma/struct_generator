#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "MapSymbols")]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolDataType {
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<WctERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<BdtString20Type>,
    #[serde(rename = "MapSymbolName", skip_serializing_if = "Option::is_none")]
    pub map_symbol_name: Option<BdtString20Type>,
    #[serde(rename = "FeatureType", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<BdtFeatureTypeType>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: BdtFeatureCodeType,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<BdtFeatureAdditionalCodeType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<BdtString1000Type>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<BdtString1000Type>,
    #[serde(rename = "Geometry")]
    pub geometry: GdtAlternativeGeometries2Type,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<SfBufferDistanceType>,
    #[serde(rename = "CanModify")]
    pub can_modify: BdtYesNoType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "DitchType", skip_serializing_if = "Option::is_none")]
    pub ditch_type: Option<WctDitchTypeType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<BdtPositiveInteger5digitsType>,
    #[serde(rename = "Depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MaterialCode", skip_serializing_if = "Option::is_none")]
    pub material_code: Option<BdtMaterialCodeType>,
    #[serde(rename = "MaterialInfoText", skip_serializing_if = "Option::is_none")]
    pub material_info_text: Option<BdtString1000Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<BdtString100Type>,
    #[serde(rename = "SpareGroupOfTrees", skip_serializing_if = "Option::is_none")]
    pub spare_group_of_trees: Option<WctSpareTreesByCategoryType>,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolsType {
    #[serde(rename = "Symbol")]
    pub symbol: Vec<MapSymbolDataType>,
}

