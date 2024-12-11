#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: String,
    #[serde(rename = "CompletionDeclarationReference")]
    pub completion_declaration_reference: String,
    #[serde(rename = "FinancingActNumber")]
    pub financing_act_number: String,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_reference: Option<String>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<BankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: YesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: YesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_declaration_text_information: Option<String>,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "ElectronicNotification")]
    pub electronic_notification: String,
    #[serde(rename = "Sender")]
    pub sender: String,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<String>,
    #[serde(rename = "SentDate")]
    pub sent_date: String,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "FinancingActRealEstates")]
    pub financing_act_real_estates: String,
    #[serde(rename = "CompletionDeclarationActors")]
    pub completion_declaration_actors: String,
    #[serde(rename = "WorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub working_representatives: Option<String>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub area_no: String,
    #[serde(rename = "FinancingActWorkCode")]
    pub financing_act_work_code: String,
    #[serde(rename = "PayeesAndRealEstates")]
    pub payees_and_real_estates: String,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

