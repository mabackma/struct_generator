#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: TreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    #[serde(flatten)]
    pub tree: TreeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(flatten)]
    pub count: CountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: TreeNumberType,
}

