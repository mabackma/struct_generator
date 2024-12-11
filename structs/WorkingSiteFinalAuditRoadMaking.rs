#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "RoadStructure", skip_serializing_if = "Option::is_none")]
    pub road_structure: Option<WorkingQualityType>,
    #[serde(rename = "RoadStructureText", skip_serializing_if = "Option::is_none")]
    pub road_structure_text: Option<String200Type>,
    #[serde(rename = "RoadStructureShape", skip_serializing_if = "Option::is_none")]
    pub road_structure_shape: Option<WorkingQualityType>,
    #[serde(rename = "RoadStructureShapeText", skip_serializing_if = "Option::is_none")]
    pub road_structure_shape_text: Option<String200Type>,
    #[serde(rename = "RoadStructureDrainage", skip_serializing_if = "Option::is_none")]
    pub road_structure_drainage: Option<WorkingQualityType>,
    #[serde(rename = "RoadStructureDrainageText", skip_serializing_if = "Option::is_none")]
    pub road_structure_drainage_text: Option<String200Type>,
    #[serde(rename = "PassingPlace", skip_serializing_if = "Option::is_none")]
    pub passing_place: Option<WorkingQualityType>,
    #[serde(rename = "PassingPlaceText", skip_serializing_if = "Option::is_none")]
    pub passing_place_text: Option<String200Type>,
    #[serde(rename = "TurningPlace", skip_serializing_if = "Option::is_none")]
    pub turning_place: Option<WorkingQualityType>,
    #[serde(rename = "TurningPlaceText", skip_serializing_if = "Option::is_none")]
    pub turning_place_text: Option<String200Type>,
    #[serde(rename = "PipeInstallation", skip_serializing_if = "Option::is_none")]
    pub pipe_installation: Option<WorkingQualityType>,
    #[serde(rename = "PipeInstallationText", skip_serializing_if = "Option::is_none")]
    pub pipe_installation_text: Option<String200Type>,
    #[serde(rename = "ErosionBlockingAction", skip_serializing_if = "Option::is_none")]
    pub erosion_blocking_action: Option<WorkingQualityType>,
    #[serde(rename = "ErosionBlockingActionText", skip_serializing_if = "Option::is_none")]
    pub erosion_blocking_action_text: Option<String200Type>,
    #[serde(rename = "WaterProtectionAction", skip_serializing_if = "Option::is_none")]
    pub water_protection_action: Option<WorkingQualityType>,
    #[serde(rename = "WaterProtectionActionText", skip_serializing_if = "Option::is_none")]
    pub water_protection_action_text: Option<String200Type>,
    #[serde(rename = "LandScaping", skip_serializing_if = "Option::is_none")]
    pub land_scaping: Option<WorkingQualityType>,
    #[serde(rename = "LandScapingText", skip_serializing_if = "Option::is_none")]
    pub land_scaping_text: Option<String200Type>,
    #[serde(rename = "FeedbackForPlanner", skip_serializing_if = "Option::is_none")]
    pub feedback_for_planner: Option<WorkingQualityType>,
    #[serde(rename = "FeedbackForPlannerText", skip_serializing_if = "Option::is_none")]
    pub feedback_for_planner_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMakingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: String20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

