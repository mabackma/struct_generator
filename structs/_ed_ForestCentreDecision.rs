#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlers {
    #[serde(flatten)]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActData {
    #[serde(flatten)]
    pub financing_act_data: FinancingActDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionType {
    #[serde(flatten)]
    pub decision_type: CoDecisionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Justification {
    #[serde(flatten)]
    pub justification: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationDeliveryDueDate {
    #[serde(flatten)]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometries {
    #[serde(flatten)]
    pub decision_geometries: DecisionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDescription {
    #[serde(flatten)]
    pub work_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Justifications {
    #[serde(flatten)]
    pub justifications: JustificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationUnitPrice {
    #[serde(flatten)]
    pub application_unit_price: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyZone {
    #[serde(flatten)]
    pub subsidy_zone: CoForestActAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandler {
    #[serde(flatten)]
    pub decision_handler: DecisionHandlerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyPercent {
    #[serde(flatten)]
    pub subsidy_percent: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometry {
    #[serde(flatten)]
    pub decision_geometry: DecisionGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksDueDate {
    #[serde(flatten)]
    pub works_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseAction {
    #[serde(flatten)]
    pub case_action: CaseActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDescription {
    #[serde(flatten)]
    pub action_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    #[serde(flatten)]
    pub action_type: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationAmount {
    #[serde(flatten)]
    pub application_amount: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionDate {
    #[serde(flatten)]
    pub decision_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDate {
    #[serde(flatten)]
    pub action_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentText {
    #[serde(flatten)]
    pub subsidy_argument_text: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiver {
    #[serde(flatten)]
    pub decision_receiver: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(flatten)]
    pub object_type: CoDecisionGeometryObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseDate {
    #[serde(flatten)]
    pub case_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgument {
    #[serde(flatten)]
    pub subsidy_argument: SubsidyArgumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RectificationDemand {
    #[serde(flatten)]
    pub rectification_demand: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationAmountUnit {
    #[serde(flatten)]
    pub application_amount_unit: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActions {
    #[serde(flatten)]
    pub case_actions: CaseActionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPrice {
    #[serde(flatten)]
    pub decided_unit_price: FccDecidedUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecision {
    #[serde(flatten)]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionNumber {
    #[serde(flatten)]
    pub decision_number: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceivers {
    #[serde(flatten)]
    pub decision_receivers: DecisionReceiversType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTotalSubsidy {
    #[serde(flatten)]
    pub application_total_subsidy: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reasons {
    #[serde(flatten)]
    pub reasons: ReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalSender {
    #[serde(flatten)]
    pub original_sender: CiContactInformationType,
}

