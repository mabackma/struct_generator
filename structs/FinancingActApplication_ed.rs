#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: OtherPublicSubstituteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: FinancingActFinancingType,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: MunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

