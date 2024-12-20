#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratum {
    #[serde(flatten)]
    pub dead_tree_stratum: DeadTreeStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeType {
    #[serde(flatten)]
    pub dead_tree_type: DeadTreeTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrata {
    #[serde(flatten)]
    pub dead_tree_strata: DeadTreeStrataType,
}

