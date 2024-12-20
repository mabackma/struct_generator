#[derive(Debug, Serialize, Deserialize)]
pub struct Questions {
    #[serde(flatten)]
    pub questions: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audition {
    #[serde(flatten)]
    pub audition: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeName {
    #[serde(flatten)]
    pub attribute_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResource {
    #[serde(flatten)]
    pub audition_resource: AuditionResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    #[serde(flatten)]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerAdditionalText {
    #[serde(flatten)]
    pub question_answer_additional_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerAsText {
    #[serde(flatten)]
    pub question_answer_as_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditName {
    #[serde(flatten)]
    pub final_audit_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(flatten)]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    #[serde(flatten)]
    pub attribute_value: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditIdentifier {
    #[serde(flatten)]
    pub final_audit_identifier: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerText {
    #[serde(flatten)]
    pub question_answer_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamic {
    #[serde(flatten)]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTrees {
    #[serde(flatten)]
    pub final_audit_spare_trees: FinalAuditSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAsText {
    #[serde(flatten)]
    pub question_as_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResources {
    #[serde(flatten)]
    pub audition_resources: AuditionResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(flatten)]
    pub attributes: AttributesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditIdentifierVersion {
    #[serde(flatten)]
    pub final_audit_identifier_version: BdtString10Type,
}

