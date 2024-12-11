#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditIdentifierVersion {
    #[serde(flatten)]
    pub final_audit_identifier_version: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audition {
    #[serde(flatten)]
    pub audition: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(flatten)]
    pub resource_type: ResourceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResources {
    #[serde(flatten)]
    pub audition_resources: AuditionResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: FinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTrees {
    #[serde(flatten)]
    pub final_audit_spare_trees: FinalAuditSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerAdditionalText {
    #[serde(flatten)]
    pub question_answer_additional_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: FinalAuditSpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamic {
    #[serde(flatten)]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerText {
    #[serde(flatten)]
    pub question_answer_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditName {
    #[serde(flatten)]
    pub final_audit_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeName {
    #[serde(flatten)]
    pub attribute_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    #[serde(flatten)]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    #[serde(flatten)]
    pub attribute_value: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResource {
    #[serde(flatten)]
    pub audition_resource: AuditionResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Questions {
    #[serde(flatten)]
    pub questions: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(flatten)]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(flatten)]
    pub value: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(flatten)]
    pub attributes: AttributesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAsText {
    #[serde(flatten)]
    pub question_as_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerAsText {
    #[serde(flatten)]
    pub question_answer_as_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: FinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditIdentifier {
    #[serde(flatten)]
    pub final_audit_identifier: String50Type,
}

