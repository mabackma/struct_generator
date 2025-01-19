use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationAmount {
    #[serde(flatten)]
    pub application_amount: CoDecimal7And2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Justifications {
    #[serde(flatten)]
    pub justifications: JustificationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionGeometries {
    #[serde(flatten)]
    pub decision_geometries: DecisionGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyZone {
    #[serde(flatten)]
    pub subsidy_zone: CoForestActAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionDate {
    #[serde(flatten)]
    pub decision_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionHandler {
    #[serde(flatten)]
    pub decision_handler: DecisionHandlerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RectificationDemand {
    #[serde(flatten)]
    pub rectification_demand: CoString5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyArgumentText {
    #[serde(flatten)]
    pub subsidy_argument_text: CoString5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionReceivers {
    #[serde(flatten)]
    pub decision_receivers: DecisionReceiversType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalSender {
    #[serde(flatten)]
    pub original_sender: CiContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reasons {
    #[serde(flatten)]
    pub reasons: ReasonsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationAmountUnit {
    #[serde(flatten)]
    pub application_amount_unit: CoForestCentreUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseAction {
    #[serde(flatten)]
    pub case_action: CaseActionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionNumber {
    #[serde(flatten)]
    pub decision_number: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDescription {
    #[serde(flatten)]
    pub work_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationDeliveryDueDate {
    #[serde(flatten)]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationUnitPrice {
    #[serde(flatten)]
    pub application_unit_price: CoMoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksDueDate {
    #[serde(flatten)]
    pub works_due_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActData {
    #[serde(flatten)]
    pub financing_act_data: FinancingActDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionGeometry {
    #[serde(flatten)]
    pub decision_geometry: DecisionGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyArgument {
    #[serde(flatten)]
    pub subsidy_argument: SubsidyArgumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Justification {
    #[serde(flatten)]
    pub justification: CoString5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDate {
    #[serde(flatten)]
    pub action_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyPercent {
    #[serde(flatten)]
    pub subsidy_percent: CoPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionType {
    #[serde(flatten)]
    pub decision_type: CoDecisionTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationTotalSubsidy {
    #[serde(flatten)]
    pub application_total_subsidy: CoMoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedUnitPrice {
    #[serde(flatten)]
    pub decided_unit_price: FccDecidedUnitPriceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDescription {
    #[serde(flatten)]
    pub action_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectType {
    #[serde(flatten)]
    pub object_type: CoDecisionGeometryObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionType {
    #[serde(flatten)]
    pub action_type: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionReceiver {
    #[serde(flatten)]
    pub decision_receiver: CiContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseDate {
    #[serde(flatten)]
    pub case_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionHandlers {
    #[serde(flatten)]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreDecision {
    #[serde(flatten)]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseActions {
    #[serde(flatten)]
    pub case_actions: CaseActionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActDataType {
    #[serde(rename = "WorksDueDate")]
    pub works_due_date: CoDateType,
    #[serde(rename = "CompletionDeclarationDeliveryDueDate")]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlerType {
    #[serde(rename = "@role")]
    pub role: String100Type,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<CoReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionContactInformationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FirstName")]
    pub first_name: CiFirstNameType,
    #[serde(rename = "LastName")]
    pub last_name: CiLastNameType,
    #[serde(rename = "PersonOrganizationName")]
    pub person_organization_name: CiPersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlersType {
    #[serde(rename = "DecisionHandler")]
    pub decision_handler: Vec<DecisionHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonType {
    #[serde(rename = "ReasonCode")]
    pub reason_code: CoString10Type,
    #[serde(rename = "ReasonDescription")]
    pub reason_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentreDecision")]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "ObjectType")]
    pub object_type: CoDecisionGeometryObjectType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<CoReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "PointLineAndPolygonGeometriesGroup")]
    pub gdt_point_line_and_polygon_geometries_group: GdtPointLineAndPolygonGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentType {
    #[serde(rename = "SubsidyArgumentText")]
    pub subsidy_argument_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: CoForestCentreWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: CoForestCentreWorkCodeType,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustificationsType {
    #[serde(rename = "Justification")]
    pub justification: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiversType {
    #[serde(rename = "DecisionReceiver")]
    pub decision_receiver: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType2,
    #[serde(rename = "SubsidyZone")]
    pub subsidy_zone: CoForestActAreaType,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionsType {
    #[serde(rename = "CaseAction")]
    pub case_action: Vec<CaseActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionType {
    #[serde(rename = "ActionType")]
    pub action_type: String,
    #[serde(rename = "ActionDescription", skip_serializing_if = "Option::is_none")]
    pub action_description: Option<CoString1000Type>,
    #[serde(rename = "ActionDate")]
    pub action_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometriesType {
    #[serde(rename = "DecisionGeometry")]
    pub decision_geometry: Vec<DecisionGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "CaseNumber")]
    pub case_number: FccFinancingActNumberType,
    #[serde(rename = "CaseDate")]
    pub case_date: CoDateType,
    #[serde(rename = "DecisionNumber")]
    pub decision_number: CoString100Type,
    #[serde(rename = "DecisionType")]
    pub decision_type: CoDecisionTypeType,
    #[serde(rename = "DecisionDate")]
    pub decision_date: CoDateType,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<FccCustomerReference>,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_application_reference: Option<FccFinancingActApplicationReference>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<FccCompletionDeclarationReference>,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_use_declaration_reference: Option<FccForestUseDeclarationReference>,
    #[serde(rename = "Acceptance")]
    pub acceptance: CoAcceptanceType,
    #[serde(rename = "CaseActions")]
    pub case_actions: CaseActionsType,
    #[serde(rename = "OriginalSender")]
    pub original_sender: CiContactInformationType,
    #[serde(rename = "DecisionReceivers")]
    pub decision_receivers: DecisionReceiversType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<CiContactInformationType>,
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
    pub rectification_demand: CoString5000Type,
    #[serde(rename = "DecisionHandlers")]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<CoReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "CostType")]
    pub cost_type: FccCostTypeType2,
    #[serde(rename = "CostTypeDescription")]
    pub cost_type_description: FccCostTypeDescriptionType,
    #[serde(rename = "SubsidyPercent", skip_serializing_if = "Option::is_none")]
    pub subsidy_percent: Option<CoPercentType>,
    #[serde(rename = "ApplicationAmount", skip_serializing_if = "Option::is_none")]
    pub application_amount: Option<CoDecimal7And2Type>,
    #[serde(rename = "ApplicationAmountUnit", skip_serializing_if = "Option::is_none")]
    pub application_amount_unit: Option<CoForestCentreUnitType>,
    #[serde(rename = "ApplicationUnitPrice", skip_serializing_if = "Option::is_none")]
    pub application_unit_price: Option<CoMoneyType>,
    #[serde(rename = "ApplicationTotalSubsidy", skip_serializing_if = "Option::is_none")]
    pub application_total_subsidy: Option<CoMoneyType>,
    #[serde(rename = "DecidedAmount", skip_serializing_if = "Option::is_none")]
    pub decided_amount: Option<FccDecidedAmountType>,
    #[serde(rename = "DecidedAmountUnit", skip_serializing_if = "Option::is_none")]
    pub decided_amount_unit: Option<FccDecidedAmountUnitType>,
    #[serde(rename = "DecidedUnitPrice", skip_serializing_if = "Option::is_none")]
    pub decided_unit_price: Option<FccDecidedUnitPriceType>,
    #[serde(rename = "DecidedTotalSubsidy")]
    pub decided_total_subsidy: FccDecidedTotalSubsidyType,
    #[serde(rename = "Reasons", skip_serializing_if = "Option::is_none")]
    pub reasons: Option<ReasonsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonsType {
    #[serde(rename = "Reason")]
    pub reason: Vec<ReasonType>,
}

