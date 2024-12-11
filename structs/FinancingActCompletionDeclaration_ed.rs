#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraFinancingApplication {
    #[serde(flatten)]
    pub extra_financing_application: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidy {
    #[serde(flatten)]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: OtherPublicSubstituteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActProjectCompleted {
    #[serde(flatten)]
    pub financing_act_project_completed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FinancingActCompletionDeclarationType,
}

