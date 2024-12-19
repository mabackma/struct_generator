#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCount {
    #[serde(flatten)]
    pub remaining_stump_count: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQuality {
    #[serde(flatten)]
    pub stump_lifting_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructed {
    #[serde(flatten)]
    pub stump_cutting_as_instructed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitableText {
    #[serde(flatten)]
    pub stump_lifting_suitable_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQualityText {
    #[serde(flatten)]
    pub stump_lifting_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidinessText {
    #[serde(flatten)]
    pub stump_tidiness_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructedText {
    #[serde(flatten)]
    pub stump_cutting_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidiness {
    #[serde(flatten)]
    pub stump_tidiness: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitable {
    #[serde(flatten)]
    pub stump_lifting_suitable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCountText {
    #[serde(flatten)]
    pub remaining_stump_count_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
}

