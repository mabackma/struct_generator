#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: FinalAuditQuestionType,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "AuditQuestion")]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
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
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "BioMassCollectionDone")]
    pub bio_mass_collection_done: YesNoType,
    #[serde(rename = "BioMassCollectionDoneText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done_text: Option<String200Type>,
    #[serde(rename = "ClearingDone")]
    pub clearing_done: YesNoType,
    #[serde(rename = "ClearingDoneText", skip_serializing_if = "Option::is_none")]
    pub clearing_done_text: Option<String200Type>,
    #[serde(rename = "SoilConditioningMethodCorrect")]
    pub soil_conditioning_method_correct: YesNoType,
    #[serde(rename = "SoilConditioningMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct_text: Option<String200Type>,
    #[serde(rename = "SoilConditioningQuality")]
    pub soil_conditioning_quality: WorkingQualityType,
    #[serde(rename = "SoilConditioningQualityText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_quality_text: Option<String200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed")]
    pub environmental_objects_noticed: YesNoType,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<String200Type>,
    #[serde(rename = "LimitsToWaterSystem")]
    pub limits_to_water_system: YesNoType,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<String200Type>,
    #[serde(rename = "WaterEconomySystemsNoticed")]
    pub water_economy_systems_noticed: YesNoType,
    #[serde(rename = "WaterEconomySystemsNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_systems_noticed_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection")]
    pub water_system_protection: YesNoType,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: YesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: YesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
    #[serde(rename = "StumpsLifted", skip_serializing_if = "Option::is_none")]
    pub stumps_lifted: Option<YesNoType>,
    #[serde(rename = "StumpsLiftedText", skip_serializing_if = "Option::is_none")]
    pub stumps_lifted_text: Option<String200Type>,
    #[serde(rename = "DitchingBreaks", skip_serializing_if = "Option::is_none")]
    pub ditching_breaks: Option<YesNoType>,
    #[serde(rename = "DitchingBreaksText", skip_serializing_if = "Option::is_none")]
    pub ditching_breaks_text: Option<String200Type>,
    #[serde(rename = "SettlingPits", skip_serializing_if = "Option::is_none")]
    pub settling_pits: Option<YesNoType>,
    #[serde(rename = "SettlingPitsText", skip_serializing_if = "Option::is_none")]
    pub settling_pits_text: Option<String200Type>,
    #[serde(rename = "SettlingBasins", skip_serializing_if = "Option::is_none")]
    pub settling_basins: Option<YesNoType>,
    #[serde(rename = "SettlingBasinsText", skip_serializing_if = "Option::is_none")]
    pub settling_basins_text: Option<String200Type>,
    #[serde(rename = "SurfaceDrainingAreas", skip_serializing_if = "Option::is_none")]
    pub surface_draining_areas: Option<YesNoType>,
    #[serde(rename = "SurfaceDrainingAreasText", skip_serializing_if = "Option::is_none")]
    pub surface_draining_areas_text: Option<String200Type>,
    #[serde(rename = "OtherConservationMethods", skip_serializing_if = "Option::is_none")]
    pub other_conservation_methods: Option<YesNoType>,
    #[serde(rename = "OtherConservationMethodsText", skip_serializing_if = "Option::is_none")]
    pub other_conservation_methods_text: Option<String200Type>,
    #[serde(rename = "NotificationsAndImprovements", skip_serializing_if = "Option::is_none")]
    pub notifications_and_improvements: Option<YesNoType>,
    #[serde(rename = "NotificationsAndImprovementsText", skip_serializing_if = "Option::is_none")]
    pub notifications_and_improvements_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

