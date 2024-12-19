#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQualityText {
    #[serde(flatten)]
    pub stump_lifting_quality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitable {
    #[serde(flatten)]
    pub stump_lifting_suitable: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQuality {
    #[serde(flatten)]
    pub stump_lifting_quality: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCount {
    #[serde(flatten)]
    pub remaining_stump_count: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitableText {
    #[serde(flatten)]
    pub stump_lifting_suitable_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCountText {
    #[serde(flatten)]
    pub remaining_stump_count_text: String200Type,
}

