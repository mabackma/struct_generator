#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSite {
    #[serde(flatten)]
    pub working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSites {
    #[serde(flatten)]
    pub working_sites: WorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamName {
    #[serde(flatten)]
    pub team_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderDelay {
    #[serde(flatten)]
    pub forwarder_delay: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    #[serde(flatten)]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSchedule {
    #[serde(flatten)]
    pub resource_schedule: ResourceScheduleType,
}

