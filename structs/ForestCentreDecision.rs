#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionsType {
    #[serde(rename = "CaseAction")]
    pub case_action: Vec<CaseActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustificationsType {
    #[serde(rename = "Justification")]
    pub justification: Vec<String5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionContactInformationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FirstName")]
    pub first_name: FirstNameType,
    #[serde(rename = "LastName")]
    pub last_name: LastNameType,
    #[serde(rename = "PersonOrganizationName")]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlersType {
    #[serde(rename = "DecisionHandler")]
    pub decision_handler: Vec<DecisionHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<ReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "SubsidyZone")]
    pub subsidy_zone: ForestActAreaType,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<ReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType2,
    #[serde(rename = "CostTypeDescription")]
    pub cost_type_description: CostTypeDescriptionType,
    #[serde(rename = "SubsidyPercent", skip_serializing_if = "Option::is_none")]
    pub subsidy_percent: Option<PercentType>,
    #[serde(rename = "ApplicationAmount", skip_serializing_if = "Option::is_none")]
    pub application_amount: Option<Decimal7And2Type>,
    #[serde(rename = "ApplicationAmountUnit", skip_serializing_if = "Option::is_none")]
    pub application_amount_unit: Option<ForestCentreUnitType>,
    #[serde(rename = "ApplicationUnitPrice", skip_serializing_if = "Option::is_none")]
    pub application_unit_price: Option<MoneyType>,
    #[serde(rename = "ApplicationTotalSubsidy", skip_serializing_if = "Option::is_none")]
    pub application_total_subsidy: Option<MoneyType>,
    #[serde(rename = "DecidedAmount", skip_serializing_if = "Option::is_none")]
    pub decided_amount: Option<DecidedAmountType>,
    #[serde(rename = "DecidedAmountUnit", skip_serializing_if = "Option::is_none")]
    pub decided_amount_unit: Option<DecidedAmountUnitType>,
    #[serde(rename = "DecidedUnitPrice", skip_serializing_if = "Option::is_none")]
    pub decided_unit_price: Option<DecidedUnitPriceType>,
    #[serde(rename = "DecidedTotalSubsidy")]
    pub decided_total_subsidy: DecidedTotalSubsidyType,
    #[serde(rename = "Reasons", skip_serializing_if = "Option::is_none")]
    pub reasons: Option<ReasonsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ForestCentreDecision")]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonsType {
    #[serde(rename = "Reason")]
    pub reason: Vec<ReasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActDataType {
    #[serde(rename = "WorksDueDate")]
    pub works_due_date: DateType,
    #[serde(rename = "CompletionDeclarationDeliveryDueDate")]
    pub completion_declaration_delivery_due_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometriesType {
    #[serde(rename = "DecisionGeometry")]
    pub decision_geometry: Vec<DecisionGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionType {
    #[serde(rename = "ActionType")]
    pub action_type: String,
    #[serde(rename = "ActionDescription", skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String1000Type>,
    #[serde(rename = "ActionDate")]
    pub action_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "ObjectType")]
    pub object_type: DecisionGeometryObjectType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<ReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "PointLineAndPolygonGeometriesGroup")]
    pub point_line_and_polygon_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonType {
    #[serde(rename = "ReasonCode")]
    pub reason_code: String10Type,
    #[serde(rename = "ReasonDescription")]
    pub reason_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentType {
    #[serde(rename = "SubsidyArgumentText")]
    pub subsidy_argument_text: Vec<String5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: ForestCentreWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: ForestCentreWorkCodeType,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlerType {
    #[serde(rename = "@role")]
    pub role: String100Type,
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "CaseNumber")]
    pub case_number: FinancingActNumberType,
    #[serde(rename = "CaseDate")]
    pub case_date: DateType,
    #[serde(rename = "DecisionNumber")]
    pub decision_number: String100Type,
    #[serde(rename = "DecisionType")]
    pub decision_type: DecisionTypeType,
    #[serde(rename = "DecisionDate")]
    pub decision_date: DateType,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_reference: Option<String>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_reference: Option<String>,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_reference: Option<String>,
    #[serde(rename = "Acceptance")]
    pub acceptance: AcceptanceType,
    #[serde(rename = "CaseActions")]
    pub case_actions: CaseActionsType,
    #[serde(rename = "OriginalSender")]
    pub original_sender: ContactInformationType,
    #[serde(rename = "DecisionReceivers")]
    pub decision_receivers: DecisionReceiversType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<ContactInformationType>,
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstatesType,
    #[serde(rename = "DecisionGeometries", skip_serializing_if = "Option::is_none")]
    pub decision_geometries: Option<DecisionGeometriesType>,
    #[serde(rename = "Works")]
    pub works: WorksType,
    #[serde(rename = "SubsidyArgument", skip_serializing_if = "Option::is_none")]
    pub subsidy_argument: Option<SubsidyArgumentType>,
    #[serde(rename = "FinancingActData", skip_serializing_if = "Option::is_none")]
    pub financing_act_data: Option<FinancingActDataType>,
    #[serde(rename = "Justifications")]
    pub justifications: JustificationsType,
    #[serde(rename = "RectificationDemand")]
    pub rectification_demand: String5000Type,
    #[serde(rename = "DecisionHandlers")]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiversType {
    #[serde(rename = "DecisionReceiver")]
    pub decision_receiver: Vec<ContactInformationType>,
}

