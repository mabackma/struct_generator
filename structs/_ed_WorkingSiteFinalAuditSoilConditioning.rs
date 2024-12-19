#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLiftedText {
    #[serde(flatten)]
    pub stumps_lifted_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreasText {
    #[serde(flatten)]
    pub surface_draining_areas_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLifted {
    #[serde(flatten)]
    pub stumps_lifted: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasins {
    #[serde(flatten)]
    pub settling_basins: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaksText {
    #[serde(flatten)]
    pub ditching_breaks_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPitsText {
    #[serde(flatten)]
    pub settling_pits_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovementsText {
    #[serde(flatten)]
    pub notifications_and_improvements_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethods {
    #[serde(flatten)]
    pub other_conservation_methods: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreas {
    #[serde(flatten)]
    pub surface_draining_areas: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaks {
    #[serde(flatten)]
    pub ditching_breaks: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethodsText {
    #[serde(flatten)]
    pub other_conservation_methods_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPits {
    #[serde(flatten)]
    pub settling_pits: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDone {
    #[serde(flatten)]
    pub clearing_done: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDoneText {
    #[serde(flatten)]
    pub clearing_done_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovements {
    #[serde(flatten)]
    pub notifications_and_improvements: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasinsText {
    #[serde(flatten)]
    pub settling_basins_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioning {
    #[serde(flatten)]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
}

