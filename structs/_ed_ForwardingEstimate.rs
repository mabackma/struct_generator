#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loads {
    #[serde(flatten)]
    pub loads: BdtPositiveInteger3digitsType,
}

