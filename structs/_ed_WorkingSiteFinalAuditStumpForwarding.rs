#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidinessText {
    #[serde(flatten)]
    pub stump_tidiness_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidiness {
    #[serde(flatten)]
    pub stump_tidiness: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructed {
    #[serde(flatten)]
    pub stump_cutting_as_instructed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructedText {
    #[serde(flatten)]
    pub stump_cutting_as_instructed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingType,
}

