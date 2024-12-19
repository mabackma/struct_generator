#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallationText {
    #[serde(flatten)]
    pub pipe_installation_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlanner {
    #[serde(flatten)]
    pub feedback_for_planner: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionAction {
    #[serde(flatten)]
    pub water_protection_action: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingAction {
    #[serde(flatten)]
    pub erosion_blocking_action: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlaceText {
    #[serde(flatten)]
    pub turning_place_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScapingText {
    #[serde(flatten)]
    pub land_scaping_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionActionText {
    #[serde(flatten)]
    pub water_protection_action_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructure {
    #[serde(flatten)]
    pub road_structure: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlaceText {
    #[serde(flatten)]
    pub passing_place_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMaking {
    #[serde(flatten)]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlace {
    #[serde(flatten)]
    pub passing_place: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingActionText {
    #[serde(flatten)]
    pub erosion_blocking_action_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureText {
    #[serde(flatten)]
    pub road_structure_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShape {
    #[serde(flatten)]
    pub road_structure_shape: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallation {
    #[serde(flatten)]
    pub pipe_installation: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlannerText {
    #[serde(flatten)]
    pub feedback_for_planner_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlace {
    #[serde(flatten)]
    pub turning_place: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShapeText {
    #[serde(flatten)]
    pub road_structure_shape_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainageText {
    #[serde(flatten)]
    pub road_structure_drainage_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScaping {
    #[serde(flatten)]
    pub land_scaping: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainage {
    #[serde(flatten)]
    pub road_structure_drainage: WorkingQualityType,
}

