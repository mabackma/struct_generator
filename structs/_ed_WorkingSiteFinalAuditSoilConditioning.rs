#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLifted {
    #[serde(flatten)]
    pub stumps_lifted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovementsText {
    #[serde(flatten)]
    pub notifications_and_improvements_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethodsText {
    #[serde(flatten)]
    pub other_conservation_methods_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLiftedText {
    #[serde(flatten)]
    pub stumps_lifted_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreasText {
    #[serde(flatten)]
    pub surface_draining_areas_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioning {
    #[serde(flatten)]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDone {
    #[serde(flatten)]
    pub clearing_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasins {
    #[serde(flatten)]
    pub settling_basins: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaks {
    #[serde(flatten)]
    pub ditching_breaks: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreas {
    #[serde(flatten)]
    pub surface_draining_areas: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDoneText {
    #[serde(flatten)]
    pub clearing_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethods {
    #[serde(flatten)]
    pub other_conservation_methods: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovements {
    #[serde(flatten)]
    pub notifications_and_improvements: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasinsText {
    #[serde(flatten)]
    pub settling_basins_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: BdtFinalAuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPits {
    #[serde(flatten)]
    pub settling_pits: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaksText {
    #[serde(flatten)]
    pub ditching_breaks_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPitsText {
    #[serde(flatten)]
    pub settling_pits_text: BdtString200Type,
}

