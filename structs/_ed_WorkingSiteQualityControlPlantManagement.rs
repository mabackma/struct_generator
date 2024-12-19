#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusReduction {
    #[serde(flatten)]
    pub radius_reduction: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameterSummary {
    #[serde(flatten)]
    pub reduction_avg_diameter_summary: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingTrees {
    #[serde(flatten)]
    pub remaining_trees: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCountSummary {
    #[serde(flatten)]
    pub reduction_count_summary: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameter {
    #[serde(flatten)]
    pub reduction_avg_diameter: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damages {
    #[serde(flatten)]
    pub damages: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionStumpHeight {
    #[serde(flatten)]
    pub reduction_stump_height: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCount {
    #[serde(flatten)]
    pub reduction_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedTrees {
    #[serde(flatten)]
    pub harvested_trees: FinalAuditAnswerType,
}

