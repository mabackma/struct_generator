use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanAndSubsidy {
    #[serde(flatten)]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: CoFinancingActFinancingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FinancingActApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "FinancingActApplicationReference")]
    pub fac_financing_act_application_reference: FinancingActApplicationReference,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<FinancingActNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<FinancingActApplicationTextInformation>,
    #[serde(rename = "Language")]
    pub fac_language: Language,
    #[serde(rename = "Sender")]
    pub fac_sender: Sender,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "SentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "ElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "EstimatedStartDate")]
    pub fac_estimated_start_date: EstimatedStartDate,
    #[serde(rename = "EstimatedEndDate")]
    pub fac_estimated_end_date: EstimatedEndDate,
    #[serde(rename = "SubsidyAmount")]
    pub fac_subsidy_amount: SubsidyAmount,
    #[serde(rename = "FinancingActWorkGroup")]
    pub fac_financing_act_work_group: FinancingActWorkGroup,
    #[serde(rename = "CopOperationProject")]
    pub fac_cop_operation_project: CopOperationProject,
    #[serde(rename = "FinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "ApplicationActors")]
    pub fac_application_actors: ApplicationActors,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<FinancingActApplicationStands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub fac_area_no: AreaNo,
    #[serde(rename = "FinancingActWorkCode")]
    pub fac_financing_act_work_code: FinancingActWorkCode,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

