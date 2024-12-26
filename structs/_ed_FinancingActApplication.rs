#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: CoFinancingActFinancingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidy {
    #[serde(flatten)]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FinancingActApplicationType,
}

