#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMakingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "RoadStructure", skip_serializing_if = "Option::is_none")]
    pub road_structure: Option<BdtWorkingQualityType>,
    #[serde(rename = "RoadStructureText", skip_serializing_if = "Option::is_none")]
    pub road_structure_text: Option<BdtString200Type>,
    #[serde(rename = "RoadStructureShape", skip_serializing_if = "Option::is_none")]
    pub road_structure_shape: Option<BdtWorkingQualityType>,
    #[serde(rename = "RoadStructureShapeText", skip_serializing_if = "Option::is_none")]
    pub road_structure_shape_text: Option<BdtString200Type>,
    #[serde(rename = "RoadStructureDrainage", skip_serializing_if = "Option::is_none")]
    pub road_structure_drainage: Option<BdtWorkingQualityType>,
    #[serde(rename = "RoadStructureDrainageText", skip_serializing_if = "Option::is_none")]
    pub road_structure_drainage_text: Option<BdtString200Type>,
    #[serde(rename = "PassingPlace", skip_serializing_if = "Option::is_none")]
    pub passing_place: Option<BdtWorkingQualityType>,
    #[serde(rename = "PassingPlaceText", skip_serializing_if = "Option::is_none")]
    pub passing_place_text: Option<BdtString200Type>,
    #[serde(rename = "TurningPlace", skip_serializing_if = "Option::is_none")]
    pub turning_place: Option<BdtWorkingQualityType>,
    #[serde(rename = "TurningPlaceText", skip_serializing_if = "Option::is_none")]
    pub turning_place_text: Option<BdtString200Type>,
    #[serde(rename = "PipeInstallation", skip_serializing_if = "Option::is_none")]
    pub pipe_installation: Option<BdtWorkingQualityType>,
    #[serde(rename = "PipeInstallationText", skip_serializing_if = "Option::is_none")]
    pub pipe_installation_text: Option<BdtString200Type>,
    #[serde(rename = "ErosionBlockingAction", skip_serializing_if = "Option::is_none")]
    pub erosion_blocking_action: Option<BdtWorkingQualityType>,
    #[serde(rename = "ErosionBlockingActionText", skip_serializing_if = "Option::is_none")]
    pub erosion_blocking_action_text: Option<BdtString200Type>,
    #[serde(rename = "WaterProtectionAction", skip_serializing_if = "Option::is_none")]
    pub water_protection_action: Option<BdtWorkingQualityType>,
    #[serde(rename = "WaterProtectionActionText", skip_serializing_if = "Option::is_none")]
    pub water_protection_action_text: Option<BdtString200Type>,
    #[serde(rename = "LandScaping", skip_serializing_if = "Option::is_none")]
    pub land_scaping: Option<BdtWorkingQualityType>,
    #[serde(rename = "LandScapingText", skip_serializing_if = "Option::is_none")]
    pub land_scaping_text: Option<BdtString200Type>,
    #[serde(rename = "FeedbackForPlanner", skip_serializing_if = "Option::is_none")]
    pub feedback_for_planner: Option<BdtWorkingQualityType>,
    #[serde(rename = "FeedbackForPlannerText", skip_serializing_if = "Option::is_none")]
    pub feedback_for_planner_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<BdtFinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: BdtFinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: BdtString20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: BdtString50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: BdtTimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: BdtYesNoType,
}

