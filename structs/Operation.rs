use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
--pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ProposalData {
    #[serde(flatten)]
    pub proposal_data: ProposalDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CompletionData {
    #[serde(flatten)]
    pub completion_data: ExtendedCompletionDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct DateSeedlingsToWorkingSite {
    #[serde(flatten)]
    pub date_seedlings_to_working_site: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: CoDifficultyClassType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PreventionCompleted {
    #[serde(flatten)]
    pub prevention_completed: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: CoControlDataOperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: CoString2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct SeedlingLotIdentifier {
    #[serde(flatten)]
    pub seedling_lot_identifier: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: CoBasalAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PreventionSubstance {
    #[serde(flatten)]
    pub prevention_substance: CoPreventionSubstanceType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: CoDecimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PlannedOperationChains {
    #[serde(flatten)]
    pub planned_operation_chains: PlannedOperationChainsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MaterialAmountUnit {
    #[serde(flatten)]
    pub material_amount_unit: CoMaterialUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: CoString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct SeedlingConditionAndQualityDescription {
    #[serde(flatten)]
    pub seedling_condition_and_quality_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MachineDescription {
    #[serde(flatten)]
    pub machine_description: CoString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperativeData {
    #[serde(flatten)]
    pub operative_data: OperativeDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CuttingVolume {
    #[serde(flatten)]
    pub cutting_volume: CuttingVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: CoMachineCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: CoTreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Status {
    #[serde(flatten)]
    pub status: CoOperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PlanningYear {
    #[serde(flatten)]
    pub planning_year: PlanningYearType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MinProposalYear {
    #[serde(flatten)]
    pub min_proposal_year: MinProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TsTreeSpeciesDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PreventionFungusOfTheGenusData {
    #[serde(flatten)]
    pub prevention_fungus_of_the_genus_data: PreventionFungusOfTheGenusDataType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: CoMachineAccessoryCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct SpecificationCode {
    #[serde(flatten)]
    pub specification_code: SpecificationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Silviculture {
    #[serde(flatten)]
    pub silviculture: SilvicultureType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: CoIdStringType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CuttingRelated {
    #[serde(flatten)]
    pub cutting_related: CuttingRelatedType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: CoYearType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PreventionSubstanceProductName {
    #[serde(flatten)]
    pub prevention_substance_product_name: CoString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PlannedOperationChain {
    #[serde(flatten)]
    pub planned_operation_chain: PlannedOperationChainType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDataType {
    #[serde(rename = "ProposalType")]
    pub proposal_type: ProposalTypeType,
    #[serde(rename = "PlanningYearAndOperationUrgencyGroup")]
    pub planning_year_and_operation_urgency_group: PlanningYearAndOperationUrgencyGroup,
    #[serde(rename = "TimeBetweenProposalYearsGroup")]
    pub time_between_proposal_years_group: TimeBetweenProposalYearsGroup,
    #[serde(rename = "ProposalAndOriginalYearGroup")]
    pub proposal_and_original_year_group: ProposalAndOriginalYearGroup,
    #[serde(rename = "ProposalDate")]
    pub proposal_date: ProposalDateType,
    #[serde(rename = "ProposalAreaGroup", skip_serializing_if = "Option::is_none")]
    pub proposal_area_group: Option<ProposalAreaGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingTypeType {
    #[serde(flatten)]
    pub base: CoCuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachineType {
    #[serde(rename = "MachineCode", skip_serializing_if = "Option::is_none")]
    pub machine_code: Option<CoMachineCodeType>,
    #[serde(rename = "MachineDescription", skip_serializing_if = "Option::is_none")]
    pub machine_description: Option<CoString500Type>,
    #[serde(rename = "MachineAccessoryCode", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_code: Option<CoMachineAccessoryCodeType>,
    #[serde(rename = "MachineAccessoryDescription", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_description: Option<CoString500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataType {
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeNameType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureTypeType {
    #[serde(flatten)]
    pub base: CoSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelatedType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(flatten)]
    pub base: CoMainTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorType {
    #[serde(rename = "ActorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<CoIdStringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorsType {
    #[serde(rename = "CompletionActor")]
    pub completion_actor: Vec<CompletionActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusDataType {
    #[serde(rename = "PreventionCompleted", skip_serializing_if = "Option::is_none")]
    pub prevention_completed: Option<CoYesNoType>,
    #[serde(rename = "PreventionSubstance", skip_serializing_if = "Option::is_none")]
    pub prevention_substance: Option<CoPreventionSubstanceType>,
    #[serde(rename = "PreventionSubstanceProductName", skip_serializing_if = "Option::is_none")]
    pub prevention_substance_product_name: Option<CoString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedCompletionDataType {
    #[serde(rename = "OperationStatus", skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<CoControlDataOperationStatusType>,
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "AlternativeIdentifier")]
    pub alternative_identifier: AlternativeIdentifierType,
    #[serde(rename = "AlternativeName", skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<AlternativeNameType>,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonOperationExtraQualifierType {
    #[serde(flatten)]
    pub base: CoCommonOperationExtraQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: StemTypeType,
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<CoAssortmentCodeType>,
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<CoAssortmentNameType>,
    #[serde(rename = "AssortmentPercent")]
    pub assortment_percent: AssortmentPercentType,
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaType {
    #[serde(flatten)]
    pub base: CoDecimal4And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAndOriginalYearGroup {
    #[serde(rename = "ProposalYear")]
    pub proposal_year: ProposalYear,
    #[serde(rename = "OriginalProposalYear", skip_serializing_if = "Option::is_none")]
    pub original_proposal_year: Option<OriginalProposalYear>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureType {
    #[serde(rename = "CuttingRelated", skip_serializing_if = "Option::is_none")]
    pub cutting_related: Option<CuttingRelatedType>,
    #[serde(rename = "Cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<CostType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolumeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityType {
    #[serde(flatten)]
    pub base: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummaryType {
    #[serde(rename = "OperativeTreeSpeciesData")]
    pub operative_tree_species_data: Vec<TsTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationNatureManagementSpecifierType {
    #[serde(flatten)]
    pub base: CoOperationNatureManagementSpecifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationsType {
    #[serde(rename = "Specification")]
    pub specification: Vec<SpecificationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationDataType {
    #[serde(rename = "SeedlingLotIdentifier", skip_serializing_if = "Option::is_none")]
    pub seedling_lot_identifier: Option<CoString100Type>,
    #[serde(rename = "SeedlingConditionAndQuality", skip_serializing_if = "Option::is_none")]
    pub seedling_condition_and_quality: Option<SeedlingConditionAndQualityType>,
    #[serde(rename = "SeedlingConditionAndQualityDescription", skip_serializing_if = "Option::is_none")]
    pub seedling_condition_and_quality_description: Option<CoString1000Type>,
    #[serde(rename = "AmountSeedlingsToPlant", skip_serializing_if = "Option::is_none")]
    pub amount_seedlings_to_plant: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "DateSeedlingsExitFromTreeNursery", skip_serializing_if = "Option::is_none")]
    pub date_seedlings_exit_from_tree_nursery: Option<CoDateType>,
    #[serde(rename = "DateSeedlingsToWorkingSite", skip_serializing_if = "Option::is_none")]
    pub date_seedlings_to_working_site: Option<CoDateType>,
    #[serde(rename = "DateSeedlingsPlanted", skip_serializing_if = "Option::is_none")]
    pub date_seedlings_planted: Option<CoDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingType {
    #[serde(rename = "CuttingVolume", skip_serializing_if = "Option::is_none")]
    pub cutting_volume: Option<CuttingVolumeType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "CuttingIncome", skip_serializing_if = "Option::is_none")]
    pub cutting_income: Option<CuttingIncomeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeType {
    #[serde(flatten)]
    pub base: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDateType {
    #[serde(rename = "@type")]
    pub r#type: DatePrecipionType,
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeBetweenProposalYearsGroup {
    #[serde(rename = "MaxProposalYear")]
    pub max_proposal_year: MaxProposalYear,
    #[serde(rename = "MinProposalYear")]
    pub min_proposal_year: MinProposalYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgencyType {
    #[serde(flatten)]
    pub base: CoOperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
    #[serde(flatten)]
    pub base: CoStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeDataType {
    #[serde(rename = "OperationTreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub operation_tree_species_summary: Option<OperationTreeSpeciesSummaryType>,
    #[serde(rename = "GrowingTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub growing_tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "MaterialAmount", skip_serializing_if = "Option::is_none")]
    pub material_amount: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "MaterialAmountUnit", skip_serializing_if = "Option::is_none")]
    pub material_amount_unit: Option<CoMaterialUnitType>,
    #[serde(rename = "TargetStemCount", skip_serializing_if = "Option::is_none")]
    pub target_stem_count: Option<CoStemCountType>,
    #[serde(rename = "TargetBasalArea", skip_serializing_if = "Option::is_none")]
    pub target_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "TargetAmount", skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<AmountType>,
    #[serde(rename = "TargetAmountUnit", skip_serializing_if = "Option::is_none")]
    pub target_amount_unit: Option<ExtendedWideUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingDirectingType {
    #[serde(flatten)]
    pub base: CoCuttingDirectingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearAndOperationUrgencyGroup {
    #[serde(rename = "PlanningYear")]
    pub planning_year: PlanningYear,
    #[serde(rename = "OperationUrgency")]
    pub operation_urgency: OperationUrgency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingExtraQualifierType {
    #[serde(flatten)]
    pub base: CoCuttingExtraQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainsType {
    #[serde(rename = "PlannedOperationChain")]
    pub planned_operation_chain: Vec<PlannedOperationChainType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: SpecificationCodeType,
    #[serde(rename = "SpecificationText", skip_serializing_if = "Option::is_none")]
    pub specification_text: Option<CoString2000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncomeType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalTypeType {
    #[serde(flatten)]
    pub base: CoProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureExtraQualifierType {
    #[serde(flatten)]
    pub base: CoSilvicultureExtraQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaGroup {
    #[serde(rename = "ProposalArea", skip_serializing_if = "Option::is_none")]
    pub proposal_area: Option<ProposalArea>,
    #[serde(rename = "ProposalAreaPercent", skip_serializing_if = "Option::is_none")]
    pub proposal_area_percent: Option<ProposalAreaPercent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDefType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@mainType")]
    pub main_type: MainTypeType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
    pub proposal_data: Option<ProposalDataType>,
    #[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
    pub completion_data: Option<ExtendedCompletionDataType>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoOperationStatusType>,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<CoDifficultyClassType>,
    #[serde(rename = "OperativeData", skip_serializing_if = "Option::is_none")]
    pub operative_data: Option<OperativeDataType>,
    #[serde(rename = "OperationInfo", skip_serializing_if = "Option::is_none")]
    pub operation_info: Option<OperationInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<CoDataSource>,
    #[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
    pub specifications: Option<SpecificationsType>,
    #[serde(rename = "Cutting")]
    pub cutting: CuttingType,
    #[serde(rename = "Silviculture")]
    pub silviculture: SilvicultureType,
}

