#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: BufferDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: FeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Length {
    #[serde(flatten)]
    pub length: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(flatten)]
    pub depth: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareGroupOfTrees {
    #[serde(flatten)]
    pub spare_group_of_trees: SpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbols {
    #[serde(flatten)]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: AlternativeGeometries2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchType {
    #[serde(flatten)]
    pub ditch_type: DitchTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolName {
    #[serde(flatten)]
    pub map_symbol_name: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(flatten)]
    pub symbol: MapSymbolDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Width {
    #[serde(flatten)]
    pub width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: MaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInfoText {
    #[serde(flatten)]
    pub material_info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: FeatureAdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanName {
    #[serde(flatten)]
    pub ditch_or_road_plan_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

