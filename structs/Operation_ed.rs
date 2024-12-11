#[derive(Debug, Serialize, Deserialize)]
pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: StemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYear {
    #[serde(flatten)]
    pub planning_year: PlanningYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: ControlDataOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: DifficultyClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalData {
    #[serde(flatten)]
    pub proposal_data: ProposalDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeData {
    #[serde(flatten)]
    pub operative_data: OperativeDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusData {
    #[serde(flatten)]
    pub prevention_fungus_of_the_genus_data: PreventionFungusOfTheGenusDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionData {
    #[serde(flatten)]
    pub completion_data: ExtendedCompletionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolume {
    #[serde(flatten)]
    pub cutting_volume: CuttingVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityDescription {
    #[serde(flatten)]
    pub seedling_condition_and_quality_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstance {
    #[serde(flatten)]
    pub prevention_substance: PreventionSubstanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: MachineAccessoryCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYear {
    #[serde(flatten)]
    pub min_proposal_year: MinProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsToWorkingSite {
    #[serde(flatten)]
    pub date_seedlings_to_working_site: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineDescription {
    #[serde(flatten)]
    pub machine_description: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingLotIdentifier {
    #[serde(flatten)]
    pub seedling_lot_identifier: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: IdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmountUnit {
    #[serde(flatten)]
    pub material_amount_unit: MaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Silviculture {
    #[serde(flatten)]
    pub silviculture: SilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: AssortmentNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChain {
    #[serde(flatten)]
    pub planned_operation_chain: PlannedOperationChainType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelated {
    #[serde(flatten)]
    pub cutting_related: CuttingRelatedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: MachineCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionCompleted {
    #[serde(flatten)]
    pub prevention_completed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCode {
    #[serde(flatten)]
    pub assortment_code: AssortmentCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCode {
    #[serde(flatten)]
    pub specification_code: SpecificationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: OperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cutting {
    #[serde(flatten)]
    pub cutting: CuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChains {
    #[serde(flatten)]
    pub planned_operation_chains: PlannedOperationChainsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceProductName {
    #[serde(flatten)]
    pub prevention_substance_product_name: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

