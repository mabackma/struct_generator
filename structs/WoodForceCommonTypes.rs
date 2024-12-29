#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: BdtSpareTreeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: SpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: BdtVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: BdtDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: BdtTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: BdtHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityOfTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortERPIdType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRatingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesType {
    #[serde(rename = "SpareTreeCategory")]
    pub spare_tree_category: SpareTreeCategoryType,
    #[serde(rename = "AmountOfSpareTrees")]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "MeanDiameterOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_of_spare_trees: Option<DiameterType>,
    #[serde(rename = "MeanHeightOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_height_of_spare_trees: Option<HeightType>,
    #[serde(rename = "VolumeOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub volume_of_spare_trees: Option<VolumeType>,
    #[serde(rename = "DiameterClassOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub diameter_class_of_spare_trees: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

