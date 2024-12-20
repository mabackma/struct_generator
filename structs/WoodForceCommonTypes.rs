#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRatingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortERPIdType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesType {
    #[serde(rename = "SpareTreeCategory")]
    pub spare_tree_category: BdtSpareTreeCategoryType,
    #[serde(rename = "AmountOfSpareTrees")]
    pub amount_of_spare_trees: BdtPositiveInteger5digitsType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<BdtTreeSpeciesType>,
    #[serde(rename = "MeanDiameterOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_of_spare_trees: Option<BdtDiameterType>,
    #[serde(rename = "MeanHeightOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_height_of_spare_trees: Option<BdtHeightType>,
    #[serde(rename = "VolumeOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub volume_of_spare_trees: Option<BdtVolumeType>,
    #[serde(rename = "DiameterClassOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub diameter_class_of_spare_trees: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<BdtString20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<BdtPositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityOfTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

