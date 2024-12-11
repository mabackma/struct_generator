#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<FinalAuditSpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Value")]
    pub value: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourceType {
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: ERPIdType,
    #[serde(rename = "QuestionAsText", skip_serializing_if = "Option::is_none")]
    pub question_as_text: Option<String200Type>,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: String50Type,
    #[serde(rename = "QuestionAnswerAsText", skip_serializing_if = "Option::is_none")]
    pub question_answer_as_text: Option<String50Type>,
    #[serde(rename = "QuestionAnswerAdditionalText", skip_serializing_if = "Option::is_none")]
    pub question_answer_additional_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesType {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourcesType {
    #[serde(rename = "AuditionResource", skip_serializing_if = "Option::is_none")]
    pub audition_resource: Option<AuditionResourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "Question")]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamicType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseDynamicType,
    #[serde(rename = "FinalAuditSpareTrees", skip_serializing_if = "Option::is_none")]
    pub final_audit_spare_trees: Option<FinalAuditSpareTreesByCategoryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseDynamicType {
    #[serde(rename = "Attributes")]
    pub attributes: AttributesType,
    #[serde(rename = "Audition")]
    pub audition: AuditionType,
    #[serde(rename = "AuditionResources", skip_serializing_if = "Option::is_none")]
    pub audition_resources: Option<AuditionResourcesType>,
    #[serde(rename = "Questions")]
    pub questions: AuditsListType,
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
    #[serde(rename = "FinalAuditerId", skip_serializing_if = "Option::is_none")]
    pub final_auditer_id: Option<String20Type>,
    #[serde(rename = "FinalAuditIdentifier")]
    pub final_audit_identifier: String50Type,
    #[serde(rename = "FinalAuditIdentifierVersion")]
    pub final_audit_identifier_version: String10Type,
    #[serde(rename = "FinalAuditName", skip_serializing_if = "Option::is_none")]
    pub final_audit_name: Option<String100Type>,
}

