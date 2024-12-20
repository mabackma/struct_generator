#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: BdtFinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: BdtFinalAuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: BdtFinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: BdtWorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: BdtString200Type,
}

