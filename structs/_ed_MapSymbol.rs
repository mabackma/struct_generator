#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(flatten)]
    pub depth: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Width {
    #[serde(flatten)]
    pub width: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolName {
    #[serde(flatten)]
    pub map_symbol_name: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInfoText {
    #[serde(flatten)]
    pub material_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbols {
    #[serde(flatten)]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareGroupOfTrees {
    #[serde(flatten)]
    pub spare_group_of_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchType {
    #[serde(flatten)]
    pub ditch_type: WctDitchTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(flatten)]
    pub symbol: MapSymbolDataType,
}

