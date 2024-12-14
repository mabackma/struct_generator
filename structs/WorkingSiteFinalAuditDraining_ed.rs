#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: MunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShape {
    #[serde(flatten)]
    pub road_structure_shape: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelated {
    #[serde(flatten)]
    pub cutting_related: CuttingRelatedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: StoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingSiteCount {
    #[serde(flatten)]
    pub planting_site_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCode {
    #[serde(flatten)]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolume {
    #[serde(flatten)]
    pub forwarded_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlannerText {
    #[serde(flatten)]
    pub feedback_for_planner_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingByMachine {
    #[serde(flatten)]
    pub cutting_by_machine: VirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: RestrictionCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: YritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocationText {
    #[serde(flatten)]
    pub ditch_cleaning_break_location_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffect {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(flatten)]
    pub count: CountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItem {
    #[serde(flatten)]
    pub tree_list_item: TreeListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: RestrictionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScapingText {
    #[serde(flatten)]
    pub land_scaping_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanner {
    #[serde(flatten)]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: CallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedStartDate {
    #[serde(flatten)]
    pub estimated_start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanStumpDiameter {
    #[serde(flatten)]
    pub mean_stump_diameter: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVolumeSummary {
    #[serde(flatten)]
    pub stand_volume_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClass {
    #[serde(flatten)]
    pub small_wood_removal_class: SmallWoodRemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceNumber {
    #[serde(flatten)]
    pub insurance_number: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractText {
    #[serde(flatten)]
    pub contract_text: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePartNumber {
    #[serde(flatten)]
    pub base_part_number: VirtaPartNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationSign {
    #[serde(flatten)]
    pub dis_qualification_sign: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxDebt {
    #[serde(flatten)]
    pub tax_debt: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperations {
    #[serde(flatten)]
    pub habitat_operations: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocation {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(flatten)]
    pub product: ProductType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSource {
    #[serde(flatten)]
    pub damage_source: FeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreement {
    #[serde(flatten)]
    pub collective_agreement: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StandBasicDataWithGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: VirtaGroundManipulationMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAuditText {
    #[serde(flatten)]
    pub fertilization_total_audit_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FinancingActCompletionDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructedText {
    #[serde(flatten)]
    pub airdrome_as_instructed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowth {
    #[serde(flatten)]
    pub volume_growth: VolumeGrowthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometry {
    #[serde(flatten)]
    pub working_site_geometry: LocatedSpecialFeature2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetails {
    #[serde(flatten)]
    pub offer_working_site_details: OfferWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathWidth {
    #[serde(flatten)]
    pub stand_vehicle_path_width: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepresentativePerson {
    #[serde(flatten)]
    pub representative_person: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: JohnsonSBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometries {
    #[serde(flatten)]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVolume {
    #[serde(flatten)]
    pub total_volume: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystem {
    #[serde(flatten)]
    pub wide_certification_system: WideCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchPulp {
    #[serde(flatten)]
    pub birch_pulp: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: ShapeDeltaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: SahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceLog {
    #[serde(flatten)]
    pub spruce_log: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: OperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperation {
    #[serde(flatten)]
    pub statistics_operation: StatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainageText {
    #[serde(flatten)]
    pub road_structure_drainage_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionAction {
    #[serde(flatten)]
    pub water_protection_action: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(flatten)]
    pub category: ImageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: AssortmentNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilImprovementEvaluation {
    #[serde(flatten)]
    pub soil_improvement_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    #[serde(flatten)]
    pub tree: TreeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRows {
    #[serde(flatten)]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchLog {
    #[serde(flatten)]
    pub birch_log: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrassControlEvaluation {
    #[serde(flatten)]
    pub grass_control_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: VirtaTreeDecimalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentatives {
    #[serde(flatten)]
    pub working_representatives: WorkingRepresentativesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShapeText {
    #[serde(flatten)]
    pub road_structure_shape_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationTextInformation {
    #[serde(flatten)]
    pub declaration_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationDate {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeed {
    #[serde(flatten)]
    pub cutting_area_preclearing_need: CuttingAreaPreclearingNeedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethod {
    #[serde(flatten)]
    pub used_pricing_method: UsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationType {
    #[serde(flatten)]
    pub reported_statistics_operation_type: ReportedStatisticsOperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: KieliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status3 {
    #[serde(flatten)]
    pub status3: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: HuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountAccounted {
    #[serde(flatten)]
    pub amount_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registered {
    #[serde(flatten)]
    pub registered: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorWrittenAgreement {
    #[serde(flatten)]
    pub sub_contractor_written_agreement: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWithSeedlings {
    #[serde(flatten)]
    pub stocking_with_seedlings: VirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasons {
    #[serde(flatten)]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasured {
    #[serde(flatten)]
    pub fertilizer_volume_measured: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCode {
    #[serde(flatten)]
    pub assortment_code: AssortmentCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeWrittenAgreement {
    #[serde(flatten)]
    pub employee_written_agreement: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrectHeightStumps {
    #[serde(flatten)]
    pub correct_height_stumps: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: VirtaHabitatSurvivingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSilverBirch {
    #[serde(flatten)]
    pub basal_area_silver_birch: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: DistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machines {
    #[serde(flatten)]
    pub machines: MachinesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroup {
    #[serde(flatten)]
    pub forest_owner_group: ForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperation {
    #[serde(flatten)]
    pub declaration_regeneration_operation: DeclarationRegenerationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjects {
    #[serde(flatten)]
    pub financing_act_application_other_subjects: FinancingActApplicationOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystems {
    #[serde(flatten)]
    pub quality_systems: QualitySystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperation {
    #[serde(flatten)]
    pub declaration_soil_preparation_operation: DeclarationSoilPreparationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStand {
    #[serde(flatten)]
    pub financing_act_application_stand: FinancingActApplicationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDone {
    #[serde(flatten)]
    pub cleaning_breaks_done: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: MeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrderText {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSpruce {
    #[serde(flatten)]
    pub mean_diameter_spruce: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYear {
    #[serde(flatten)]
    pub planning_year: PlanningYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCode {
    #[serde(flatten)]
    pub additional_code: AdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescription {
    #[serde(flatten)]
    pub document_description: DocumentDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: SpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMin {
    #[serde(flatten)]
    pub length_min: LengthMinType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Silviculture {
    #[serde(flatten)]
    pub silviculture: SilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_height_other_tree_species: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingContract {
    #[serde(flatten)]
    pub working_contract: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionCategory {
    #[serde(flatten)]
    pub payment_transaction_category: MoneyTransactionCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPart {
    #[serde(flatten)]
    pub target_part: TargetPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: ValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidy {
    #[serde(flatten)]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: UlkomaaHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: MainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingActionText {
    #[serde(flatten)]
    pub erosion_blocking_action_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: Decimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: ForestCentreDataMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaledMass {
    #[serde(flatten)]
    pub scaled_mass: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPrice {
    #[serde(flatten)]
    pub total_price: TotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStands {
    #[serde(flatten)]
    pub financing_act_completion_stands: FinancingActCompletionStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(flatten)]
    pub attachment: AttachmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpsCount {
    #[serde(flatten)]
    pub scalps_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCutting {
    #[serde(flatten)]
    pub clear_cutting: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationReference {
    #[serde(flatten)]
    pub financing_act_application_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionalityText {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceList {
    #[serde(flatten)]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: RegisterUnitIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: VirtaStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MineralSoilLayer {
    #[serde(flatten)]
    pub mineral_soil_layer: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: MeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfo {
    #[serde(flatten)]
    pub stand_wood_trade_info: StandWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    #[serde(flatten)]
    pub r#type: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasalAreaSummary {
    #[serde(flatten)]
    pub stand_basal_area_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsActive {
    #[serde(flatten)]
    pub is_active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: GammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: KuudesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousSameAreaMooseDamageCompensationYear {
    #[serde(flatten)]
    pub previous_same_area_moose_damage_compensation_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: HenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: SyntymaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseMode {
    #[serde(flatten)]
    pub purchase_mode: PurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(flatten)]
    pub data: hexBinary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enddate {
    #[serde(flatten)]
    pub enddate: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandTextInformation {
    #[serde(flatten)]
    pub declaration_stand_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamageCount {
    #[serde(flatten)]
    pub root_damage_count: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: MachineAccessoryCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: KuolemaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomass {
    #[serde(flatten)]
    pub branch_biomass: BranchBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAmount {
    #[serde(flatten)]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsRightsOwnerRepresentative {
    #[serde(flatten)]
    pub cuttings_rights_owner_representative: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(flatten)]
    pub value: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentative {
    #[serde(flatten)]
    pub working_representative: WorkingRepresentativeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometries {
    #[serde(flatten)]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biomass {
    #[serde(flatten)]
    pub biomass: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationScope {
    #[serde(flatten)]
    pub cultivation_scope: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub financing_act_application_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate {
    #[serde(flatten)]
    pub tree_stand_data_date: TreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructedText {
    #[serde(flatten)]
    pub drain_storage_as_instructed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    #[serde(flatten)]
    pub reason: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstate {
    #[serde(flatten)]
    pub payee_and_real_estate: PayeeAndRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealization {
    #[serde(flatten)]
    pub object_realization: ObjectRealizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDate {
    #[serde(flatten)]
    pub contract_ending_date: ContractEndingDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClass {
    #[serde(flatten)]
    pub assortment_class: AssortmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: StandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractor {
    #[serde(flatten)]
    pub sub_contractor: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: UnitNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifier {
    #[serde(flatten)]
    pub forest_damage_qualifier: ForestDamageQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingAction {
    #[serde(flatten)]
    pub erosion_blocking_action: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountPine {
    #[serde(flatten)]
    pub stem_count_pine: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurpose {
    #[serde(flatten)]
    pub cutting_purpose: CuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    #[serde(flatten)]
    pub email: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStand {
    #[serde(flatten)]
    pub financing_act_completion_stand: FinancingActCompletionStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRow {
    #[serde(flatten)]
    pub operation_row: OperationRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: AlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantEvaluation {
    #[serde(flatten)]
    pub plant_evaluation: VirtaPlantEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentage {
    #[serde(flatten)]
    pub dis_qualification_percentage: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: FeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub machine: MachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractors {
    #[serde(flatten)]
    pub sub_contractors: SubContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalAccidentInsurance {
    #[serde(flatten)]
    pub legal_accident_insurance: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructed {
    #[serde(flatten)]
    pub stump_cutting_as_instructed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlanner {
    #[serde(flatten)]
    pub feedback_for_planner: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeData {
    #[serde(flatten)]
    pub operative_data: OperativeDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubCategory {
    #[serde(flatten)]
    pub sub_category: ImageSubCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationCommitment {
    #[serde(flatten)]
    pub declaration_regeneration_commitment: RegenerationCommitmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamageCount {
    #[serde(flatten)]
    pub stem_damage_count: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase2youngCropCount {
    #[serde(flatten)]
    pub phase2young_crop_count: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitable {
    #[serde(flatten)]
    pub stump_lifting_suitable: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallation {
    #[serde(flatten)]
    pub pipe_installation: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstate {
    #[serde(flatten)]
    pub financing_act_real_estate: FinancingActRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocation {
    #[serde(flatten)]
    pub excavation_soil_location: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calibration {
    #[serde(flatten)]
    pub calibration: CalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationDate {
    #[serde(flatten)]
    pub calibration_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumber {
    #[serde(flatten)]
    pub stratum_number: StratumNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeName {
    #[serde(flatten)]
    pub whole_name: WholeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: ValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallationText {
    #[serde(flatten)]
    pub pipe_installation_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipType {
    #[serde(flatten)]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidiness {
    #[serde(flatten)]
    pub stump_tidiness: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: RoadUsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountTarget {
    #[serde(flatten)]
    pub stem_count_target: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkQuality {
    #[serde(flatten)]
    pub work_quality: VirtaWorkQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaHardWood {
    #[serde(flatten)]
    pub basal_area_hard_wood: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionality {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSites {
    #[serde(flatten)]
    pub offer_working_sites: OfferWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerReference {
    #[serde(flatten)]
    pub customer_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: AttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoles {
    #[serde(flatten)]
    pub user_roles: UserRolesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratum {
    #[serde(flatten)]
    pub stem_distribution_stratum: StemDistributionStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDate {
    #[serde(flatten)]
    pub image_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffectText {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYear {
    #[serde(flatten)]
    pub min_proposal_year: MinProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: ViidesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlan {
    #[serde(flatten)]
    pub include_payment_plan: IncludePaymentPlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: MunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningDepth {
    #[serde(flatten)]
    pub soil_conditioning_depth: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: ShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A1 {
    #[serde(flatten)]
    pub a1: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightPine {
    #[serde(flatten)]
    pub mean_height_pine: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonReference {
    #[serde(flatten)]
    pub declaration_polygon_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstablishedPartNumber {
    #[serde(flatten)]
    pub established_part_number: VirtaPartNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActors {
    #[serde(flatten)]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperation {
    #[serde(flatten)]
    pub habitat_operation: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationType {
    #[serde(flatten)]
    pub fertilization_type: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowedText {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityCode {
    #[serde(flatten)]
    pub nationality_code: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: SilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatAdvertisement {
    #[serde(flatten)]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BIC {
    #[serde(flatten)]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageClass {
    #[serde(flatten)]
    pub damage_class: VirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(flatten)]
    pub review: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct E101 {
    #[serde(flatten)]
    pub e101: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedPlantEvaluation {
    #[serde(flatten)]
    pub seed_plant_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSiteCountSummary {
    #[serde(flatten)]
    pub plant_site_count_summary: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: VirtaExceptionalPermitForHandlingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedVolume {
    #[serde(flatten)]
    pub planned_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: FirstNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: IkaluokkaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferDate {
    #[serde(flatten)]
    pub call_for_offer_date: CallForOfferDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    #[serde(flatten)]
    pub file_type: FileTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cutting {
    #[serde(flatten)]
    pub cutting: CuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumber {
    #[serde(flatten)]
    pub processing_area_number: ProcessingAreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: ValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructedText {
    #[serde(flatten)]
    pub stump_cutting_as_instructed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingValue {
    #[serde(flatten)]
    pub cutting_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: WorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Languages {
    #[serde(flatten)]
    pub languages: LanguagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICEName {
    #[serde(flatten)]
    pub i_c_e_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryStatus {
    #[serde(flatten)]
    pub geometry_status: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationMainTreeSpecies {
    #[serde(flatten)]
    pub declaration_main_tree_species: TreeSpeciesConciseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMeanDiameter {
    #[serde(flatten)]
    pub stub_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinePulp {
    #[serde(flatten)]
    pub pine_pulp: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamage {
    #[serde(flatten)]
    pub previous_moose_damage: PreviousMooseDamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystem {
    #[serde(flatten)]
    pub quality_system: QualitySystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orientation {
    #[serde(flatten)]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationType {
    #[serde(flatten)]
    pub wood_lot_information_type: WoodLotInformationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: HuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: PostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDate {
    #[serde(flatten)]
    pub working_site_plan_date: WorkingSitePlanDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalData {
    #[serde(flatten)]
    pub proposal_data: ProposalDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: HuoltosuhdeTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSites {
    #[serde(flatten)]
    pub contract_working_sites: ContractWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: FeatureAdditionalInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPoint {
    #[serde(flatten)]
    pub supply_point: SupplyPointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: MaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: PankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDocument {
    #[serde(flatten)]
    pub power_of_attorney_document: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub target: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TooHeightStumps {
    #[serde(flatten)]
    pub too_height_stumps: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: BaseRealEstateType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: VirtaTreeDecimalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstates {
    #[serde(flatten)]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestReference {
    #[serde(flatten)]
    pub request_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandExtraInfo {
    #[serde(flatten)]
    pub stand_extra_info: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChains {
    #[serde(flatten)]
    pub planned_operation_chains: PlannedOperationChainsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountInSpot {
    #[serde(flatten)]
    pub amount_in_spot: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlaceText {
    #[serde(flatten)]
    pub turning_place_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbrokenText {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegister {
    #[serde(flatten)]
    pub employer_register: EmployerRegisterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: HeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetId {
    #[serde(flatten)]
    pub target_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStands {
    #[serde(flatten)]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidinessText {
    #[serde(flatten)]
    pub stump_tidiness_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: ControlDataOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpStemCount {
    #[serde(flatten)]
    pub stump_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: HenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: FinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: LajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub user_role: UserRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: AlayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationAdjustment {
    #[serde(flatten)]
    pub calibration_adjustment: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClass {
    #[serde(flatten)]
    pub height_class: HeightClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationMunicipality {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_municipality: MunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnoverMoundsCount {
    #[serde(flatten)]
    pub turnover_mounds_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometry {
    #[serde(flatten)]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalVolume {
    #[serde(flatten)]
    pub small_wood_removal_volume: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4damageCount {
    #[serde(flatten)]
    pub class4damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstates {
    #[serde(flatten)]
    pub financing_act_real_estates: FinancingActRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: TreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummary {
    #[serde(flatten)]
    pub tree_species_summary: TreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: LocatedSpecialFeature3Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureText {
    #[serde(flatten)]
    pub road_structure_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: IdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(flatten)]
    pub authorization: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalName {
    #[serde(flatten)]
    pub additional_name: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticed {
    #[serde(flatten)]
    pub environment_cleanliness_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationType {
    #[serde(flatten)]
    pub regeneration_type: VirtaRegenerationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffer {
    #[serde(flatten)]
    pub call_for_offer: CallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: GroupNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: VirtaCultivationMaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamages {
    #[serde(flatten)]
    pub root_damages: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Startdate {
    #[serde(flatten)]
    pub startdate: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: CertificationSystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumber {
    #[serde(flatten)]
    pub financing_act_number: FinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publicity {
    #[serde(flatten)]
    pub publicity: PublicityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedStemCount {
    #[serde(flatten)]
    pub planned_stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason2 {
    #[serde(flatten)]
    pub reason2: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestroyedCuttingValue {
    #[serde(flatten)]
    pub destroyed_cutting_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffers {
    #[serde(flatten)]
    pub related_call_for_offers: RelatedCallForOffersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: EmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamages {
    #[serde(flatten)]
    pub stem_damages: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingArea {
    #[serde(flatten)]
    pub processing_area: ProcessingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperation {
    #[serde(flatten)]
    pub declaration_other_operation: DeclarationOtherOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverallTotalSubsidy {
    #[serde(flatten)]
    pub overall_total_subsidy: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineDescription {
    #[serde(flatten)]
    pub machine_description: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonText {
    #[serde(flatten)]
    pub dis_qualification_reason_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandReference {
    #[serde(flatten)]
    pub declaration_stand_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestType {
    #[serde(flatten)]
    pub forest_type: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPayment {
    #[serde(flatten)]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photographer {
    #[serde(flatten)]
    pub photographer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrix {
    #[serde(flatten)]
    pub price_matrix: PriceMatrixType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReference {
    #[serde(flatten)]
    pub subsidy_applier_reference: SubsidyApplierReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreDebtCollectionRegister {
    #[serde(flatten)]
    pub pre_debt_collection_register: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    #[serde(flatten)]
    pub user_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingFreeText {
    #[serde(flatten)]
    pub training_free_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLots {
    #[serde(flatten)]
    pub wood_lots: WoodLotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(flatten)]
    pub position: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedDensity {
    #[serde(flatten)]
    pub recommended_density: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSender {
    #[serde(flatten)]
    pub call_for_offer_business_sender: CallForOfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeRegistration {
    #[serde(flatten)]
    pub trade_registration: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulgeHeight {
    #[serde(flatten)]
    pub bulge_height: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub forest_centre_message_reference_type: ForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: LoppuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: LoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferDescription {
    #[serde(flatten)]
    pub related_call_for_offer_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: KuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumOrigin {
    #[serde(flatten)]
    pub stratum_origin: SeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeId {
    #[serde(flatten)]
    pub payee_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: BICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationApplicant {
    #[serde(flatten)]
    pub compensation_applicant: ContactInformationBankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary {
    #[serde(flatten)]
    pub tree_stand_summary: TreeStandSummary2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: DrainageStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainApplier {
    #[serde(flatten)]
    pub main_applier: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: PuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSilverBirch {
    #[serde(flatten)]
    pub mean_height_silver_birch: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: SubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplication {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetails {
    #[serde(flatten)]
    pub contract_working_site_details: ContractWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountNotified {
    #[serde(flatten)]
    pub amount_notified: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubDiameter {
    #[serde(flatten)]
    pub stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRightsOwner {
    #[serde(flatten)]
    pub cutting_rights_owner: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOutsideSpot {
    #[serde(flatten)]
    pub amount_outside_spot: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_diameter_other_tree_species: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationPercentage {
    #[serde(flatten)]
    pub participation_percentage: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(flatten)]
    pub map_symbol_type: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamagesText {
    #[serde(flatten)]
    pub tree_or_ground_damages_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSites {
    #[serde(flatten)]
    pub call_for_offer_working_sites: CallForOfferWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: VoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedCount {
    #[serde(flatten)]
    pub not_damaged_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmountUnit {
    #[serde(flatten)]
    pub material_amount_unit: MaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolume {
    #[serde(flatten)]
    pub harvested_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpedMoundsCount {
    #[serde(flatten)]
    pub scalped_mounds_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trainings {
    #[serde(flatten)]
    pub trainings: TrainingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometry {
    #[serde(flatten)]
    pub financing_act_completion_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygon {
    #[serde(flatten)]
    pub declaration_polygon: DeclarationPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: PostilokeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: UlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterHardWood {
    #[serde(flatten)]
    pub mean_diameter_hard_wood: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: RealEstateNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstates {
    #[serde(flatten)]
    pub base_real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceIdMJ {
    #[serde(flatten)]
    pub resource_id_m_j: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeOrdered {
    #[serde(flatten)]
    pub fertilizer_volume_ordered: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: VirtaInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub used_pricing_method_type: UsedPricingMethodTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: EtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: AmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumber {
    #[serde(flatten)]
    pub cost_type_number: CostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: Document4MBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuttings {
    #[serde(flatten)]
    pub cuttings: CuttingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRow {
    #[serde(flatten)]
    pub round_wood_sales_row: RoundWoodSalesRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedCompensation {
    #[serde(flatten)]
    pub received_compensation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomass {
    #[serde(flatten)]
    pub stem_biomass: StemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: VirtaHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantities {
    #[serde(flatten)]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogM3sum {
    #[serde(flatten)]
    pub log_m3sum: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: ImageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: ContractIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifier {
    #[serde(flatten)]
    pub stout_timber_classifier: StoutTimberClassifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercent {
    #[serde(flatten)]
    pub saw_log_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadStemCount {
    #[serde(flatten)]
    pub dead_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrata {
    #[serde(flatten)]
    pub stem_distribution_strata: StemDistributionStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstance {
    #[serde(flatten)]
    pub prevention_substance: PreventionSubstanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityFreeText {
    #[serde(flatten)]
    pub nationality_free_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlaceText {
    #[serde(flatten)]
    pub passing_place_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlace {
    #[serde(flatten)]
    pub passing_place: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetails {
    #[serde(flatten)]
    pub call_for_offer_working_site_details: CallForOfferWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Woodlot {
    #[serde(flatten)]
    pub woodlot: WoodLotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainage {
    #[serde(flatten)]
    pub road_structure_drainage: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsurance {
    #[serde(flatten)]
    pub compensation_by_insurance: CompensationByInsuranceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItem {
    #[serde(flatten)]
    pub price_item: PriceItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusData {
    #[serde(flatten)]
    pub prevention_fungus_of_the_genus_data: PreventionFungusOfTheGenusDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotification {
    #[serde(flatten)]
    pub electronic_notification: ElectronicNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPerson {
    #[serde(flatten)]
    pub technical_contact_person: TechnicalContactPersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: BetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MooseDamageDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandStemDamagesPercentage {
    #[serde(flatten)]
    pub stand_stem_damages_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: ForestHaulageDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandLocatedOnIsland {
    #[serde(flatten)]
    pub stand_located_on_island: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRows {
    #[serde(flatten)]
    pub operation_rows: OperationRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDate {
    #[serde(flatten)]
    pub offer_expiration_date: OfferExpirationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: ToinenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDate {
    #[serde(flatten)]
    pub felling_right_validity_date: FellingRightValidityDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClasses {
    #[serde(flatten)]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Active {
    #[serde(flatten)]
    pub active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotLength {
    #[serde(flatten)]
    pub nearest_cultivated_spot_length: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTime {
    #[serde(flatten)]
    pub change_time: ChangeTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCount {
    #[serde(flatten)]
    pub cutting_stem_count: CuttingStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataText {
    #[serde(flatten)]
    pub metadata_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDoneText {
    #[serde(flatten)]
    pub cleaning_breaks_done_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingEstimation {
    #[serde(flatten)]
    pub clearing_estimation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamages {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterPine {
    #[serde(flatten)]
    pub mean_diameter_pine: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypes {
    #[serde(flatten)]
    pub service_types: ServiceTypesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClass {
    #[serde(flatten)]
    pub removal_class: RemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: NimilajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlace {
    #[serde(flatten)]
    pub turning_place: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperations {
    #[serde(flatten)]
    pub declaration_other_operations: DeclarationOtherOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: TurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: LajiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: ShapeGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetParts {
    #[serde(flatten)]
    pub target_parts: TargetPartsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum {
    #[serde(flatten)]
    pub tree_stratum: TreeStratum2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_silviculture_info: CallForOfferSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: VirtaTreeDecimalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethod {
    #[serde(flatten)]
    pub pricing_method: UsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDate {
    #[serde(flatten)]
    pub document_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCareInfo {
    #[serde(flatten)]
    pub employee_health_care_info: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructed {
    #[serde(flatten)]
    pub airdrome_as_instructed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstates {
    #[serde(flatten)]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeHeight {
    #[serde(flatten)]
    pub tree_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: PostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCount {
    #[serde(flatten)]
    pub remaining_stump_count: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygons {
    #[serde(flatten)]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(flatten)]
    pub language: LanguageCode1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: StemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RightToSpecifyBankAccountsOfPaymentTransactions {
    #[serde(flatten)]
    pub right_to_specify_bank_accounts_of_payment_transactions: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: DitchingYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: VirtaHabitatTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClass {
    #[serde(flatten)]
    pub document_class: DocumentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: StatusryhmaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: VATInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionActionText {
    #[serde(flatten)]
    pub water_protection_action_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub financing_act_completion_declaration_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: CostTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocationText {
    #[serde(flatten)]
    pub excavation_soil_location_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationData {
    #[serde(flatten)]
    pub objects_realization_data: ObjectsRealizationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwner {
    #[serde(flatten)]
    pub real_estate_owner: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingId {
    #[serde(flatten)]
    pub training_id: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: MachineCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQuality {
    #[serde(flatten)]
    pub stump_lifting_quality: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructed {
    #[serde(flatten)]
    pub drain_storage_as_instructed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitableText {
    #[serde(flatten)]
    pub stump_lifting_suitable_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsible {
    #[serde(flatten)]
    pub forest_use_declaration_responsible: ForestUseDeclarationResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    #[serde(flatten)]
    pub suggestion: VirtaSuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: AmmattiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: HuoneistotunnisteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeDescription {
    #[serde(flatten)]
    pub wood_lot_information_type_description: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousMessage {
    #[serde(flatten)]
    pub update_previous_message: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQualityText {
    #[serde(flatten)]
    pub stump_lifting_quality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: AssortmentGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActors {
    #[serde(flatten)]
    pub application_actors: ApplicationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompact {
    #[serde(flatten)]
    pub assortment_compact: AssortmentCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgVehiclePathDistance {
    #[serde(flatten)]
    pub stand_avg_vehicle_path_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationReference {
    #[serde(flatten)]
    pub completion_declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: FaksinumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingBeginningCode {
    #[serde(flatten)]
    pub seedling_beginning_code: SeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: HeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountHardWood {
    #[serde(flatten)]
    pub stem_count_hard_wood: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstates {
    #[serde(flatten)]
    pub declaration_real_estates: DeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaReference {
    #[serde(flatten)]
    pub processing_area_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationDescription {
    #[serde(flatten)]
    pub compensation_description: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub area_type: AreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: SellersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueSurplus {
    #[serde(flatten)]
    pub expected_value_surplus: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbroken {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: SellerRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsAssortmentCompactClasses {
    #[serde(flatten)]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: DevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1damageCount {
    #[serde(flatten)]
    pub class1damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumber {
    #[serde(flatten)]
    pub forest_use_declaration_number: ForestUseDeclarationNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDate {
    #[serde(flatten)]
    pub v_a_t_registration_date: VATRegistrationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAnnouncedAmount {
    #[serde(flatten)]
    pub target_announced_amount: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: KansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopOperationProject {
    #[serde(flatten)]
    pub cop_operation_project: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsSummaries {
    #[serde(flatten)]
    pub sample_plots_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionCompleted {
    #[serde(flatten)]
    pub prevention_completed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: SpareTreeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasuredText {
    #[serde(flatten)]
    pub fertilizer_volume_measured_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActor {
    #[serde(flatten)]
    pub completion_declaration_actor: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: AlkuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: HarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsData {
    #[serde(flatten)]
    pub forest_statistics_data: ForestStatisticsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamagesText {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SproutForestControlEvaluation {
    #[serde(flatten)]
    pub sprout_forest_control_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityControlDate {
    #[serde(flatten)]
    pub quality_control_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomass {
    #[serde(flatten)]
    pub stump_biomass: StumpBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCode {
    #[serde(flatten)]
    pub specification_code: SpecificationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamages {
    #[serde(flatten)]
    pub previous_moose_damages: PreviousMooseDamagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: KolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCountText {
    #[serde(flatten)]
    pub remaining_stump_count_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSpruce {
    #[serde(flatten)]
    pub mean_height_spruce: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2damageCount {
    #[serde(flatten)]
    pub class2damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislation {
    #[serde(flatten)]
    pub compensation_by_legislation: CompensationByLegislationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountingDate {
    #[serde(flatten)]
    pub accounting_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsToWorkingSite {
    #[serde(flatten)]
    pub date_seedlings_to_working_site: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpMeanDiameter {
    #[serde(flatten)]
    pub stump_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: FinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferId {
    #[serde(flatten)]
    pub related_call_for_offer_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(flatten)]
    pub payment_transaction_type: MoneyTransactionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCode {
    #[serde(flatten)]
    pub other_habitat_code: OtherHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: VirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: AreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRows {
    #[serde(flatten)]
    pub loggings_rows: LoggingsRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreas {
    #[serde(flatten)]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesData {
    #[serde(flatten)]
    pub tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_wood_trade_info: CallForOfferWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResource {
    #[serde(flatten)]
    pub planned_resource: PlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationMode {
    #[serde(flatten)]
    pub operation_mode: OperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPersonInFinland {
    #[serde(flatten)]
    pub contact_person_in_finland: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode {
    #[serde(flatten)]
    pub language_code: LanguageCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitValue {
    #[serde(flatten)]
    pub unit_value: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStands {
    #[serde(flatten)]
    pub financing_act_application_stands: FinancingActApplicationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: WeibullType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathTooDeepPercentage {
    #[serde(flatten)]
    pub stand_vehicle_path_too_deep_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSpruce {
    #[serde(flatten)]
    pub stem_count_spruce: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDiameter {
    #[serde(flatten)]
    pub tree_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContarctorId {
    #[serde(flatten)]
    pub contarctor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolume {
    #[serde(flatten)]
    pub cutting_volume: CuttingVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCode {
    #[serde(flatten)]
    pub grade_code: GradeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValue {
    #[serde(flatten)]
    pub wood_lot_information_value: WoodLotInformationValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineLog {
    #[serde(flatten)]
    pub pine_log: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocationText {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathTooDeep {
    #[serde(flatten)]
    pub vehicle_path_too_deep: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(flatten)]
    pub label: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructure {
    #[serde(flatten)]
    pub road_structure: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandRootDamagesPercentage {
    #[serde(flatten)]
    pub stand_root_damages_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareOfOwnerShip {
    #[serde(flatten)]
    pub share_of_owner_ship: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyReceivesPayment {
    #[serde(flatten)]
    pub attorney_receives_payment: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiability {
    #[serde(flatten)]
    pub cutting_planner_liability: CuttingPlannerLiabilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: SahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: MunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCare {
    #[serde(flatten)]
    pub employee_health_care: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: YritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousDeclaration {
    #[serde(flatten)]
    pub update_previous_declaration: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: MeanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionData {
    #[serde(flatten)]
    pub completion_data: ExtendedCompletionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSilverBirch {
    #[serde(flatten)]
    pub mean_diameter_silver_birch: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPractice {
    #[serde(flatten)]
    pub cutting_realization_practice: CuttingRealizationPracticeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleData {
    #[serde(flatten)]
    pub scale_data: ScaleDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManipulationMethod {
    #[serde(flatten)]
    pub manipulation_method: WorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status5 {
    #[serde(flatten)]
    pub status5: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileName {
    #[serde(flatten)]
    pub document_file_name: DocumentFileNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMax {
    #[serde(flatten)]
    pub length_max: LengthMaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: SukupuoliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityDescription {
    #[serde(flatten)]
    pub seedling_condition_and_quality_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractId {
    #[serde(flatten)]
    pub purchase_contract_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceCompany {
    #[serde(flatten)]
    pub insurance_company: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingClassifiation {
    #[serde(flatten)]
    pub harvesting_classifiation: VirtaHarvestingClassificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: DifficultyClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTooHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_too_height_stumps_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employment {
    #[serde(flatten)]
    pub employment: EmploymentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolume {
    #[serde(flatten)]
    pub saw_log_volume: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCode {
    #[serde(flatten)]
    pub country_code: ISO3166char2CountryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwners {
    #[serde(flatten)]
    pub real_estate_owners: RealEstateOwnersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyType {
    #[serde(flatten)]
    pub company_type: CompanyTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: MunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActor {
    #[serde(flatten)]
    pub application_actor: ApplicationActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDate {
    #[serde(flatten)]
    pub training_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status2 {
    #[serde(flatten)]
    pub status2: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClass {
    #[serde(flatten)]
    pub declaration_development_class: DeclarationDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIsAuthorizedForEstate {
    #[serde(flatten)]
    pub project_is_authorized_for_estate: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: MinimumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSummary {
    #[serde(flatten)]
    pub basal_area_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeCount {
    #[serde(flatten)]
    pub tree_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrder {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: SiviilisaatyTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTimberValue {
    #[serde(flatten)]
    pub other_timber_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNumber {
    #[serde(flatten)]
    pub part_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePensionCertificate {
    #[serde(flatten)]
    pub employee_pension_certificate: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFormat {
    #[serde(flatten)]
    pub file_format: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    #[serde(flatten)]
    pub date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: EnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChain {
    #[serde(flatten)]
    pub planned_operation_chain: PlannedOperationChainType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticedText {
    #[serde(flatten)]
    pub water_economy_system_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceProductName {
    #[serde(flatten)]
    pub prevention_substance_product_name: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: AreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFile {
    #[serde(flatten)]
    pub external_file: ExternalFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: ExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAudit {
    #[serde(flatten)]
    pub fertilization_total_audit: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: FinalAuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceMass {
    #[serde(flatten)]
    pub control_reference_mass: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(flatten)]
    pub age: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessive {
    #[serde(flatten)]
    pub thinning_too_excessive: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootRotControlEvaluation {
    #[serde(flatten)]
    pub root_rot_control_evaluation: VirtaRootRotControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountOtherTreeSpecies {
    #[serde(flatten)]
    pub stem_count_other_tree_species: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SprucePulp {
    #[serde(flatten)]
    pub spruce_pulp: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: DiameterClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStand {
    #[serde(flatten)]
    pub declaration_stand: DeclarationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: BufferDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationReference {
    #[serde(flatten)]
    pub moose_damage_declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationAmount {
    #[serde(flatten)]
    pub compensation_amount: Decimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDate {
    #[serde(flatten)]
    pub operation_date: DateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TreeStrata2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryId {
    #[serde(flatten)]
    pub geometry_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: SceneryWorkPermissionNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalSawlogVolume {
    #[serde(flatten)]
    pub total_sawlog_volume: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPulpwoodVolume {
    #[serde(flatten)]
    pub total_pulpwood_volume: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfo {
    #[serde(flatten)]
    pub stand_silviculture_info: StandSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMoundsCount {
    #[serde(flatten)]
    pub ditch_mounds_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Height {
    #[serde(flatten)]
    pub height: MeanHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingLotIdentifier {
    #[serde(flatten)]
    pub seedling_lot_identifier: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSpruce {
    #[serde(flatten)]
    pub basal_area_spruce: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubStemCount {
    #[serde(flatten)]
    pub stub_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    #[serde(flatten)]
    pub seller: SellerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCode {
    #[serde(flatten)]
    pub assortment_class_code: AssortmentClassCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: VakinainenKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpecies {
    #[serde(flatten)]
    pub other_tree_species: OtherTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroups {
    #[serde(flatten)]
    pub work_code_groups: WorkCodeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Telephone {
    #[serde(flatten)]
    pub telephone: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: BankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMainGroup {
    #[serde(flatten)]
    pub assortment_main_group: AssortmentMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracy {
    #[serde(flatten)]
    pub cutting_accuracy: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamages {
    #[serde(flatten)]
    pub tree_or_ground_damages: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyTimberValue {
    #[serde(flatten)]
    pub energy_timber_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: KuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeState {
    #[serde(flatten)]
    pub change_state: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationNumber {
    #[serde(flatten)]
    pub completion_declaration_number: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: SukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: HarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentageTotal {
    #[serde(flatten)]
    pub dis_qualification_percentage_total: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: CallForOfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeSummary {
    #[serde(flatten)]
    pub volume_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletion {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifier {
    #[serde(flatten)]
    pub tree_identifier: TreeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsDetectedArea {
    #[serde(flatten)]
    pub parts_detected_area: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: OtherPublicSubstituteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentDate {
    #[serde(flatten)]
    pub sent_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScaping {
    #[serde(flatten)]
    pub land_scaping: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: PostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: KutsumaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: NormalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDate {
    #[serde(flatten)]
    pub moose_damage_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedEndDate {
    #[serde(flatten)]
    pub estimated_end_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnit {
    #[serde(flatten)]
    pub consumption_unit: ConsumptionUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub data_source: DataSourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRow {
    #[serde(flatten)]
    pub loggings_row: LoggingsRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirthDate {
    #[serde(flatten)]
    pub birth_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroup {
    #[serde(flatten)]
    pub financing_act_work_group: FinancingActWorkGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: TreeSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueCoefficient {
    #[serde(flatten)]
    pub expected_value_coefficient: PositiveDecimalMax1IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICETelephone {
    #[serde(flatten)]
    pub i_c_e_telephone: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethods {
    #[serde(flatten)]
    pub used_pricing_methods: UsedPricingMethodsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAccounting {
    #[serde(flatten)]
    pub final_accounting: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreements {
    #[serde(flatten)]
    pub collective_agreements: CollectiveAgreementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDate {
    #[serde(flatten)]
    pub contract_beginning_date: ContractBeginningDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightHardWood {
    #[serde(flatten)]
    pub mean_height_hard_wood: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrata {
    #[serde(flatten)]
    pub dead_tree_strata: DeadTreeStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    #[serde(flatten)]
    pub district: ThinningDistrictType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: NeljasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessiveCount {
    #[serde(flatten)]
    pub thinning_too_excessive_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WorkingSiteGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticed {
    #[serde(flatten)]
    pub water_economy_system_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantity {
    #[serde(flatten)]
    pub statistics_quantity: StatisticsQuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(flatten)]
    pub identifier_type: IdentifierTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBAN {
    #[serde(flatten)]
    pub iban: IBANType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMaking {
    #[serde(flatten)]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedWorkAmount {
    #[serde(flatten)]
    pub completed_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: PaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: CompactSoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: QuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeType {
    #[serde(flatten)]
    pub dead_tree_type: DeadTreeTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolumeAccounted {
    #[serde(flatten)]
    pub harvested_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: CareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffer {
    #[serde(flatten)]
    pub related_call_for_offer: RelatedCallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraFinancingApplication {
    #[serde(flatten)]
    pub extra_financing_application: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratum {
    #[serde(flatten)]
    pub dead_tree_stratum: DeadTreeStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticedText {
    #[serde(flatten)]
    pub environment_cleanliness_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetExtraInfo {
    #[serde(flatten)]
    pub target_extra_info: VirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrder {
    #[serde(flatten)]
    pub harvesting_order: HarvestingOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: SilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCutting {
    #[serde(flatten)]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: OsoiteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: FeatureAdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClasses {
    #[serde(flatten)]
    pub assortment_classes: AssortmentClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankCode {
    #[serde(flatten)]
    pub bank_code: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjects {
    #[serde(flatten)]
    pub financing_act_completion_other_subjects: FinancingActCompletionOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedWorkAmount {
    #[serde(flatten)]
    pub planned_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    #[serde(flatten)]
    pub training: TrainingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(flatten)]
    pub attachments: AttachmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchesInAdditionToCultivation {
    #[serde(flatten)]
    pub ditches_in_addition_to_cultivation: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowed {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3damageCount {
    #[serde(flatten)]
    pub class3damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalEstimation {
    #[serde(flatten)]
    pub total_estimation: VirtaTotalEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: KatuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierId {
    #[serde(flatten)]
    pub subsidy_applier_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometry {
    #[serde(flatten)]
    pub financing_act_application_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: VirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescription {
    #[serde(flatten)]
    pub payment_transaction_description: PaymentTransactionDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: KuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingMethod {
    #[serde(flatten)]
    pub cutting_method: CuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaOtherTreeSpecies {
    #[serde(flatten)]
    pub basal_area_other_tree_species: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpM3sum {
    #[serde(flatten)]
    pub pulp_m3sum: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolume {
    #[serde(flatten)]
    pub pulp_wood_volume: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityInsurance {
    #[serde(flatten)]
    pub liability_insurance: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: ShapeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSilverBirch {
    #[serde(flatten)]
    pub stem_count_silver_birch: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentReference {
    #[serde(flatten)]
    pub payment_reference: PaymentsReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerContractorId {
    #[serde(flatten)]
    pub owner_contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CumulativePointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNo {
    #[serde(flatten)]
    pub area_no: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDominantHeight {
    #[serde(flatten)]
    pub stand_avg_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinary {
    #[serde(flatten)]
    pub file_binary: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactions {
    #[serde(flatten)]
    pub payment_transactions: PaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: IBANTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    #[serde(flatten)]
    pub products: ProductsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaPine {
    #[serde(flatten)]
    pub basal_area_pine: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActProjectCompleted {
    #[serde(flatten)]
    pub financing_act_project_completed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotHeight {
    #[serde(flatten)]
    pub nearest_cultivated_spot_height: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionEnds {
    #[serde(flatten)]
    pub silviculture_restriction_ends: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolumeAccounted {
    #[serde(flatten)]
    pub forwarded_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValue {
    #[serde(flatten)]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: AgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: EtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesData {
    #[serde(flatten)]
    pub round_wood_sales_data: RoundWoodSalesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: StatisticsPurchaseModeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classification {
    #[serde(flatten)]
    pub classification: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKey {
    #[serde(flatten)]
    pub working_site_key: WorkingSiteKeyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: EdellinenSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginDate {
    #[serde(flatten)]
    pub begin_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotWidth {
    #[serde(flatten)]
    pub nearest_cultivated_spot_width: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracySign {
    #[serde(flatten)]
    pub cutting_accuracy_sign: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomass {
    #[serde(flatten)]
    pub leaf_biomass: LeafBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contractors {
    #[serde(flatten)]
    pub contractors: ContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiers {
    #[serde(flatten)]
    pub tree_identifiers: TreeIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetNumber {
    #[serde(flatten)]
    pub target_number: PositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationYear {
    #[serde(flatten)]
    pub operation_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartStatus {
    #[serde(flatten)]
    pub target_part_status: VirtaTargetPartStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: ShapeBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WorkingSitePlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystem {
    #[serde(flatten)]
    pub certification_system: CertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReason {
    #[serde(flatten)]
    pub dis_qualification_reason: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCode {
    #[serde(flatten)]
    pub financing_act_work_code: FinancingActWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandCorrectHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_correct_height_stumps_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub contact_person: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review2 {
    #[serde(flatten)]
    pub review2: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocation {
    #[serde(flatten)]
    pub ditch_cleaning_break_location: WorkingQualityType,
}

