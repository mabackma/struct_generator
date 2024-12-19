#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FacFinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "FacAreaNo")]
    pub fac_area_no: String,
    #[serde(rename = "FacFinancingActWorkCode")]
    pub fac_financing_act_work_code: String,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: String,
    #[serde(rename = "FacFinancingActApplicationReference")]
    pub fac_financing_act_application_reference: String,
    #[serde(rename = "FacFinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<String>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<String>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<String>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: String,
    #[serde(rename = "FacSender")]
    pub fac_sender: String,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<String>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: String,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: String,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "FacEstimatedStartDate")]
    pub fac_estimated_start_date: String,
    #[serde(rename = "FacEstimatedEndDate")]
    pub fac_estimated_end_date: String,
    #[serde(rename = "FacSubsidyAmount")]
    pub fac_subsidy_amount: String,
    #[serde(rename = "FacFinancingActWorkGroup")]
    pub fac_financing_act_work_group: String,
    #[serde(rename = "FacCopOperationProject")]
    pub fac_cop_operation_project: String,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: String,
    #[serde(rename = "FacApplicationActors")]
    pub fac_application_actors: String,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<String>,
}

