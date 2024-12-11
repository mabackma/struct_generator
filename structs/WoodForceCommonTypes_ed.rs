#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: SpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: SpareTreeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: HeightType,
}

