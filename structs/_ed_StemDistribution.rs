#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

