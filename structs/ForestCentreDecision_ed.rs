#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationAmount {
    #[serde(flatten)]
    pub application_amount: Decimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Works {
    #[serde(flatten)]
    pub works: WorksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(flatten)]
    pub object_type: DecisionGeometryObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionNumber {
    #[serde(flatten)]
    pub decision_number: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyZone {
    #[serde(flatten)]
    pub subsidy_zone: ForestActAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseDate {
    #[serde(flatten)]
    pub case_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonCode {
    #[serde(flatten)]
    pub reason_code: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTotalSubsidy {
    #[serde(flatten)]
    pub application_total_subsidy: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: FirstNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionDate {
    #[serde(flatten)]
    pub decision_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationDeliveryDueDate {
    #[serde(flatten)]
    pub completion_declaration_delivery_due_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometries {
    #[serde(flatten)]
    pub decision_geometries: DecisionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseAction {
    #[serde(flatten)]
    pub case_action: CaseActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    #[serde(flatten)]
    pub reason: ReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: ForestCentreWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDescription {
    #[serde(flatten)]
    pub action_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonDescription {
    #[serde(flatten)]
    pub reason_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActions {
    #[serde(flatten)]
    pub case_actions: CaseActionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmount {
    #[serde(flatten)]
    pub decided_amount: DecidedAmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnit {
    #[serde(flatten)]
    pub decided_amount_unit: DecidedAmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reasons {
    #[serde(flatten)]
    pub reasons: ReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyPercent {
    #[serde(flatten)]
    pub subsidy_percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDescription {
    #[serde(flatten)]
    pub work_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: RealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidy {
    #[serde(flatten)]
    pub subsidy: SubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentText {
    #[serde(flatten)]
    pub subsidy_argument_text: String5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescription {
    #[serde(flatten)]
    pub cost_type_description: CostTypeDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationAmountUnit {
    #[serde(flatten)]
    pub application_amount_unit: ForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDate {
    #[serde(flatten)]
    pub action_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlers {
    #[serde(flatten)]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Justification {
    #[serde(flatten)]
    pub justification: String5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acceptance {
    #[serde(flatten)]
    pub acceptance: AcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActData {
    #[serde(flatten)]
    pub financing_act_data: FinancingActDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentreDecisionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RectificationDemand {
    #[serde(flatten)]
    pub rectification_demand: String5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: ForestCentreWorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalSender {
    #[serde(flatten)]
    pub original_sender: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    #[serde(flatten)]
    pub action_type: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgument {
    #[serde(flatten)]
    pub subsidy_argument: SubsidyArgumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecision {
    #[serde(flatten)]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionType {
    #[serde(flatten)]
    pub decision_type: DecisionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: CostTypeType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandler {
    #[serde(flatten)]
    pub decision_handler: DecisionHandlerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiver {
    #[serde(flatten)]
    pub decision_receiver: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(flatten)]
    pub work: WorkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksDueDate {
    #[serde(flatten)]
    pub works_due_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Justifications {
    #[serde(flatten)]
    pub justifications: JustificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceivers {
    #[serde(flatten)]
    pub decision_receivers: DecisionReceiversType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometry {
    #[serde(flatten)]
    pub decision_geometry: DecisionGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseNumber {
    #[serde(flatten)]
    pub case_number: FinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationUnitPrice {
    #[serde(flatten)]
    pub application_unit_price: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPrice {
    #[serde(flatten)]
    pub decided_unit_price: DecidedUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidies {
    #[serde(flatten)]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidy {
    #[serde(flatten)]
    pub decided_total_subsidy: DecidedTotalSubsidyType,
}

