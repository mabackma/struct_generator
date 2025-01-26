use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDataAndSubsidy {
    #[serde(flatten)]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActProjectCompleted {
    #[serde(flatten)]
    pub financing_act_project_completed: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraFinancingApplication {
    #[serde(flatten)]
    pub extra_financing_application: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FinancingActCompletionDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub fac_update_previous_message: FacUpdatePreviousMessage,
    #[serde(rename = "CompletionDeclarationReference")]
    pub fac_completion_declaration_reference: FacCompletionDeclarationReference,
    #[serde(rename = "FinancingActNumber")]
    pub fac_financing_act_number: FacFinancingActNumber,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_reference: Option<FacFinancingActApplicationReference>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<FacCustomerReference>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<BankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: CoYesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: CoYesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_declaration_text_information: Option<FacFinancingActCompletionDeclarationTextInformation>,
    #[serde(rename = "Language")]
    pub fac_language: FacLanguage,
    #[serde(rename = "ElectronicNotification")]
    pub fac_electronic_notification: FacElectronicNotification,
    #[serde(rename = "Sender")]
    pub fac_sender: FacSender,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<FacAttorney>,
    #[serde(rename = "SentDate")]
    pub fac_sent_date: FacSentDate,
    #[serde(rename = "StartDate")]
    pub fac_start_date: FacStartDate,
    #[serde(rename = "EndDate")]
    pub fac_end_date: FacEndDate,
    #[serde(rename = "FinancingActRealEstates")]
    pub fac_financing_act_real_estates: FacFinancingActRealEstates,
    #[serde(rename = "CompletionDeclarationActors")]
    pub fac_completion_declaration_actors: FacCompletionDeclarationActors,
    #[serde(rename = "WorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub fac_working_representatives: Option<FacWorkingRepresentatives>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<FacDocuments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub fac_area_no: FacAreaNo,
    #[serde(rename = "FinancingActWorkCode")]
    pub fac_financing_act_work_code: FacFinancingActWorkCode,
    #[serde(rename = "PayeesAndRealEstates")]
    pub fac_payees_and_real_estates: FacPayeesAndRealEstates,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_stands: Option<FacFinancingActCompletionStands>,
}

