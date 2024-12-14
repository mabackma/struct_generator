#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub area_no: PositiveInteger4digitsType,
    #[serde(rename = "FinancingActWorkCode")]
    pub financing_act_work_code: FinancingActWorkCodeType,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
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
    pub update_previous_message: YesNoType,
    #[serde(rename = "FinancingActApplicationReference")]
    pub financing_act_application_reference: ReferenceType,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<FinancingActNumberType>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<ReferenceType>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_text_information: Option<String2000Type>,
    #[serde(rename = "Language")]
    pub language: LanguageCode1Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<AttorneyType>,
    #[serde(rename = "SentDate")]
    pub sent_date: DateType,
    #[serde(rename = "ElectronicNotification")]
    pub electronic_notification: ElectronicNotificationType,
    #[serde(rename = "FinancingType")]
    pub financing_type: FinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumberType,
    #[serde(rename = "EstimatedStartDate")]
    pub estimated_start_date: DateType,
    #[serde(rename = "EstimatedEndDate")]
    pub estimated_end_date: DateType,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
    #[serde(rename = "FinancingActWorkGroup")]
    pub financing_act_work_group: FinancingActWorkGroupType,
    #[serde(rename = "CopOperationProject")]
    pub cop_operation_project: YesNoType,
    #[serde(rename = "FinancingActRealEstates")]
    pub financing_act_real_estates: FinancingActRealEstatesType,
    #[serde(rename = "ApplicationActors")]
    pub application_actors: ApplicationActorsType,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_stands: Option<FinancingActApplicationStandsType>,
}

