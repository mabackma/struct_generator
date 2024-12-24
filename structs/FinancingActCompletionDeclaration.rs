#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "FacAreaNo")]
    pub fac_area_no: String,
    #[serde(rename = "FacFinancingActWorkCode")]
    pub fac_financing_act_work_code: String,
    #[serde(rename = "FacPayeesAndRealEstates")]
    pub fac_payees_and_real_estates: String,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: String,
    #[serde(rename = "FacCompletionDeclarationReference")]
    pub fac_completion_declaration_reference: String,
    #[serde(rename = "FacFinancingActNumber")]
    pub fac_financing_act_number: String,
    #[serde(rename = "FacFinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_reference: Option<String>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<String>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<CoBankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: CoYesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: CoYesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_declaration_text_information: Option<String>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: String,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: String,
    #[serde(rename = "FacSender")]
    pub fac_sender: String,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<String>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: String,
    #[serde(rename = "FacStartDate")]
    pub fac_start_date: String,
    #[serde(rename = "FacEndDate")]
    pub fac_end_date: String,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: String,
    #[serde(rename = "FacCompletionDeclarationActors")]
    pub fac_completion_declaration_actors: String,
    #[serde(rename = "FacWorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub fac_working_representatives: Option<String>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FacFinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_stands: Option<String>,
}

