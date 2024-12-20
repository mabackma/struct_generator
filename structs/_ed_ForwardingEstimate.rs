#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: WctERPIdType,
}

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

