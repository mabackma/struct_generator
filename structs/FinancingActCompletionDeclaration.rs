use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FinancingActCompletionDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDataAndSubsidy {
    #[serde(flatten)]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraFinancingApplication {
    #[serde(flatten)]
    pub extra_financing_application: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: OtherPublicSubstituteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActProjectCompleted {
    #[serde(flatten)]
    pub financing_act_project_completed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FacAreaNo")]
    pub fac_area_no: AreaNo,
    #[serde(rename = "FacFinancingActWorkCode")]
    pub fac_financing_act_work_code: FinancingActWorkCode,
    #[serde(rename = "FacPayeesAndRealEstates")]
    pub fac_payees_and_real_estates: PayeesAndRealEstates,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FacFinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_stands: Option<FinancingActCompletionStands>,
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
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "FacCompletionDeclarationReference")]
    pub fac_completion_declaration_reference: CompletionDeclarationReference,
    #[serde(rename = "FacFinancingActNumber")]
    pub fac_financing_act_number: FinancingActNumber,
    #[serde(rename = "FacFinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_reference: Option<FinancingActApplicationReference>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<BankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: YesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: YesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_declaration_text_information: Option<FinancingActCompletionDeclarationTextInformation>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: Language,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "FacSender")]
    pub fac_sender: Sender,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "FacStartDate")]
    pub fac_start_date: StartDate,
    #[serde(rename = "FacEndDate")]
    pub fac_end_date: EndDate,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "FacCompletionDeclarationActors")]
    pub fac_completion_declaration_actors: CompletionDeclarationActors,
    #[serde(rename = "FacWorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub fac_working_representatives: Option<WorkingRepresentatives>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

