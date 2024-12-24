#[derive(Debug, Serialize, Deserialize)]
pub struct Loads {
    #[serde(flatten)]
    pub loads: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    #[serde(flatten)]
    pub day: BdtDateType,
}

