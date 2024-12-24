#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: CddDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum {
    #[serde(flatten)]
    pub tree_stratum: TreeStratum2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TreeStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: CoMeanHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: MeanDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CoCurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumOrigin {
    #[serde(flatten)]
    pub stratum_origin: CoSeedlingOriginType,
}

