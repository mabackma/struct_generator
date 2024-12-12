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
    pub update_previous_message: String,
    #[serde(rename = "FinancingActApplicationReference")]
    pub financing_act_application_reference: String,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<String>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_text_information: Option<String>,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Sender")]
    pub sender: String,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<String>,
    #[serde(rename = "SentDate")]
    pub sent_date: String,
    #[serde(rename = "ElectronicNotification")]
    pub electronic_notification: String,
    #[serde(rename = "FinancingType")]
    pub financing_type: FinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumberType,
    #[serde(rename = "EstimatedStartDate")]
    pub estimated_start_date: String,
    #[serde(rename = "EstimatedEndDate")]
    pub estimated_end_date: String,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: String,
    #[serde(rename = "FinancingActWorkGroup")]
    pub financing_act_work_group: String,
    #[serde(rename = "CopOperationProject")]
    pub cop_operation_project: String,
    #[serde(rename = "FinancingActRealEstates")]
    pub financing_act_real_estates: String,
    #[serde(rename = "ApplicationActors")]
    pub application_actors: String,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub area_no: String,
    #[serde(rename = "FinancingActWorkCode")]
    pub financing_act_work_code: String,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_stands: Option<String>,
}

