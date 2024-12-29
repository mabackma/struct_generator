#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: CoOtherPublicSubstituteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FinancingActApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: CoFinancingActFinancingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(rename = "FacAreaNo")]
    pub fac_area_no: AreaNo,
    #[serde(rename = "FacFinancingActWorkCode")]
    pub fac_financing_act_work_code: FinancingActWorkCode,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "FacFinancingActApplicationReference")]
    pub fac_financing_act_application_reference: FinancingActApplicationReference,
    #[serde(rename = "FacFinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<FinancingActNumber>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<FinancingActApplicationTextInformation>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: Language,
    #[serde(rename = "FacSender")]
    pub fac_sender: Sender,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "FacEstimatedStartDate")]
    pub fac_estimated_start_date: EstimatedStartDate,
    #[serde(rename = "FacEstimatedEndDate")]
    pub fac_estimated_end_date: EstimatedEndDate,
    #[serde(rename = "FacSubsidyAmount")]
    pub fac_subsidy_amount: SubsidyAmount,
    #[serde(rename = "FacFinancingActWorkGroup")]
    pub fac_financing_act_work_group: FinancingActWorkGroup,
    #[serde(rename = "FacCopOperationProject")]
    pub fac_cop_operation_project: CopOperationProject,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "FacApplicationActors")]
    pub fac_application_actors: ApplicationActors,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FacFinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<FinancingActApplicationStands>,
}

