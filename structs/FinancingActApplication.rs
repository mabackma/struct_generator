use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FinancingActApplicationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: CoFinancingActFinancingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanAndSubsidy {
    #[serde(flatten)]
    pub plan_and_subsidy: PlanAndSubsidyType,
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
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub fac_update_previous_message: FacUpdatePreviousMessage,
    #[serde(rename = "FinancingActApplicationReference")]
    pub fac_financing_act_application_reference: FacFinancingActApplicationReference,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<FacFinancingActNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<FacCustomerReference>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<FacFinancingActApplicationTextInformation>,
    #[serde(rename = "Language")]
    pub fac_language: FacLanguage,
    #[serde(rename = "Sender")]
    pub fac_sender: FacSender,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<FacAttorney>,
    #[serde(rename = "SentDate")]
    pub fac_sent_date: FacSentDate,
    #[serde(rename = "ElectronicNotification")]
    pub fac_electronic_notification: FacElectronicNotification,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "EstimatedStartDate")]
    pub fac_estimated_start_date: FacEstimatedStartDate,
    #[serde(rename = "EstimatedEndDate")]
    pub fac_estimated_end_date: FacEstimatedEndDate,
    #[serde(rename = "SubsidyAmount")]
    pub fac_subsidy_amount: FacSubsidyAmount,
    #[serde(rename = "FinancingActWorkGroup")]
    pub fac_financing_act_work_group: FacFinancingActWorkGroup,
    #[serde(rename = "CopOperationProject")]
    pub fac_cop_operation_project: FacCopOperationProject,
    #[serde(rename = "FinancingActRealEstates")]
    pub fac_financing_act_real_estates: FacFinancingActRealEstates,
    #[serde(rename = "ApplicationActors")]
    pub fac_application_actors: FacApplicationActors,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<FacDocuments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<FacFinancingActApplicationStands>,
}

