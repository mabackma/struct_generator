#[derive(Debug, Serialize, Deserialize)]
pub struct TeamName {
    #[serde(flatten)]
    pub team_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSites {
    #[serde(flatten)]
    pub working_sites: WorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderDelay {
    #[serde(flatten)]
    pub forwarder_delay: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSchedule {
    #[serde(flatten)]
    pub resource_schedule: ResourceScheduleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSite {
    #[serde(flatten)]
    pub working_site: WorkingSiteType,
}

