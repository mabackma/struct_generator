use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: DiameterType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: SpareTreeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: SpareTreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: VolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: HeightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearingType {
    #[serde(rename = "responsible_of_pre_clearing_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthType {
    #[serde(rename = "month_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorTreeSpeciesType {
    #[serde(rename = "scale_factor_tree_species_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchTypeType {
    #[serde(rename = "ditch_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethodType {
    #[serde(rename = "pricing_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageType {
    #[serde(rename = "common_message_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumberType {
    #[serde(flatten)]
    pub base: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(rename = "area_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERPIdType {
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRatingType {
    #[serde(rename = "load_rating_type.base")]
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
pub struct TaxNumberType {
    #[serde(flatten)]
    pub base: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityOfTreeSpeciesType {
    #[serde(rename = "quality_of_tree_species_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortERPIdType {
    #[serde(flatten)]
    pub base: String20Type,
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
pub struct HopperTypeType {
    #[serde(rename = "hopper_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordTreeSpeciesType {
    #[serde(rename = "stanford_tree_species_type.base")]
    pub base: i32,
}

