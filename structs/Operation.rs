#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: SpecificationCodeType,
    #[serde(rename = "SpecificationText", skip_serializing_if = "Option::is_none")]
    pub specification_text: Option<CoString2000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorsType {
    #[serde(rename = "CompletionActor")]
    pub completion_actor: Vec<CompletionActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataType {
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureTypeType {
    #[serde(flatten)]
    pub base: CoSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
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
pub struct ProposalDataType {
    #[serde(rename = "ProposalType")]
    pub proposal_type: ProposalTypeType,
    #[serde(rename = "PlanningYearAndOperationUrgencyGroup")]
    pub planning_year_and_operation_urgency_group: String,
    #[serde(rename = "TimeBetweenProposalYearsGroup")]
    pub time_between_proposal_years_group: String,
    #[serde(rename = "ProposalAndOriginalYearGroup")]
    pub proposal_and_original_year_group: String,
    #[serde(rename = "ProposalDate")]
    pub proposal_date: ProposalDateType,
    #[serde(rename = "ProposalAreaGroup", skip_serializing_if = "Option::is_none")]
    pub proposal_area_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDefType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@mainType")]
    pub main_type: MainTypeType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
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
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
    pub specifications: Option<SpecificationsType>,
    #[serde(rename = "Cutting")]
    pub cutting: CuttingType,
    #[serde(rename = "Silviculture")]
    pub silviculture: SilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeBetweenProposalYearsGroup {
    #[serde(rename = "MinProposalYear")]
    pub min_proposal_year: MinProposalYear,
    #[serde(rename = "MaxProposalYear")]
    pub max_proposal_year: MaxProposalYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAndOriginalYearGroup {
    #[serde(rename = "ProposalYear")]
    pub proposal_year: ProposalYear,
    #[serde(rename = "OriginalProposalYear", skip_serializing_if = "Option::is_none")]
    pub original_proposal_year: Option<OriginalProposalYear>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaType {
    #[serde(flatten)]
    pub base: CoDecimal4And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureExtraQualifierType {
    #[serde(flatten)]
    pub base: CoSilvicultureExtraQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingTypeType {
    #[serde(flatten)]
    pub base: CoCuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainsType {
    #[serde(rename = "PlannedOperationChain")]
    pub planned_operation_chain: Vec<PlannedOperationChainType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummaryType {
    #[serde(rename = "OperativeTreeSpeciesData")]
    pub operative_tree_species_data: Vec<TsTreeSpeciesDataType>,
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
pub struct CuttingType {
    #[serde(rename = "CuttingVolume", skip_serializing_if = "Option::is_none")]
    pub cutting_volume: Option<CuttingVolumeType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "CuttingIncome", skip_serializing_if = "Option::is_none")]
    pub cutting_income: Option<CuttingIncomeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedWideUnitType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelatedType {
    #[serde(flatten)]
    pub base: CoYesNoType,
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
pub struct OperationTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationNatureManagementSpecifierType {
    #[serde(flatten)]
    pub base: CoOperationNatureManagementSpecifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
    #[serde(flatten)]
    pub base: CoStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeType {
    #[serde(flatten)]
    pub base: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaGroup {
    #[serde(rename = "ProposalAreaPercent", skip_serializing_if = "Option::is_none")]
    pub proposal_area_percent: Option<ProposalAreaPercent>,
    #[serde(rename = "ProposalArea", skip_serializing_if = "Option::is_none")]
    pub proposal_area: Option<ProposalArea>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorType {
    #[serde(rename = "ActorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<CoIdStringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDateType {
    #[serde(rename = "@type")]
    pub r#type: CoDatePrecipionType,
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalTypeType {
    #[serde(flatten)]
    pub base: CoProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationsType {
    #[serde(rename = "Specification")]
    pub specification: Vec<SpecificationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(flatten)]
    pub base: CoMainTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeNameType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolumeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureType {
    #[serde(rename = "CuttingRelated", skip_serializing_if = "Option::is_none")]
    pub cutting_related: Option<CuttingRelatedType>,
    #[serde(rename = "Cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<CostType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCodeType {
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
pub struct TreeSpeciesType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearAndOperationUrgencyGroup {
    #[serde(rename = "PlanningYear")]
    pub planning_year: PlanningYear,
    #[serde(rename = "OperationUrgency")]
    pub operation_urgency: OperationUrgency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "AlternativeIdentifier")]
    pub alternative_identifier: AlternativeIdentifierType,
    #[serde(rename = "AlternativeName", skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<AlternativeNameType>,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityType {
    #[serde(flatten)]
    pub base: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncomeType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
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
pub struct OperationUrgencyType {
    #[serde(flatten)]
    pub base: CoOperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingExtraQualifierType {
    #[serde(flatten)]
    pub base: CoCuttingExtraQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingDirectingType {
    #[serde(flatten)]
    pub base: CoCuttingDirectingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonOperationExtraQualifierType {
    #[serde(flatten)]
    pub base: CoCommonOperationExtraQualifierType,
}

