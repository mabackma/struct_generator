#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: WorkingSiteStatusType,
}

