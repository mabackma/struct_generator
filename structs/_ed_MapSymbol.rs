#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolName {
    #[serde(flatten)]
    pub map_symbol_name: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Width {
    #[serde(flatten)]
    pub width: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanName {
    #[serde(flatten)]
    pub ditch_or_road_plan_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareGroupOfTrees {
    #[serde(flatten)]
    pub spare_group_of_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(flatten)]
    pub symbol: MapSymbolDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: BdtMaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchType {
    #[serde(flatten)]
    pub ditch_type: WctDitchTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbols {
    #[serde(flatten)]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Length {
    #[serde(flatten)]
    pub length: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInfoText {
    #[serde(flatten)]
    pub material_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: GdtAlternativeGeometries2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(flatten)]
    pub depth: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

