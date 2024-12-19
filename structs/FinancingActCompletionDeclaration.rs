#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_stands: Option<FinancingActCompletionStandsType>,
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
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: YesNoType,
    #[serde(rename = "CompletionDeclarationReference")]
    pub completion_declaration_reference: ReferenceType,
    #[serde(rename = "FinancingActNumber")]
    pub financing_act_number: FinancingActNumberType,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_reference: Option<ReferenceType>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<ReferenceType>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<BankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: YesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: YesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_declaration_text_information: Option<String2000Type>,
    #[serde(rename = "Language")]
    pub language: LanguageCode1Type,
    #[serde(rename = "ElectronicNotification")]
    pub electronic_notification: ElectronicNotificationType,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<ContactInformationType>,
    #[serde(rename = "SentDate")]
    pub sent_date: DateType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "FinancingActRealEstates")]
    pub financing_act_real_estates: FinancingActRealEstatesType,
    #[serde(rename = "CompletionDeclarationActors")]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
    #[serde(rename = "WorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub working_representatives: Option<WorkingRepresentativesType>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub area_no: PositiveInteger4digitsType,
    #[serde(rename = "FinancingActWorkCode")]
    pub financing_act_work_code: FinancingActWorkCodeType,
    #[serde(rename = "PayeesAndRealEstates")]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

