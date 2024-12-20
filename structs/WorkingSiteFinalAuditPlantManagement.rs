#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: BdtPositiveInteger2digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: BdtFinalAuditQuestionType,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<BdtFinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: BdtFinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: BdtString50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: BdtTimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "AuditQuestion")]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "PlantManagementQuality")]
    pub plant_management_quality: BdtWorkingQualityType,
    #[serde(rename = "PlantManagementQualityText", skip_serializing_if = "Option::is_none")]
    pub plant_management_quality_text: Option<BdtString200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: BdtYesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: BdtYesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: BdtYesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: BdtYesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<BdtString200Type>,
}

