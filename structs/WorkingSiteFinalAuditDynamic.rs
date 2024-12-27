#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamicType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseDynamicType,
    #[serde(rename = "FinalAuditSpareTrees", skip_serializing_if = "Option::is_none")]
    pub final_audit_spare_trees: Option<FinalAuditSpareTreesByCategoryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<WctFinalAuditSpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeType {
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Value")]
    pub value: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "Question")]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourcesType {
    #[serde(rename = "AuditionResource", skip_serializing_if = "Option::is_none")]
    pub audition_resource: Option<AuditionResourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: WctERPIdType,
    #[serde(rename = "QuestionAsText", skip_serializing_if = "Option::is_none")]
    pub question_as_text: Option<BdtString200Type>,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: BdtString50Type,
    #[serde(rename = "QuestionAnswerAsText", skip_serializing_if = "Option::is_none")]
    pub question_answer_as_text: Option<BdtString50Type>,
    #[serde(rename = "QuestionAnswerAdditionalText", skip_serializing_if = "Option::is_none")]
    pub question_answer_additional_text: Option<BdtString200Type>,
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
pub struct AuditionResourceType {
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesType {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeType,
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
    #[serde(rename = "FinalAuditerId", skip_serializing_if = "Option::is_none")]
    pub final_auditer_id: Option<BdtString20Type>,
    #[serde(rename = "FinalAuditIdentifier")]
    pub final_audit_identifier: BdtString50Type,
    #[serde(rename = "FinalAuditIdentifierVersion")]
    pub final_audit_identifier_version: BdtString10Type,
    #[serde(rename = "FinalAuditName", skip_serializing_if = "Option::is_none")]
    pub final_audit_name: Option<BdtString100Type>,
}

