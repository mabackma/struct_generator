#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracy {
    #[serde(flatten)]
    pub cutting_accuracy: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: JhsHuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterModel {
    #[serde(flatten)]
    pub harvester_model: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier4 {
    #[serde(flatten)]
    pub work_code_qualifier4: BdtWorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedSeedlingCount {
    #[serde(flatten)]
    pub not_damaged_seedling_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDocument {
    #[serde(flatten)]
    pub power_of_attorney_document: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrectHeightStumps {
    #[serde(flatten)]
    pub correct_height_stumps: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstate {
    #[serde(flatten)]
    pub financing_act_real_estate: FinancingActRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomass {
    #[serde(flatten)]
    pub branch_biomass: BranchBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: CoExtendedMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingDepthErrorCount {
    #[serde(flatten)]
    pub planting_depth_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrderText {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetails {
    #[serde(flatten)]
    pub silviculture_restriction_details: SilvicultureRestrictionDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumOrigin {
    #[serde(flatten)]
    pub stratum_origin: CoSeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub offer_working_site_silviculture_info: OfferWorkingSiteSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlWorkingSiteForwardingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: BdtHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratum {
    #[serde(flatten)]
    pub dead_tree_stratum: DeadTreeStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: CddGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificate {
    #[serde(flatten)]
    pub measurement_certificate: MeasurementCertificateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogCount {
    #[serde(flatten)]
    pub log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTexts {
    #[serde(flatten)]
    pub payment_texts: PaymentTextsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchLog {
    #[serde(flatten)]
    pub birch_log: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstates {
    #[serde(flatten)]
    pub base_real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptances {
    #[serde(flatten)]
    pub business_acceptances: BusinessAcceptancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCountSummary {
    #[serde(flatten)]
    pub reduction_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidy {
    #[serde(flatten)]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightClass {
    #[serde(flatten)]
    pub weight_class: BdtPositiveInteger1digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentYear {
    #[serde(flatten)]
    pub deployment_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReference {
    #[serde(flatten)]
    pub subsidy_applier_reference: SubsidyApplierReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendWorkingAloneNotification {
    #[serde(flatten)]
    pub send_working_alone_notification: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: BdtFeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: FccPowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformation {
    #[serde(flatten)]
    pub data_information: DataInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitableText {
    #[serde(flatten)]
    pub forest_energy_suitable_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSet {
    #[serde(flatten)]
    pub forest_property_data_set: ForestPropertyDataSetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: JhsKuolemaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingDistance {
    #[serde(flatten)]
    pub nearest_seedling_distance: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRepresentativePerson {
    #[serde(flatten)]
    pub owner_representative_person: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithCaravan {
    #[serde(flatten)]
    pub kilometers_with_caravan: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRows {
    #[serde(flatten)]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(flatten)]
    pub temperature: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(flatten)]
    pub id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    pub resource: ResourceResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShapeText {
    #[serde(flatten)]
    pub road_structure_shape_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: BdtDifficultyClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KemeraId {
    #[serde(flatten)]
    pub kemera_id: VirtaIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationUseLog {
    #[serde(flatten)]
    pub calibration_use_log: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLifted {
    #[serde(flatten)]
    pub stumps_lifted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlledOperationDescription {
    #[serde(flatten)]
    pub controlled_operation_description: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSite {
    #[serde(flatten)]
    pub working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusReduction {
    #[serde(flatten)]
    pub radius_reduction: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrdererResponsibilityDocumentsChecked {
    #[serde(flatten)]
    pub orderer_responsibility_documents_checked: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationScope {
    #[serde(flatten)]
    pub cultivation_scope: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringDate {
    #[serde(flatten)]
    pub self_monitoring_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterPine {
    #[serde(flatten)]
    pub mean_diameter_pine: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: CoSubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightPine {
    #[serde(flatten)]
    pub mean_height_pine: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub service: OrganizationServiceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningMeters {
    #[serde(flatten)]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAreaId {
    #[serde(flatten)]
    pub material_area_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelCount {
    #[serde(flatten)]
    pub level_count: CoPositiveInteger1digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportArea {
    #[serde(flatten)]
    pub transport_area: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicData {
    #[serde(flatten)]
    pub object_basic_data: ObjectBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: JhsSahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBH {
    #[serde(flatten)]
    pub dbh: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesFromMapSymbols {
    #[serde(flatten)]
    pub spare_trees_from_map_symbols: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotification {
    #[serde(flatten)]
    pub electronic_notification: CoElectronicNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeature {
    #[serde(flatten)]
    pub control_data_special_feature: ControlDataSpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedDeadStemCount {
    #[serde(flatten)]
    pub cultivated_dead_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSpruce {
    #[serde(flatten)]
    pub stem_count_spruce: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreasText {
    #[serde(flatten)]
    pub surface_draining_areas_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulks {
    #[serde(flatten)]
    pub stem_type_bulks: StemTypeBulksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountOtherTreeSpecies {
    #[serde(flatten)]
    pub stem_count_other_tree_species: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolume {
    #[serde(flatten)]
    pub stem_type_volume: StemTypeVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalName {
    #[serde(flatten)]
    pub additional_name: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaAndMapEvaluation {
    #[serde(flatten)]
    pub area_and_map_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringStandArea {
    #[serde(flatten)]
    pub self_monitoring_stand_area: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlaceText {
    #[serde(flatten)]
    pub turning_place_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyReceivesPayment {
    #[serde(flatten)]
    pub attorney_receives_payment: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: WtcoDocumentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonCode {
    #[serde(flatten)]
    pub reason_code: CoString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classification {
    #[serde(flatten)]
    pub classification: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsRightsOwnerRepresentative {
    #[serde(flatten)]
    pub cuttings_rights_owner_representative: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyTimberValue {
    #[serde(flatten)]
    pub energy_timber_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferText {
    #[serde(flatten)]
    pub offer_text: OfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceStatus {
    #[serde(flatten)]
    pub business_acceptance_status: CoBusinessAcceptanceStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinDiameter {
    #[serde(flatten)]
    pub min_diameter: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementNS {
    #[serde(flatten)]
    pub key_element_n_s: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueCoefficient {
    #[serde(flatten)]
    pub expected_value_coefficient: CoPositiveDecimalMax1IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassCollectionDone {
    #[serde(flatten)]
    pub bio_mass_collection_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub used_pricing_method_type: UsedPricingMethodTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePensionCertificate {
    #[serde(flatten)]
    pub employee_pension_certificate: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageFinishedDate {
    #[serde(flatten)]
    pub forest_haulage_finished_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Active {
    #[serde(flatten)]
    pub active: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: VirtaHabitatSurvivingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDate {
    #[serde(flatten)]
    pub moose_damage_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatment {
    #[serde(flatten)]
    pub stump_treatment: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedWorkAmount {
    #[serde(flatten)]
    pub completed_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovementsText {
    #[serde(flatten)]
    pub notifications_and_improvements_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowedText {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPrice {
    #[serde(flatten)]
    pub decided_unit_price: FccDecidedUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRightsOwner {
    #[serde(flatten)]
    pub cutting_rights_owner: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute5 {
    #[serde(flatten)]
    pub attribute5: BdtWorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreas {
    #[serde(flatten)]
    pub surface_draining_areas: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadUnitInvoiced {
    #[serde(flatten)]
    pub work_load_unit_invoiced: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiers {
    #[serde(flatten)]
    pub tree_identifiers: TreeIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionEnds {
    #[serde(flatten)]
    pub silviculture_restriction_ends: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoad {
    #[serde(flatten)]
    pub partitial_load: PartitialLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationType {
    #[serde(flatten)]
    pub fertilization_type: CoString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometry {
    #[serde(flatten)]
    pub object_geometry: ObjectGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damage {
    #[serde(flatten)]
    pub damage: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: WtcoSellersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluation {
    #[serde(flatten)]
    pub self_monitoring_evaluation: SelfMonitoringEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRow {
    #[serde(flatten)]
    pub loggings_row: LoggingsRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSender {
    #[serde(flatten)]
    pub call_for_offer_business_sender: WtcoCallForOfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogKey {
    #[serde(flatten)]
    pub log_key: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: BdtDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier3 {
    #[serde(flatten)]
    pub work_code_qualifier3: BdtWorkCodeQualifierType3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreinformDate {
    #[serde(flatten)]
    pub preinform_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialShipment {
    #[serde(flatten)]
    pub material_shipment: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderId {
    #[serde(flatten)]
    pub forwarder_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItem {
    #[serde(flatten)]
    pub price_item: PriceItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReply {
    #[serde(flatten)]
    pub forest_centre_reply: ForestCentreReplyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerAsText {
    #[serde(flatten)]
    pub question_answer_as_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTime {
    #[serde(flatten)]
    pub working_site_work_time: WorkingSiteWorkTimeWorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactorId {
    #[serde(flatten)]
    pub contactor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationTextInformation {
    #[serde(flatten)]
    pub declaration_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDone {
    #[serde(flatten)]
    pub cleaning_breaks_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCompletionDate {
    #[serde(flatten)]
    pub work_completion_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwner {
    #[serde(flatten)]
    pub real_estate_owner: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationMainTreeSpecies {
    #[serde(flatten)]
    pub declaration_main_tree_species: CoTreeSpeciesConciseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: CiFirstNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureMethodCorrectText {
    #[serde(flatten)]
    pub silviculture_method_correct_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubStemCount {
    #[serde(flatten)]
    pub stub_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElevatorCertificate {
    #[serde(flatten)]
    pub elevator_certificate: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankCode {
    #[serde(flatten)]
    pub bank_code: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorderingWithWaterAreaOrStream {
    #[serde(flatten)]
    pub bordering_with_water_area_or_stream: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumes {
    #[serde(flatten)]
    pub assortment_volumes: AssortmentVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometry {
    #[serde(flatten)]
    pub working_site_geometry: SfLocatedSpecialFeature2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameter {
    #[serde(flatten)]
    pub log_diameter: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDoneText {
    #[serde(flatten)]
    pub clearing_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson4 {
    #[serde(flatten)]
    pub extra_person4: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: BdtWorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCertification {
    #[serde(flatten)]
    pub forest_certification: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub data_source: DataSourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegister {
    #[serde(flatten)]
    pub employer_register: EmployerRegisterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: WtcoCallForOfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCurrent {
    #[serde(flatten)]
    pub stand_trees_current: StandTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    #[serde(flatten)]
    pub organization: OrganizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: JhsSyntymaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub result_of_accessibility_analysis: ResultOfAccessibilityAnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountInSpot {
    #[serde(flatten)]
    pub amount_in_spot: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaMethod {
    #[serde(flatten)]
    pub test_area_method: BdtSamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensity {
    #[serde(flatten)]
    pub target_density: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    #[serde(flatten)]
    pub offer: OfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActors {
    #[serde(flatten)]
    pub application_actors: ApplicationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedToSend {
    #[serde(flatten)]
    pub authorized_to_send: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: BdtServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acknowledge {
    #[serde(flatten)]
    pub acknowledge: AcknowledgeAcknowledgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestry {
    #[serde(flatten)]
    pub financing_sustainable_forestry: FinancingSustainableForestryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLiftedText {
    #[serde(flatten)]
    pub stumps_lifted_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletion {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStands {
    #[serde(flatten)]
    pub financing_act_application_stands: FinancingActApplicationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMeanDepth {
    #[serde(flatten)]
    pub ditch_mean_depth: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndLoadNumber {
    #[serde(flatten)]
    pub end_load_number: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitable {
    #[serde(flatten)]
    pub stump_lifting_suitable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinLength {
    #[serde(flatten)]
    pub min_length: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStand {
    #[serde(flatten)]
    pub financing_act_completion_stand: FinancingActCompletionStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistanceUnit {
    #[serde(flatten)]
    pub transport_distance_unit: BdtDistanceUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocationText {
    #[serde(flatten)]
    pub excavation_soil_location_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwner {
    #[serde(flatten)]
    pub estate_owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYear {
    #[serde(flatten)]
    pub min_proposal_year: MinProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclaration {
    #[serde(flatten)]
    pub control_forest_use_declaration: ControlForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1 {
    #[serde(flatten)]
    pub work_load1: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: VirtaExceptionalPermitForHandlingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(flatten)]
    pub authorization_to_send_wso_information: AuthorizationToSendWsoInformation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingPlantingDepth {
    #[serde(flatten)]
    pub nearest_seedling_planting_depth: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaOtherTreeSpecies {
    #[serde(flatten)]
    pub basal_area_other_tree_species: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(flatten)]
    pub review: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: BdtMeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesProposalForestHaulageDistances {
    #[serde(flatten)]
    pub storages_proposal_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandLocatedOnIsland {
    #[serde(flatten)]
    pub stand_located_on_island: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumber {
    #[serde(flatten)]
    pub processing_area_number: ProcessingAreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlOverallEvaluationData {
    #[serde(flatten)]
    pub control_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilviculture {
    #[serde(flatten)]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureWorkingSiteQualityControlSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersLogs {
    #[serde(flatten)]
    pub sellers_logs: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyData {
    #[serde(flatten)]
    pub forest_property_data: FdForestPropertyDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemiDry {
    #[serde(flatten)]
    pub semi_dry: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumplingLevelingErrorCount {
    #[serde(flatten)]
    pub dumpling_leveling_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassCollection {
    #[serde(flatten)]
    pub biomass_collection: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enddate {
    #[serde(flatten)]
    pub enddate: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticed {
    #[serde(flatten)]
    pub water_economy_system_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPits {
    #[serde(flatten)]
    pub settling_pits: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceNumber {
    #[serde(flatten)]
    pub insurance_number: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SittingMoneyKilometers {
    #[serde(flatten)]
    pub sitting_money_kilometers: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Questions {
    #[serde(flatten)]
    pub questions: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectorName {
    #[serde(flatten)]
    pub inspector_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status5 {
    #[serde(flatten)]
    pub status5: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetails {
    #[serde(flatten)]
    pub call_for_offer_working_site_details: CallForOfferWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaHardWood {
    #[serde(flatten)]
    pub basal_area_hard_wood: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformation {
    #[serde(flatten)]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethods {
    #[serde(flatten)]
    pub other_conservation_methods: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationReference {
    #[serde(flatten)]
    pub financing_act_application_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorVerified {
    #[serde(flatten)]
    pub sub_contractor_verified: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDate {
    #[serde(flatten)]
    pub contract_ending_date: ContractEndingDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemId {
    #[serde(flatten)]
    pub stem_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode {
    #[serde(flatten)]
    pub language_code: BdtLanguageCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparatedParcel {
    #[serde(flatten)]
    pub unseparated_parcel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resource {
    #[serde(flatten)]
    pub resource: XlinkresourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesStratumLeaving {
    #[serde(flatten)]
    pub stand_trees_stratum_leaving: StandTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: FccCostTypeType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentatives {
    #[serde(flatten)]
    pub working_representatives: WorkingRepresentativesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginNotificationAllowed {
    #[serde(flatten)]
    pub begin_notification_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotLength {
    #[serde(flatten)]
    pub nearest_cultivated_spot_length: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OldDestinationStorage {
    #[serde(flatten)]
    pub old_destination_storage: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2 {
    #[serde(flatten)]
    pub work_load2: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDescription {
    #[serde(flatten)]
    pub work_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogCount {
    #[serde(flatten)]
    pub control_log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryStatus {
    #[serde(flatten)]
    pub geometry_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementId {
    #[serde(flatten)]
    pub announcement_id: AnnouncementIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtinguisherVerificationDate {
    #[serde(flatten)]
    pub extinguisher_verification_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordResourceId {
    #[serde(flatten)]
    pub stanford_resource_id: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactMunicipality {
    #[serde(flatten)]
    pub contact_municipality: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluations {
    #[serde(flatten)]
    pub control_evaluations: ControlEvaluationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: JhsPaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMeanWidth {
    #[serde(flatten)]
    pub ditch_mean_width: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    #[serde(flatten)]
    pub measurement: MeasurementDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    #[serde(flatten)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locator {
    #[serde(flatten)]
    pub locator: XlinklocatorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperId {
    #[serde(flatten)]
    pub caliper_id: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiver {
    #[serde(flatten)]
    pub decision_receiver: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeys {
    #[serde(flatten)]
    pub object_keys: ObjectKeysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemInUse {
    #[serde(flatten)]
    pub external_system_in_use: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageId {
    #[serde(flatten)]
    pub common_message_id: WctCommonMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeed {
    #[serde(flatten)]
    pub cutting_area_preclearing_need: CuttingAreaPreclearingNeedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperation {
    #[serde(flatten)]
    pub statistics_operation: StatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometry {
    #[serde(flatten)]
    pub decision_geometry: DecisionGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocation {
    #[serde(flatten)]
    pub resource_location: ResourceLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamic {
    #[serde(flatten)]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicWorkingSiteFinalAuditDynamicType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDuration {
    #[serde(flatten)]
    pub felling_right_duration: FellingRightDurationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Humidity {
    #[serde(flatten)]
    pub humidity: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperApplication {
    #[serde(flatten)]
    pub caliper_application: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovements {
    #[serde(flatten)]
    pub notifications_and_improvements: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTimeEnd {
    #[serde(flatten)]
    pub operation_time_end: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(flatten)]
    pub owner: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolumeAccounted {
    #[serde(flatten)]
    pub forwarded_volume_accounted: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstates {
    #[serde(flatten)]
    pub financing_act_real_estates: FinancingActRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: BdtFinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbrokenText {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalEstimation {
    #[serde(flatten)]
    pub total_estimation: VirtaTotalEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weight {
    #[serde(flatten)]
    pub weight: BdtInteger7digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(flatten)]
    pub route: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainPlanningDone {
    #[serde(flatten)]
    pub terrain_planning_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentDate {
    #[serde(flatten)]
    pub sent_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractId {
    #[serde(flatten)]
    pub purchase_contract_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListId {
    #[serde(flatten)]
    pub list_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockIsFSFBlock {
    #[serde(flatten)]
    pub block_is_f_s_f_block: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsible {
    #[serde(flatten)]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperation {
    #[serde(flatten)]
    pub declaration_regeneration_operation: CoDeclarationRegenerationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectId {
    #[serde(flatten)]
    pub parent_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationAmount {
    #[serde(flatten)]
    pub compensation_amount: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalTidinessNoticedText {
    #[serde(flatten)]
    pub environmental_tidiness_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingWeight {
    #[serde(flatten)]
    pub working_weight: BdtWorkingWeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasSupport {
    #[serde(flatten)]
    pub has_support: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedPlantEvaluation {
    #[serde(flatten)]
    pub seed_plant_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Days {
    #[serde(flatten)]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDisplayId {
    #[serde(flatten)]
    pub storage_display_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingTrees {
    #[serde(flatten)]
    pub remaining_trees: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverallTotalSubsidy {
    #[serde(flatten)]
    pub overall_total_subsidy: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMin {
    #[serde(flatten)]
    pub length_min: LengthMinType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPersonInFinland {
    #[serde(flatten)]
    pub contact_person_in_finland: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelLabel {
    #[serde(flatten)]
    pub parcel_label: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(flatten)]
    pub category: BdtImageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationReference {
    #[serde(flatten)]
    pub moose_damage_declaration_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: CoHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandArea {
    #[serde(flatten)]
    pub control_stand_area: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvertisementDating {
    #[serde(flatten)]
    pub advertisement_dating: VirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: JhsValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: JhsOsoiteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilData {
    #[serde(flatten)]
    pub soil_data: StBaseSoilDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: CoPriorityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginDate {
    #[serde(flatten)]
    pub begin_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicData {
    #[serde(flatten)]
    pub self_monitoring_basic_data: SelfMonitoringBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformation {
    #[serde(flatten)]
    pub control_additional_information: ControlAdditionalInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contractors {
    #[serde(flatten)]
    pub contractors: WctContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousMessage {
    #[serde(flatten)]
    pub update_previous_message: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifier {
    #[serde(flatten)]
    pub tree_identifier: TreeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidy {
    #[serde(flatten)]
    pub decided_total_subsidy: FccDecidedTotalSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: JhsAlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstates {
    #[serde(flatten)]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: JhsEtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationMode {
    #[serde(flatten)]
    pub operation_mode: OperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasured {
    #[serde(flatten)]
    pub fertilizer_volume_measured: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: BdtAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationDescription {
    #[serde(flatten)]
    pub length_calibration_description: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyType {
    #[serde(flatten)]
    pub company_type: BdtCompanyTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionActionText {
    #[serde(flatten)]
    pub water_protection_action_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeUnit {
    #[serde(flatten)]
    pub fee_unit: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItem {
    #[serde(flatten)]
    pub value_list_item: ValueListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearing {
    #[serde(flatten)]
    pub pre_clearing: BdtYesNoNotNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1DamageCount {
    #[serde(flatten)]
    pub class1_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDeviceCheckRequired {
    #[serde(flatten)]
    pub measure_device_check_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationEvaluationApproval {
    #[serde(flatten)]
    pub association_evaluation_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trainings {
    #[serde(flatten)]
    pub trainings: TrainingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoles {
    #[serde(flatten)]
    pub user_roles: UserRolesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerRepresentativePerson {
    #[serde(flatten)]
    pub customer_representative_person: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBAN {
    #[serde(flatten)]
    pub iban: IBANType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerType {
    #[serde(flatten)]
    pub customer_type: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjects {
    #[serde(flatten)]
    pub child_objects: ChildObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseDate {
    #[serde(flatten)]
    pub case_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnit {
    #[serde(flatten)]
    pub material_unit: BdtMaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamagePercentage {
    #[serde(flatten)]
    pub root_damage_percentage: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceId {
    #[serde(flatten)]
    pub business_acceptance_id: BusinessAcceptanceIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    #[serde(flatten)]
    pub tree: TreeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChange {
    #[serde(flatten)]
    pub assortments_change: AssortmentChangeDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute3 {
    #[serde(flatten)]
    pub attribute3: BdtWorkCodeQualifierType3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    #[serde(flatten)]
    pub contact_request: ContactRequestType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationWorkingSiteFinalAuditFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedEnergyWood {
    #[serde(flatten)]
    pub announced_energy_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlocks {
    #[serde(flatten)]
    pub previous_blocks: PreviousBlockInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousSameAreaMooseDamageCompensationYear {
    #[serde(flatten)]
    pub previous_same_area_moose_damage_compensation_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCode {
    #[serde(flatten)]
    pub assortment_code: CoAssortmentCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MddMooseDamageDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadWorkingSiteWorkLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActor {
    #[serde(flatten)]
    pub responsible_actor: ResponsibleActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleAssortmentType {
    #[serde(flatten)]
    pub scale_assortment_type: BdtScaleAssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: CoString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingWorkingSiteFinalAuditStumpForwardingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessage {
    #[serde(flatten)]
    pub forest_centre_message: ForestCentreMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Height {
    #[serde(flatten)]
    pub height: HeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasins {
    #[serde(flatten)]
    pub settling_basins: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolMapSymbolType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppliedLength {
    #[serde(flatten)]
    pub applied_length: Decimal6_2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClass {
    #[serde(flatten)]
    pub terrain_class: BdtTerrainClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFNumber {
    #[serde(flatten)]
    pub f_s_f_number: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClass {
    #[serde(flatten)]
    pub small_wood_removal_class: CoSmallWoodRemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CddCumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubPriceArea {
    #[serde(flatten)]
    pub stub_price_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwners {
    #[serde(flatten)]
    pub forest_owners: ForestOwnersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
    #[serde(flatten)]
    pub removal_class_type: RemovalClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SprucePulp {
    #[serde(flatten)]
    pub spruce_pulp: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingFreeText {
    #[serde(flatten)]
    pub training_free_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantity {
    #[serde(flatten)]
    pub statistics_quantity: StatisticsQuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActData {
    #[serde(flatten)]
    pub financing_act_data: FinancingActDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationCompanyInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeSummary {
    #[serde(flatten)]
    pub volume_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChains {
    #[serde(flatten)]
    pub planned_operation_chains: PlannedOperationChainsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestExtraInfo {
    #[serde(flatten)]
    pub harvest_extra_info: TgtVirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleTransformation {
    #[serde(flatten)]
    pub scale_transformation: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub financing_act_completion_declaration_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub real_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICETelephone {
    #[serde(flatten)]
    pub i_c_e_telephone: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationType {
    #[serde(flatten)]
    pub wood_lot_information_type: WoodLotInformationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPayment {
    #[serde(flatten)]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmount {
    #[serde(flatten)]
    pub decided_amount: FccDecidedAmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageVolume {
    #[serde(flatten)]
    pub average_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStartDate {
    #[serde(flatten)]
    pub service_start_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: SfLocatedSpecialFeature1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: CoMachineAccessoryCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: CddShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageClass {
    #[serde(flatten)]
    pub damage_class: VirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationData {
    #[serde(flatten)]
    pub forest_realization_data: ForestRealizationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffer {
    #[serde(flatten)]
    pub related_call_for_offer: RelatedCallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: SfBufferDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceProductName {
    #[serde(flatten)]
    pub prevention_substance_product_name: CoString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageClass {
    #[serde(flatten)]
    pub storage_class: BdtStorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidiness {
    #[serde(flatten)]
    pub stump_tidiness: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetExtraInfo {
    #[serde(flatten)]
    pub target_extra_info: VirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachment {
    #[serde(flatten)]
    pub quality_attachment: QualityAttachmentQualityAttachmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StormWorkingSite {
    #[serde(flatten)]
    pub storm_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubDiameter {
    #[serde(flatten)]
    pub stub_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceivers {
    #[serde(flatten)]
    pub decision_receivers: DecisionReceiversType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationType {
    #[serde(flatten)]
    pub reported_statistics_operation_type: CoReportedStatisticsOperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderEmail {
    #[serde(flatten)]
    pub sender_email: CiEmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: VirtaTreeDecimalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumes {
    #[serde(flatten)]
    pub stem_type_volumes: StemTypeVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionStumpHeight {
    #[serde(flatten)]
    pub reduction_stump_height: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceDate {
    #[serde(flatten)]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accepted {
    #[serde(flatten)]
    pub accepted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutOfServiceStartDate {
    #[serde(flatten)]
    pub out_of_service_start_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: ImageImageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogLengthClass {
    #[serde(flatten)]
    pub log_length_class: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescription {
    #[serde(flatten)]
    pub payment_transaction_description: PaymentTransactionDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurpose {
    #[serde(flatten)]
    pub cutting_purpose: CoCuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlNo {
    #[serde(flatten)]
    pub control_no: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaks {
    #[serde(flatten)]
    pub ditching_breaks: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RightToSpecifyBankAccountsOfPaymentTransactions {
    #[serde(flatten)]
    pub right_to_specify_bank_accounts_of_payment_transactions: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: CiLastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreas {
    #[serde(flatten)]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(flatten)]
    pub count: BdtPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingFinishedDate {
    #[serde(flatten)]
    pub stump_lifting_finished_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute2 {
    #[serde(flatten)]
    pub attribute2: BdtWorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathWidth {
    #[serde(flatten)]
    pub stand_vehicle_path_width: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructedText {
    #[serde(flatten)]
    pub drain_storage_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameterSummary {
    #[serde(flatten)]
    pub reduction_avg_diameter_summary: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystem {
    #[serde(flatten)]
    pub wide_certification_system: WideCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectId {
    #[serde(flatten)]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjects {
    #[serde(flatten)]
    pub financing_act_application_other_subjects: FinancingActApplicationOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stem {
    #[serde(flatten)]
    pub stem: StemDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsAssortmentCompactClasses {
    #[serde(flatten)]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeState {
    #[serde(flatten)]
    pub change_state: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleData {
    #[serde(flatten)]
    pub scale_data: ScaleDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCountText {
    #[serde(flatten)]
    pub remaining_stump_count_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sawinghours {
    #[serde(flatten)]
    pub sawinghours: SawinghoursDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorWrittenAgreement {
    #[serde(flatten)]
    pub sub_contractor_written_agreement: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: JhsPostilokeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HavesterModelYear {
    #[serde(flatten)]
    pub havester_model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNo {
    #[serde(flatten)]
    pub project_no: ProjectNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchPulp {
    #[serde(flatten)]
    pub birch_pulp: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreDebtCollectionRegister {
    #[serde(flatten)]
    pub pre_debt_collection_register: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataName {
    #[serde(flatten)]
    pub data_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: CoWideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier1 {
    #[serde(flatten)]
    pub work_code_qualifier1: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdate {
    #[serde(flatten)]
    pub forest_data_update: CodForestDataUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalVolume {
    #[serde(flatten)]
    pub small_wood_removal_volume: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: BdtMaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandler {
    #[serde(flatten)]
    pub decision_handler: DecisionHandlerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationData {
    #[serde(flatten)]
    pub objects_realization_data: ObjectsRealizationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrata {
    #[serde(flatten)]
    pub dead_tree_strata: DeadTreeStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionCompleted {
    #[serde(flatten)]
    pub prevention_completed: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDate {
    #[serde(flatten)]
    pub image_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasicData {
    #[serde(flatten)]
    pub sample_plot_basic_data: SamplePlotBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStands {
    #[serde(flatten)]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgument {
    #[serde(flatten)]
    pub subsidy_argument: SubsidyArgumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementId {
    #[serde(flatten)]
    pub measurement_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumber {
    #[serde(flatten)]
    pub sequence_number: SequenceNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    #[serde(flatten)]
    pub email: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supplier {
    #[serde(flatten)]
    pub supplier: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationCode {
    #[serde(flatten)]
    pub operation_code: CoObjectProtectionOperationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProposals {
    #[serde(flatten)]
    pub storage_proposals: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotificationsAlways {
    #[serde(flatten)]
    pub send_notifications_always: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterVolume {
    #[serde(flatten)]
    pub harvester_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethod {
    #[serde(flatten)]
    pub preferred_contacting_method: CoPreferredContactingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaksText {
    #[serde(flatten)]
    pub ditching_breaks_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealization {
    #[serde(flatten)]
    pub object_realization: ObjectRealizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityControlDate {
    #[serde(flatten)]
    pub quality_control_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessMessageTimeStamp {
    #[serde(flatten)]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentStorageVolume {
    #[serde(flatten)]
    pub sent_storage_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreinformDetails {
    #[serde(flatten)]
    pub preinform_details: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation2 {
    #[serde(flatten)]
    pub organisation2: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OldCode {
    #[serde(flatten)]
    pub old_code: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAuditText {
    #[serde(flatten)]
    pub fertilization_total_audit_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deliveries {
    #[serde(flatten)]
    pub deliveries: DeliveriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditIdentifierVersion {
    #[serde(flatten)]
    pub final_audit_identifier_version: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowed {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: BdtYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRating {
    #[serde(flatten)]
    pub load_rating: WctLoadRatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferDate {
    #[serde(flatten)]
    pub call_for_offer_date: CallForOfferDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectOverallEvaluationData {
    #[serde(flatten)]
    pub object_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictions {
    #[serde(flatten)]
    pub silviculture_restrictions: SilvicultureRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: CoTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationAdjustment {
    #[serde(flatten)]
    pub length_calibration_adjustment: BdtInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDate {
    #[serde(flatten)]
    pub inspection_date: CoDateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTidinessText {
    #[serde(flatten)]
    pub stump_tidiness_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonPersonificationId {
    #[serde(flatten)]
    pub non_personification_id: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInfoText {
    #[serde(flatten)]
    pub material_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAsText {
    #[serde(flatten)]
    pub question_as_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedLength {
    #[serde(flatten)]
    pub announced_length: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: JhsValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationMunicipality {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_municipality: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelId {
    #[serde(flatten)]
    pub parcel_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClass {
    #[serde(flatten)]
    pub drying_class: BdtDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(flatten)]
    pub attributes: AttributesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStand {
    #[serde(flatten)]
    pub financing_act_application_stand: FinancingActApplicationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingWorkingSiteAccountingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(flatten)]
    pub vehicle: BdtVehicleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPitsText {
    #[serde(flatten)]
    pub settling_pits_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: BdtSoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesData {
    #[serde(flatten)]
    pub round_wood_sales_data: RoundWoodSalesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCutting {
    #[serde(flatten)]
    pub stand_trees_cutting: StandTreesCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: RestrictionCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanStumpDiameter {
    #[serde(flatten)]
    pub mean_stump_diameter: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: BdtString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineLog {
    #[serde(flatten)]
    pub pine_log: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructed {
    #[serde(flatten)]
    pub airdrome_as_instructed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffect {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformation {
    #[serde(flatten)]
    pub pre_clearing_information: PreClearingInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassified {
    #[serde(flatten)]
    pub assortment_volumes_unclassified: AssortmentVolumesUnclassifiedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperations {
    #[serde(flatten)]
    pub object_protection_operations: ObjectProtectionOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantities {
    #[serde(flatten)]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataReference {
    #[serde(flatten)]
    pub common_object_data_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVolumeSummary {
    #[serde(flatten)]
    pub stand_volume_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpM3sum {
    #[serde(flatten)]
    pub pulp_m3sum: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingValue {
    #[serde(flatten)]
    pub cutting_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SproutForestControlEvaluation {
    #[serde(flatten)]
    pub sprout_forest_control_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocation {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditIdentifier {
    #[serde(flatten)]
    pub final_audit_identifier: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: JhsHuoltosuhdeTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(flatten)]
    pub symbol: MapSymbolDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNumber {
    #[serde(flatten)]
    pub part_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organizations {
    #[serde(flatten)]
    pub organizations: OrganizationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerText {
    #[serde(flatten)]
    pub question_answer_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompartmentId {
    #[serde(flatten)]
    pub compartment_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: CddBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(flatten)]
    pub envelope: EnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingByMachine {
    #[serde(flatten)]
    pub cutting_by_machine: VirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Advertiser {
    #[serde(flatten)]
    pub advertiser: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantPositionErrorCount {
    #[serde(flatten)]
    pub plant_position_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StbStandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainWorkCode {
    #[serde(flatten)]
    pub main_work_code: BdtMainWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainageText {
    #[serde(flatten)]
    pub road_structure_drainage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReference {
    #[serde(flatten)]
    pub control_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRole {
    #[serde(flatten)]
    pub person_role: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperational {
    #[serde(flatten)]
    pub working_site_operational: WorkingSiteOperationalWorkingSiteOperationalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTimeStart {
    #[serde(flatten)]
    pub operation_time_start: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractValidDate {
    #[serde(flatten)]
    pub contract_valid_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantStorageManagementText {
    #[serde(flatten)]
    pub plant_storage_management_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeId {
    #[serde(flatten)]
    pub payee_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlannerText {
    #[serde(flatten)]
    pub feedback_for_planner_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQuality {
    #[serde(flatten)]
    pub stump_lifting_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationYear {
    #[serde(flatten)]
    pub operation_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCode {
    #[serde(flatten)]
    pub owner_ship_type_code: CoOwnerShipTypeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedEndDate {
    #[serde(flatten)]
    pub estimated_end_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: WtcoCallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BdtBearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SumTableArea {
    #[serde(flatten)]
    pub sum_table_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemsNoticed {
    #[serde(flatten)]
    pub water_economy_systems_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureMethodCorrect {
    #[serde(flatten)]
    pub silviculture_method_correct: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPart {
    #[serde(flatten)]
    pub target_part: TargetPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: BdtFeatureAdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsDetectedArea {
    #[serde(flatten)]
    pub parts_detected_area: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingSuitableText {
    #[serde(flatten)]
    pub stump_lifting_suitable_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogDiameter {
    #[serde(flatten)]
    pub control_log_diameter: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct title {
    #[serde(flatten)]
    pub title: XlinktitleEltType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TstTreeStrata2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpedMoundsCount {
    #[serde(flatten)]
    pub scalped_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBasicData {
    #[serde(flatten)]
    pub control_basic_data: ControlBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Languages {
    #[serde(flatten)]
    pub languages: LanguagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(flatten)]
    pub header: HeaderHeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadGreenMass {
    #[serde(flatten)]
    pub load_green_mass: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessage {
    #[serde(flatten)]
    pub common_message: CommonMessageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: JhsKieliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCode {
    #[serde(flatten)]
    pub additional_code: AdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingWorkingSiteFinalAuditDrainingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractors {
    #[serde(flatten)]
    pub sub_contractors: SubContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferId {
    #[serde(flatten)]
    pub related_call_for_offer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataMooseDamageData {
    #[serde(flatten)]
    pub control_data_moose_damage_data: MooseDamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MixedForestRegenarationMethods {
    #[serde(flatten)]
    pub mixed_forest_regenaration_methods: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractor {
    #[serde(flatten)]
    pub sub_contractor: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Density {
    #[serde(flatten)]
    pub density: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reasons {
    #[serde(flatten)]
    pub reasons: ReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryId {
    #[serde(flatten)]
    pub geometry_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActors {
    #[serde(flatten)]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InTerrain {
    #[serde(flatten)]
    pub in_terrain: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentID {
    #[serde(flatten)]
    pub assortment_i_d: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisList {
    #[serde(flatten)]
    pub fee_basis_list: FeeBasisListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolume {
    #[serde(flatten)]
    pub forwarded_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeRegistration {
    #[serde(flatten)]
    pub trade_registration: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetParts {
    #[serde(flatten)]
    pub target_parts: TargetPartsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightHardWood {
    #[serde(flatten)]
    pub mean_height_hard_wood: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSourceDescription {
    #[serde(flatten)]
    pub damage_source_description: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfreezingDate {
    #[serde(flatten)]
    pub unfreezing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilPreparationSpotsAreEnough {
    #[serde(flatten)]
    pub soil_preparation_spots_are_enough: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(flatten)]
    pub value: ValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FacdFinancingActCompletionDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMoundsCount {
    #[serde(flatten)]
    pub ditch_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationDescription {
    #[serde(flatten)]
    pub diameter_calibration_description: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanner {
    #[serde(flatten)]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDate {
    #[serde(flatten)]
    pub action_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationWorkingSiteQualityNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentreDecisionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: CddWeibullType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamName {
    #[serde(flatten)]
    pub team_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaRequired {
    #[serde(flatten)]
    pub test_area_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(flatten)]
    pub object: CodForestCentreControlObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionAction {
    #[serde(flatten)]
    pub water_protection_action: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(flatten)]
    pub version: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityOrganizations {
    #[serde(flatten)]
    pub publicity_organizations: OOrganizationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    #[serde(flatten)]
    pub products: PrProductsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biomass {
    #[serde(flatten)]
    pub biomass: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidValue {
    #[serde(flatten)]
    pub paid_value: PaidValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
    #[serde(flatten)]
    pub services: ServicesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kilometers {
    #[serde(flatten)]
    pub kilometers: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroups {
    #[serde(flatten)]
    pub work_code_groups: WorkCodeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecision {
    #[serde(flatten)]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
    #[serde(flatten)]
    pub load: LoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClodMoistureErrorCount {
    #[serde(flatten)]
    pub clod_moisture_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticedText {
    #[serde(flatten)]
    pub environment_cleanliness_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRiskDescription {
    #[serde(flatten)]
    pub work_safety_risk_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagement {
    #[serde(flatten)]
    pub control_data_swamp_forest_management: ControlDataSwampForestManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustmentButtLog {
    #[serde(flatten)]
    pub diameter_calibration_adjustment_butt_log: BdtInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsSummaries {
    #[serde(flatten)]
    pub sample_plots_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: CddJohnsonSBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: CddScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCode {
    #[serde(flatten)]
    pub peripheral_code: BdtPeripheralCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDryingClass {
    #[serde(flatten)]
    pub storage_drying_class: BdtStorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: CoMachineCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderUserId {
    #[serde(flatten)]
    pub sender_user_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallationText {
    #[serde(flatten)]
    pub pipe_installation_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypes {
    #[serde(flatten)]
    pub service_types: ServiceTypesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCategory {
    #[serde(flatten)]
    pub evaluation_category: CoEvaluationSubjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: JhsPankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MastoInspection {
    #[serde(flatten)]
    pub masto_inspection: VirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNo {
    #[serde(flatten)]
    pub parcel_no: BdtPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(flatten)]
    pub model: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectData {
    #[serde(flatten)]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerReference {
    #[serde(flatten)]
    pub customer_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyToDo {
    #[serde(flatten)]
    pub ready_to_do: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirthDate {
    #[serde(flatten)]
    pub birth_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeltedWater {
    #[serde(flatten)]
    pub melted_water: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedData {
    #[serde(flatten)]
    pub tree_stand_based_data: StTreeStandBasedDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerId {
    #[serde(flatten)]
    pub measurer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCutting {
    #[serde(flatten)]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingWorkingSiteQualityControlCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrix {
    #[serde(flatten)]
    pub price_matrix: PriceMatrixType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadId {
    #[serde(flatten)]
    pub work_load_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Percent {
    #[serde(flatten)]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpirationDate {
    #[serde(flatten)]
    pub expiration_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratum {
    #[serde(flatten)]
    pub stem_distribution_stratum: StemDistributionStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceId {
    #[serde(flatten)]
    pub service_buyer_resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MineralSoilLayer {
    #[serde(flatten)]
    pub mineral_soil_layer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    #[serde(flatten)]
    pub suggestion: VirtaSuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWithSeedlings {
    #[serde(flatten)]
    pub stocking_with_seedlings: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstablishedPartNumber {
    #[serde(flatten)]
    pub established_part_number: VirtaPartNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCode {
    #[serde(flatten)]
    pub assortment_class_code: AsAssortmentClassCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnowOrIce {
    #[serde(flatten)]
    pub snow_or_ice: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityCode {
    #[serde(flatten)]
    pub nationality_code: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    #[serde(flatten)]
    pub comments: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(flatten)]
    pub attachments: AttachmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCount {
    #[serde(flatten)]
    pub reduction_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinished {
    #[serde(flatten)]
    pub working_site_finished: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumidityMeasured {
    #[serde(flatten)]
    pub humidity_measured: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFile {
    #[serde(flatten)]
    pub stanford_file: StanfordFileStanfordFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: CddVarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReferenceType {
    #[serde(flatten)]
    pub object_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummary {
    #[serde(flatten)]
    pub tree_species_summary: TreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryNumber {
    #[serde(flatten)]
    pub delivery_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateWorkingSiteOperationalUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSection {
    #[serde(flatten)]
    pub diameter_section: SectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyer {
    #[serde(flatten)]
    pub service_buyer: ServiceBuyerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometryType {
    #[serde(flatten)]
    pub help_geometry_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveDate {
    #[serde(flatten)]
    pub remove_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmissionTime {
    #[serde(flatten)]
    pub transmission_time: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(flatten)]
    pub object_type: CoDecisionGeometryObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactions {
    #[serde(flatten)]
    pub payment_transactions: WtcoPaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorResource {
    #[serde(flatten)]
    pub sub_contractor_resource: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: CoMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(flatten)]
    pub payment: PaymentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: RestrictionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionCategory {
    #[serde(flatten)]
    pub payment_transaction_category: CoMoneyTransactionCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamages {
    #[serde(flatten)]
    pub root_damages: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotWidth {
    #[serde(flatten)]
    pub nearest_cultivated_spot_width: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDate {
    #[serde(flatten)]
    pub working_site_plan_date: WorkingSitePlanDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercent {
    #[serde(flatten)]
    pub saw_log_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: CoIdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingBy {
    #[serde(flatten)]
    pub cutting_by: CoVirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportCompany {
    #[serde(flatten)]
    pub transport_company: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Finished {
    #[serde(flatten)]
    pub finished: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocation {
    #[serde(flatten)]
    pub habitat_location: CoHabitatLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotHeight {
    #[serde(flatten)]
    pub nearest_cultivated_spot_height: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationStandFellingPurpose {
    #[serde(flatten)]
    pub forest_use_declaration_stand_felling_purpose: BdtCuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageFinishedDate {
    #[serde(flatten)]
    pub storage_finished_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCode {
    #[serde(flatten)]
    pub financing_act_work_code: CoFinancingActWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCare {
    #[serde(flatten)]
    pub employee_health_care: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumber {
    #[serde(flatten)]
    pub object_number: ObjectNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisWorkingSiteFeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(flatten)]
    pub payment_transaction_type: CoMoneyTransactionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelYear {
    #[serde(flatten)]
    pub model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperation {
    #[serde(flatten)]
    pub declaration_soil_preparation_operation: CoDeclarationSoilPreparationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    #[serde(flatten)]
    pub log: LogDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringDeviceVersion {
    #[serde(flatten)]
    pub measuring_device_version: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasalAreaSummary {
    #[serde(flatten)]
    pub stand_basal_area_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumber {
    #[serde(flatten)]
    pub cost_type_number: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditName {
    #[serde(flatten)]
    pub final_audit_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClass {
    #[serde(flatten)]
    pub assortment_class: AssortmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIncrementAllowed {
    #[serde(flatten)]
    pub assortment_increment_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDay {
    #[serde(flatten)]
    pub calendar_day: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerSampleAreaApproval {
    #[serde(flatten)]
    pub owner_sample_area_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaPine {
    #[serde(flatten)]
    pub basal_area_pine: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actors {
    #[serde(flatten)]
    pub actors: ActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: VirtaInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamages {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDominantHeight {
    #[serde(flatten)]
    pub stand_avg_dominant_height: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCase {
    #[serde(flatten)]
    pub use_case: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethod {
    #[serde(flatten)]
    pub used_pricing_method: WtcoUsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrder {
    #[serde(flatten)]
    pub silviculture_order: SilvicultureOrderSilvicultureOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDate {
    #[serde(flatten)]
    pub v_a_t_registration_date: WtcoVATRegistrationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectData {
    #[serde(flatten)]
    pub common_object_data: CommonObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonDescription {
    #[serde(flatten)]
    pub reason_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationNumber {
    #[serde(flatten)]
    pub completion_declaration_number: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(flatten)]
    pub action: CoActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparatedParcelNumber {
    #[serde(flatten)]
    pub unseparated_parcel_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IslandWorkingSite {
    #[serde(flatten)]
    pub island_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inspection {
    #[serde(flatten)]
    pub inspection: InspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPlanStandId {
    #[serde(flatten)]
    pub forest_plan_stand_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityDateBegin {
    #[serde(flatten)]
    pub validity_date_begin: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameOfAPI {
    #[serde(flatten)]
    pub service_name_of_a_p_i: BdtServiceNameofAPIType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: JhsAlayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescription {
    #[serde(flatten)]
    pub document_description: DocumentDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTax {
    #[serde(flatten)]
    pub advance_tax: AdvanceTaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingSoilCompressed {
    #[serde(flatten)]
    pub nearest_seedling_soil_compressed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeName {
    #[serde(flatten)]
    pub whole_name: WholeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: JhsSukupuoliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformation {
    #[serde(flatten)]
    pub f_s_f_information: FSFInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMeanDiameter {
    #[serde(flatten)]
    pub stub_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationText {
    #[serde(flatten)]
    pub evaluation_text: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolumeAccounted {
    #[serde(flatten)]
    pub harvested_volume_accounted: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithBreakHouse {
    #[serde(flatten)]
    pub kilometers_with_break_house: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCode {
    #[serde(flatten)]
    pub contract_code: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: VirtaGroundManipulationMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionData {
    #[serde(flatten)]
    pub inspection_data: InspectionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payments {
    #[serde(flatten)]
    pub payments: PaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: BdtFertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestReference {
    #[serde(flatten)]
    pub request_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetId {
    #[serde(flatten)]
    pub target_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: CoSilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderDelay {
    #[serde(flatten)]
    pub forwarder_delay: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDate {
    #[serde(flatten)]
    pub training_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: CddNormalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherEvaluation {
    #[serde(flatten)]
    pub other_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeCount {
    #[serde(flatten)]
    pub tree_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeDescription {
    #[serde(flatten)]
    pub wood_lot_information_type_description: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometries {
    #[serde(flatten)]
    pub help_geometries: HelpGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnit {
    #[serde(flatten)]
    pub decided_amount_unit: FccDecidedAmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDate {
    #[serde(flatten)]
    pub document_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingDistance {
    #[serde(flatten)]
    pub forwarding_distance: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Targets {
    #[serde(flatten)]
    pub targets: TargetsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasons {
    #[serde(flatten)]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurements {
    #[serde(flatten)]
    pub log_measurements: LogMeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChain {
    #[serde(flatten)]
    pub planned_operation_chain: PlannedOperationChainType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSender {
    #[serde(flatten)]
    pub offer_business_sender: OfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: BdtWideDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareGroupOfTrees {
    #[serde(flatten)]
    pub spare_group_of_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDate {
    #[serde(flatten)]
    pub offer_date: OfferDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedWorkingTimeConsumption {
    #[serde(flatten)]
    pub estimated_working_time_consumption: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthOfDitchDiggedDuringSoilPreparation {
    #[serde(flatten)]
    pub length_of_ditch_digged_during_soil_preparation: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeData {
    #[serde(flatten)]
    pub operative_data: OperativeDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObject {
    #[serde(flatten)]
    pub geometry_object: GeometryObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minutes {
    #[serde(flatten)]
    pub minutes: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationDate {
    #[serde(flatten)]
    pub length_calibration_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedVolume {
    #[serde(flatten)]
    pub planned_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: CoAmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursBusinessDay {
    #[serde(flatten)]
    pub working_hours_business_day: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: CiPersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingSiteCount {
    #[serde(flatten)]
    pub planting_site_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumidityPercentage {
    #[serde(flatten)]
    pub humidity_percentage: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    #[serde(flatten)]
    pub date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDate {
    #[serde(flatten)]
    pub control_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationSign {
    #[serde(flatten)]
    pub dis_qualification_sign: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: JhsEdellinenSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedCompensation {
    #[serde(flatten)]
    pub received_compensation: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursSaturday {
    #[serde(flatten)]
    pub working_hours_saturday: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingWorkQuality {
    #[serde(flatten)]
    pub planting_work_quality: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: JhsLajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    #[serde(flatten)]
    pub height_class_type: HeightClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingClassifiation {
    #[serde(flatten)]
    pub harvesting_classifiation: VirtaHarvestingClassificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachments {
    #[serde(flatten)]
    pub quality_attachments: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status2 {
    #[serde(flatten)]
    pub status2: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingExecutionTime {
    #[serde(flatten)]
    pub pre_clearing_execution_time: FSFValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetails {
    #[serde(flatten)]
    pub contract_working_site_details: ContractWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchesInAdditionToCultivation {
    #[serde(flatten)]
    pub ditches_in_addition_to_cultivation: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReference {
    #[serde(flatten)]
    pub object_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoosePercentage {
    #[serde(flatten)]
    pub moose_percentage: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingAction {
    #[serde(flatten)]
    pub erosion_blocking_action: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: BdtString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageFreeText {
    #[serde(flatten)]
    pub common_message_free_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeType {
    #[serde(flatten)]
    pub fee_type: BdtFeeBasisValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinary {
    #[serde(flatten)]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisks {
    #[serde(flatten)]
    pub work_safety_risks: WorkSafetyRisksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReason {
    #[serde(flatten)]
    pub dis_qualification_reason: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    #[serde(flatten)]
    pub user_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub goal_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationType {
    #[serde(flatten)]
    pub regeneration_type: VirtaRegenerationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompact {
    #[serde(flatten)]
    pub assortment_compact: AssortmentCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRightsInfo {
    #[serde(flatten)]
    pub access_rights_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: JhsToinenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTimestamp {
    #[serde(flatten)]
    pub send_timestamp: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStands {
    #[serde(flatten)]
    pub financing_act_completion_stands: FinancingActCompletionStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Population {
    #[serde(flatten)]
    pub population: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClass {
    #[serde(flatten)]
    pub declaration_development_class: CoDeclarationDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTimberValue {
    #[serde(flatten)]
    pub other_timber_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientApplicationId {
    #[serde(flatten)]
    pub client_application_id: BdtClientApplicationIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingStumpCount {
    #[serde(flatten)]
    pub remaining_stump_count: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSilverBirch {
    #[serde(flatten)]
    pub stem_count_silver_birch: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTotalSubsidy {
    #[serde(flatten)]
    pub application_total_subsidy: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: JhsUlkomaaHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectTypeSpecifier {
    #[serde(flatten)]
    pub child_object_type_specifier: ObjectTypeSpecifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    #[serde(flatten)]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocation {
    #[serde(flatten)]
    pub excavation_soil_location: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manufacturer {
    #[serde(flatten)]
    pub manufacturer: BdtMachineManufacturerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSilverBirch {
    #[serde(flatten)]
    pub mean_height_silver_birch: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPerson {
    #[serde(flatten)]
    pub technical_contact_person: TechnicalContactPersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderScaleModel {
    #[serde(flatten)]
    pub loader_scale_model: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClasses {
    #[serde(flatten)]
    pub assortment_classes: AssortmentClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4damageCount {
    #[serde(flatten)]
    pub class4damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: JhsKuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTree {
    #[serde(flatten)]
    pub stand_tree: StandTreeCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub user_role: BdtUserRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(flatten)]
    pub message_type: CoMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotifications {
    #[serde(flatten)]
    pub send_notifications: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeIntervalForMeasuringSamplePlot {
    #[serde(flatten)]
    pub time_interval_for_measuring_sample_plot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: JhsAmmattiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActions {
    #[serde(flatten)]
    pub case_actions: CaseActionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    #[serde(flatten)]
    pub error_message: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalSawlogVolume {
    #[serde(flatten)]
    pub total_sawlog_volume: CoSawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct E101 {
    #[serde(flatten)]
    pub e101: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingVolume {
    #[serde(flatten)]
    pub stand_trees_cutting_volume: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotification {
    #[serde(flatten)]
    pub working_site_end_notification: WorkingSiteEndNotificationWorkingSiteEndNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePerson {
    #[serde(flatten)]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderScaleModelYear {
    #[serde(flatten)]
    pub loader_scale_model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: BdtFinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifier {
    #[serde(flatten)]
    pub forest_damage_qualifier: CoForestDamageQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeOrdered {
    #[serde(flatten)]
    pub fertilizer_volume_ordered: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingActionText {
    #[serde(flatten)]
    pub erosion_blocking_action_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingCode {
    #[serde(flatten)]
    pub working_code: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(flatten)]
    pub inspection_type: VirtaInspectionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: CddMeanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(flatten)]
    pub parent_object_type: ObjectTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: FudForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsValueForceWorkingSite {
    #[serde(flatten)]
    pub is_value_force_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantCountSummary {
    #[serde(flatten)]
    pub plant_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitCostArea {
    #[serde(flatten)]
    pub unit_cost_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(flatten)]
    pub work_type: CoPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSiteCountSummary {
    #[serde(flatten)]
    pub plant_site_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPayment {
    #[serde(flatten)]
    pub forest_fund_payment: ForestFundPaymentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObject {
    #[serde(flatten)]
    pub child_object: ChildObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembers {
    #[serde(flatten)]
    pub work_grou_members: WorkGrouMembersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountNotified {
    #[serde(flatten)]
    pub amount_notified: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCount {
    #[serde(flatten)]
    pub cutting_stem_count: CuttingStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeltingDate {
    #[serde(flatten)]
    pub melting_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: CoUsingRightResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionType {
    #[serde(flatten)]
    pub decision_type: CoDecisionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometries {
    #[serde(flatten)]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeciduousTreeTargetDensityPercent {
    #[serde(flatten)]
    pub deciduous_tree_target_density_percent: BdtPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: CoStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessiveCount {
    #[serde(flatten)]
    pub thinning_too_excessive_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: BdtWorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingWorkingSiteFinalAuditBioMassForwardingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueList {
    #[serde(flatten)]
    pub value_list: ValueListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCode {
    #[serde(flatten)]
    pub status_code: StatusCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSpruce {
    #[serde(flatten)]
    pub basal_area_spruce: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDiameter {
    #[serde(flatten)]
    pub tree_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationReason {
    #[serde(flatten)]
    pub length_calibration_reason: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    #[serde(flatten)]
    pub contract_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub target: TargetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaId {
    #[serde(flatten)]
    pub virta_id: VirtaIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocationText {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestType {
    #[serde(flatten)]
    pub forest_type: BdtFertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDoneText {
    #[serde(flatten)]
    pub cleaning_breaks_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectData {
    #[serde(flatten)]
    pub forest_object_data: ForestObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResource {
    #[serde(flatten)]
    pub planned_resource: WtcPlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFormat {
    #[serde(flatten)]
    pub file_format: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQualityText {
    #[serde(flatten)]
    pub bio_mass_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractText {
    #[serde(flatten)]
    pub contract_text: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileName {
    #[serde(flatten)]
    pub document_file_name: DocumentFileNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: CddDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Width {
    #[serde(flatten)]
    pub width: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation3 {
    #[serde(flatten)]
    pub organisation3: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometry {
    #[serde(flatten)]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDate {
    #[serde(flatten)]
    pub delivery_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometries {
    #[serde(flatten)]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroup {
    #[serde(flatten)]
    pub financing_act_work_group: CoFinancingActWorkGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_height_other_tree_species: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    #[serde(flatten)]
    pub cutting_stem_count_type: CuttingStemCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffers {
    #[serde(flatten)]
    pub related_call_for_offers: RelatedCallForOffersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchType {
    #[serde(flatten)]
    pub ditch_type: WctDitchTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    #[serde(flatten)]
    pub day: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arc {
    #[serde(flatten)]
    pub arc: XlinkarcType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: JhsKutsumaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4DamageCount {
    #[serde(flatten)]
    pub class4_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopOperationProject {
    #[serde(flatten)]
    pub cop_operation_project: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantStorageManagement {
    #[serde(flatten)]
    pub plant_storage_management: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: JhsNimilajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClasses {
    #[serde(flatten)]
    pub assortment_compact_classes: AsAssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: CddShapeBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status3 {
    #[serde(flatten)]
    pub status3: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ReParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: JhsFaksinumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Removed {
    #[serde(flatten)]
    pub removed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: VirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionalityText {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamagesText {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandRootDamagesPercentage {
    #[serde(flatten)]
    pub stand_root_damages_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalData {
    #[serde(flatten)]
    pub proposal_data: ProposalDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: SubsidyApplierBaseContactAndEstateInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalTreeSpecies {
    #[serde(flatten)]
    pub goal_tree_species: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethod {
    #[serde(flatten)]
    pub pricing_method: WctPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwners {
    #[serde(flatten)]
    pub real_estate_owners: RealEstateOwnersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRows {
    #[serde(flatten)]
    pub operation_rows: OperationRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpStemCount {
    #[serde(flatten)]
    pub stump_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTag {
    #[serde(flatten)]
    pub entity_tag: CoEntityTagType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResources {
    #[serde(flatten)]
    pub audition_resources: AuditionResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(flatten)]
    pub map_symbol_type: BdtFeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameter {
    #[serde(flatten)]
    pub reduction_avg_diameter: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    #[serde(flatten)]
    pub roles: RolesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePayments {
    #[serde(flatten)]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountAccounted {
    #[serde(flatten)]
    pub amount_accounted: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainDamageOutsideStand {
    #[serde(flatten)]
    pub terrain_damage_outside_stand: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calibration {
    #[serde(flatten)]
    pub calibration: CalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Justifications {
    #[serde(flatten)]
    pub justifications: JustificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caliper {
    #[serde(flatten)]
    pub caliper: CaliperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAnnouncedAmount {
    #[serde(flatten)]
    pub target_announced_amount: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestsType {
    #[serde(flatten)]
    pub contact_requests_type: ContactRequests,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioning {
    #[serde(flatten)]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningWorkingSiteFinalAuditSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CddCumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubCategory {
    #[serde(flatten)]
    pub sub_category: BdtImageSubCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstates {
    #[serde(flatten)]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceName {
    #[serde(flatten)]
    pub resource_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(flatten)]
    pub product: ProductType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientType {
    #[serde(flatten)]
    pub recipient_type: RecipientTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: JhsHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationDate {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: JhsViidesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedStemCount {
    #[serde(flatten)]
    pub planned_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acceptance {
    #[serde(flatten)]
    pub acceptance: CoAcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanName {
    #[serde(flatten)]
    pub ditch_or_road_plan_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationStandExtraInfo {
    #[serde(flatten)]
    pub forest_use_declaration_stand_extra_info: BdtString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePartNumber {
    #[serde(flatten)]
    pub base_part_number: VirtaPartNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSections {
    #[serde(flatten)]
    pub diameter_sections: DiameterSectionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employment {
    #[serde(flatten)]
    pub employment: EmploymentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: JhsBICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProduction {
    #[serde(flatten)]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionWorkingSiteForwardedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingQualityText {
    #[serde(flatten)]
    pub stump_lifting_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: JhsPuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumber {
    #[serde(flatten)]
    pub financing_act_number: FccFinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Altitude {
    #[serde(flatten)]
    pub altitude: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loads {
    #[serde(flatten)]
    pub loads: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryName {
    #[serde(flatten)]
    pub delivery_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementId {
    #[serde(flatten)]
    pub key_element_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
    #[serde(flatten)]
    pub phase: VirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantedSeedlingCountInCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarliestInspectionDate {
    #[serde(flatten)]
    pub earliest_inspection_date: CoDateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: JhsEnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2DamageCount {
    #[serde(flatten)]
    pub class2_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason2 {
    #[serde(flatten)]
    pub reason2: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    #[serde(flatten)]
    pub seller: SellerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: WtcoVATInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesData {
    #[serde(flatten)]
    pub tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureData {
    #[serde(flatten)]
    pub special_feature_data: SpecialFeatureDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCoordinates {
    #[serde(flatten)]
    pub stem_coordinates: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDestinationStorage {
    #[serde(flatten)]
    pub new_destination_storage: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: CoDrainageStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCode {
    #[serde(flatten)]
    pub reply_code: CoReplyCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionData {
    #[serde(flatten)]
    pub completion_data: ExtendedCompletionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionDate {
    #[serde(flatten)]
    pub decision_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamagePercentage {
    #[serde(flatten)]
    pub stem_damage_percentage: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringDeviceLastControl {
    #[serde(flatten)]
    pub measuring_device_last_control: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: WorkingSiteStatusWorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasuredText {
    #[serde(flatten)]
    pub fertilizer_volume_measured_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CddCumulativePointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelPosition {
    #[serde(flatten)]
    pub label_position: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentMonth {
    #[serde(flatten)]
    pub deployment_month: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRestriction {
    #[serde(flatten)]
    pub operation_restriction: OperationRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSilverBirch {
    #[serde(flatten)]
    pub mean_diameter_silver_birch: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathTooDeep {
    #[serde(flatten)]
    pub vehicle_path_too_deep: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountingDate {
    #[serde(flatten)]
    pub accounting_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperation {
    #[serde(flatten)]
    pub declaration_other_operation: CoDeclarationOtherOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: CoIdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: BdtStorageLandOwnerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinishedDate {
    #[serde(flatten)]
    pub working_site_finished_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonText {
    #[serde(flatten)]
    pub dis_qualification_reason_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingId {
    #[serde(flatten)]
    pub training_id: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyZone {
    #[serde(flatten)]
    pub subsidy_zone: CoForestActAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    #[serde(flatten)]
    pub r#type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: JhsKuudesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKey {
    #[serde(flatten)]
    pub working_site_key: WorkingSiteKeyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseAction {
    #[serde(flatten)]
    pub case_action: CaseActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandBasicData {
    #[serde(flatten)]
    pub control_stand_basic_data: ControlStandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsAllowed {
    #[serde(flatten)]
    pub sub_contractors_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolume {
    #[serde(flatten)]
    pub saw_log_volume: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracySign {
    #[serde(flatten)]
    pub cutting_accuracy_sign: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialVolume {
    #[serde(flatten)]
    pub material_volume: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferDescription {
    #[serde(flatten)]
    pub related_call_for_offer_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: JhsIBANTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaledMass {
    #[serde(flatten)]
    pub scaled_mass: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjects {
    #[serde(flatten)]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCode {
    #[serde(flatten)]
    pub new_code: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsData {
    #[serde(flatten)]
    pub forest_statistics_data: ForestStatisticsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSchedule {
    #[serde(flatten)]
    pub resource_schedule: ResourceScheduleResourceScheduleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notices {
    #[serde(flatten)]
    pub notices: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    #[serde(flatten)]
    pub small_wood_removal_class_type: SmallWoodRemovalClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassFinishedDate {
    #[serde(flatten)]
    pub biomass_finished_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status1 {
    #[serde(flatten)]
    pub status1: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCode {
    #[serde(flatten)]
    pub grade_code: CoGradeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentType {
    #[serde(flatten)]
    pub payment_type: PaymentTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstance {
    #[serde(flatten)]
    pub prevention_substance: CoPreventionSubstanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payee {
    #[serde(flatten)]
    pub payee: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfoAsText {
    #[serde(flatten)]
    pub key_info_as_text: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub machine: BdtMachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: JhsHuoneistotunnisteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation4 {
    #[serde(flatten)]
    pub organisation4: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delivery {
    #[serde(flatten)]
    pub delivery: DeliveryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnit {
    #[serde(flatten)]
    pub consumption_unit: ConsumptionUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterId {
    #[serde(flatten)]
    pub harvester_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIsAuthorizedForEstate {
    #[serde(flatten)]
    pub project_is_authorized_for_estate: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSpruce {
    #[serde(flatten)]
    pub mean_diameter_spruce: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CoCuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactor {
    #[serde(flatten)]
    pub scale_factor: ScaleFactorDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplication {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(flatten)]
    pub list_item: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityData {
    #[serde(flatten)]
    pub accessibility_data: AccessibilityDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeft {
    #[serde(flatten)]
    pub save_trees_left: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethods {
    #[serde(flatten)]
    pub used_pricing_methods: WtcoUsedPricingMethodsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulgeHeight {
    #[serde(flatten)]
    pub bulge_height: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: WtcoRoadUsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithTrailer {
    #[serde(flatten)]
    pub kilometers_with_trailer: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShape {
    #[serde(flatten)]
    pub road_structure_shape: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelection {
    #[serde(flatten)]
    pub target_selection: VirtaTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQuality {
    #[serde(flatten)]
    pub bio_mass_quality: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActProjectCompleted {
    #[serde(flatten)]
    pub financing_act_project_completed: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactLocationInformation {
    #[serde(flatten)]
    pub contact_location_information: GdtAlternativeGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDays {
    #[serde(flatten)]
    pub degree_days: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringData {
    #[serde(flatten)]
    pub self_monitoring_data: CodForestCentreSelfMonitoringDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: OpSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(flatten)]
    pub age: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOutsideSpot {
    #[serde(flatten)]
    pub amount_outside_spot: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hoppers {
    #[serde(flatten)]
    pub hoppers: HoppersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(flatten)]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementWorkingSiteQualityControlPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountHardWood {
    #[serde(flatten)]
    pub stem_count_hard_wood: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperLocationFromGPS {
    #[serde(flatten)]
    pub hopper_location_from_g_p_s: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethod {
    #[serde(flatten)]
    pub soil_conditioning_method: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActor {
    #[serde(flatten)]
    pub completion_declaration_actor: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: GdtPolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuttings {
    #[serde(flatten)]
    pub cuttings: CuttingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute1 {
    #[serde(flatten)]
    pub attribute1: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessageType {
    #[serde(flatten)]
    pub original_message_type: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Works {
    #[serde(flatten)]
    pub works: WorksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDate {
    #[serde(flatten)]
    pub insert_date: InsertDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: CoOtherPublicSubstituteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRow {
    #[serde(flatten)]
    pub operation_row: OperationRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrder {
    #[serde(flatten)]
    pub harvesting_order: HarvestingOrderHarvestingOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementWorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSpruce {
    #[serde(flatten)]
    pub mean_height_spruce: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActor {
    #[serde(flatten)]
    pub application_actor: ApplicationActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(flatten)]
    pub identifier_type: IdentifierTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomass {
    #[serde(flatten)]
    pub remaining_biomass: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: CddShapeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolume {
    #[serde(flatten)]
    pub pulp_wood_volume: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineDescription {
    #[serde(flatten)]
    pub machine_description: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerContractorId {
    #[serde(flatten)]
    pub owner_contractor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingContract {
    #[serde(flatten)]
    pub working_contract: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateForwardingEstimateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedGroundWood {
    #[serde(flatten)]
    pub detected_ground_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanId {
    #[serde(flatten)]
    pub ditch_or_road_plan_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocations {
    #[serde(flatten)]
    pub resource_locations: ResourceLocationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoTextMandatory {
    #[serde(flatten)]
    pub info_text_mandatory: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMainGroup {
    #[serde(flatten)]
    pub assortment_main_group: CoAssortmentMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasinsText {
    #[serde(flatten)]
    pub settling_basins_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifier {
    #[serde(flatten)]
    pub stout_timber_classifier: StoutTimberClassifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingBeginningCode {
    #[serde(flatten)]
    pub seedling_beginning_code: CoSeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationInvolvement {
    #[serde(flatten)]
    pub association_involvement: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsToWorkingSite {
    #[serde(flatten)]
    pub date_seedlings_to_working_site: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review2 {
    #[serde(flatten)]
    pub review2: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTime {
    #[serde(flatten)]
    pub change_time: ChangeTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessive {
    #[serde(flatten)]
    pub thinning_too_excessive: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: JhsUlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaReference {
    #[serde(flatten)]
    pub processing_area_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmation {
    #[serde(flatten)]
    pub order_confirmation: OrderConfirmationOrderConfirmationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Envelopes {
    #[serde(flatten)]
    pub envelopes: EnvelopesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cutting {
    #[serde(flatten)]
    pub cutting: CuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitable {
    #[serde(flatten)]
    pub forest_energy_suitable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCode {
    #[serde(flatten)]
    pub specification_code: SpecificationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationDescription {
    #[serde(flatten)]
    pub compensation_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registered {
    #[serde(flatten)]
    pub registered: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameterClass {
    #[serde(flatten)]
    pub log_diameter_class: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandFinished {
    #[serde(flatten)]
    pub stand_finished: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseMode {
    #[serde(flatten)]
    pub purchase_mode: PurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamageCount {
    #[serde(flatten)]
    pub stem_damage_count: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureQuality {
    #[serde(flatten)]
    pub silviculture_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionOfOrigin {
    #[serde(flatten)]
    pub region_of_origin: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganisationId {
    #[serde(flatten)]
    pub organisation_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandReference {
    #[serde(flatten)]
    pub declaration_stand_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootRotControlEvaluation {
    #[serde(flatten)]
    pub root_rot_control_evaluation: VirtaRootRotControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowthPlaceDataSource {
    #[serde(flatten)]
    pub growth_place_data_source: CoDataSourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: CertificationSystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainReason {
    #[serde(flatten)]
    pub main_reason: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbroken {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken: BdtYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSource {
    #[serde(flatten)]
    pub damage_source: CoFeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygons {
    #[serde(flatten)]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlanner {
    #[serde(flatten)]
    pub feedback_for_planner: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolume {
    #[serde(flatten)]
    pub cutting_volume: CuttingVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorCode {
    #[serde(flatten)]
    pub error_code: CoString25Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelEndTime {
    #[serde(flatten)]
    pub travel_end_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometry {
    #[serde(flatten)]
    pub line_geometry: LineGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrder {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentText {
    #[serde(flatten)]
    pub subsidy_argument_text: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidity {
    #[serde(flatten)]
    pub silviculture_validity: SilvicultureValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: BdtWorkingSitePlanningOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate {
    #[serde(flatten)]
    pub tree_stand_data_date: TsTreeStandDataDate2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceList {
    #[serde(flatten)]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: VirtaCultivationMaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptance {
    #[serde(flatten)]
    pub business_acceptance: BusinessAcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationDeliveryDueDate {
    #[serde(flatten)]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidy {
    #[serde(flatten)]
    pub subsidy: SubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationAdjustment {
    #[serde(flatten)]
    pub diameter_calibration_adjustment: BdtInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometries {
    #[serde(flatten)]
    pub decision_geometries: DecisionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusSmsOperatorStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPractice {
    #[serde(flatten)]
    pub cutting_realization_practice: CoCuttingRealizationPracticeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: BdtStoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    #[serde(flatten)]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerActionApproval {
    #[serde(flatten)]
    pub owner_action_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: JhsNeljasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethodCorrectText {
    #[serde(flatten)]
    pub soil_conditioning_method_correct_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocation {
    #[serde(flatten)]
    pub ditch_cleaning_break_location: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payees {
    #[serde(flatten)]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductKey {
    #[serde(flatten)]
    pub product_key: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignData {
    #[serde(flatten)]
    pub harvesting_sign_data: HarvestingSignDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructure {
    #[serde(flatten)]
    pub road_structure: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidenceLength {
    #[serde(flatten)]
    pub vehicle_path_subsidence_length: Decimal3_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurements {
    #[serde(flatten)]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: BdtFeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueSurplus {
    #[serde(flatten)]
    pub expected_value_surplus: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Length {
    #[serde(flatten)]
    pub length: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingFinishedDate {
    #[serde(flatten)]
    pub cutting_finished_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibility {
    #[serde(flatten)]
    pub transport_accessibility: CoTransportAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDate {
    #[serde(flatten)]
    pub power_of_attorney_date: FccPowerOfAttorneyDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Month {
    #[serde(flatten)]
    pub month: WctMonthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: VirtaTreeDecimalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalTidinessNoticed {
    #[serde(flatten)]
    pub environmental_tidiness_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingWorkingSiteFinalAuditStumpLiftingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: CoIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeType {
    #[serde(flatten)]
    pub dead_tree_type: DeadTreeTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpecies {
    #[serde(flatten)]
    pub other_tree_species: OtherTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3damageCount {
    #[serde(flatten)]
    pub class3damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeHeight {
    #[serde(flatten)]
    pub tree_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibility {
    #[serde(flatten)]
    pub harvesting_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledArea {
    #[serde(flatten)]
    pub fulfilled_area: FulfilledAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomass {
    #[serde(flatten)]
    pub stump_biomass: StumpBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSites {
    #[serde(flatten)]
    pub working_sites: WorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationWorkingSiteQualityControlFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: CoTreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometry {
    #[serde(flatten)]
    pub financing_act_application_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumes {
    #[serde(flatten)]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerName {
    #[serde(flatten)]
    pub measurer_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceLog {
    #[serde(flatten)]
    pub spruce_log: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessages {
    #[serde(flatten)]
    pub error_messages: ErrorMessagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityDateEnd {
    #[serde(flatten)]
    pub validity_date_end: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentReference {
    #[serde(flatten)]
    pub payment_reference: PaymentsReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseList {
    #[serde(flatten)]
    pub fee_base_list: FeeBaseListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountTarget {
    #[serde(flatten)]
    pub stem_count_target: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: JhsMaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: JhsSiviilisaatyTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfulfilledArea {
    #[serde(flatten)]
    pub unfulfilled_area: GdtPolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataId {
    #[serde(flatten)]
    pub data_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingArea {
    #[serde(flatten)]
    pub working_area: WorkingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: FccLocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: DocumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectBasicData {
    #[serde(flatten)]
    pub control_object_basic_data: ControlObjectBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffectText {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationApplicant {
    #[serde(flatten)]
    pub compensation_applicant: ContactInformationBankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffer {
    #[serde(flatten)]
    pub call_for_offer: CallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManual {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualWorkingSiteHarvestingQualityControlManualType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSourceCode {
    #[serde(flatten)]
    pub damage_source_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousDeclaration {
    #[serde(flatten)]
    pub update_previous_declaration: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSummary {
    #[serde(flatten)]
    pub basal_area_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartLoadNumber {
    #[serde(flatten)]
    pub start_load_number: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulk {
    #[serde(flatten)]
    pub stem_type_bulk: StemTypeBulkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystem {
    #[serde(flatten)]
    pub quality_system: BdtQualitySystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Percentage {
    #[serde(flatten)]
    pub percentage: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCode {
    #[serde(flatten)]
    pub other_habitat_code: CoOtherHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeparateSpareTrees {
    #[serde(flatten)]
    pub separate_spare_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultValue {
    #[serde(flatten)]
    pub default_value: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActingDate {
    #[serde(flatten)]
    pub acting_date: ActingDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCode {
    #[serde(flatten)]
    pub user_code: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: WtcoSellerRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub contact_person: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatAdvertisement {
    #[serde(flatten)]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalStemCount {
    #[serde(flatten)]
    pub goal_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orientation {
    #[serde(flatten)]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: CddShapeGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringType {
    #[serde(flatten)]
    pub self_monitoring_type: CoSelfMonitoringTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary {
    #[serde(flatten)]
    pub tree_stand_summary: TssTreeStandSummary2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjects {
    #[serde(flatten)]
    pub parent_objects: ParentObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: BdtSpareTreeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentWorkingSiteVolume {
    #[serde(flatten)]
    pub sent_working_site_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damages {
    #[serde(flatten)]
    pub damages: DamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audition {
    #[serde(flatten)]
    pub audition: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiability {
    #[serde(flatten)]
    pub cutting_planner_liability: CuttingPlannerLiabilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: JhsHuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListItem {
    #[serde(flatten)]
    pub fee_basis_list_item: FeebasisListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interrupted {
    #[serde(flatten)]
    pub interrupted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: JhsAlkuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedPlantsErrorCount {
    #[serde(flatten)]
    pub damaged_plants_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbols {
    #[serde(flatten)]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: JhsTurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethodCorrect {
    #[serde(flatten)]
    pub soil_conditioning_method_correct: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamage {
    #[serde(flatten)]
    pub previous_moose_damage: PreviousMooseDamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute4 {
    #[serde(flatten)]
    pub attribute4: BdtWorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicityType {
    #[serde(flatten)]
    pub call_for_offer_with_publicity_type: CallForOfferWithPublicity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformation {
    #[serde(flatten)]
    pub authorizations_to_send_wso_information: AuthorizationsToSendWsoInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: StbAreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningQuality {
    #[serde(flatten)]
    pub soil_conditioning_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotification {
    #[serde(flatten)]
    pub forwarding_notification: ForwardingNotificationForwardingNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCareInfo {
    #[serde(flatten)]
    pub employee_health_care_info: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    #[serde(flatten)]
    pub action_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: JhsIkaluokkaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluations {
    #[serde(flatten)]
    pub self_monitoring_evaluations: SelfMonitoringEvaluationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumpsText {
    #[serde(flatten)]
    pub high_stumps_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinuousCoverForestry {
    #[serde(flatten)]
    pub continuous_cover_forestry: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlan {
    #[serde(flatten)]
    pub include_payment_plan: WtcoIncludePaymentPlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationCommitment {
    #[serde(flatten)]
    pub declaration_regeneration_commitment: CoRegenerationCommitmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibration {
    #[serde(flatten)]
    pub diameter_calibration: DiameterCalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributes {
    #[serde(flatten)]
    pub tree_species_attributes: TreeSpeciesAttributesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    #[serde(flatten)]
    pub attribute_value: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub terrain_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: CoOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvesting {
    #[serde(flatten)]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingWorkingSiteFinalAuditHarvestingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannedForHarvestingDate {
    #[serde(flatten)]
    pub working_site_planned_for_harvesting_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_wood_trade_info: WtcoCallForOfferWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislation {
    #[serde(flatten)]
    pub compensation_by_legislation: CompensationByLegislationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeVolume {
    #[serde(flatten)]
    pub change_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockState {
    #[serde(flatten)]
    pub previous_block_state: BdtPreviousBlockStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemName {
    #[serde(flatten)]
    pub external_system_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: JhsPostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowth {
    #[serde(flatten)]
    pub volume_growth: VolumeGrowthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlledOperationType {
    #[serde(flatten)]
    pub controlled_operation_type: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingControlRequired {
    #[serde(flatten)]
    pub cutting_control_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyMode {
    #[serde(flatten)]
    pub company_mode: BdtCompanyModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataText {
    #[serde(flatten)]
    pub metadata_text: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StStandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson1 {
    #[serde(flatten)]
    pub extra_person1: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanVolume {
    #[serde(flatten)]
    pub mean_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageData {
    #[serde(flatten)]
    pub error_message_data: ErrorMessageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operator {
    #[serde(flatten)]
    pub operator: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScaping {
    #[serde(flatten)]
    pub land_scaping: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoniness {
    #[serde(flatten)]
    pub restriction_based_on_stoniness: CoRestrictionBasedOnStoninessType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(flatten)]
    pub child_object_type: CoObjectTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSites {
    #[serde(flatten)]
    pub call_for_offer_working_sites: CallForOfferWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamageCount {
    #[serde(flatten)]
    pub root_damage_count: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationUnitPrice {
    #[serde(flatten)]
    pub application_unit_price: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: JhsKansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: VirtaHabitatTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationReference {
    #[serde(flatten)]
    pub completion_declaration_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraFinancingApplication {
    #[serde(flatten)]
    pub extra_financing_application: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectData {
    #[serde(flatten)]
    pub self_monitoring_object_data: SelfMonitoringObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValue {
    #[serde(flatten)]
    pub wood_lot_information_value: WoodLotInformationValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRange {
    #[serde(flatten)]
    pub load_range: LoadRangeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocationText {
    #[serde(flatten)]
    pub ditch_cleaning_break_location_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: DueDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffersType {
    #[serde(flatten)]
    pub call_for_offers_type: CallForOffers,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolume {
    #[serde(flatten)]
    pub assortment_matrix_volume: AssortmentMatrixVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterHardWood {
    #[serde(flatten)]
    pub mean_diameter_hard_wood: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidy {
    #[serde(flatten)]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWIthSeedlings {
    #[serde(flatten)]
    pub stocking_w_ith_seedlings: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase2youngCropCount {
    #[serde(flatten)]
    pub phase2young_crop_count: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceOrOtherCompensation {
    #[serde(flatten)]
    pub insurance_or_other_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetNumber {
    #[serde(flatten)]
    pub target_number: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathMeanWidth {
    #[serde(flatten)]
    pub vehicle_path_mean_width: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationWorkingSiteTravelNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersLogsInfo {
    #[serde(flatten)]
    pub sellers_logs_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManipulationMethod {
    #[serde(flatten)]
    pub manipulation_method: BdtWorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFile {
    #[serde(flatten)]
    pub external_file: ExternalFileExternalFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: BdtVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(flatten)]
    pub language: BdtLanguageCode1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: JhsCareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationAmountUnit {
    #[serde(flatten)]
    pub application_amount_unit: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountPine {
    #[serde(flatten)]
    pub stem_count_pine: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(flatten)]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreNotificationAllowed {
    #[serde(flatten)]
    pub pre_notification_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomass {
    #[serde(flatten)]
    pub stem_biomass: StemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TooHeightStumps {
    #[serde(flatten)]
    pub too_height_stumps: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomassText {
    #[serde(flatten)]
    pub remaining_biomass_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusTimestamp {
    #[serde(flatten)]
    pub status_timestamp: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    #[serde(flatten)]
    pub training: TrainingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActor {
    #[serde(flatten)]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMaking {
    #[serde(flatten)]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingWorkingSiteFinalAuditRoadMakingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreas {
    #[serde(flatten)]
    pub working_areas: WorkingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: BdtTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureQualityText {
    #[serde(flatten)]
    pub silviculture_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthClass {
    #[serde(flatten)]
    pub length_class: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnWeight {
    #[serde(flatten)]
    pub own_weight: BdtPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3DamageCount {
    #[serde(flatten)]
    pub class3_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInfo {
    #[serde(flatten)]
    pub request_info: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSilverBirch {
    #[serde(flatten)]
    pub basal_area_silver_birch: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantity {
    #[serde(flatten)]
    pub absolute_quantity: AbsoluteQuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: SamplePlotTreesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathMeanDistance {
    #[serde(flatten)]
    pub vehicle_path_mean_distance: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandStemDamagesPercentage {
    #[serde(flatten)]
    pub stand_stem_damages_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfo {
    #[serde(flatten)]
    pub stand_wood_trade_info: StandWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDate {
    #[serde(flatten)]
    pub offer_expiration_date: OfferExpirationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperations {
    #[serde(flatten)]
    pub declaration_other_operations: DeclarationOtherOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningDepth {
    #[serde(flatten)]
    pub soil_conditioning_depth: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidity {
    #[serde(flatten)]
    pub f_s_f_validity: FSFValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantEvaluation {
    #[serde(flatten)]
    pub plant_evaluation: VirtaPlantEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalAccidentInsurance {
    #[serde(flatten)]
    pub legal_accident_insurance: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlock {
    #[serde(flatten)]
    pub previous_block: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmountUnit {
    #[serde(flatten)]
    pub material_amount_unit: CoMaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub offer_working_site_wood_trade_info: OfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: CoFinancingActFinancingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationUserInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingMethod {
    #[serde(flatten)]
    pub cutting_method: OpCuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPersonText {
    #[serde(flatten)]
    pub extra_person_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOffer {
    #[serde(flatten)]
    pub included_in_offer: IncludedInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluation {
    #[serde(flatten)]
    pub control_evaluation: ControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingEstimation {
    #[serde(flatten)]
    pub clearing_estimation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestroyedCuttingValue {
    #[serde(flatten)]
    pub destroyed_cutting_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactors {
    #[serde(flatten)]
    pub scale_factors: ScaleFactorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVolume {
    #[serde(flatten)]
    pub total_volume: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeName {
    #[serde(flatten)]
    pub code_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountLeft {
    #[serde(flatten)]
    pub amount_left: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: CoForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainApplier {
    #[serde(flatten)]
    pub main_applier: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(flatten)]
    pub work: WorkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygon {
    #[serde(flatten)]
    pub declaration_polygon: DeclarationPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delivered {
    #[serde(flatten)]
    pub delivered: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamageOutsideStand {
    #[serde(flatten)]
    pub tree_damage_outside_stand: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: WctFinalAuditSpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureContractNumber {
    #[serde(flatten)]
    pub silviculture_contract_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperNumber {
    #[serde(flatten)]
    pub hopper_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureText {
    #[serde(flatten)]
    pub road_structure_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkPointCount {
    #[serde(flatten)]
    pub work_point_count: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoadId {
    #[serde(flatten)]
    pub partitial_load_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstates {
    #[serde(flatten)]
    pub declaration_real_estates: DeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonReference {
    #[serde(flatten)]
    pub declaration_polygon_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDate {
    #[serde(flatten)]
    pub payment_permission_date: PaymentPermissionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryInfo {
    #[serde(flatten)]
    pub delivery_info: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionData {
    #[serde(flatten)]
    pub restriction_data: StRestrictionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectProtectionOperationsData {
    #[serde(flatten)]
    pub self_monitoring_object_protection_operations_data: SelfMonitoringObjectProtectionOperationsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticedText {
    #[serde(flatten)]
    pub water_economy_system_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceCompany {
    #[serde(flatten)]
    pub insurance_company: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSites {
    #[serde(flatten)]
    pub offer_working_sites: OfferWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperation {
    #[serde(flatten)]
    pub habitat_operation: CoHabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OWorkingSite {
    #[serde(flatten)]
    pub o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogVolume {
    #[serde(flatten)]
    pub log_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksDueDate {
    #[serde(flatten)]
    pub works_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCode {
    #[serde(flatten)]
    pub evaluation_code: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(flatten)]
    pub role: OrganizationRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaNumber {
    #[serde(flatten)]
    pub test_area_number: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassified {
    #[serde(flatten)]
    pub assortment_volume_unclassified: AssortmentVolumeUnclassifiedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publicity {
    #[serde(flatten)]
    pub publicity: CoPublicityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A1 {
    #[serde(flatten)]
    pub a1: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNo {
    #[serde(flatten)]
    pub area_no: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScapingText {
    #[serde(flatten)]
    pub land_scaping_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(flatten)]
    pub attachment: AttachmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControl {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlWorkingSiteHarvestingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResource {
    #[serde(flatten)]
    pub audition_resource: AuditionResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: WctTaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargeSummaryReportRequired {
    #[serde(flatten)]
    pub large_summary_report_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCases {
    #[serde(flatten)]
    pub use_cases: ForestDataUpdateUseCasesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDeliveringEvaluation {
    #[serde(flatten)]
    pub declaration_delivering_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstate {
    #[serde(flatten)]
    pub payee_and_real_estate: PayeeAndRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasAssortmentChanges {
    #[serde(flatten)]
    pub has_assortment_changes: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationType {
    #[serde(flatten)]
    pub notification_type: NotificationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(flatten)]
    pub label: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreements {
    #[serde(flatten)]
    pub collective_agreements: CollectiveAgreementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDate {
    #[serde(flatten)]
    pub felling_right_validity_date: FellingRightValidityDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: CddShapeDeltaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WtcoWorkingSiteGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationId {
    #[serde(flatten)]
    pub registration_id: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedArea {
    #[serde(flatten)]
    pub announced_area: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogVolume {
    #[serde(flatten)]
    pub control_log_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionFileSendFrequency {
    #[serde(flatten)]
    pub production_file_send_frequency: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryType {
    #[serde(flatten)]
    pub material_delivery_type: MaterialDeliveryTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLots {
    #[serde(flatten)]
    pub wood_lots: WoodLotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Productivity {
    #[serde(flatten)]
    pub productivity: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystems {
    #[serde(flatten)]
    pub quality_systems: QualitySystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation1 {
    #[serde(flatten)]
    pub organisation1: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChanges {
    #[serde(flatten)]
    pub assortments_changes: AssortmentsChangesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    #[serde(flatten)]
    pub file_type: FileTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCode {
    #[serde(flatten)]
    pub request_code: RequestCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPulpwoodVolume {
    #[serde(flatten)]
    pub total_pulpwood_volume: CoPulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantLocationErrorCount {
    #[serde(flatten)]
    pub plant_location_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StbStandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadCount {
    #[serde(flatten)]
    pub load_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlLogLength {
    #[serde(flatten)]
    pub control_log_length: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitValue {
    #[serde(flatten)]
    pub unit_value: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilviculture {
    #[serde(flatten)]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureWorkingSiteFinalAuditSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityInsurance {
    #[serde(flatten)]
    pub liability_insurance: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(flatten)]
    pub resource_type: BdtResourceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlaceText {
    #[serde(flatten)]
    pub passing_place_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingLotIdentifier {
    #[serde(flatten)]
    pub seedling_lot_identifier: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDate {
    #[serde(flatten)]
    pub create_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bulk {
    #[serde(flatten)]
    pub bulk: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPrice {
    #[serde(flatten)]
    pub total_price: WtcTotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelated {
    #[serde(flatten)]
    pub cutting_related: CuttingRelatedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: BdtHarvestingStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsurance {
    #[serde(flatten)]
    pub compensation_by_insurance: CompensationByInsuranceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FaaFinancingActApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipType {
    #[serde(flatten)]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumps {
    #[serde(flatten)]
    pub high_stumps: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    #[serde(flatten)]
    pub district: BdtThinningDistrictType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpRaising {
    #[serde(flatten)]
    pub stump_raising: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroup {
    #[serde(flatten)]
    pub forest_owner_group: ForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageData {
    #[serde(flatten)]
    pub damage_data: DamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationDescription {
    #[serde(flatten)]
    pub evaluation_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningWorkingSiteQualityControlSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessage {
    #[serde(flatten)]
    pub original_message: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataRegeneration {
    #[serde(flatten)]
    pub control_data_regeneration: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: JhsLajiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogLength {
    #[serde(flatten)]
    pub log_length: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursSunday {
    #[serde(flatten)]
    pub working_hours_sunday: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDate {
    #[serde(flatten)]
    pub operation_date: CoDateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub financing_act_application_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjects {
    #[serde(flatten)]
    pub financing_act_completion_other_subjects: FinancingActCompletionOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    #[serde(flatten)]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignControlClassifier {
    #[serde(flatten)]
    pub harvesting_sign_control_classifier: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialSupplier {
    #[serde(flatten)]
    pub material_supplier: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceType {
    #[serde(flatten)]
    pub control_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMax {
    #[serde(flatten)]
    pub length_max: LengthMaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedTrees {
    #[serde(flatten)]
    pub harvested_trees: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomControlStemRejectedReason {
    #[serde(flatten)]
    pub random_control_stem_rejected_reason: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: BdtAssortmentGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: BdtTurningPointClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadStemCount {
    #[serde(flatten)]
    pub dead_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkQuality {
    #[serde(flatten)]
    pub work_quality: VirtaWorkQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationReason {
    #[serde(flatten)]
    pub diameter_calibration_reason: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: JhsLoppuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationAdjustment {
    #[serde(flatten)]
    pub calibration_adjustment: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomass {
    #[serde(flatten)]
    pub leaf_biomass: LeafBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainDamage {
    #[serde(flatten)]
    pub main_damage: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub tree_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttribute {
    #[serde(flatten)]
    pub tree_species_attribute: TreeSpeciesAttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDescription {
    #[serde(flatten)]
    pub operation_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: JhsKolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibration {
    #[serde(flatten)]
    pub length_calibration: LengthCalibrationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: BdtWorkingSitePlanningStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructed {
    #[serde(flatten)]
    pub stump_cutting_as_instructed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpCuttingAsInstructedText {
    #[serde(flatten)]
    pub stump_cutting_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedStartDate {
    #[serde(flatten)]
    pub estimated_start_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationDate {
    #[serde(flatten)]
    pub diameter_calibration_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetails {
    #[serde(flatten)]
    pub offer_working_site_details: OfferWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingFinishedDate {
    #[serde(flatten)]
    pub harvesting_finished_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClass {
    #[serde(flatten)]
    pub removal_class: CoRemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedWorkAmount {
    #[serde(flatten)]
    pub planned_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub is_forest_haulage_distance_continued: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListItem {
    #[serde(flatten)]
    pub fee_base_list_item: FeebaseListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethodsText {
    #[serde(flatten)]
    pub other_conservation_methods_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItem {
    #[serde(flatten)]
    pub tree_list_item: TliTreeListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(flatten)]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareOfOwnerShip {
    #[serde(flatten)]
    pub share_of_owner_ship: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationContactPerson {
    #[serde(flatten)]
    pub notification_contact_person: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommercialName {
    #[serde(flatten)]
    pub commercial_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlers {
    #[serde(flatten)]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxDebt {
    #[serde(flatten)]
    pub tax_debt: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadVolume {
    #[serde(flatten)]
    pub load_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Justification {
    #[serde(flatten)]
    pub justification: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSealingErrorCount {
    #[serde(flatten)]
    pub plant_sealing_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeLeft {
    #[serde(flatten)]
    pub volume_left: BdtDecimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfo {
    #[serde(flatten)]
    pub stand_silviculture_info: StandSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDate {
    #[serde(flatten)]
    pub contract_beginning_date: ContractBeginningDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub results_of_accessibility_analysis: ResultsOfAccessibilityAnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: JhsKuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: JhsKatuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumber {
    #[serde(flatten)]
    pub stratum_number: StratumNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCode {
    #[serde(flatten)]
    pub country_code: CoISO3166char2CountryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRemarks {
    #[serde(flatten)]
    pub other_remarks: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2damageCount {
    #[serde(flatten)]
    pub class2damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningQualityText {
    #[serde(flatten)]
    pub soil_conditioning_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeatures {
    #[serde(flatten)]
    pub control_data_special_features: ControlDataSpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICEName {
    #[serde(flatten)]
    pub i_c_e_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamages {
    #[serde(flatten)]
    pub previous_moose_damages: PreviousMooseDamagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanlinessClass {
    #[serde(flatten)]
    pub cleanliness_class: BdtCleanlinessClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAccounting {
    #[serde(flatten)]
    pub final_accounting: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainage {
    #[serde(flatten)]
    pub road_structure_drainage: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Covered {
    #[serde(flatten)]
    pub covered: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartStatus {
    #[serde(flatten)]
    pub target_part_status: VirtaTargetPartStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RectificationDemand {
    #[serde(flatten)]
    pub rectification_demand: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswerAdditionalText {
    #[serde(flatten)]
    pub question_answer_additional_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: JhsVakinainenKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlace {
    #[serde(flatten)]
    pub turning_place: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentageTotal {
    #[serde(flatten)]
    pub dis_qualification_percentage_total: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectionType {
    #[serde(flatten)]
    pub selection_type: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidies {
    #[serde(flatten)]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Startdate {
    #[serde(flatten)]
    pub startdate: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessages {
    #[serde(flatten)]
    pub common_messages: CommonMessagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgVehiclePathDistance {
    #[serde(flatten)]
    pub stand_avg_vehicle_path_distance: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSites {
    #[serde(flatten)]
    pub contract_working_sites: ContractWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepresentativePerson {
    #[serde(flatten)]
    pub representative_person: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TsTreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlace {
    #[serde(flatten)]
    pub passing_place: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensations {
    #[serde(flatten)]
    pub total_compensations: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContarctorId {
    #[serde(flatten)]
    pub contarctor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstruction {
    #[serde(flatten)]
    pub control_data_forest_road_construction: ControlDataForestRoadConstructionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub area_type: AreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeftText {
    #[serde(flatten)]
    pub save_trees_left_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: BdtMeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: JhsVoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1damageCount {
    #[serde(flatten)]
    pub class1damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalSender {
    #[serde(flatten)]
    pub original_sender: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: CddMinimumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescription {
    #[serde(flatten)]
    pub cost_type_description: FccCostTypeDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Woodlot {
    #[serde(flatten)]
    pub woodlot: WoodLotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeName {
    #[serde(flatten)]
    pub attribute_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertileType {
    #[serde(flatten)]
    pub fertile_type: BdtMaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYear {
    #[serde(flatten)]
    pub planning_year: PlanningYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OffersType {
    #[serde(flatten)]
    pub offers_type: Offers,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VAT {
    #[serde(flatten)]
    pub vat: VATType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machines {
    #[serde(flatten)]
    pub machines: MachinesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearing {
    #[serde(flatten)]
    pub responsible_of_pre_clearing: WctResponsibleOfPreClearingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_status: BdtOrderStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalInspectionId {
    #[serde(flatten)]
    pub internal_inspection_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamagesText {
    #[serde(flatten)]
    pub tree_or_ground_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamages {
    #[serde(flatten)]
    pub stem_damages: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelinationObjectOrderId {
    #[serde(flatten)]
    pub delination_object_order_id: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendar {
    #[serde(flatten)]
    pub week_calendar: WeekCalendarWeekCalendarType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierId {
    #[serde(flatten)]
    pub subsidy_applier_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStand {
    #[serde(flatten)]
    pub declaration_stand: DeclarationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructed {
    #[serde(flatten)]
    pub drain_storage_as_instructed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelStartTime {
    #[serde(flatten)]
    pub travel_start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson2 {
    #[serde(flatten)]
    pub extra_person2: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hours {
    #[serde(flatten)]
    pub hours: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photographer {
    #[serde(flatten)]
    pub photographer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum {
    #[serde(flatten)]
    pub tree_stratum: TreeStratum2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandTextInformation {
    #[serde(flatten)]
    pub declaration_stand_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCost {
    #[serde(flatten)]
    pub evaluation_cost: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationHandlingInstructions {
    #[serde(flatten)]
    pub certification_handling_instructions: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrata {
    #[serde(flatten)]
    pub stem_distribution_strata: StemDistributionStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductGroupName {
    #[serde(flatten)]
    pub product_group_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub forest_centre_message_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(flatten)]
    pub depth: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub offer_working_site_payment_transactions: WtcoOfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutOfServiceEndDate {
    #[serde(flatten)]
    pub out_of_service_end_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystem {
    #[serde(flatten)]
    pub certification_system: WtcoCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WtcoWorkingSitePlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentage {
    #[serde(flatten)]
    pub dis_qualification_percentage: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: CoStatisticsPurchaseModeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperation {
    #[serde(flatten)]
    pub object_protection_operation: ObjectProtectionOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteState {
    #[serde(flatten)]
    pub complete_state: CoCompleteStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethods {
    #[serde(flatten)]
    pub preferred_contacting_methods: PreferredContactingMethodsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructedText {
    #[serde(flatten)]
    pub airdrome_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClass {
    #[serde(flatten)]
    pub height_class: CoHeightClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_silviculture_info: CallForOfferSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreement {
    #[serde(flatten)]
    pub collective_agreement: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluation {
    #[serde(flatten)]
    pub preclearing_evaluation: CoPreclearingEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAudit {
    #[serde(flatten)]
    pub fertilization_total_audit: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KemeraMunicipalityId {
    #[serde(flatten)]
    pub kemera_municipality_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDescription {
    #[serde(flatten)]
    pub action_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnoverMoundsCount {
    #[serde(flatten)]
    pub turnover_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceIdMJ {
    #[serde(flatten)]
    pub resource_id_m_j: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(flatten)]
    pub authorization: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatmentText {
    #[serde(flatten)]
    pub stump_treatment_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpMeanDiameter {
    #[serde(flatten)]
    pub stump_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Telephone {
    #[serde(flatten)]
    pub telephone: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObject {
    #[serde(flatten)]
    pub parent_object: ParentObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInformation {
    #[serde(flatten)]
    pub additional_information: AdditionalInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolName {
    #[serde(flatten)]
    pub map_symbol_name: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessages {
    #[serde(flatten)]
    pub status_messages: StatusMessageLanguageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunicationType {
    #[serde(flatten)]
    pub communication_type: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyPercent {
    #[serde(flatten)]
    pub subsidy_percent: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDone {
    #[serde(flatten)]
    pub clearing_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendDate {
    #[serde(flatten)]
    pub send_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: WctQualityOfTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeTreeReduction {
    #[serde(flatten)]
    pub sample_plot_size_tree_reduction: SamplePlotSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageFinished {
    #[serde(flatten)]
    pub storage_finished: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityFreeText {
    #[serde(flatten)]
    pub nationality_free_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationDate {
    #[serde(flatten)]
    pub notification_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandCorrectHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_correct_height_stumps_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: CoSawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(flatten)]
    pub actor: ActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: JhsStatusryhmaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationTimestamp {
    #[serde(flatten)]
    pub location_timestamp: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilImprovementEvaluation {
    #[serde(flatten)]
    pub soil_improvement_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Law {
    #[serde(flatten)]
    pub law: VirtaLawType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyTo {
    #[serde(flatten)]
    pub reply_to: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: BdtMeasurementPlaceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathTooDeepPercentage {
    #[serde(flatten)]
    pub stand_vehicle_path_too_deep_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTrees {
    #[serde(flatten)]
    pub final_audit_spare_trees: FinalAuditSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hopper {
    #[serde(flatten)]
    pub hopper: HopperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsActive {
    #[serde(flatten)]
    pub is_active: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripherals {
    #[serde(flatten)]
    pub peripherals: PeripheralsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureText {
    #[serde(flatten)]
    pub offer_working_site_silviculture_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentText {
    #[serde(flatten)]
    pub payment_text: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandExtraInfo {
    #[serde(flatten)]
    pub stand_extra_info: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier2 {
    #[serde(flatten)]
    pub work_code_qualifier2: BdtWorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistance {
    #[serde(flatten)]
    pub transport_distance: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: CddMaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogM3sum {
    #[serde(flatten)]
    pub log_m3sum: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessage {
    #[serde(flatten)]
    pub status_message: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClass {
    #[serde(flatten)]
    pub document_class: BdtDocumentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantCount {
    #[serde(flatten)]
    pub plant_count: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingArea {
    #[serde(flatten)]
    pub processing_area: ProcessingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamages {
    #[serde(flatten)]
    pub tree_or_ground_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(flatten)]
    pub position: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatus {
    #[serde(flatten)]
    pub project_status: VirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMember {
    #[serde(flatten)]
    pub work_grou_member: WorkGrouMemberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocations {
    #[serde(flatten)]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsServiceBuyerResourceLocationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValue {
    #[serde(flatten)]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityDescription {
    #[serde(flatten)]
    pub seedling_condition_and_quality_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: BdtTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumber {
    #[serde(flatten)]
    pub forest_use_declaration_number: FccForestUseDeclarationNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementName {
    #[serde(flatten)]
    pub key_element_name: CoString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassCollectionDoneText {
    #[serde(flatten)]
    pub bio_mass_collection_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityUnit {
    #[serde(flatten)]
    pub productivity_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SodWorkingSite {
    #[serde(flatten)]
    pub sod_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRow {
    #[serde(flatten)]
    pub round_wood_sales_row: RoundWoodSalesRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: VirtaTreeDecimalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPoint {
    #[serde(flatten)]
    pub supply_point: SupplyPointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionality {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(flatten)]
    pub hopper_type: WctHopperTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticed {
    #[serde(flatten)]
    pub environment_cleanliness_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedCount {
    #[serde(flatten)]
    pub not_damaged_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrassControlEvaluation {
    #[serde(flatten)]
    pub grass_control_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRows {
    #[serde(flatten)]
    pub loggings_rows: LoggingsRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceMass {
    #[serde(flatten)]
    pub control_reference_mass: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationPercentage {
    #[serde(flatten)]
    pub participation_percentage: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedEnergyWood {
    #[serde(flatten)]
    pub detected_energy_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperations {
    #[serde(flatten)]
    pub habitat_operations: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinePulp {
    #[serde(flatten)]
    pub pine_pulp: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNotNeeded {
    #[serde(flatten)]
    pub forest_use_declaration_not_needed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCount {
    #[serde(flatten)]
    pub image_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpsCount {
    #[serde(flatten)]
    pub scalps_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentative {
    #[serde(flatten)]
    pub working_representative: WorkingRepresentativeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemsNoticedText {
    #[serde(flatten)]
    pub water_economy_systems_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PilotName {
    #[serde(flatten)]
    pub pilot_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier5 {
    #[serde(flatten)]
    pub work_code_qualifier5: BdtWorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationAmount {
    #[serde(flatten)]
    pub application_amount: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BIC {
    #[serde(flatten)]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingText {
    #[serde(flatten)]
    pub pre_clearing_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCutting {
    #[serde(flatten)]
    pub clear_cutting: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometry {
    #[serde(flatten)]
    pub financing_act_completion_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProduction {
    #[serde(flatten)]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionWorkingSiteHarvestedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_diameter_other_tree_species: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTooHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_too_height_stumps_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeWrittenAgreement {
    #[serde(flatten)]
    pub employee_written_agreement: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusData {
    #[serde(flatten)]
    pub prevention_fungus_of_the_genus_data: PreventionFungusOfTheGenusDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallation {
    #[serde(flatten)]
    pub pipe_installation: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Silviculture {
    #[serde(flatten)]
    pub silviculture: SilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson3 {
    #[serde(flatten)]
    pub extra_person3: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveIncomplete {
    #[serde(flatten)]
    pub save_incomplete: VirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: SfValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesControl {
    #[serde(flatten)]
    pub special_features_control: ControlDataSpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1Unit {
    #[serde(flatten)]
    pub work_load1_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    #[serde(flatten)]
    pub reason: ReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2Unit {
    #[serde(flatten)]
    pub work_load2_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadInvoiced {
    #[serde(flatten)]
    pub work_load_invoiced: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAmount {
    #[serde(flatten)]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedDensity {
    #[serde(flatten)]
    pub recommended_density: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationDate {
    #[serde(flatten)]
    pub calibration_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerInvolvement {
    #[serde(flatten)]
    pub owner_involvement: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantedSeedlingCountInNotCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_not_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolume {
    #[serde(flatten)]
    pub harvested_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeText {
    #[serde(flatten)]
    pub fee_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseNumber {
    #[serde(flatten)]
    pub case_number: FccFinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionNumber {
    #[serde(flatten)]
    pub decision_number: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActorsType {
    #[serde(rename = "InformedActor")]
    pub informed_actor: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantitiesType {
    #[serde(rename = "StatisticsQuantity")]
    pub statistics_quantity: Vec<StatisticsQuantityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: chrono::NaiveDate,
    #[serde(rename = "EndDate")]
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction1Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YearType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: RestrictionsMainGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: ParcelNumberType,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlLineString")]
    pub gml_line_string: LineString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal2FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManualType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "InfoText")]
    pub info_text: String200Type,
    #[serde(rename = "Measurements")]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfowsWorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<PositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiLastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkresourceType {
    #[serde(rename = "XlinkresourceModel")]
    pub xlinkresource_model: resourceModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdentifierValueType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDurationType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeedType {
    #[serde(flatten)]
    pub base: YesNoSellerResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicDataType {
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<ReferenceType>,
    #[serde(rename = "NonPersonificationId", skip_serializing_if = "Option::is_none")]
    pub non_personification_id: Option<CoString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<CoReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformationType {
    #[serde(rename = "ResponsibleOfPreClearing", skip_serializing_if = "Option::is_none")]
    pub responsible_of_pre_clearing: Option<ResponsibleOfPreClearingType>,
    #[serde(rename = "PreClearingExecutionTime", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_execution_time: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationsType {
    #[serde(rename = "Specification")]
    pub specification: Vec<SpecificationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryObjectType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<String20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "BeginDate")]
    pub begin_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageStateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumesType {
    #[serde(rename = "AssortmentMatrixVolume")]
    pub assortment_matrix_volume: Vec<AssortmentMatrixVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate")]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
    #[serde(rename = "FinancingActApplicationGeometries")]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateMmDdYyyyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationRegenerationOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandNumberExtensionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpSilvicultureTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax1FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrixType {
    #[serde(rename = "PriceItem")]
    pub price_item: Vec<PriceItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaysType {
    #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
    pub day: Option<Vec<DayType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingRealizationPracticeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsuranceType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "InsuranceCompany", skip_serializing_if = "Option::is_none")]
    pub insurance_company: Option<CoString500Type>,
    #[serde(rename = "InsuranceNumber", skip_serializing_if = "Option::is_none")]
    pub insurance_number: Option<CoString100Type>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString2000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlPolygon")]
    pub gml_polygon: Polygon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: String20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ContractCode", skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<String50Type>,
    #[serde(rename = "ValidityDateBegin")]
    pub validity_date_begin: DateType,
    #[serde(rename = "ValidityDateEnd", skip_serializing_if = "Option::is_none")]
    pub validity_date_end: Option<DateType>,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "ContractInfo", skip_serializing_if = "Option::is_none")]
    pub contract_info: Option<String1000Type>,
    #[serde(rename = "MeasureDeviceCheckRequired")]
    pub measure_device_check_required: YesNoType,
    #[serde(rename = "CompanyMode", skip_serializing_if = "Option::is_none")]
    pub company_mode: Option<CompanyModeType>,
    #[serde(rename = "SubContractorsAllowed")]
    pub sub_contractors_allowed: YesNoType,
    #[serde(rename = "WorkingAreas", skip_serializing_if = "Option::is_none")]
    pub working_areas: Option<WorkingAreasType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonOperationExtraQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferTextType {
    #[serde(flatten)]
    pub base: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSahkopostiosoiteTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaGroup {
    #[serde(rename = "ProposalArea", skip_serializing_if = "Option::is_none")]
    pub proposal_area: Option<ProposalArea>,
    #[serde(rename = "ProposalAreaPercent", skip_serializing_if = "Option::is_none")]
    pub proposal_area_percent: Option<ProposalAreaPercent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiersType {
    #[serde(rename = "TreeIdentifier", skip_serializing_if = "Option::is_none")]
    pub tree_identifier: Option<Vec<TreeIdentifierType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    #[serde(flatten)]
    pub base: StbStandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fromType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSubCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceType {
    #[serde(rename = "StorageForestHaulageDistanceGroup")]
    pub storage_forest_haulage_distance_group: StorageForestHaulageDistanceGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSoilConditioningBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoRestrictionBasedOnStoninessType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Value")]
    pub value: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoRoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString500Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesType {
    #[serde(rename = "SpareTreeCategory")]
    pub spare_tree_category: SpareTreeCategoryType,
    #[serde(rename = "AmountOfSpareTrees")]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "MeanDiameterOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_of_spare_trees: Option<DiameterType>,
    #[serde(rename = "MeanHeightOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_height_of_spare_trees: Option<HeightType>,
    #[serde(rename = "VolumeOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub volume_of_spare_trees: Option<VolumeType>,
    #[serde(rename = "DiameterClassOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub diameter_class_of_spare_trees: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeasurementCertificateTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementsType {
    #[serde(rename = "Measurement")]
    pub measurement: Vec<MeasurementDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDataType {
    #[serde(rename = "MeasurementId")]
    pub measurement_id: PositiveIntegerType,
    #[serde(rename = "Measurer")]
    pub measurer: String50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "SelectionType")]
    pub selection_type: String10Type,
    #[serde(rename = "Temperature")]
    pub temperature: i32,
    #[serde(rename = "ProductKey")]
    pub product_key: ERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: PositiveInteger5digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: PositiveInteger5digitsType,
    #[serde(rename = "LogCount")]
    pub log_count: PositiveInteger2digitsType,
    #[serde(rename = "ControlLogCount")]
    pub control_log_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtQualitySystemType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSuggestionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(rename = "InternalInspectionId")]
    pub internal_inspection_id: String,
    #[serde(rename = "VirtaId")]
    pub virta_id: VirtaIdType,
    #[serde(rename = "InspectionType", skip_serializing_if = "Option::is_none")]
    pub inspection_type: Option<VirtaInspectionTypeType>,
    #[serde(rename = "SaveIncomplete", skip_serializing_if = "Option::is_none")]
    pub save_incomplete: Option<VirtaSaveIncompleteType>,
    #[serde(rename = "Status1", skip_serializing_if = "Option::is_none")]
    pub status1: Option<CoChangeStateType>,
    #[serde(rename = "AnnouncementId", skip_serializing_if = "Option::is_none")]
    pub announcement_id: Option<AnnouncementIdType>,
    #[serde(rename = "KemeraId", skip_serializing_if = "Option::is_none")]
    pub kemera_id: Option<VirtaIdType>,
    #[serde(rename = "EstateOwner", skip_serializing_if = "Option::is_none")]
    pub estate_owner: Option<string>,
    #[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub municipality_number: Option<JhsKuntaKoodiTyyppi>,
    #[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<string>,
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<string>,
    #[serde(rename = "UnseparatedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel_number: Option<string>,
    #[serde(rename = "UnseparatedParcel", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel: Option<string>,
    #[serde(rename = "KemeraMunicipalityId", skip_serializing_if = "Option::is_none")]
    pub kemera_municipality_id: Option<string>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<VirtaTargetSelectionType>,
    #[serde(rename = "Population", skip_serializing_if = "Option::is_none")]
    pub population: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "MastoInspection", skip_serializing_if = "Option::is_none")]
    pub masto_inspection: Option<VirtaMastoInspectionType>,
    #[serde(rename = "WorkType", skip_serializing_if = "Option::is_none")]
    pub work_type: Option<CoPositiveInteger2digitsType>,
    #[serde(rename = "Phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<VirtaPhaseType>,
    #[serde(rename = "ProjectStatus", skip_serializing_if = "Option::is_none")]
    pub project_status: Option<VirtaProjectStatusType>,
    #[serde(rename = "AnnouncedArea", skip_serializing_if = "Option::is_none")]
    pub announced_area: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AnnouncedLength", skip_serializing_if = "Option::is_none")]
    pub announced_length: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Law", skip_serializing_if = "Option::is_none")]
    pub law: Option<VirtaLawType>,
    #[serde(rename = "EarliestInspectionDate", skip_serializing_if = "Option::is_none")]
    pub earliest_inspection_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "Advertiser", skip_serializing_if = "Option::is_none")]
    pub advertiser: Option<VirtaAdvertiserType>,
    #[serde(rename = "Operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<VirtaAdvertiserType>,
    #[serde(rename = "AdvertisementDating", skip_serializing_if = "Option::is_none")]
    pub advertisement_dating: Option<VirtaAdvertisementDatingType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "TreeDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand: Option<SpVirtaEvaluationType>,
    #[serde(rename = "TerrainDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand: Option<SpVirtaEvaluationType>,
    #[serde(rename = "InspectionDate", skip_serializing_if = "Option::is_none")]
    pub inspection_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<TgtVirtaExtraInfoType>,
    #[serde(rename = "HarvestExtraInfo", skip_serializing_if = "Option::is_none")]
    pub harvest_extra_info: Option<TgtVirtaExtraInfoType>,
    #[serde(rename = "SumTableArea", skip_serializing_if = "Option::is_none")]
    pub sum_table_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "StubPriceArea", skip_serializing_if = "Option::is_none")]
    pub stub_price_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "UnitCostArea", skip_serializing_if = "Option::is_none")]
    pub unit_cost_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "EvaluationCost", skip_serializing_if = "Option::is_none")]
    pub evaluation_cost: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "InsuranceOrOtherCompensation", skip_serializing_if = "Option::is_none")]
    pub insurance_or_other_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "OwnerInvolvement", skip_serializing_if = "Option::is_none")]
    pub owner_involvement: Option<CoVirtaYesNoType>,
    #[serde(rename = "AssociationInvolvement", skip_serializing_if = "Option::is_none")]
    pub association_involvement: Option<CoVirtaYesNoType>,
    #[serde(rename = "OwnerSampleAreaApproval", skip_serializing_if = "Option::is_none")]
    pub owner_sample_area_approval: Option<VirtaApprovalType>,
    #[serde(rename = "OwnerActionApproval", skip_serializing_if = "Option::is_none")]
    pub owner_action_approval: Option<VirtaApprovalType>,
    #[serde(rename = "MoosePercentage", skip_serializing_if = "Option::is_none")]
    pub moose_percentage: Option<CoPercentType>,
    #[serde(rename = "AssociationEvaluationApproval", skip_serializing_if = "Option::is_none")]
    pub association_evaluation_approval: Option<VirtaApprovalType>,
    #[serde(rename = "Targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetsType>,
    #[serde(rename = "HelpGeometries", skip_serializing_if = "Option::is_none")]
    pub help_geometries: Option<HelpGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstructionType {
    #[serde(rename = "AppliedLength")]
    pub applied_length: Vec<Decimal6_2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExtraInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaximumType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String50Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger5digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentWithFraction2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHarvestingSeasonType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataType {
    #[serde(rename = "TrainingId")]
    pub training_id: PositiveInteger4digitsType,
    #[serde(rename = "TrainingFreeText", skip_serializing_if = "Option::is_none")]
    pub training_free_text: Option<String50Type>,
    #[serde(rename = "TrainingDate")]
    pub training_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfBasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMainGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatOperationsType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<FinalAuditSpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataOperationStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrataType {
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Vec<DeadTreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityType {
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorsType {
    #[serde(rename = "ApplicationActor")]
    pub application_actor: Vec<ApplicationActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtThinningDistrictType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CddDistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: DistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMassType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct typeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccPowerOfAttorneyType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: CfowsWorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType1 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformationType {
    #[serde(rename = "AuthorizationToSendWsoInformation")]
    pub authorization_to_send_wso_information: Vec<AuthorizationToSendWsoInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlayksikkoNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclarationType {
    #[serde(rename = "CuttingRealizationPractice", skip_serializing_if = "Option::is_none")]
    pub cutting_realization_practice: Option<CuttingTypeType>,
    #[serde(rename = "HarvestingSignControlClassifier", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_control_classifier: Option<EvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoltosuhdeTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "GdtSimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: SimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xsbase64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDataType {
    #[serde(rename = "DataInformation")]
    pub data_information: DataInformationType,
    #[serde(rename = "Inspection")]
    pub inspection: Vec<InspectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataType {
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: TreeStandBasedDataGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "MooseDamageDeclarationReference")]
    pub moose_damage_declaration_reference: CoReferenceType,
    #[serde(rename = "MooseDamageDate")]
    pub moose_damage_date: CoDateType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<CoString2000Type>,
    #[serde(rename = "CompensationApplicant")]
    pub compensation_applicant: ContactInformationBankAccountType,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "FccAttorney", skip_serializing_if = "Option::is_none")]
    pub fcc_attorney: Option<Attorney>,
    #[serde(rename = "MooseDamageDeclarationRealEstates")]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
    #[serde(rename = "CompensationByInsurance")]
    pub compensation_by_insurance: CompensationByInsuranceType,
    #[serde(rename = "CompensationByLegislation")]
    pub compensation_by_legislation: CompensationByLegislationType,
    #[serde(rename = "PreviousMooseDamages", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damages: Option<PreviousMooseDamagesType>,
    #[serde(rename = "FccDocuments", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoQuantityUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionTypeType {
    #[serde(rename = "base")]
    pub base: String,
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
pub struct CoForestCentreUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<PositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferWoodTradeInfoType {
    #[serde(rename = "PurchaseMode")]
    pub purchase_mode: PurchaseModeType,
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
    #[serde(rename = "IncludeForestFundPayment")]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
    #[serde(rename = "UsedPricingMethods", skip_serializing_if = "Option::is_none")]
    pub used_pricing_methods: Option<UsedPricingMethodsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationOfAttorneyType {
    #[serde(rename = "Authorization")]
    pub authorization: YesNoType,
    #[serde(rename = "Date")]
    pub date: DateType,
    #[serde(rename = "AttorneyReceivesPayment")]
    pub attorney_receives_payment: YesNoType,
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: String2000Type,
    #[serde(rename = "PowerOfAttorneyDocument", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_document: Option<base64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBasicDataType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<CoUseCaseType>,
    #[serde(rename = "ControlNo", skip_serializing_if = "Option::is_none")]
    pub control_no: Option<String100Type>,
    #[serde(rename = "ForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_number: Option<ForestUseDeclarationNumberType>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<FinancingActNumberType>,
    #[serde(rename = "ControlReferenceType", skip_serializing_if = "Option::is_none")]
    pub control_reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "ControlReference", skip_serializing_if = "Option::is_none")]
    pub control_reference: Option<ReferenceType>,
    #[serde(rename = "ControlDate", skip_serializing_if = "Option::is_none")]
    pub control_date: Option<DateType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<TargetSelectionType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerResponsible {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseDynamicType {
    #[serde(rename = "Attributes")]
    pub attributes: AttributesType,
    #[serde(rename = "Audition")]
    pub audition: AuditionType,
    #[serde(rename = "AuditionResources", skip_serializing_if = "Option::is_none")]
    pub audition_resources: Option<AuditionResourcesType>,
    #[serde(rename = "Questions")]
    pub questions: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitesType {
    #[serde(rename = "WorkingSite", skip_serializing_if = "Option::is_none")]
    pub working_site: Option<Vec<WorkingSiteType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTimeStampType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationsType {
    #[serde(rename = "ControlEvaluation")]
    pub control_evaluation: Vec<ControlEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeaturesType {
    #[serde(rename = "ControlDataSpecialFeature")]
    pub control_data_special_feature: Vec<ControlDataSpecialFeatureType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureType {
    #[serde(rename = "SfFeatureDataGroup")]
    pub sf_feature_data_group: FeatureDataGroup,
    #[serde(rename = "SfFeatureInfo", skip_serializing_if = "Option::is_none")]
    pub sf_feature_info: Option<FeatureInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger4digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSubGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentType {
    #[serde(rename = "SubsidyArgumentText")]
    pub subsidy_argument_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseCompactStandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<StbStandNumberType>,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data_date: Option<StbStandBasicDataDateType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: WctERPIdType,
    #[serde(rename = "QuestionAsText", skip_serializing_if = "Option::is_none")]
    pub question_as_text: Option<String200Type>,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: BdtString50Type,
    #[serde(rename = "QuestionAnswerAsText", skip_serializing_if = "Option::is_none")]
    pub question_answer_as_text: Option<String50Type>,
    #[serde(rename = "QuestionAnswerAdditionalText", skip_serializing_if = "Option::is_none")]
    pub question_answer_additional_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringImageType {
    #[serde(flatten)]
    pub base: ImageBaseType,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<ImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<ImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<FeatureCodeType>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<PointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<String50Type>,
    #[serde(rename = "ImageDate")]
    pub image_date: TimeStampType,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: PowerOfAttorneyType,
    #[serde(rename = "RightToSpecifyBankAccountsOfPaymentTransactions")]
    pub right_to_specify_bank_accounts_of_payment_transactions: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuudesRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDensityClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctCommonMessageType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal7And2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "FudrForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<ForestUseDeclarationReference>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AsAssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<AssortmentClasses>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupsType {
    #[serde(rename = "SpareTreeGroup")]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingStemTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType4 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlerType {
    #[serde(rename = "@role")]
    pub role: String100Type,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString100Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OrganizationName")]
    pub organization_name: String100Type,
    #[serde(rename = "RepresentativePerson")]
    pub representative_person: ContactInformationType,
    #[serde(rename = "ContactPersonInFinland")]
    pub contact_person_in_finland: ContactInformationType,
    #[serde(rename = "CompanyType")]
    pub company_type: CompanyTypeType,
    #[serde(rename = "QualitySystems", skip_serializing_if = "Option::is_none")]
    pub quality_systems: Option<QualitySystemsType>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
    #[serde(rename = "TradeRegistration", skip_serializing_if = "Option::is_none")]
    pub trade_registration: Option<DateType>,
    #[serde(rename = "TaxDebt")]
    pub tax_debt: DateType,
    #[serde(rename = "EmployeePensionCertificate", skip_serializing_if = "Option::is_none")]
    pub employee_pension_certificate: Option<DateType>,
    #[serde(rename = "EmployeeHealthCare")]
    pub employee_health_care: YesNoType,
    #[serde(rename = "EmployeeHealthCareInfo", skip_serializing_if = "Option::is_none")]
    pub employee_health_care_info: Option<String100Type>,
    #[serde(rename = "CollectiveAgreements")]
    pub collective_agreements: CollectiveAgreementsType,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "LiabilityInsurance", skip_serializing_if = "Option::is_none")]
    pub liability_insurance: Option<DateType>,
    #[serde(rename = "LegalAccidentInsurance", skip_serializing_if = "Option::is_none")]
    pub legal_accident_insurance: Option<DateType>,
    #[serde(rename = "SubContractorWrittenAgreement")]
    pub sub_contractor_written_agreement: YesNoType,
    #[serde(rename = "EmployeeWrittenAgreement")]
    pub employee_written_agreement: YesNoType,
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: YesNoType,
    #[serde(rename = "PreDebtCollectionRegister")]
    pub pre_debt_collection_register: YesNoType,
    #[serde(rename = "EmployerRegister")]
    pub employer_register: EmployerRegisterType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String50Type>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String20Type>,
    #[serde(rename = "IsActive")]
    pub is_active: YesNoType,
    #[serde(rename = "SubContractors", skip_serializing_if = "Option::is_none")]
    pub sub_contractors: Option<SubContractorsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
    #[serde(rename = "ForestUseDeclarationNotNeeded", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_not_needed: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSceneryWorkPermissionNeededType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfIdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsYritysTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationDevelopmentClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRootRotControlEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStemTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMakingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTurningPointClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityType {
    #[serde(flatten)]
    pub base: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CoBankAccount", skip_serializing_if = "Option::is_none")]
    pub co_bank_account: Option<BankAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDevelopmentClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4TotalDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagementType {
    #[serde(rename = "DitchMeanDepth")]
    pub ditch_mean_depth: String,
    #[serde(rename = "DitchMeanWidth")]
    pub ditch_mean_width: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlOverallEvaluationDataType {
    #[serde(rename = "RestrictionBasedOnStoniness", skip_serializing_if = "Option::is_none")]
    pub restriction_based_on_stoniness: Option<RestrictionBasedOnStoninessType>,
    #[serde(rename = "PreclearingEvaluation", skip_serializing_if = "Option::is_none")]
    pub preclearing_evaluation: Option<PreclearingEvaluationType>,
    #[serde(rename = "DeclarationDeliveringEvaluation", skip_serializing_if = "Option::is_none")]
    pub declaration_delivering_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "TreeDamageOutsideStandEvaluation", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "TerrainDamageOutsideStandEvaluation", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand_evaluation: Option<EvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingDataType {
    #[serde(rename = "SeedlingStratum")]
    pub seedling_stratum: Vec<SeedlingStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyAndPercentType {
    #[serde(rename = "AbsoluteQuantity")]
    pub absolute_quantity: AbsoluteQuantityType,
    #[serde(rename = "Percent")]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataGroup {
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageState>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<MainGroup>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryTextType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRangeType {
    #[serde(rename = "StartLoadNumber")]
    pub start_load_number: PositiveInteger4digitsType,
    #[serde(rename = "EndLoadNumber")]
    pub end_load_number: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccountingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContarctorId")]
    pub contarctor_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "AccountingDate")]
    pub accounting_date: TimeStampType,
    #[serde(rename = "FinalAccounting")]
    pub final_accounting: YesNoType,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaYesNoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatAdvertisementType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHenkiloTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoSellersType {
    #[serde(rename = "@subsidyPossibility")]
    pub subsidy_possibility: YesNoNotKnownType,
    #[serde(rename = "@sellerGroup")]
    pub seller_group: SellerGroupType,
    #[serde(rename = "Seller")]
    pub seller: Vec<SellerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: String20Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "ControlReferenceMass")]
    pub control_reference_mass: Decimal1FractionDigitType,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "ScaleData")]
    pub scale_data: Vec<ScaleDataType>,
    #[serde(rename = "Calibration", skip_serializing_if = "Option::is_none")]
    pub calibration: Option<Vec<CalibrationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnersType {
    #[serde(rename = "ForestOwner")]
    pub forest_owner: Vec<ForestOwnerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: ReRealEstatesWithOwnersInformationType2,
    #[serde(rename = "DeclarationPolygons")]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageDataType {
    #[serde(rename = "Damage")]
    pub damage: Vec<DamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicity {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: Vec<CallForOffer>,
    #[serde(rename = "Publicity")]
    pub publicity: PublicityType,
    #[serde(rename = "PublicityOrganizations", skip_serializing_if = "Option::is_none")]
    pub publicity_organizations: Option<OrganizationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectProtectionOperationsDataType {
    #[serde(rename = "BorderingWithWaterAreaOrStream", skip_serializing_if = "Option::is_none")]
    pub bordering_with_water_area_or_stream: Option<YesNoType>,
    #[serde(rename = "LengthOfDitchDiggedDuringSoilPreparation", skip_serializing_if = "Option::is_none")]
    pub length_of_ditch_digged_during_soil_preparation: Option<PositiveInteger6digitsType>,
    #[serde(rename = "ObjectProtectionOperations", skip_serializing_if = "Option::is_none")]
    pub object_protection_operations: Option<ObjectProtectionOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPankkitiliTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiPersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoMeanHeightType>,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<CoSeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNoType {
    #[serde(flatten)]
    pub base: CoReferenceType,
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
pub struct AssortmentVolumesType {
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: Vec<AssortmentVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSuggestionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaInspectionTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiEmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "DBH")]
    pub dbh: PositiveInteger3digitsType,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiFirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataGroup {
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<StandQuality>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<MainTreeSpecies>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSoilTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccForestCentreMessageReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorsType {
    #[serde(rename = "CompletionActor")]
    pub completion_actor: Vec<CompletionActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
    #[serde(flatten)]
    pub base: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysisType {
    #[serde(rename = "Accessibility")]
    pub accessibility: AccessibilityType,
    #[serde(rename = "Percentage")]
    pub percentage: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<string>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<WtcoDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRegenerationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "LengthCalibrationDate")]
    pub length_calibration_date: TimeStampType,
    #[serde(rename = "LengthCalibrationReason")]
    pub length_calibration_reason: String200Type,
    #[serde(rename = "LengthCalibrationDescription")]
    pub length_calibration_description: String200Type,
    #[serde(rename = "LengthCalibrationAdjustment")]
    pub length_calibration_adjustment: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatePrecipionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType3 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreasType {
    #[serde(rename = "FulfilledArea")]
    pub fulfilled_area: Vec<FulfilledAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTargetSelectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationSubjectType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger6digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "Question")]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstituteType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassifiedType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: String50Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<String50Type>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<String5Type>,
    #[serde(rename = "ProductGroupName", skip_serializing_if = "Option::is_none")]
    pub product_group_name: Option<String50Type>,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationsType {
    #[serde(rename = "SilviculturalOperation")]
    pub silvicultural_operation: Vec<SilviculturalOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDamageQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TliTreeListItemType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TreeNumber", skip_serializing_if = "Option::is_none")]
    pub tree_number: Option<integer>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<CoStoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoAgeType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<CoDiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<CoMeanHeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<CoVolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TreeIdentifiers", skip_serializing_if = "Option::is_none")]
    pub tree_identifiers: Option<TreeIdentifiersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliersType {
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<SubsidyApplierBaseContactAndEstateInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPaymentTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoISO639char2LanguageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkinTyyppi {
    #[serde(rename = "base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "FertilizationTotalAudit")]
    pub fertilization_total_audit: WorkingQualityType,
    #[serde(rename = "FertilizationTotalAuditText", skip_serializing_if = "Option::is_none")]
    pub fertilization_total_audit_text: Option<String200Type>,
    #[serde(rename = "TreeOrGroundDamages")]
    pub tree_or_ground_damages: YesNoType,
    #[serde(rename = "TreeOrGroundDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_or_ground_damages_text: Option<String200Type>,
    #[serde(rename = "EnvironmentCleanlinessNoticed")]
    pub environment_cleanliness_noticed: YesNoType,
    #[serde(rename = "EnvironmentCleanlinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environment_cleanliness_noticed_text: Option<String200Type>,
    #[serde(rename = "DrainStorageAsInstructed")]
    pub drain_storage_as_instructed: YesNoType,
    #[serde(rename = "DrainStorageAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub drain_storage_as_instructed_text: Option<String200Type>,
    #[serde(rename = "AirdromeAsInstructed", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed: Option<YesNoType>,
    #[serde(rename = "AirdromeAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed_text: Option<String200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: YesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<String200Type>,
    #[serde(rename = "WaterEconomySystemNoticed")]
    pub water_economy_system_noticed: YesNoType,
    #[serde(rename = "WaterEconomySystemNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_system_noticed_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection")]
    pub water_system_protection: YesNoType,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: YesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: YesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaProjectStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccCostTypeType2 {
    #[serde(flatten)]
    pub base: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSmallWoodRemovalClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountType {
    #[serde(flatten)]
    pub base: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegisterType {
    #[serde(rename = "Registered")]
    pub registered: YesNoType,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBankReferenceNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTotalEstimationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfFeatureDataGroup {
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestryType {
    #[serde(rename = "BlockIsFSFBlock", skip_serializing_if = "Option::is_none")]
    pub block_is_f_s_f_block: Option<YesNoType>,
    #[serde(rename = "FSFInformation", skip_serializing_if = "Option::is_none")]
    pub f_s_f_information: Option<Vec<FSFInformationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChangesType {
    #[serde(rename = "AssortmentsChange", skip_serializing_if = "Option::is_none")]
    pub assortments_change: Option<Vec<AssortmentChangeDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "LocationTimestamp")]
    pub location_timestamp: TimeStampType,
    #[serde(rename = "Location")]
    pub location: PointGeometryType,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPolygonOrMultiPolygon2Type {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<pointProperty>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocument4MBType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActFinancingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwsWorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<PositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "ReRealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<RealEstates>,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationsType {
    #[serde(rename = "SelfMonitoringEvaluation")]
    pub self_monitoring_evaluation: Vec<SelfMonitoringEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentWithFraction1Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemSelectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsUlkomaaHenkiloTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometriesGroup {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyTransactionCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAmountUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlkuHetkiTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
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
pub struct PublicityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StBaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<BaseSoilDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSyntymaPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinktitleEltType {
    #[serde(rename = "XlinktitleModel")]
    pub xlinktitle_model: titleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger5digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformationType {
    #[serde(rename = "ServiceBuyer")]
    pub service_buyer: Vec<ServiceBuyerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingDirectingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageVersionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger1digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<BaseSoilDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureDataGroup {
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoDocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFinancingActFinancingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString20Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditAnswerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferType {
    #[serde(rename = "RelatedCallForOfferId")]
    pub related_call_for_offer_id: String,
    #[serde(rename = "RelatedCallForOfferDescription", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offer_description: Option<String1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostilokerolyhenneTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockInfoType {
    #[serde(rename = "PreviousBlock", skip_serializing_if = "Option::is_none")]
    pub previous_block: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoMessageTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMaxType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteKirjainTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwnerType {
    #[serde(rename = "CiNameAndOrganizationGroup")]
    pub ci_name_and_organization_group: Vec<NameAndOrganizationGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandsType {
    #[serde(rename = "FinancingActApplicationStand")]
    pub financing_act_application_stand: Vec<FinancingActApplicationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassesType {
    #[serde(rename = "AssortmentClass")]
    pub assortment_class: Vec<AssortmentClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal6TotalDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFertilityClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPeripheralCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString50Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString1500Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompanyTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAndOriginalYearGroup {
    #[serde(rename = "ProposalYear")]
    pub proposal_year: ProposalYear,
    #[serde(rename = "OriginalProposalYear", skip_serializing_if = "Option::is_none")]
    pub original_proposal_year: Option<OriginalProposalYear>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSellerGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: StbAreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpVirtaEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "AverageVolume")]
    pub average_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiNameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: OrganizationName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDataUpdateUseCaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoDigitPositiveIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlBaseCuttingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<BdtDateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "CallForOfferWorkingSiteDetails")]
    pub call_for_offer_working_site_details: Vec<CallForOfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal3FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct actuateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisksType {
    #[serde(rename = "WorkSafetyRiskDescription")]
    pub work_safety_risk_description: Vec<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctTaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "ProductUserId", skip_serializing_if = "Option::is_none")]
    pub product_user_id: Option<String50Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "FileFormat")]
    pub file_format: String5Type,
    #[serde(rename = "Label")]
    pub label: String100Type,
    #[serde(rename = "DocumentClass", skip_serializing_if = "Option::is_none")]
    pub document_class: Option<DocumentClassType>,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNeljasRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometriesType {
    #[serde(rename = "HgPolygonGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_polygon_geometry: Option<Vec<PolygonGeometry>>,
    #[serde(rename = "HgLineGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_line_geometry: Option<Vec<LineGeometry>>,
    #[serde(rename = "HgPointGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_point_geometry: Option<Vec<PointGeometry>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecisionGeometryObjectType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSelfMonitoringTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString10Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuolemaPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocumentType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentDate", skip_serializing_if = "Option::is_none")]
    pub document_date: Option<DateType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkinTyyppi {
    #[serde(rename = "base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsPurchaseModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMemberType {
    #[serde(rename = "ResourceId")]
    pub resource_id: ShortERPIdType,
    #[serde(rename = "UserId")]
    pub user_id: ShortERPIdType,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformationType {
    #[serde(rename = "DataName")]
    pub data_name: String,
    #[serde(rename = "DataId")]
    pub data_id: String,
    #[serde(rename = "InspectorName")]
    pub inspector_name: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String50Type,
    #[serde(rename = "StartDate")]
    pub start_date: TimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<TimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "LoadCount")]
    pub load_count: u32,
    #[serde(rename = "Load")]
    pub load: Vec<LoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: PositiveInteger3digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "FertilizerVolumeOrdered")]
    pub fertilizer_volume_ordered: PositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasured")]
    pub fertilizer_volume_measured: PositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasuredText", skip_serializing_if = "Option::is_none")]
    pub fertilizer_volume_measured_text: Option<String200Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightResponsibleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoChangeStateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorsType {
    #[serde(rename = "ScaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<Vec<ScaleFactorDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: CoStratumNumberType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "Age")]
    pub age: AgeType,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: CoMeanHeightType,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<CoSeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaApprovalType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDataType {
    #[serde(rename = "Startdate")]
    pub startdate: DateType,
    #[serde(rename = "Enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<DateType>,
    #[serde(rename = "WorkingContract")]
    pub working_contract: YesNoType,
    #[serde(rename = "Active")]
    pub active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreSelfMonitoringObjectType>,
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
pub struct StbAreaDecreaseType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditerTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentDate", skip_serializing_if = "Option::is_none")]
    pub document_date: Option<DateType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeCuttingType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCertificationSystemType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodsType {
    #[serde(rename = "UsedPricingMethod")]
    pub used_pricing_method: Vec<UsedPricingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesType {
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<StorageType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawinghoursDataType {
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
    #[serde(rename = "Minutes")]
    pub minutes: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkarcModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifierType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCuttingType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlBaseCuttingType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "QualityControlDate")]
    pub quality_control_date: BdtDateType,
    #[serde(rename = "SamplePlotsSummaries")]
    pub sample_plots_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<String20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "OrderStatus")]
    pub order_status: OrderStatusType,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwnersType {
    #[serde(rename = "RealEstateOwner")]
    pub real_estate_owner: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatLocationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CuttingPlannerLiability", skip_serializing_if = "Option::is_none")]
    pub cutting_planner_liability: Option<CuttingPlannerLiabilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEdellinenSukuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemsType {
    #[serde(rename = "QualitySystem", skip_serializing_if = "Option::is_none")]
    pub quality_system: Option<Vec<QualitySystemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "Stem")]
    pub stem: Vec<StemDataType>,
    #[serde(rename = "LengthCalibration", skip_serializing_if = "Option::is_none")]
    pub length_calibration: Option<Vec<LengthCalibrationType>>,
    #[serde(rename = "DiameterCalibration", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration: Option<Vec<DiameterCalibrationType>>,
    #[serde(rename = "Caliper", skip_serializing_if = "Option::is_none")]
    pub caliper: Option<CaliperType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(rename = "ChildObjectType")]
    pub child_object_type: CoObjectTypeType,
    #[serde(rename = "ChildObjectTypeSpecifier", skip_serializing_if = "Option::is_none")]
    pub child_object_type_specifier: Option<ObjectTypeSpecifierType>,
    #[serde(rename = "ChildObjectId")]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesType {
    #[serde(rename = "UserRole")]
    pub user_role: Vec<UserRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditQuestionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "ProcessingAreaNumber")]
    pub processing_area_number: ProcessingAreaNumberType,
    #[serde(rename = "ProcessingAreaReference")]
    pub processing_area_reference: CoReferenceType,
    #[serde(rename = "DeclarationStands")]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDrainageStateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataTreeStandDataDateType {
    #[serde(flatten)]
    pub base: TsTreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString5000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeStateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount")]
    pub completed_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesType {
    #[serde(rename = "Service")]
    pub service: Vec<OrganizationServiceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: SpecificationCodeType,
    #[serde(rename = "SpecificationText", skip_serializing_if = "Option::is_none")]
    pub specification_text: Option<CoString2000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionsMainGroup {
    #[serde(rename = "CuttingRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction_ends: Option<CuttingRestrictionEnds>,
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction: Option<CuttingRestriction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5TotalDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<FccLocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimumType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct roleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaDamageClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoElectronicNotificationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtOrderStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType2,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoPaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformationType {
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "Contractors")]
    pub contractors: ContractorsType,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: String20Type,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address")]
    pub address: String50Type,
    #[serde(rename = "PostalCode")]
    pub postal_code: String10Type,
    #[serde(rename = "PostOffice")]
    pub post_office: String50Type,
    #[serde(rename = "BirthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<DateType>,
    #[serde(rename = "Telephone")]
    pub telephone: String20Type,
    #[serde(rename = "ICEName", skip_serializing_if = "Option::is_none")]
    pub i_c_e_name: Option<String100Type>,
    #[serde(rename = "ICETelephone", skip_serializing_if = "Option::is_none")]
    pub i_c_e_telephone: Option<String20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String100Type>,
    #[serde(rename = "UserRoles")]
    pub user_roles: UserRolesType,
    #[serde(rename = "UserName")]
    pub user_name: String100Type,
    #[serde(rename = "AdditionalName", skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String20Type>,
    #[serde(rename = "Trainings")]
    pub trainings: TrainingsType,
    #[serde(rename = "Employment")]
    pub employment: EmploymentDataType,
    #[serde(rename = "Machines", skip_serializing_if = "Option::is_none")]
    pub machines: Option<MachinesType>,
    #[serde(rename = "Languages")]
    pub languages: LanguagesType,
    #[serde(rename = "NationalityCode")]
    pub nationality_code: String5Type,
    #[serde(rename = "NationalityFreeText", skip_serializing_if = "Option::is_none")]
    pub nationality_free_text: Option<String50Type>,
    #[serde(rename = "WorkCodeGroups", skip_serializing_if = "Option::is_none")]
    pub work_code_groups: Option<WorkCodeGroupsType>,
    #[serde(rename = "Location")]
    pub location: PointGeometryType,
    #[serde(rename = "E101")]
    pub e101: YesNoType,
    #[serde(rename = "A1")]
    pub a1: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestActAreaType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStandQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNimilajiKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "TravelStartTime")]
    pub travel_start_time: TimeStampType,
    #[serde(rename = "TravelEndTime")]
    pub travel_end_time: TimeStampType,
    #[serde(rename = "Vehicle")]
    pub vehicle: VehicleType,
    #[serde(rename = "Kilometers")]
    pub kilometers: PositiveInteger4digitsType,
    #[serde(rename = "Route")]
    pub route: String200Type,
    #[serde(rename = "KilometersWithTrailer", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_trailer: Option<PositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithCaravan", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_caravan: Option<PositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithBreakHouse", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_break_house: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson1", skip_serializing_if = "Option::is_none")]
    pub extra_person1: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson2", skip_serializing_if = "Option::is_none")]
    pub extra_person2: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson3", skip_serializing_if = "Option::is_none")]
    pub extra_person3: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson4", skip_serializing_if = "Option::is_none")]
    pub extra_person4: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPersonText", skip_serializing_if = "Option::is_none")]
    pub extra_person_text: Option<String200Type>,
    #[serde(rename = "SittingMoneyKilometers", skip_serializing_if = "Option::is_none")]
    pub sitting_money_kilometers: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotKnownType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: DistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationSoilPreparationOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String2000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesType {
    #[serde(rename = "OtherTreeSpecies")]
    pub other_tree_species: Vec<OtherTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaMastoInspectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadType {
    #[serde(rename = "LoadNumber")]
    pub load_number: u32,
    #[serde(rename = "ForwardingDistance")]
    pub forwarding_distance: u32,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "PartitialLoad")]
    pub partitial_load: Vec<PartitialLoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger2digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString500Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadType {
    #[serde(rename = "Acknowledge")]
    pub acknowledge: AcknowledgeType,
    #[serde(rename = "CompanyInformation")]
    pub company_information: CompanyInformationType,
    #[serde(rename = "Contract")]
    pub contract: ContractType,
    #[serde(rename = "ExternalFile")]
    pub external_file: ExternalFileType,
    #[serde(rename = "ForwardingEstimate")]
    pub forwarding_estimate: ForwardingEstimateType,
    #[serde(rename = "ForwardingNotification")]
    pub forwarding_notification: ForwardingNotificationType,
    #[serde(rename = "HarvestingOrder")]
    pub harvesting_order: HarvestingOrderType,
    #[serde(rename = "Image")]
    pub image: ImageType,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: MapSymbolType,
    #[serde(rename = "OrderConfirmation")]
    pub order_confirmation: OrderConfirmationType,
    #[serde(rename = "ProductInstructionProductInstruction")]
    pub product_instruction_product_instruction: ProductInstruction,
    #[serde(rename = "QualityAttachment")]
    pub quality_attachment: QualityAttachmentType,
    #[serde(rename = "Resource")]
    pub resource: ResourceType,
    #[serde(rename = "ResourceSchedule")]
    pub resource_schedule: ResourceScheduleType,
    #[serde(rename = "SilvicultureOrder")]
    pub silviculture_order: SilvicultureOrderType,
    #[serde(rename = "ServiceBuyerResourceLocations")]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
    #[serde(rename = "SmsOperatorStatus")]
    pub sms_operator_status: SmsOperatorStatusType,
    #[serde(rename = "StanfordFile")]
    pub stanford_file: StanfordFileType,
    #[serde(rename = "UserInformation")]
    pub user_information: UserInformationType,
    #[serde(rename = "WeekCalendar")]
    pub week_calendar: WeekCalendarType,
    #[serde(rename = "WorkingSiteAccounting")]
    pub working_site_accounting: WorkingSiteAccountingType,
    #[serde(rename = "WorkingSiteEndNotification")]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
    #[serde(rename = "WorkingSiteFeeBasis")]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
    #[serde(rename = "WorkingSiteFinalAuditBioMassForwarding")]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditDraining")]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
    #[serde(rename = "WorkingSiteFinalAuditFertilization")]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
    #[serde(rename = "WorkingSiteFinalAuditHarvesting")]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement")]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
    #[serde(rename = "WorkingSiteFinalAuditRoadMaking")]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture")]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning")]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
    #[serde(rename = "WorkingSiteFinalAuditStumpForwarding")]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditStumpLifting")]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
    #[serde(rename = "WorkingSiteFinalAuditDynamic")]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
    #[serde(rename = "WorkingSiteForwardedProduction")]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionType,
    #[serde(rename = "WorkingSiteForwardingQualityControl")]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestedProduction")]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
    #[serde(rename = "WorkingSiteHarvestingQualityControl")]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestingQualityControlManual")]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
    #[serde(rename = "WorkingSiteOperational")]
    pub working_site_operational: WorkingSiteOperationalType,
    #[serde(rename = "WorkingSiteOperationalUpdate")]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
    #[serde(rename = "WorkingSiteQualityControlCutting")]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
    #[serde(rename = "WorkingSiteQualityControlFertilization")]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement")]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
    #[serde(rename = "WorkingSiteQualityControlSilviculture")]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning")]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
    #[serde(rename = "WorkingSiteQualityNotification")]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
    #[serde(rename = "WorkingSiteStatus")]
    pub working_site_status: WorkingSiteStatusType,
    #[serde(rename = "WorkingSiteTravelNotification")]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
    #[serde(rename = "WorkingSiteWorkLoad")]
    pub working_site_work_load: WorkingSiteWorkLoadType,
    #[serde(rename = "WorkingSiteWorkTime")]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "DeclarationPolygonReference")]
    pub declaration_polygon_reference: CoReferenceType,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesType {
    #[serde(rename = "Trees")]
    pub trees: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "Status5")]
    pub status5: CoChangeStateType,
    #[serde(rename = "TreeNumber")]
    pub tree_number: String,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeDiameter", skip_serializing_if = "Option::is_none")]
    pub tree_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeHeight", skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "TreeCount", skip_serializing_if = "Option::is_none")]
    pub tree_count: Option<CoStemCountType>,
    #[serde(rename = "WorkQuality", skip_serializing_if = "Option::is_none")]
    pub work_quality: Option<VirtaWorkQualityType>,
    #[serde(rename = "DamageClass", skip_serializing_if = "Option::is_none")]
    pub damage_class: Option<VirtaDamageClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayType {
    #[serde(rename = "CalendarDay")]
    pub calendar_day: DateType,
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryType {
    #[serde(rename = "DeliveryUserId")]
    pub delivery_user_id: String50Type,
    #[serde(rename = "DeliveryInfo")]
    pub delivery_info: String50Type,
    #[serde(rename = "DeliveryName")]
    pub delivery_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPriorityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsingRightResponsibleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageDataType {
    #[serde(rename = "CommonMessageId", skip_serializing_if = "Option::is_none")]
    pub common_message_id: Option<CommonMessageType>,
    #[serde(rename = "CommonMessageFreeText", skip_serializing_if = "Option::is_none")]
    pub common_message_free_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: MultiPolygon,
    #[serde(rename = "GmlMultiSurface")]
    pub gml_multi_surface: MultiSurface,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSaveIncompleteType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsType {
    #[serde(rename = "Cutting")]
    pub cutting: Vec<CuttingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataMessageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5_1Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteJakokirjainTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaType {
    #[serde(flatten)]
    pub base: CoDecimal4And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachmentType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<String50Type>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: DateType,
    #[serde(rename = "Version")]
    pub version: String10Type,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummaryType {
    #[serde(rename = "TreeSpeciesData")]
    pub tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcType {
    #[serde(rename = "XlinkarcModel")]
    pub xlinkarc_model: arcModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal4And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotNeededType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfFeatureInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaPhaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpfLocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "GdtSimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: SimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiabilityType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummaryType {
    #[serde(rename = "OperativeTreeSpeciesData")]
    pub operative_tree_species_data: Vec<TsTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationOtherOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeatureType {
    #[serde(rename = "HabitatCode", skip_serializing_if = "Option::is_none")]
    pub habitat_code: Option<ExtendedHabitatCodeType>,
    #[serde(rename = "HabitatType", skip_serializing_if = "Option::is_none")]
    pub habitat_type: Option<HabitatTypeType>,
    #[serde(rename = "HabitatSurviving", skip_serializing_if = "Option::is_none")]
    pub habitat_surviving: Option<YesNoType>,
    #[serde(rename = "ExceptionalPermitForHandling", skip_serializing_if = "Option::is_none")]
    pub exceptional_permit_for_handling: Option<YesNoType>,
    #[serde(rename = "HabitatLocation", skip_serializing_if = "Option::is_none")]
    pub habitat_location: Option<HabitatLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuvausTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLajiKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "SelfMonitoringBasicData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_basic_data: Option<SelfMonitoringBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "Works", skip_serializing_if = "Option::is_none")]
    pub works: Option<WorksType>,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_plant_management: Option<WorkingSiteFinalAuditPlantManagementSelfMonitoringWorkingSiteFinalAuditPlantManagementType>,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_plant_management: Option<WorkingSiteQualityControlPlantManagementSelfMonitoringWorkingSiteQualityControlPlantManagementType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<SelfMonitoringImageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumesType {
    #[serde(rename = "StemTypeVolume")]
    pub stem_type_volume: Vec<StemTypeVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMaterialCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandsType {
    #[serde(rename = "DeclarationStand")]
    pub declaration_stand: Vec<DeclarationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<PaymentsRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateAssortmentRowType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiversType {
    #[serde(rename = "DecisionReceiver")]
    pub decision_receiver: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTimeStampType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FdForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "ReRealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<RealEstates>,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataType {
    #[serde(rename = "SoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub soil_data_group: Option<SoilDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingExtraQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3TotalDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDiameterClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCuttingPurposeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationType {
    #[serde(rename = "EvaluationCategory", skip_serializing_if = "Option::is_none")]
    pub evaluation_category: Option<EvaluationSubjectType>,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<integer>,
    #[serde(rename = "EvaluationDescription", skip_serializing_if = "Option::is_none")]
    pub evaluation_description: Option<String1000Type>,
    #[serde(rename = "MainReason", skip_serializing_if = "Option::is_none")]
    pub main_reason: Option<YesNoType>,
    #[serde(rename = "ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<integer>,
    #[serde(rename = "ReasonDescription", skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct showType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPracticeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctMonthType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPaymentType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonsType {
    #[serde(rename = "DeclarationPolygon")]
    pub declaration_polygon: Vec<DeclarationPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveriesType {
    #[serde(rename = "Delivery")]
    pub delivery: Vec<DeliveryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationNatureManagementSpecifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsValtiotunnusKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaMastoInspectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourceType {
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlCuttingType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlBaseCuttingType,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<BdtDateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<BdtString20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineManufacturerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncomeType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<FeatureDataGroup>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<CoDateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKieliKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionMainGroup {
    #[serde(rename = "SilvicultureRestrictions", skip_serializing_if = "Option::is_none")]
    pub silviculture_restrictions: Option<SilvicultureRestrictions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "SilviculturalOperations", skip_serializing_if = "Option::is_none")]
    pub silvicultural_operations: Option<SilviculturalOperationsType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<PrProductsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcroleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRestrictionsType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extendedModel {
    #[serde(rename = "xlink_locator")]
    pub xlink_locator: xlink_locator,
    #[serde(rename = "xlink_title")]
    pub xlink_title: xlink_title,
    #[serde(rename = "xlink_resource")]
    pub xlink_resource: xlink_resource,
    #[serde(rename = "xlink_arc")]
    pub xlink_arc: xlink_arc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDataType {
    #[serde(rename = "EvaluationCost")]
    pub evaluation_cost: Decimal7And2PositiveType,
    #[serde(rename = "PartEastingCoordinate")]
    pub part_easting_coordinate: String,
    #[serde(rename = "PartNorthingCoordinate")]
    pub part_northing_coordinate: String,
    #[serde(rename = "InsuranceOrOtherCompensation")]
    pub insurance_or_other_compensation: Decimal7And2PositiveType,
    #[serde(rename = "TotalCompensation")]
    pub total_compensation: Decimal7And2PositiveType,
    #[serde(rename = "Compensation")]
    pub compensation: Decimal7And2PositiveType,
    #[serde(rename = "AlreadyPaidCompensation")]
    pub already_paid_compensation: Decimal7And2PositiveType,
    #[serde(rename = "TotalCompensations")]
    pub total_compensations: Decimal7And2PositiveType,
    #[serde(rename = "OwnerInvolvement")]
    pub owner_involvement: VirtaYesNoType,
    #[serde(rename = "AssociationInvolvement")]
    pub association_involvement: VirtaYesNoType,
    #[serde(rename = "MoosePercentage")]
    pub moose_percentage: PercentType,
    #[serde(rename = "Class1DamageCount")]
    pub class1_damage_count: StemCountType,
    #[serde(rename = "Class2DamageCount")]
    pub class2_damage_count: StemCountType,
    #[serde(rename = "Class3DamageCount")]
    pub class3_damage_count: StemCountType,
    #[serde(rename = "Class4DamageCount")]
    pub class4_damage_count: StemCountType,
    #[serde(rename = "DamagedSeedlingCount")]
    pub damaged_seedling_count: StemCountType,
    #[serde(rename = "NotDamagedSeedlingCount")]
    pub not_damaged_seedling_count: StemCountType,
    #[serde(rename = "LargePeelDamage")]
    pub large_peel_damage: StemCountType,
    #[serde(rename = "SmallPeelDamage")]
    pub small_peel_damage: StemCountType,
    #[serde(rename = "NeedForTreatment")]
    pub need_for_treatment: VirtaYesNoType,
    #[serde(rename = "RepairPlantingCosts")]
    pub repair_planting_costs: Decimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FudForestUseDeclarationType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
    #[serde(rename = "ForestUseDeclarationNotNeeded", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_not_needed: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingWeightType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfoType {
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<CoStemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoVATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPublicityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreaType {
    #[serde(rename = "OperationalRegion")]
    pub operational_region: String50Type,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorType {
    #[serde(rename = "@actorType")]
    pub actor_type: ActorTypeType,
    #[serde(rename = "@actorTypeSpecifier")]
    pub actor_type_specifier: ActorTypeSpecifierType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "ControlAdditionalInformation", skip_serializing_if = "Option::is_none")]
    pub control_additional_information: Option<ControlAdditionalInformationType>,
    #[serde(rename = "PowerOfAttorney", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney: Option<FccPowerOfAttorneyType>,
    #[serde(rename = "PowerOfAttorneyDate", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_date: Option<FccPowerOfAttorneyDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeysType {
    #[serde(rename = "StandKeyGroup1")]
    pub stand_key_group1: StandKeyGroup1,
    #[serde(rename = "StandKeyGroup2")]
    pub stand_key_group2: StandKeyGroup2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRestrictionType {
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String1000Type>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
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
pub struct UsingRightType {
    #[serde(rename = "UsingRightExists")]
    pub using_right_exists: CoYesNoNotKnownType,
    #[serde(rename = "UsingRightExaminedDate")]
    pub using_right_examined_date: CoDateType,
    #[serde(rename = "UsingRightCompensation")]
    pub using_right_compensation: CoYesNoNotKnownType,
    #[serde(rename = "UsingRightCompensationAmount", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_amount: Option<CoDecimal7And2Type>,
    #[serde(rename = "UsingRightCompensationDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_description: Option<CoString1500Type>,
    #[serde(rename = "UsingRightCompensationResponsible")]
    pub using_right_compensation_responsible: CoUsingRightResponsibleType,
    #[serde(rename = "UsingRightDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "DiameterCalibrationDate")]
    pub diameter_calibration_date: TimeStampType,
    #[serde(rename = "DiameterCalibrationReason")]
    pub diameter_calibration_reason: String200Type,
    #[serde(rename = "DiameterCalibrationDescription")]
    pub diameter_calibration_description: String200Type,
    #[serde(rename = "DiameterCalibrationAdjustment")]
    pub diameter_calibration_adjustment: Integer3digitsType,
    #[serde(rename = "DiameterCalibrationAdjustmentButtLog", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration_adjustment_butt_log: Option<Integer3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "StartDate")]
    pub start_date: TimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<TimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: TimeStampType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<base64Binary>,
    #[serde(rename = "StemTypeVolumes")]
    pub stem_type_volumes: StemTypeVolumesType,
    #[serde(rename = "AssortmentVolumes")]
    pub assortment_volumes: AssortmentVolumesType,
    #[serde(rename = "AssortmentVolumesUnclassified", skip_serializing_if = "Option::is_none")]
    pub assortment_volumes_unclassified: Option<AssortmentVolumesUnclassifiedType>,
    #[serde(rename = "AssortmentMatrixVolumes")]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
    #[serde(rename = "ProductUserIds")]
    pub product_user_ids: Vec<ProductUserIdsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteNumeroTyyppi {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfBasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<FeatureDataGroup>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<CoDateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMaterialUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(rename = "ServiceNameOfAPI")]
    pub service_name_of_a_p_i: ServiceNameofAPIType,
    #[serde(rename = "AuthorizedToSend")]
    pub authorized_to_send: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctLoadRatingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSiteNameType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String25Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMinType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEtuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKeyType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtHeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListType {
    #[serde(rename = "ValueListItem")]
    pub value_list_item: Vec<ValueListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionModelGroup {
    #[serde(rename = "Weibull")]
    pub weibull: Weibull,
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSB,
    #[serde(rename = "Normal")]
    pub normal: Normal,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "Gamma")]
    pub gamma: Gamma,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetaType {
    #[serde(rename = "ShapeAlfa")]
    pub shape_alfa: ShapeAlfaType,
    #[serde(rename = "ShapeBeta")]
    pub shape_beta: ShapeBetaType,
    #[serde(rename = "Minimum")]
    pub minimum: MinimumType,
    #[serde(rename = "Maximum")]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPreclearingEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometryType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<CoAreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<CoAreaType>,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfoType {
    #[serde(rename = "PurchaseMode")]
    pub purchase_mode: PurchaseModeType,
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
    #[serde(rename = "IncludeForestFundPayment")]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
    #[serde(rename = "UsedPricingMethods", skip_serializing_if = "Option::is_none")]
    pub used_pricing_methods: Option<UsedPricingMethodsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistributionType {
    #[serde(rename = "CumulativePoint")]
    pub cumulative_point: Vec<CumulativePointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType2 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "FacFinancingActApplicationReference")]
    pub fac_financing_act_application_reference: FinancingActApplicationReference,
    #[serde(rename = "FacFinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<FinancingActNumber>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<FinancingActApplicationTextInformation>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: Language,
    #[serde(rename = "FacSender")]
    pub fac_sender: Sender,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "FacEstimatedStartDate")]
    pub fac_estimated_start_date: EstimatedStartDate,
    #[serde(rename = "FacEstimatedEndDate")]
    pub fac_estimated_end_date: EstimatedEndDate,
    #[serde(rename = "FacSubsidyAmount")]
    pub fac_subsidy_amount: SubsidyAmount,
    #[serde(rename = "FacFinancingActWorkGroup")]
    pub fac_financing_act_work_group: FinancingActWorkGroup,
    #[serde(rename = "FacCopOperationProject")]
    pub fac_cop_operation_project: CopOperationProject,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "FacApplicationActors")]
    pub fac_application_actors: ApplicationActors,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDeltaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode1Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpCuttingTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "SoilDataGroup")]
    pub soil_data_group: SoilDataGroup,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: TreeStandBasedDataGroup,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: RestrictionsMainGroup,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StbStandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolumeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstatesType {
    #[serde(rename = "PayeeAndRealEstate")]
    pub payee_and_real_estate: Vec<PayeeAndRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct labelType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoMessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<CiEmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: FudForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandBasicDataDateType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHarvestingClassificationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSitePlanningOperationStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: MultiPolygon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorType {
    #[serde(rename = "ActorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<CoIdStringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumeroTyyppi {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeType {
    #[serde(rename = "OwnerShipTypeCode")]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax1IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoWideUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGammaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMachineTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDefinedType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger6digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate2Type {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<TreeStrata2Type>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TreeStandSummary2Type>,
    #[serde(rename = "SdTrees", skip_serializing_if = "Option::is_none")]
    pub sd_trees: Option<Trees>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceType {
    #[serde(rename = "SubsidyApplierId")]
    pub subsidy_applier_id: CoIdStringNotEmptyType,
    #[serde(rename = "MainApplier")]
    pub main_applier: YesNoType,
    #[serde(rename = "ShareOfOwnerShip", skip_serializing_if = "Option::is_none")]
    pub share_of_owner_ship: Option<PercentType>,
    #[serde(rename = "OwnerShipType")]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger2digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsMaatunnusKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEtunimetNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaGroundManipulationMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYearType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ResourceIdMJ", skip_serializing_if = "Option::is_none")]
    pub resource_id_m_j: Option<String20Type>,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: String20Type,
    #[serde(rename = "ForestOwner", skip_serializing_if = "Option::is_none")]
    pub forest_owner: Option<String100Type>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "DisQualificationPercentageTotal")]
    pub dis_qualification_percentage_total: Decimal2FractionDigitsType,
    #[serde(rename = "CuttingAccuracy")]
    pub cutting_accuracy: Decimal2FractionDigitsType,
    #[serde(rename = "DisQualificationSign")]
    pub dis_qualification_sign: String5Type,
    #[serde(rename = "CuttingAccuracySign")]
    pub cutting_accuracy_sign: String5Type,
    #[serde(rename = "Document", skip_serializing_if = "Option::is_none")]
    pub document: Option<base64Binary>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<base64Binary>,
    #[serde(rename = "DisQualificationReasons")]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPreviousBlockStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameofAPIType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeeBasisValueType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkextendedModel {
    #[serde(rename = "xlink_locator")]
    pub xlink_locator: xlink_locator,
    #[serde(rename = "xlink_title")]
    pub xlink_title: xlink_title,
    #[serde(rename = "xlink_resource")]
    pub xlink_resource: xlink_resource,
    #[serde(rename = "xlink_arc")]
    pub xlink_arc: xlink_arc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoSellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionAsUnitGroup {
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(rename = "QuantityUnit", skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<QuantityUnit>,
    #[serde(rename = "TotalValue", skip_serializing_if = "Option::is_none")]
    pub total_value: Option<TotalValue>,
    #[serde(rename = "UnitValue", skip_serializing_if = "Option::is_none")]
    pub unit_value: Option<UnitValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringNotEmptyType,
    #[serde(rename = "PayeeId")]
    pub payee_id: CoIdStringNotEmptyType,
    #[serde(rename = "ParticipationPercentage", skip_serializing_if = "Option::is_none")]
    pub participation_percentage: Option<CoPercentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCultivationMaterialType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger3digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Description")]
    pub description: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "Objects")]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentChangeDataType {
    #[serde(rename = "OldDestinationStorage")]
    pub old_destination_storage: String20Type,
    #[serde(rename = "OldCode")]
    pub old_code: String50Type,
    #[serde(rename = "NewDestinationStorage")]
    pub new_destination_storage: String20Type,
    #[serde(rename = "NewCode")]
    pub new_code: String50Type,
    #[serde(rename = "ChangeVolume")]
    pub change_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtInteger3digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtClientApplicationIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctHopperTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPuhelinnumeroTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralsType {
    #[serde(rename = "PeripheralCode", skip_serializing_if = "Option::is_none")]
    pub peripheral_code: Option<Vec<PeripheralCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSeedlingOriginType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer3digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsViidesRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReasonType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XshexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
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
pub struct ForestDataStandardSchemaPackageSubversionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<ERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Volume")]
    pub volume: PositiveInteger4digitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<WorkCodeUnitType>,
    #[serde(rename = "Loads")]
    pub loads: PositiveInteger3digitsType,
    #[serde(rename = "Day")]
    pub day: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTerrainClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDryingClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainsType {
    #[serde(rename = "PlannedOperationChain")]
    pub planned_operation_chain: Vec<PlannedOperationChainType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<CoPreferredContactingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<RealEstatesGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamagesType {
    #[serde(rename = "PreviousMooseDamage")]
    pub previous_moose_damage: Vec<PreviousMooseDamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSahkoinenAsiointiTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTargetSelectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneAndTelefaxGroup {
    #[serde(rename = "MobilePhoneNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<MobilePhoneNumber>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    #[serde(rename = "TelefaxNumber", skip_serializing_if = "Option::is_none")]
    pub telefax_number: Option<TelefaxNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermissionType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreasType {
    #[serde(rename = "ProcessingArea")]
    pub processing_area: Vec<ProcessingAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumberType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaPlantEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgencyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<CodForestObjectDataObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformationType {
    #[serde(rename = "PreinformDate", skip_serializing_if = "Option::is_none")]
    pub preinform_date: Option<CoDateType>,
    #[serde(rename = "PreinformDetails", skip_serializing_if = "Option::is_none")]
    pub preinform_details: Option<CoString1000Type>,
    #[serde(rename = "InTerrain", skip_serializing_if = "Option::is_none")]
    pub in_terrain: Option<CoYesNoType>,
    #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuntaNumeroTyyppi {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensityGroup {
    #[serde(rename = "DeciduousTreeTargetDensityPercent", skip_serializing_if = "Option::is_none")]
    pub deciduous_tree_target_density_percent: Option<DeciduousTreeTargetDensityPercent>,
    #[serde(rename = "TargetDensity", skip_serializing_if = "Option::is_none")]
    pub target_density: Option<TargetDensity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstAndLastNameGroup {
    #[serde(rename = "LastName")]
    pub last_name: LastName,
    #[serde(rename = "FirstName")]
    pub first_name: FirstName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: StbAreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesConciseType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<CoHeightClassType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoHeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<CoDiameterClassType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoDiameterType>,
    #[serde(rename = "RemovalClass", skip_serializing_if = "Option::is_none")]
    pub removal_class: Option<CoRemovalClassType>,
    #[serde(rename = "CuttingStemCount", skip_serializing_if = "Option::is_none")]
    pub cutting_stem_count: Option<CuttingStemCountType>,
    #[serde(rename = "SmallWoodRemovalClass", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_class: Option<CoSmallWoodRemovalClassType>,
    #[serde(rename = "SmallWoodRemovalVolume", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_volume: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletion")]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<CoString2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingWeightType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: String10Type,
    #[serde(rename = "FeeListId", skip_serializing_if = "Option::is_none")]
    pub fee_list_id: Option<PositiveIntegerType>,
    #[serde(rename = "FeeYesNo", skip_serializing_if = "Option::is_none")]
    pub fee_yes_no: Option<YesNoType>,
    #[serde(rename = "FeeValue", skip_serializing_if = "Option::is_none")]
    pub fee_value: Option<String10Type>,
    #[serde(rename = "FeeAssortment", skip_serializing_if = "Option::is_none")]
    pub fee_assortment: Option<String50Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "NeedToCheck", skip_serializing_if = "Option::is_none")]
    pub need_to_check: Option<YesNoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "FeeBaseList", skip_serializing_if = "Option::is_none")]
    pub fee_base_list: Option<FeeBaseListType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlanType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AsAssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureType {
    #[serde(rename = "CuttingRelated", skip_serializing_if = "Option::is_none")]
    pub cutting_related: Option<CuttingRelatedType>,
    #[serde(rename = "Cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<CostType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDateType {
    #[serde(rename = "@type")]
    pub r#type: DatePrecipionType,
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstatesType {
    #[serde(rename = "FinancingActRealEstate")]
    pub financing_act_real_estate: Vec<FinancingActRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String20Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourcesType {
    #[serde(rename = "AuditionResource", skip_serializing_if = "Option::is_none")]
    pub audition_resource: Option<AuditionResourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolDataType {
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<ERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<String20Type>,
    #[serde(rename = "MapSymbolName", skip_serializing_if = "Option::is_none")]
    pub map_symbol_name: Option<String20Type>,
    #[serde(rename = "FeatureType", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<FeatureTypeType>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCodeType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<String1000Type>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<String1000Type>,
    #[serde(rename = "Geometry")]
    pub geometry: AlternativeGeometries2Type,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "CanModify")]
    pub can_modify: YesNoType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "DitchType", skip_serializing_if = "Option::is_none")]
    pub ditch_type: Option<DitchTypeType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<PositiveInteger5digitsType>,
    #[serde(rename = "Depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MaterialCode", skip_serializing_if = "Option::is_none")]
    pub material_code: Option<MaterialCodeType>,
    #[serde(rename = "MaterialInfoText", skip_serializing_if = "Option::is_none")]
    pub material_info_text: Option<String1000Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<String100Type>,
    #[serde(rename = "SpareGroupOfTrees", skip_serializing_if = "Option::is_none")]
    pub spare_group_of_trees: Option<SpareTreesByCategoryType>,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDepotAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartsType {
    #[serde(rename = "TpTargetPart")]
    pub tp_target_part: Vec<TargetPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAddressGroup {
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "StateCode", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<StateCode>,
    #[serde(rename = "StateText", skip_serializing_if = "Option::is_none")]
    pub state_text: Option<StateText>,
    #[serde(rename = "PostOffice")]
    pub post_office: PostOffice,
    #[serde(rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
    #[serde(rename = "CountryText", skip_serializing_if = "Option::is_none")]
    pub country_text: Option<CountryText>,
    #[serde(rename = "PostalCode")]
    pub postal_code: PostalCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSiviilisaatyTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString3000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbAreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYesNoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLoppuPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumeType {
    #[serde(rename = "ProductUserId")]
    pub product_user_id: String100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: String50Type,
    #[serde(rename = "Quality")]
    pub quality: String5Type,
    #[serde(rename = "LengthClass")]
    pub length_class: PositiveInteger4digitsType,
    #[serde(rename = "DiameterClass")]
    pub diameter_class: PositiveInteger4digitsType,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMinType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaAdvertiserType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hrefType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferType {
    #[serde(rename = "@callForOfferId")]
    pub call_for_offer_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@versionNo")]
    pub version_no: i32,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "OfferBusinessSender")]
    pub offer_business_sender: OfferBusinessSenderType,
    #[serde(rename = "CallForOfferBusinessSender", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_business_sender: Option<WtcoCallForOfferBusinessSenderType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "OfferDate")]
    pub offer_date: OfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "OfferText", skip_serializing_if = "Option::is_none")]
    pub offer_text: Option<OfferTextType>,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<WtcoCallForOfferTextType>,
    #[serde(rename = "WsOfferWorkingSites")]
    pub ws_offer_working_sites: OfferWorkingSites,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsVakinainenKytkinTyyppi {
    #[serde(rename = "base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPaymentType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger1digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleEltType {
    #[serde(rename = "XlinktitleModel")]
    pub xlinktitle_model: titleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecisionTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtImageSubCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCodeType {
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FacFinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<FinancingActApplicationStands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResourceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKansalaisuusKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUnitPerHectareType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesType {
    #[serde(rename = "Machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<Vec<MachineTypeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsedPricingMethodTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedAmountType {
    #[serde(flatten)]
    pub base: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: OrganizationName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistanceType {
    #[serde(flatten)]
    pub base: CoDecimal4And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: Decimal2FractionDigitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: Decimal1FractionDigitType,
    #[serde(rename = "StemCount")]
    pub stem_count: PositiveIntegerType,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<PositiveIntegerType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<PositiveIntegerType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreementsType {
    #[serde(rename = "CollectiveAgreement")]
    pub collective_agreement: Vec<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandsType {
    #[serde(rename = "FinancingActCompletionStand")]
    pub financing_act_completion_stand: Vec<FinancingActCompletionStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal1FractionDigitType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(rename = "HopperNumber")]
    pub hopper_number: String20Type,
    #[serde(rename = "HopperType")]
    pub hopper_type: HopperTypeType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Geometry")]
    pub geometry: PointGeometryType,
    #[serde(rename = "HopperLocationFromGPS")]
    pub hopper_location_from_g_p_s: YesNoType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPhaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDefType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@mainType")]
    pub main_type: MainTypeType,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "OperationStatus")]
    pub operation_status: CoOperationStatusType,
    #[serde(rename = "ActingDate")]
    pub acting_date: ActingDateType,
    #[serde(rename = "ResponsibleActor", skip_serializing_if = "Option::is_none")]
    pub responsible_actor: Option<ResponsibleActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal1FractionDigitType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetSelectionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotsType {
    #[serde(rename = "Woodlot")]
    pub woodlot: Vec<WoodLotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumeroTyyppi {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentrePayments")]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDataGroup {
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<RestrictionData>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedData>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicData>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicData>,
    #[serde(rename = "SelfMonitoringObjectProtectionOperationsData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_object_protection_operations_data: Option<SelfMonitoringObjectProtectionOperationsData>,
    #[serde(rename = "op_Operations", skip_serializing_if = "Option::is_none")]
    pub op__operations: Option<op_Operations>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicData>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Documents>,
    #[serde(rename = "ControlOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub control_overall_evaluation_data: Option<ControlOverallEvaluationData>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<Actors>,
    #[serde(rename = "ControlDataRegeneration", skip_serializing_if = "Option::is_none")]
    pub control_data_regeneration: Option<ControlDataRegeneration>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StandBasicData>,
    #[serde(rename = "workingSiteFinalAuditRoadMaking_WorkingSiteFinalAuditRoadMaking", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_road_making__working_site_final_audit_road_making: Option<workingSiteFinalAuditRoadMaking_WorkingSiteFinalAuditRoadMaking>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjects>,
    #[serde(rename = "ts_TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts__tree_stand_data: Option<ts_TreeStandData>,
    #[serde(rename = "ControlDataMooseDamageData", skip_serializing_if = "Option::is_none")]
    pub control_data_moose_damage_data: Option<ControlDataMooseDamageData>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicData>,
    #[serde(rename = "MapSymbol_MapSymbol", skip_serializing_if = "Option::is_none")]
    pub map_symbol__map_symbol: Option<MapSymbol_MapSymbol>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<SoilData>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignData>,
    #[serde(rename = "workingSiteFinalAuditDraining_WorkingSiteFinalAuditDraining", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_draining__working_site_final_audit_draining: Option<workingSiteFinalAuditDraining_WorkingSiteFinalAuditDraining>,
    #[serde(rename = "st_SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st__special_features: Option<st_SpecialFeatures>,
    #[serde(rename = "ControlDataWaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub control_data_water_system_protection: Option<ControlDataWaterSystemProtection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseHarvestingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<BdtString20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPlantEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBearingCapacityClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataType {
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeInfoType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialType {
    #[serde(rename = "MaterialId")]
    pub material_id: String20Type,
    #[serde(rename = "MaterialCode")]
    pub material_code: MaterialCodeType,
    #[serde(rename = "CommercialName", skip_serializing_if = "Option::is_none")]
    pub commercial_name: Option<String100Type>,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: MaterialUnitType,
    #[serde(rename = "Supplier", skip_serializing_if = "Option::is_none")]
    pub supplier: Option<String50Type>,
    #[serde(rename = "MaterialProducer", skip_serializing_if = "Option::is_none")]
    pub material_producer: Option<String50Type>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "GrainSize", skip_serializing_if = "Option::is_none")]
    pub grain_size: Option<PositiveInteger3digitsType>,
    #[serde(rename = "DeliveryNumber", skip_serializing_if = "Option::is_none")]
    pub delivery_number: Option<String20Type>,
    #[serde(rename = "Delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<YesNoType>,
    #[serde(rename = "DeliveryDate", skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<DateType>,
    #[serde(rename = "PackagingDate", skip_serializing_if = "Option::is_none")]
    pub packaging_date: Option<DateType>,
    #[serde(rename = "FreezingDate", skip_serializing_if = "Option::is_none")]
    pub freezing_date: Option<DateType>,
    #[serde(rename = "MeltingDate", skip_serializing_if = "Option::is_none")]
    pub melting_date: Option<DateType>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String100Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "MaterialDeliveryType", skip_serializing_if = "Option::is_none")]
    pub material_delivery_type: Option<MaterialDeliveryTypeType>,
    #[serde(rename = "RegionOfOrigin", skip_serializing_if = "Option::is_none")]
    pub region_of_origin: Option<String10Type>,
    #[serde(rename = "DegreeDays", skip_serializing_if = "Option::is_none")]
    pub degree_days: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Altitude", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "OfferWorkingSiteDetails")]
    pub offer_working_site_details: Vec<OfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccPowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSBType {
    #[serde(rename = "ShapeGamma")]
    pub shape_gamma: ShapeGammaType,
    #[serde(rename = "ShapeDelta")]
    pub shape_delta: ShapeDeltaType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location")]
    pub location: LocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFertilityClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal4And2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsVoimassaoloKytkinTyyppi {
    #[serde(rename = "base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureRestrictionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount")]
    pub planned_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO3166char2CountryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@type")]
    pub r#type: ObjectTypeType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ParentObjects", skip_serializing_if = "Option::is_none")]
    pub parent_objects: Option<ParentObjectsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StStandBasicDataType>,
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeOfForestObjectType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString200Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType5 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationsType {
    #[serde(rename = "ResourceLocation")]
    pub resource_location: Vec<ResourceLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoObjectTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger5digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "Explanation")]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompanyModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactStandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StbStandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItemType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TreeNumber", skip_serializing_if = "Option::is_none")]
    pub tree_number: Option<integer>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<CoStoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoAgeType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<CoDiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<CoMeanHeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<CoVolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TreeIdentifiers", skip_serializing_if = "Option::is_none")]
    pub tree_identifiers: Option<TreeIdentifiersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinklocatorModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
    #[serde(rename = "Message")]
    pub message: PayloadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPartNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAgeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger3digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: StbStandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperationStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(flatten)]
    pub base: StbStandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSilvicultureBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonNameGroup {
    #[serde(rename = "WholeName")]
    pub whole_name: WholeName,
    #[serde(rename = "PersonOrganizationName", skip_serializing_if = "Option::is_none")]
    pub person_organization_name: Option<PersonOrganizationName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: CoStatisticsPurchaseModeCodeType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "StatisticsAssortmentCompactClasses")]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectsType {
    #[serde(rename = "ChildObject")]
    pub child_object: Vec<ChildObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingDirectingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBusinessAcceptanceStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierBaseContactAndEstateInfoType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<PaymentsRealEstatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(rename = "PaymentTransactionCategory")]
    pub payment_transaction_category: CoMoneyTransactionCategoryType,
    #[serde(rename = "PaymentTransactionType")]
    pub payment_transaction_type: CoMoneyTransactionTypeType,
    #[serde(rename = "Value")]
    pub value: MoneyType,
    #[serde(rename = "PaymentTransactionAsUnitGroup")]
    pub payment_transaction_as_unit_group: PaymentTransactionAsUnitGroup,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "PaymentTransactionDescription", skip_serializing_if = "Option::is_none")]
    pub payment_transaction_description: Option<PaymentTransactionDescriptionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReplyType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Acceptance")]
    pub acceptance: CoAcceptanceType,
    #[serde(rename = "ReplyCode")]
    pub reply_code: CoReplyCodeType,
    #[serde(rename = "MessageType")]
    pub message_type: CoMessageType,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<CoReferenceType>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<CoDateType>,
    #[serde(rename = "RegistrationId", skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<CoReferenceType>,
    #[serde(rename = "ErrorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<ErrorMessagesType>,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaLawType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningDistrictType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleAttrType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurerTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccLocationEstateType {
    #[serde(flatten)]
    pub base: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaInspectionMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaAdvertisementDatingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtVehicleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO639char2LanguageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectDataGroup", skip_serializing_if = "Option::is_none")]
    pub object_data_group: Option<ObjectDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionGroup {
    #[serde(rename = "SilvicultureRestriction")]
    pub silviculture_restriction: SilvicultureRestriction,
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<SilvicultureRestrictionEnds>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfLocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "StandNumber")]
    pub stand_number: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsCareOfTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreForestDataUpdateObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EbEnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislationType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "CompensationDescription", skip_serializing_if = "Option::is_none")]
    pub compensation_description: Option<string>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendarType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptancesType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "BaBusinessAcceptance", skip_serializing_if = "Option::is_none")]
    pub ba_business_acceptance: Option<Vec<BusinessAcceptance>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtInteger7digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequests {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: Vec<ContactRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataGroup {
    #[serde(rename = "MainGroup")]
    pub main_group: MainGroup,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageState>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlPoint")]
    pub gml_point: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStorageLandOwnerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDataType {
    #[serde(rename = "StemId")]
    pub stem_id: PositiveIntegerType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<HarvestingStemTypeType>,
    #[serde(rename = "SelectionType")]
    pub selection_type: ControlStemSelectionType,
    #[serde(rename = "RandomControlStemRejectedReason", skip_serializing_if = "Option::is_none")]
    pub random_control_stem_rejected_reason: Option<String100Type>,
    #[serde(rename = "StemCoordinates", skip_serializing_if = "Option::is_none")]
    pub stem_coordinates: Option<PointGeometryType>,
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaApprovalType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference14Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsType {
    #[serde(rename = "Product")]
    pub product: Vec<ProductType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extended {
    #[serde(rename = "XlinkextendedModel", skip_serializing_if = "Option::is_none")]
    pub xlinkextended_model: Option<Vec<extendedModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistancesType {
    #[serde(rename = "StorageForestHaulageDistance")]
    pub storage_forest_haulage_distance: Vec<StorageForestHaulageDistanceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: SimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    #[serde(rename = "base")]
    pub base: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesType {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationBankAccountType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CoBankAccount")]
    pub co_bank_account: BankAccount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger2digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: Vec<CallForOffer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RwsRoundWoodSalesRows", skip_serializing_if = "Option::is_none")]
    pub rws_round_wood_sales_rows: Option<RoundWoodSalesRows>,
    #[serde(rename = "OperationRows", skip_serializing_if = "Option::is_none")]
    pub operation_rows: Option<OperationRowsType>,
    #[serde(rename = "LoggingsRows", skip_serializing_if = "Option::is_none")]
    pub loggings_rows: Option<LoggingsRowsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolsType {
    #[serde(rename = "Symbol")]
    pub symbol: Vec<MapSymbolDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummaryType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "MeanAge")]
    pub mean_age: MeanAgeType,
    #[serde(rename = "BasalArea")]
    pub basal_area: CoBasalAreaType,
    #[serde(rename = "StemCount")]
    pub stem_count: StemCountType,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<CoDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: CoMeanHeightType,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume")]
    pub volume: VolumeType,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth")]
    pub volume_growth: VolumeGrowthType,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeededType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: pointProperty,
    #[serde(rename = "GmllineStringProperty")]
    pub gmlline_string_property: lineStringProperty,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityOfTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaReviewType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoQuantityUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCertificationSystemType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferTextType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeStandDataMomentType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsIBANTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScheduleType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "FinancingActApplicationGeometry")]
    pub financing_act_application_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVATStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEnsimmainenRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "ContactInformation")]
    pub contact_information: CiContactInformationType,
    #[serde(rename = "ContactMunicipality")]
    pub contact_municipality: CoMunicipalityNumberType,
    #[serde(rename = "ContactLocationInformation")]
    pub contact_location_information: GdtAlternativeGeometriesType,
    #[serde(rename = "RequestCode")]
    pub request_code: RequestCodeType,
    #[serde(rename = "RequestInfo", skip_serializing_if = "Option::is_none")]
    pub request_info: Option<CoString2000Type>,
    #[serde(rename = "PreferredContactingMethods", skip_serializing_if = "Option::is_none")]
    pub preferred_contacting_methods: Option<PreferredContactingMethodsType>,
    #[serde(rename = "CreateDate")]
    pub create_date: CoDateType,
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<CoDateType>,
    #[serde(rename = "ForestPropertyDataSet", skip_serializing_if = "Option::is_none")]
    pub forest_property_data_set: Option<ForestPropertyDataSetType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTransportAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsValiaikainenHenkiloTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "StatisticsOperation")]
    pub statistics_operation: StatisticsOperationType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "ReportedStatisticsOperationType")]
    pub reported_statistics_operation_type: CoReportedStatisticsOperationTypeType,
    #[serde(rename = "StatisticsQuantities")]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInformationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassExtensionsType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: String20Type,
    #[serde(rename = "PurchaseContractExtraInfo", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_extra_info: Option<String3000Type>,
    #[serde(rename = "WorkingSiteNumber")]
    pub working_site_number: WorkingSiteNumberType,
    #[serde(rename = "WorkingSiteName")]
    pub working_site_name: String100Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: String20Type,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "CustomerType", skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<String50Type>,
    #[serde(rename = "SilvicultureContractNumber", skip_serializing_if = "Option::is_none")]
    pub silviculture_contract_number: Option<String20Type>,
    #[serde(rename = "Organisation1", skip_serializing_if = "Option::is_none")]
    pub organisation1: Option<String20Type>,
    #[serde(rename = "Organisation2", skip_serializing_if = "Option::is_none")]
    pub organisation2: Option<String20Type>,
    #[serde(rename = "Organisation3", skip_serializing_if = "Option::is_none")]
    pub organisation3: Option<String20Type>,
    #[serde(rename = "Organisation4", skip_serializing_if = "Option::is_none")]
    pub organisation4: Option<String20Type>,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "ServiceBuyerContactInformation")]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
    #[serde(rename = "CustomerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub customer_representative_person: Option<ContactInformationType>,
    #[serde(rename = "ForestOwners")]
    pub forest_owners: ForestOwnersType,
    #[serde(rename = "AuthorizationsToSendWsoInformation", skip_serializing_if = "Option::is_none")]
    pub authorizations_to_send_wso_information: Option<AuthorizationsToSendWsoInformationType>,
    #[serde(rename = "TransportCompany", skip_serializing_if = "Option::is_none")]
    pub transport_company: Option<ContactInformationType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "Accessibility")]
    pub accessibility: AccessibilityType,
    #[serde(rename = "TerrainClass", skip_serializing_if = "Option::is_none")]
    pub terrain_class: Option<TerrainClassType>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "ResultsOfAccessibilityAnalysis", skip_serializing_if = "Option::is_none")]
    pub results_of_accessibility_analysis: Option<ResultsOfAccessibilityAnalysisType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<PositiveInteger5digitsType>,
    #[serde(rename = "TerrainPlanningDone", skip_serializing_if = "Option::is_none")]
    pub terrain_planning_done: Option<YesNoType>,
    #[serde(rename = "PreClearingInformation", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_information: Option<PreClearingInformationType>,
    #[serde(rename = "WorkingSitePlanningStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_status: Option<WorkingSitePlanningStatusType>,
    #[serde(rename = "WorkingSitePlanningOperation", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_operation: Option<WorkingSitePlanningOperationStatusType>,
    #[serde(rename = "WorkingSitePlanningInfo", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_info: Option<String3000Type>,
    #[serde(rename = "WorkingSitePlannedForHarvestingDate", skip_serializing_if = "Option::is_none")]
    pub working_site_planned_for_harvesting_date: Option<DateType>,
    #[serde(rename = "WorkingSiteStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_status: Option<WorkingSiteStatusType>,
    #[serde(rename = "ReadyToDo")]
    pub ready_to_do: YesNoType,
    #[serde(rename = "SellersLogs", skip_serializing_if = "Option::is_none")]
    pub sellers_logs: Option<String200Type>,
    #[serde(rename = "SellersLogsInfo", skip_serializing_if = "Option::is_none")]
    pub sellers_logs_info: Option<String1000Type>,
    #[serde(rename = "WorkingsiteInfo", skip_serializing_if = "Option::is_none")]
    pub workingsite_info: Option<String3000Type>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<WorkingSitePriorityType>,
    #[serde(rename = "OperationTimeStart", skip_serializing_if = "Option::is_none")]
    pub operation_time_start: Option<DateType>,
    #[serde(rename = "OperationTimeEnd", skip_serializing_if = "Option::is_none")]
    pub operation_time_end: Option<DateType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration: Option<ForestUseDeclarationType>,
    #[serde(rename = "OperationRestriction", skip_serializing_if = "Option::is_none")]
    pub operation_restriction: Option<OperationRestrictionType>,
    #[serde(rename = "ContractValidDate", skip_serializing_if = "Option::is_none")]
    pub contract_valid_date: Option<DateType>,
    #[serde(rename = "MaterialAreaId", skip_serializing_if = "Option::is_none")]
    pub material_area_id: Option<String20Type>,
    #[serde(rename = "QualityAttachments", skip_serializing_if = "Option::is_none")]
    pub quality_attachments: Option<String100Type>,
    #[serde(rename = "TransportArea", skip_serializing_if = "Option::is_none")]
    pub transport_area: Option<String10Type>,
    #[serde(rename = "HasSupport", skip_serializing_if = "Option::is_none")]
    pub has_support: Option<YesNoType>,
    #[serde(rename = "FinancingSustainableForestry", skip_serializing_if = "Option::is_none")]
    pub financing_sustainable_forestry: Option<FinancingSustainableForestryType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<DateType>,
    #[serde(rename = "PreNotificationAllowed", skip_serializing_if = "Option::is_none")]
    pub pre_notification_allowed: Option<YesNoType>,
    #[serde(rename = "BeginNotificationAllowed", skip_serializing_if = "Option::is_none")]
    pub begin_notification_allowed: Option<YesNoType>,
    #[serde(rename = "SendNotificationsAlways", skip_serializing_if = "Option::is_none")]
    pub send_notifications_always: Option<YesNoType>,
    #[serde(rename = "LargeSummaryReportRequired", skip_serializing_if = "Option::is_none")]
    pub large_summary_report_required: Option<YesNoType>,
    #[serde(rename = "TestAreaMethod", skip_serializing_if = "Option::is_none")]
    pub test_area_method: Option<SamplePlotType>,
    #[serde(rename = "TestAreaRequired", skip_serializing_if = "Option::is_none")]
    pub test_area_required: Option<YesNoType>,
    #[serde(rename = "IslandWorkingSite", skip_serializing_if = "Option::is_none")]
    pub island_working_site: Option<YesNoType>,
    #[serde(rename = "StormWorkingSite", skip_serializing_if = "Option::is_none")]
    pub storm_working_site: Option<YesNoType>,
    #[serde(rename = "SodWorkingSite", skip_serializing_if = "Option::is_none")]
    pub sod_working_site: Option<YesNoType>,
    #[serde(rename = "CanCultivateInAutumn", skip_serializing_if = "Option::is_none")]
    pub can_cultivate_in_autumn: Option<YesNoType>,
    #[serde(rename = "MixedForestRegenarationMethods", skip_serializing_if = "Option::is_none")]
    pub mixed_forest_regenaration_methods: Option<YesNoType>,
    #[serde(rename = "IsValueForceWorkingSite", skip_serializing_if = "Option::is_none")]
    pub is_value_force_working_site: Option<YesNoType>,
    #[serde(rename = "ForestCertification")]
    pub forest_certification: Vec<CertificationSystemType>,
    #[serde(rename = "CertificationHandlingInstructions", skip_serializing_if = "Option::is_none")]
    pub certification_handling_instructions: Option<String3000Type>,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<SpareTreesByCategoryType>,
    #[serde(rename = "DryingClass", skip_serializing_if = "Option::is_none")]
    pub drying_class: Option<DryingClassType>,
    #[serde(rename = "HumidityPercentage", skip_serializing_if = "Option::is_none")]
    pub humidity_percentage: Option<Decimal1FractionDigitType>,
    #[serde(rename = "CuttingControlRequired", skip_serializing_if = "Option::is_none")]
    pub cutting_control_required: Option<YesNoType>,
    #[serde(rename = "CuttingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub cutting_finished_date: Option<DateType>,
    #[serde(rename = "StumpLiftingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_finished_date: Option<DateType>,
    #[serde(rename = "BiomassFinishedDate", skip_serializing_if = "Option::is_none")]
    pub biomass_finished_date: Option<DateType>,
    #[serde(rename = "AssortmentIncrementAllowed", skip_serializing_if = "Option::is_none")]
    pub assortment_increment_allowed: Option<YesNoType>,
    #[serde(rename = "EnvironmentalObjectInfo", skip_serializing_if = "Option::is_none")]
    pub environmental_object_info: Option<String3000Type>,
    #[serde(rename = "WorkingSafetyInfo", skip_serializing_if = "Option::is_none")]
    pub working_safety_info: Option<String3000Type>,
    #[serde(rename = "AccessRightsInfo", skip_serializing_if = "Option::is_none")]
    pub access_rights_info: Option<String3000Type>,
    #[serde(rename = "MainWorkCode")]
    pub main_work_code: MainWorkCodeType,
    #[serde(rename = "ProductionFileSendFrequency", skip_serializing_if = "Option::is_none")]
    pub production_file_send_frequency: Option<PositiveIntegerType>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<String50Type>,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<PricingMethodType>,
    #[serde(rename = "ContinuousCoverForestry", skip_serializing_if = "Option::is_none")]
    pub continuous_cover_forestry: Option<YesNoType>,
    #[serde(rename = "PreviousBlockState", skip_serializing_if = "Option::is_none")]
    pub previous_block_state: Option<PreviousBlockStatusType>,
    #[serde(rename = "PreviousBlocks", skip_serializing_if = "Option::is_none")]
    pub previous_blocks: Option<PreviousBlockInfoType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "Stands")]
    pub stands: StandsType,
    #[serde(rename = "Storages", skip_serializing_if = "Option::is_none")]
    pub storages: Option<StoragesType>,
    #[serde(rename = "StoragesForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StemTypeBulks", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulks: Option<StemTypeBulksType>,
    #[serde(rename = "TreeSpeciesAttributes", skip_serializing_if = "Option::is_none")]
    pub tree_species_attributes: Option<TreeSpeciesAttributesType>,
    #[serde(rename = "ProductUserIds", skip_serializing_if = "Option::is_none")]
    pub product_user_ids: Option<ProductUserIdsType>,
    #[serde(rename = "DiameterSections", skip_serializing_if = "Option::is_none")]
    pub diameter_sections: Option<DiameterSectionsType>,
    #[serde(rename = "OtherRemarks", skip_serializing_if = "Option::is_none")]
    pub other_remarks: Option<String3000Type>,
    #[serde(rename = "Deliveries", skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<DeliveriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReductionType {
    #[serde(rename = "StumpStemCount", skip_serializing_if = "Option::is_none")]
    pub stump_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StumpMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stump_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingsType {
    #[serde(rename = "Training", skip_serializing_if = "Option::is_none")]
    pub training: Option<Vec<TrainingDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyTransactionTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsTurvakieltoKytkinTyyppi {
    #[serde(rename = "base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsUlkomaaPostitoimipaikkaNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaLawType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAreaTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostinumeroKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtAssortmentGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "PaymentReference", skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<PaymentsReferenceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDryingClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersType {
    #[serde(rename = "@subsidyPossibility")]
    pub subsidy_possibility: YesNoNotKnownType,
    #[serde(rename = "@sellerGroup")]
    pub seller_group: SellerGroupType,
    #[serde(rename = "Seller")]
    pub seller: Vec<SellerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "FinancingActCompletionGeometry")]
    pub financing_act_completion_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingSeasonType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStampType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: WtcPlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureIdentifierExtensionType {
    #[serde(rename = "base")]
    pub base: String,
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
pub struct JhsKolmasRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPaayksikkoNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItemType {
    #[serde(rename = "ListId")]
    pub list_id: PositiveIntegerType,
    #[serde(rename = "ListItem")]
    pub list_item: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityDataType {
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "TransportAccessibility", skip_serializing_if = "Option::is_none")]
    pub transport_accessibility: Option<TransportAccessibilityType>,
    #[serde(rename = "HarvestingAccessibility", skip_serializing_if = "Option::is_none")]
    pub harvesting_accessibility: Option<HarvestingAccessibilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSoilConditioningBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    #[serde(flatten)]
    pub base: StbStandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3_1Type {
    #[serde(rename = "base")]
    pub base: f64,
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
    #[serde(rename = "FccCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<CustomerReference>,
    #[serde(rename = "FccFinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_application_reference: Option<FinancingActApplicationReference>,
    #[serde(rename = "FccCompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<CompletionDeclarationReference>,
    #[serde(rename = "FccForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_use_declaration_reference: Option<ForestUseDeclarationReference>,
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
pub struct FeebaseListItemType {
    #[serde(rename = "Id")]
    pub id: PositiveIntegerType,
    #[serde(rename = "FeeValue")]
    pub fee_value: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectDataType {
    #[serde(rename = "SelfMonitoringStandArea", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_stand_area: Option<AreaType>,
    #[serde(rename = "GoalTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub goal_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "GoalStemCount", skip_serializing_if = "Option::is_none")]
    pub goal_stem_count: Option<StemCountType>,
    #[serde(rename = "GoalAmountOfSoilPreparationSpot", skip_serializing_if = "Option::is_none")]
    pub goal_amount_of_soil_preparation_spot: Option<PositiveInteger5digitsType>,
    #[serde(rename = "RealAmountOfSoilPreparationSpot", skip_serializing_if = "Option::is_none")]
    pub real_amount_of_soil_preparation_spot: Option<PositiveInteger5digitsType>,
    #[serde(rename = "EstimatedWorkingTimeConsumption", skip_serializing_if = "Option::is_none")]
    pub estimated_working_time_consumption: Option<PositiveInteger5digitsType>,
    #[serde(rename = "TimeIntervalForMeasuringSamplePlot", skip_serializing_if = "Option::is_none")]
    pub time_interval_for_measuring_sample_plot: Option<PositiveInteger5digitsType>,
    #[serde(rename = "Notices", skip_serializing_if = "Option::is_none")]
    pub notices: Option<String1000Type>,
    #[serde(rename = "WorkSafetyRisks", skip_serializing_if = "Option::is_none")]
    pub work_safety_risks: Option<WorkSafetyRisksType>,
    #[serde(rename = "SoilPreparationSpotsAreEnough", skip_serializing_if = "Option::is_none")]
    pub soil_preparation_spots_are_enough: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: FccPowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricesAndCurrencyGroup {
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "TotalPrice")]
    pub total_price: TotalPrice,
    #[serde(rename = "Currency")]
    pub currency: Currency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIdentifierGroup {
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<AssortmentCode>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpecies,
    #[serde(rename = "StemType")]
    pub stem_type: StemType,
    #[serde(rename = "AssortmentMainGroup", skip_serializing_if = "Option::is_none")]
    pub assortment_main_group: Option<AssortmentMainGroup>,
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<AssortmentName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType2 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "EstateRegisterUnitGroup")]
    pub estate_register_unit_group: EstateRegisterUnitGroup,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub location_municipality_number: Option<CoMunicipalityNumberType>,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<CoMunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoadType {
    #[serde(rename = "PartitialLoadId")]
    pub partitial_load_id: u32,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "LoadVolume", skip_serializing_if = "Option::is_none")]
    pub load_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "LoadGreenMass")]
    pub load_green_mass: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EbEnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeSpecifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotificationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDrainingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIDType {
    #[serde(flatten)]
    pub base: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CddDistributionModelGroup {
    #[serde(rename = "Weibull")]
    pub weibull: Weibull,
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSB,
    #[serde(rename = "Normal")]
    pub normal: Normal,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "Gamma")]
    pub gamma: Gamma,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingPurposeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerType {
    #[serde(rename = "PersonId", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String20Type>,
    #[serde(rename = "OrganisationId", skip_serializing_if = "Option::is_none")]
    pub organisation_id: Option<String20Type>,
    #[serde(rename = "PersonRole", skip_serializing_if = "Option::is_none")]
    pub person_role: Option<String50Type>,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String20Type,
    #[serde(rename = "EmailAddress")]
    pub email_address: String50Type,
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "SendWorkingAloneNotification", skip_serializing_if = "Option::is_none")]
    pub send_working_alone_notification: Option<YesNoType>,
    #[serde(rename = "SendNotifications", skip_serializing_if = "Option::is_none")]
    pub send_notifications: Option<YesNoType>,
    #[serde(rename = "NotificationContactPerson", skip_serializing_if = "Option::is_none")]
    pub notification_contact_person: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DeclarationStandReference")]
    pub declaration_stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: StbAreaType,
    #[serde(rename = "CuttingPurpose")]
    pub cutting_purpose: CoCuttingPurposeType,
    #[serde(rename = "CuttingRealizationPractice")]
    pub cutting_realization_practice: CoCuttingRealizationPracticeType,
    #[serde(rename = "ForestDamageQualifier", skip_serializing_if = "Option::is_none")]
    pub forest_damage_qualifier: Option<CoForestDamageQualifierType>,
    #[serde(rename = "HabitatCode")]
    pub habitat_code: CoHabitatCodeType,
    #[serde(rename = "HabitatOperations", skip_serializing_if = "Option::is_none")]
    pub habitat_operations: Option<HabitatOperationsType>,
    #[serde(rename = "OtherHabitatCode", skip_serializing_if = "Option::is_none")]
    pub other_habitat_code: Option<CoOtherHabitatCodeType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "DeclarationMainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub declaration_main_tree_species: Option<CoTreeSpeciesConciseType>,
    #[serde(rename = "DeclarationDevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub declaration_development_class: Option<CoDeclarationDevelopmentClassType>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<CoAgeType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "DeclarationRegenerationCommitment", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_commitment: Option<CoRegenerationCommitmentType>,
    #[serde(rename = "DeclarationRegenerationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_operation: Option<CoDeclarationRegenerationOperationType>,
    #[serde(rename = "DeclarationSoilPreparationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_soil_preparation_operation: Option<CoDeclarationSoilPreparationOperationType>,
    #[serde(rename = "DeclarationOtherOperations", skip_serializing_if = "Option::is_none")]
    pub declaration_other_operations: Option<DeclarationOtherOperationsType>,
    #[serde(rename = "DeclarationStandTextInformation", skip_serializing_if = "Option::is_none")]
    pub declaration_stand_text_information: Option<CoString2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsStatusryhmaTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustificationsType {
    #[serde(rename = "Justification")]
    pub justification: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctResponsibleOfPreClearingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: WtcoPurchaseModeType,
    #[serde(rename = "AssortmentCompactClasses")]
    pub assortment_compact_classes: AsAssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentAllElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: AssortmentIdentifierGroup,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<CoUsedPricingMethodType>,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: QuantityGroup,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<PricesAndCurrencyGroup>,
    #[serde(rename = "DimensionRequirementsGroup", skip_serializing_if = "Option::is_none")]
    pub dimension_requirements_group: Option<DimensionRequirementsGroup>,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "GradeCode", skip_serializing_if = "Option::is_none")]
    pub grade_code: Option<CoGradeCodeType>,
    #[serde(rename = "WoodLots", skip_serializing_if = "Option::is_none")]
    pub wood_lots: Option<WoodLotsType>,
    #[serde(rename = "MeasurementMethod", skip_serializing_if = "Option::is_none")]
    pub measurement_method: Option<CoMeasurementMethodType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
    #[serde(rename = "PriceMatrix", skip_serializing_if = "Option::is_none")]
    pub price_matrix: Option<PriceMatrixType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaCuttingByMachineType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString1000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCleanlinessClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<CoDateType>,
    #[serde(rename = "SilvicultureValidity", skip_serializing_if = "Option::is_none")]
    pub silviculture_validity: Option<SilvicultureValidityType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<PrProductsType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureText", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_text: Option<string>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<CoReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingExtraQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureExtraQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageType {
    #[serde(flatten)]
    pub base: ImageBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Category")]
    pub category: ImageCategoryType,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<ImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<FeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<ERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<String20Type>,
    #[serde(rename = "Position")]
    pub position: PointGeometryType,
    #[serde(rename = "InfoText")]
    pub info_text: String200Type,
    #[serde(rename = "Photographer")]
    pub photographer: String50Type,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<PositiveInteger3digitsType>,
    #[serde(rename = "ImageDate")]
    pub image_date: TimeStampType,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionsType {
    #[serde(rename = "SilvicultureRestrictionDetails")]
    pub silviculture_restriction_details: Vec<SilvicultureRestrictionDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "FudrForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<ForestUseDeclarationReference>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AsAssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<AssortmentClasses>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FacAreaNo")]
    pub fac_area_no: AreaNo,
    #[serde(rename = "FacFinancingActWorkCode")]
    pub fac_financing_act_work_code: FinancingActWorkCode,
    #[serde(rename = "FacPayeesAndRealEstates")]
    pub fac_payees_and_real_estates: PayeesAndRealEstates,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentDetailsType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CaseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<FccFinancingActNumberType>,
    #[serde(rename = "FccForestCentreMessageReferenceType", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "FccForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference: Option<ForestCentreMessageReference>,
    #[serde(rename = "FccCompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<CompletionDeclarationReference>,
    #[serde(rename = "FccCompletionDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_number: Option<CompletionDeclarationNumber>,
    #[serde(rename = "FccCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<CustomerReference>,
    #[serde(rename = "FccArrivalDate")]
    pub fcc_arrival_date: ArrivalDate,
    #[serde(rename = "FccPaymentReference")]
    pub fcc_payment_reference: PaymentReference,
    #[serde(rename = "FccPaymentDate")]
    pub fcc_payment_date: PaymentDate,
    #[serde(rename = "FccBankAccount")]
    pub fcc_bank_account: BankAccount,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<CiContactInformationType>,
    #[serde(rename = "FccContactPerson")]
    pub fcc_contact_person: ContactPerson,
    #[serde(rename = "PaymentTexts", skip_serializing_if = "Option::is_none")]
    pub payment_texts: Option<PaymentTextsType>,
    #[serde(rename = "FccOverallTotalSubsidy")]
    pub fcc_overall_total_subsidy: OverallTotalSubsidy,
    #[serde(rename = "SubsidyAppliers")]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String500Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal7And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: BdtString1000Type,
    #[serde(flatten)]
    pub base: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "CodAdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub cod_additional_details: Option<AdditionalDetails>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfFeatureAdditionalInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: NotEmptyTreeSpeciesType,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "SawlogPercent", skip_serializing_if = "Option::is_none")]
    pub sawlog_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "TotalSawlogVolume", skip_serializing_if = "Option::is_none")]
    pub total_sawlog_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "TotalPulpwoodVolume", skip_serializing_if = "Option::is_none")]
    pub total_pulpwood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TotalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<CoVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreDataMessageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeasurementMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHarvestingAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamicType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseDynamicType,
    #[serde(rename = "FinalAuditSpareTrees", skip_serializing_if = "Option::is_none")]
    pub final_audit_spare_trees: Option<FinalAuditSpareTreesByCategoryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasicDataType {
    #[serde(rename = "@timeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<TwoDigitPositiveIntegerType>,
    #[serde(rename = "SamplePlotSize", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size: Option<SamplePlotSizeType>,
    #[serde(rename = "SamplePlotSizeTreeReduction", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size_tree_reduction: Option<SamplePlotSizeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageType {
    #[serde(rename = "PreviousMooseDamageEvaluationDate", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_date: Option<CoDateType>,
    #[serde(rename = "PreviousMooseDamageEvaluationMunicipality", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_municipality: Option<CoMunicipalityNameType>,
    #[serde(rename = "PreviousSameAreaMooseDamageCompensationYear", skip_serializing_if = "Option::is_none")]
    pub previous_same_area_moose_damage_compensation_year: Option<CoYearType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem", skip_serializing_if = "Option::is_none")]
    pub certification_system: Option<Vec<WtcoCertificationSystemType>>,
    #[serde(flatten)]
    pub base: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<FdForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataMomentType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(rename = "Contractors")]
    pub contractors: ContractorsType,
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<String100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<String20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "ResourceName")]
    pub resource_name: String50Type,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<MachineManufacturerType>,
    #[serde(rename = "Model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String50Type>,
    #[serde(rename = "ModelYear", skip_serializing_if = "Option::is_none")]
    pub model_year: Option<YearType>,
    #[serde(rename = "HarvesterModel", skip_serializing_if = "Option::is_none")]
    pub harvester_model: Option<String50Type>,
    #[serde(rename = "HavesterModelYear", skip_serializing_if = "Option::is_none")]
    pub havester_model_year: Option<YearType>,
    #[serde(rename = "DeploymentYear", skip_serializing_if = "Option::is_none")]
    pub deployment_year: Option<YearType>,
    #[serde(rename = "DeploymentMonth", skip_serializing_if = "Option::is_none")]
    pub deployment_month: Option<String5Type>,
    #[serde(rename = "OwnWeight", skip_serializing_if = "Option::is_none")]
    pub own_weight: Option<PositiveInteger6digitsType>,
    #[serde(rename = "WorkingWeight", skip_serializing_if = "Option::is_none")]
    pub working_weight: Option<WorkingWeightType>,
    #[serde(rename = "CommunicationType", skip_serializing_if = "Option::is_none")]
    pub communication_type: Option<String100Type>,
    #[serde(rename = "MeasuringDeviceVersion", skip_serializing_if = "Option::is_none")]
    pub measuring_device_version: Option<String50Type>,
    #[serde(rename = "MeasuringDeviceLastControl", skip_serializing_if = "Option::is_none")]
    pub measuring_device_last_control: Option<DateType>,
    #[serde(rename = "LoaderScaleModel", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model: Option<String50Type>,
    #[serde(rename = "LoaderScaleModelYear", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model_year: Option<YearType>,
    #[serde(rename = "ServiceStartDate")]
    pub service_start_date: DateType,
    #[serde(rename = "OutOfServiceStartDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_start_date: Option<DateType>,
    #[serde(rename = "OutOfServiceEndDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_end_date: Option<DateType>,
    #[serde(rename = "LoadRating", skip_serializing_if = "Option::is_none")]
    pub load_rating: Option<LoadRatingType>,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<PointGeometryType>,
    #[serde(rename = "ElevatorCertificate", skip_serializing_if = "Option::is_none")]
    pub elevator_certificate: Option<YesNoType>,
    #[serde(rename = "ExtinguisherVerificationDate", skip_serializing_if = "Option::is_none")]
    pub extinguisher_verification_date: Option<DateType>,
    #[serde(rename = "Telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String50Type>,
    #[serde(rename = "SubContractorResource", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_resource: Option<YesNoType>,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: String20Type,
    #[serde(rename = "SubContractorVerified", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_verified: Option<YesNoType>,
    #[serde(rename = "Removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<YesNoType>,
    #[serde(rename = "RemoveDate", skip_serializing_if = "Option::is_none")]
    pub remove_date: Option<DateType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "OrdererResponsibilityDocumentsChecked", skip_serializing_if = "Option::is_none")]
    pub orderer_responsibility_documents_checked: Option<YesNoType>,
    #[serde(rename = "Peripherals", skip_serializing_if = "Option::is_none")]
    pub peripherals: Option<PeripheralsType>,
    #[serde(rename = "ExternalSystemInUse", skip_serializing_if = "Option::is_none")]
    pub external_system_in_use: Option<YesNoType>,
    #[serde(rename = "ExternalSystemName", skip_serializing_if = "Option::is_none")]
    pub external_system_name: Option<String50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraStemTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoIncludePaymentPlanType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativesType {
    #[serde(rename = "WorkingRepresentative")]
    pub working_representative: Vec<WorkingRepresentativeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContactorId")]
    pub contactor_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Interrupted")]
    pub interrupted: YesNoType,
    #[serde(rename = "FulfilledArea", skip_serializing_if = "Option::is_none")]
    pub fulfilled_area: Option<FulfilledAreasType>,
    #[serde(rename = "UnfulfilledArea", skip_serializing_if = "Option::is_none")]
    pub unfulfilled_area: Option<PolygonOrMultiPolygon2Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal2FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "OOffer", skip_serializing_if = "Option::is_none")]
    pub o_offer: Option<Vec<Offer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortERPIdType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingClassificationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsType {
    #[serde(rename = "Payment")]
    pub payment: Vec<ForestCentrePaymentDetailsType>,
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
    #[serde(rename = "GdtPointLineAndPolygonGeometriesGroup")]
    pub gdt_point_line_and_polygon_geometries_group: PointLineAndPolygonGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: CallForOffer,
    #[serde(rename = "CfowsCFOWorkingSite")]
    pub cfows_c_f_o_working_site: CFOWorkingSite,
    #[serde(rename = "BaBusinessAcceptance")]
    pub ba_business_acceptance: BusinessAcceptance,
    #[serde(rename = "WppPayment")]
    pub wpp_payment: Payment,
    #[serde(rename = "WpmcMeasurementCertificate")]
    pub wpmc_measurement_certificate: MeasurementCertificate,
    #[serde(rename = "FudrForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: ForestUseDeclarationReferences,
    #[serde(rename = "WstcrContract")]
    pub wstcr_contract: Contract,
    #[serde(rename = "WstoOffer")]
    pub wsto_offer: Offer,
    #[serde(rename = "OwsOWorkingSite")]
    pub ows_o_working_site: OWorkingSite,
    #[serde(rename = "OsuOperations")]
    pub osu_operations: Operations,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: ContactRequest,
    #[serde(rename = "MsMapSymbol")]
    pub ms_map_symbol: MapSymbol,
    #[serde(rename = "AckAcknowledge")]
    pub ack_acknowledge: Acknowledge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TsTreeStandDataType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulksType {
    #[serde(rename = "StemTypeBulk", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulk: Option<Vec<StemTypeBulkType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: Decimal3FractionDigitsType,
    #[serde(rename = "AmountLeft")]
    pub amount_left: Decimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePriorityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkarcType {
    #[serde(rename = "XlinkarcModel")]
    pub xlinkarc_model: arcModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaRegenerationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMessageTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlaceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal3And2PositiveType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDistanceUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffersType {
    #[serde(rename = "RelatedCallForOffer")]
    pub related_call_for_offer: Vec<RelatedCallForOfferType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDataType {
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<String100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<String20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "WorkingSites")]
    pub working_sites: WorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String200Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreControlObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferTextType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetkiTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesDataType {
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RoundWoodSalesRows")]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotType {
    #[serde(rename = "WoodLotInformationGroup")]
    pub wood_lot_information_group: WoodLotInformationGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<Vec<MaterialType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonType {
    #[serde(rename = "ReasonCode")]
    pub reason_code: CoString10Type,
    #[serde(rename = "ReasonDescription")]
    pub reason_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingOriginType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestOwnerGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKutsumaNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberType {
    #[serde(flatten)]
    pub base: JhsPuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYearType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNameType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDateType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString25Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTextsType {
    #[serde(rename = "PaymentText")]
    pub payment_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoRealEstateType {
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<RealEstatesGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestDataUpdateType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingRestrictionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileNameType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoadType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<String20Type>,
    #[serde(rename = "WorkLoadId")]
    pub work_load_id: u64,
    #[serde(rename = "Accepted")]
    pub accepted: YesNoType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<String20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<String50Type>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<String50Type>,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier1", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "WorkCodeQualifier2", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "WorkCodeQualifier3", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "WorkCodeQualifier4", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "WorkCodeQualifier5", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier5: Option<Vec<WorkCodeQualifierType5>>,
    #[serde(rename = "WorkLoad1")]
    pub work_load1: Decimal2FractionDigitsType,
    #[serde(rename = "WorkLoad1Unit")]
    pub work_load1_unit: WorkCodeUnitType,
    #[serde(rename = "TransportDistance", skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TransportDistanceUnit", skip_serializing_if = "Option::is_none")]
    pub transport_distance_unit: Option<DistanceUnitType>,
    #[serde(rename = "WorkLoad2", skip_serializing_if = "Option::is_none")]
    pub work_load2: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoad2Unit", skip_serializing_if = "Option::is_none")]
    pub work_load2_unit: Option<WorkCodeUnitType>,
    #[serde(rename = "WorkLoadInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_invoiced: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoadUnitInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_unit_invoiced: Option<WorkCodeUnitType>,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StandFinished", skip_serializing_if = "Option::is_none")]
    pub stand_finished: Option<YesNoType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeStampType>,
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeStampType>,
    #[serde(rename = "SavingTime", skip_serializing_if = "Option::is_none")]
    pub saving_time: Option<TimeStampType>,
    #[serde(rename = "WorkPointCount", skip_serializing_if = "Option::is_none")]
    pub work_point_count: Option<PositiveInteger5digitsType>,
    #[serde(rename = "WorkGrouMembers", skip_serializing_if = "Option::is_none")]
    pub work_grou_members: Option<WorkGrouMembersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetType {
    #[serde(rename = "Status2")]
    pub status2: CoChangeStateType,
    #[serde(rename = "TargetId")]
    pub target_id: String,
    #[serde(rename = "TargetNumber")]
    pub target_number: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
    #[serde(rename = "BasePartNumber")]
    pub base_part_number: VirtaPartNumberType,
    #[serde(rename = "EstablishedPartNumber")]
    pub established_part_number: VirtaPartNumberType,
    #[serde(rename = "TargetAnnouncedAmount")]
    pub target_announced_amount: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "HabitatAdvertisement")]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
    #[serde(rename = "TargetExtraInfo")]
    pub target_extra_info: VirtaExtraInfoType,
    #[serde(rename = "GmlPolygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<Polygon>,
    #[serde(rename = "TargetParts", skip_serializing_if = "Option::is_none")]
    pub target_parts: Option<TargetPartsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageType {
    #[serde(rename = "MainDamage", skip_serializing_if = "Option::is_none")]
    pub main_damage: Option<YesNoType>,
    #[serde(rename = "DamageSourceCode", skip_serializing_if = "Option::is_none")]
    pub damage_source_code: Option<string>,
    #[serde(rename = "DamageSourceDescription", skip_serializing_if = "Option::is_none")]
    pub damage_source_description: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String3000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoReportedStatisticsOperationTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestCentreSelfMonitoringDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: YesNoType,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTimeType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "StartTime")]
    pub start_time: TimeStampType,
    #[serde(rename = "EndTime")]
    pub end_time: TimeStampType,
    #[serde(rename = "SavingTime")]
    pub saving_time: TimeStampType,
    #[serde(rename = "Sawinghours", skip_serializing_if = "Option::is_none")]
    pub sawinghours: Option<SawinghoursDataType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandBasicDataType {
    #[serde(rename = "ControlStandArea", skip_serializing_if = "Option::is_none")]
    pub control_stand_area: Option<AreaType>,
    #[serde(rename = "InspectionMethod", skip_serializing_if = "Option::is_none")]
    pub inspection_method: Option<InspectionMethodType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnitType {
    #[serde(flatten)]
    pub base: CoUnitPerHectareType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysisType {
    #[serde(rename = "ResultOfAccessibilityAnalysis")]
    pub result_of_accessibility_analysis: Vec<ResultOfAccessibilityAnalysisType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEvaluationSubjectType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditAnswerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: YesNoType,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListType {
    #[serde(rename = "FeeBasisListItem")]
    pub fee_basis_list_item: Vec<FeebasisListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCurrencyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOtherPublicSubstituteType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateType {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrataType {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoActionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcPlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPreventionSubstanceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdStringType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccCostTypeDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanlinessClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperationsType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesGroup {
    #[serde(rename = "RealEstate")]
    pub real_estate: RealEstate,
    #[serde(rename = "RealEstateName", skip_serializing_if = "Option::is_none")]
    pub real_estate_name: Option<RealEstateName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumeroTyyppi {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSitePlanningStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString2000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsibleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebasisListItemType {
    #[serde(rename = "Id")]
    pub id: PositiveIntegerType,
    #[serde(rename = "FeeText")]
    pub fee_text: String50Type,
    #[serde(rename = "FeeUnit")]
    pub fee_unit: String10Type,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectingMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ImageCount")]
    pub image_count: PositiveInteger2digitsType,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<String20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<String50Type>,
    #[serde(rename = "WorkingSitePlanningStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_status: Option<WorkingSitePlanningStatusType>,
    #[serde(rename = "WorkingSitePlanningOperation", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_operation: Option<WorkingSitePlanningOperationStatusType>,
    #[serde(rename = "WorkingSitePlanningInfo", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_info: Option<String3000Type>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "Storages", skip_serializing_if = "Option::is_none")]
    pub storages: Option<StoragesType>,
    #[serde(rename = "StoragesForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "StorageProposals", skip_serializing_if = "Option::is_none")]
    pub storage_proposals: Option<StoragesType>,
    #[serde(rename = "StoragesProposalForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_proposal_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "ProductUserIds", skip_serializing_if = "Option::is_none")]
    pub product_user_ids: Option<ProductUserIdsType>,
    #[serde(rename = "CanCultivateInAutumn", skip_serializing_if = "Option::is_none")]
    pub can_cultivate_in_autumn: Option<YesNoType>,
    #[serde(rename = "DelinationObjectOrderId", skip_serializing_if = "Option::is_none")]
    pub delination_object_order_id: Option<String200Type>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "WorkingsiteInfo", skip_serializing_if = "Option::is_none")]
    pub workingsite_info: Option<String3000Type>,
    #[serde(rename = "PurchaseContractExtraInfo", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_extra_info: Option<String3000Type>,
    #[serde(rename = "EnvironmentalObjectInfo", skip_serializing_if = "Option::is_none")]
    pub environmental_object_info: Option<String3000Type>,
    #[serde(rename = "WorkingSafetyInfo", skip_serializing_if = "Option::is_none")]
    pub working_safety_info: Option<String3000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoWoodLotInformationTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertiserType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoNotNeededType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesConciseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocumentClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionsType {
    #[serde(rename = "DiameterSection")]
    pub diameter_section: Vec<SectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDifficultyClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalType {
    #[serde(rename = "Mean")]
    pub mean: MeanType,
    #[serde(rename = "Variance")]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "TrTree")]
    pub tr_tree: Vec<Tree>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstateType {
    #[serde(flatten)]
    pub base: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetailsType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<CoString1500Type>,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: JhsAlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetkiTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLocationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: OwsWorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal3FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityGroup {
    #[serde(rename = "Quantity")]
    pub quantity: Quantity,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: QuantityUnit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfoType {
    #[serde(rename = "CuttingAreaPreclearingNeed", skip_serializing_if = "Option::is_none")]
    pub cutting_area_preclearing_need: Option<CuttingAreaPreclearingNeedType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "StoutTimberClassifier", skip_serializing_if = "Option::is_none")]
    pub stout_timber_classifier: Option<StoutTimberClassifierType>,
    #[serde(rename = "LoggingAccessibility", skip_serializing_if = "Option::is_none")]
    pub logging_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<CoStemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDigitPositiveIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<WorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: Decimal1FractionDigitType,
    #[serde(rename = "Orientation")]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReviewType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarianceType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaYesNoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType2 {
    #[serde(flatten)]
    pub base: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: CoMessageType,
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: String,
    #[serde(rename = "GeometryObjects")]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FourDigitPositiveIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "StandId")]
    pub stand_id: String20Type,
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
    #[serde(rename = "RegisterUnitId", skip_serializing_if = "Option::is_none")]
    pub register_unit_id: Option<String20Type>,
    #[serde(rename = "ParcelNo", skip_serializing_if = "Option::is_none")]
    pub parcel_no: Option<PositiveInteger6digitsType>,
    #[serde(rename = "ParcelLabel", skip_serializing_if = "Option::is_none")]
    pub parcel_label: Option<String100Type>,
    #[serde(rename = "ForestPlanStandId", skip_serializing_if = "Option::is_none")]
    pub forest_plan_stand_id: Option<String10Type>,
    #[serde(rename = "DitchOrRoadPlanId", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_id: Option<String10Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<String100Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "StumpTreatment", skip_serializing_if = "Option::is_none")]
    pub stump_treatment: Option<YesNoType>,
    #[serde(rename = "BiomassCollection", skip_serializing_if = "Option::is_none")]
    pub biomass_collection: Option<YesNoType>,
    #[serde(rename = "StumpRaising", skip_serializing_if = "Option::is_none")]
    pub stump_raising: Option<YesNoType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<WideDevelopmentClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<YesNoType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveIntegerType>,
    #[serde(rename = "LabelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<PointGeometryType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<String1000Type>,
    #[serde(rename = "ForestUseDeclarationStandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_stand_extra_info: Option<String2000Type>,
    #[serde(rename = "ForestUseDeclarationStandFellingPurpose", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_stand_felling_purpose: Option<CuttingPurposeType>,
    #[serde(rename = "StandTreesCuttingVolume", skip_serializing_if = "Option::is_none")]
    pub stand_trees_cutting_volume: Option<PositiveInteger4digitsType>,
    #[serde(rename = "GrowingTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub growing_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TargetDensityGroup", skip_serializing_if = "Option::is_none")]
    pub target_density_group: Option<TargetDensityGroup>,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "StandTreesCutting", skip_serializing_if = "Option::is_none")]
    pub stand_trees_cutting: Option<StandTreesCuttingType>,
    #[serde(rename = "StandTreesCurrent", skip_serializing_if = "Option::is_none")]
    pub stand_trees_current: Option<StandTreesType>,
    #[serde(rename = "StandTreesStratumLeaving", skip_serializing_if = "Option::is_none")]
    pub stand_trees_stratum_leaving: Option<StandTreesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: BdtPositiveInteger2digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea")]
    pub service_buyer_area: String20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6_2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeCharType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbIdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionType {
    #[serde(rename = "CoRestrictionType")]
    pub co_restriction_type: RestrictionType,
    #[serde(rename = "CoRestrictionCode")]
    pub co_restriction_code: RestrictionCode,
    #[serde(rename = "RestrictionStartDate", skip_serializing_if = "Option::is_none")]
    pub restriction_start_date: Option<StartDateType>,
    #[serde(rename = "RestrictionEndDate", skip_serializing_if = "Option::is_none")]
    pub restriction_end_date: Option<EndDateType>,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "RestrictionDescription", skip_serializing_if = "Option::is_none")]
    pub restriction_description: Option<CoString1500Type>,
    #[serde(rename = "RestrictionOutOfObject", skip_serializing_if = "Option::is_none")]
    pub restriction_out_of_object: Option<CoYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "BusinessId")]
    pub business_id: String,
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "Roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RolesType>>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServicesType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString200Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "@DeliveryUserId")]
    pub delivery_user_id: String50Type,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: Vec<BdtString100Type>,
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsIkaluokkaTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateMmDdYyyyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestObjectDataObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectKeys", skip_serializing_if = "Option::is_none")]
    pub object_keys: Option<ObjectKeysType>,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveIntegerType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax1IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSumTableAreaType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot", skip_serializing_if = "Option::is_none")]
    pub sample_plot: Option<Vec<SamplePlotType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger6digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationNatureManagementSpecifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSilvicultureBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMaxType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TgtVirtaExtraInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlUseCaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRatingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageDataType {
    #[serde(rename = "ReferenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<CoReferenceType>,
    #[serde(rename = "KeyElementNS", skip_serializing_if = "Option::is_none")]
    pub key_element_n_s: Option<CoString500Type>,
    #[serde(rename = "KeyElementName", skip_serializing_if = "Option::is_none")]
    pub key_element_name: Option<CoString200Type>,
    #[serde(rename = "KeyElementId", skip_serializing_if = "Option::is_none")]
    pub key_element_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "KeyInfoAsText", skip_serializing_if = "Option::is_none")]
    pub key_info_as_text: Option<CoString2000Type>,
    #[serde(rename = "ErrorCode")]
    pub error_code: CoString25Type,
    #[serde(rename = "ErrorMessage")]
    pub error_message: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsType {
    #[serde(rename = "SubContractor", skip_serializing_if = "Option::is_none")]
    pub sub_contractor: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMainGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction2Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowsType {
    #[serde(rename = "LoggingsRow")]
    pub loggings_row: Vec<LoggingsRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoppersType {
    #[serde(rename = "Hopper", skip_serializing_if = "Option::is_none")]
    pub hopper: Option<Vec<HopperType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax5IntegralPartMax1FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesType {
    #[serde(rename = "Role")]
    pub role: Vec<OrganizationRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlkuPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@operationId")]
    pub operation_id: String,
    #[serde(rename = "@standId")]
    pub stand_id: String,
    #[serde(rename = "@productId")]
    pub product_id: String,
    #[serde(rename = "ProductKeyGroup")]
    pub product_key_group: ProductKeyGroup,
    #[serde(rename = "ProductName")]
    pub product_name: CoString500Type,
    #[serde(rename = "Quantity")]
    pub quantity: CoDecimal2FractionDigitsType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoWideUnitType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<WtcTotalPriceType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "Consumption", skip_serializing_if = "Option::is_none")]
    pub consumption: Option<ConsumptionType>,
    #[serde(rename = "ConsumptionUnit", skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<ConsumptionUnitType>,
    #[serde(rename = "PlannedResource", skip_serializing_if = "Option::is_none")]
    pub planned_resource: Option<WtcPlannedResourceType>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer7digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactSoilTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoInspectionMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document4MBType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDaysAreaType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: AlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceGroup {
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: ForestHaulageDistance,
    #[serde(rename = "StorageId")]
    pub storage_id: StorageId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSCCertificationSystemSubTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String20Type,
    #[serde(rename = "NotificationType")]
    pub notification_type: NotificationTypeType,
    #[serde(rename = "RecipientType")]
    pub recipient_type: RecipientTypeType,
    #[serde(rename = "SenderUserId")]
    pub sender_user_id: String20Type,
    #[serde(rename = "SendTimestamp")]
    pub send_timestamp: TimeStampType,
    #[serde(rename = "StatusTimestamp")]
    pub status_timestamp: TimeStampType,
    #[serde(rename = "StatusCode")]
    pub status_code: StatusCodeType,
    #[serde(rename = "OriginalMessage")]
    pub original_message: String1000Type,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsType {
    #[serde(rename = "Attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCodeType {
    #[serde(flatten)]
    pub base: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "AlternativeIdentifier")]
    pub alternative_identifier: AlternativeIdentifierType,
    #[serde(rename = "AlternativeName", skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<AlternativeNameType>,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembersType {
    #[serde(rename = "WorkGrouMember")]
    pub work_grou_member: Vec<WorkGrouMemberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<ImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<ImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<FeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<ERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<String20Type>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<PointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<String50Type>,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<PositiveInteger3digitsType>,
    #[serde(rename = "ImageDate", skip_serializing_if = "Option::is_none")]
    pub image_date: Option<TimeStampType>,
    #[serde(rename = "Filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<base64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeType {
    #[serde(rename = "ReplyTo")]
    pub reply_to: String50Type,
    #[serde(rename = "StatusCode")]
    pub status_code: PositiveInteger3digitsType,
    #[serde(rename = "StatusMessage")]
    pub status_message: String1000Type,
    #[serde(rename = "OriginalMessageType")]
    pub original_message_type: String50Type,
    #[serde(rename = "StatusMessages", skip_serializing_if = "Option::is_none")]
    pub status_messages: Option<StatusMessageLanguageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPerHectareType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDataType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestUseDeclarationResponsibleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrataType {
    #[serde(rename = "StemDistributionStratum")]
    pub stem_distribution_stratum: Vec<StemDistributionStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1500Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtImageCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<WtcoRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaryType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "AgeSummary", skip_serializing_if = "Option::is_none")]
    pub age_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StemCountSummary")]
    pub stem_count_summary: PositiveInteger5digitsType,
    #[serde(rename = "MeanHeightSummary")]
    pub mean_height_summary: Decimal1FractionDigitType,
    #[serde(rename = "MeanDiameterSummary")]
    pub mean_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelatedType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasisType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "FeeBasis")]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCuttingByMachineType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsOsoiteNumeroTyyppi {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAssortmentMainGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetPartStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTaxType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValueType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionType {
    #[serde(rename = "Organisation1", skip_serializing_if = "Option::is_none")]
    pub organisation1: Option<String20Type>,
    #[serde(rename = "Organisation2", skip_serializing_if = "Option::is_none")]
    pub organisation2: Option<String20Type>,
    #[serde(rename = "Organisation3", skip_serializing_if = "Option::is_none")]
    pub organisation3: Option<String20Type>,
    #[serde(rename = "Organisation4", skip_serializing_if = "Option::is_none")]
    pub organisation4: Option<String20Type>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "Description")]
    pub description: String100Type,
    #[serde(rename = "Code")]
    pub code: String20Type,
    #[serde(rename = "MinDiameter")]
    pub min_diameter: PositiveIntegerType,
    #[serde(rename = "MinLength")]
    pub min_length: PositiveIntegerType,
    #[serde(rename = "UserCode")]
    pub user_code: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: String100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "Quality")]
    pub quality: String5Type,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionNoType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationsType {
    #[serde(rename = "ObjectProtectionOperation")]
    pub object_protection_operation: Vec<ObjectProtectionOperationType>,
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
pub struct FinancingActRealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType2,
    #[serde(rename = "SubsidyApplierReferenceList")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "EnvlEnvelope")]
    pub envl_envelope: Vec<Envelope>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteStateType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlersType {
    #[serde(rename = "DecisionHandler")]
    pub decision_handler: Vec<DecisionHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSukuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetsType {
    #[serde(rename = "TgtTarget")]
    pub tgt_target: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidityType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger3digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLoppuHetkiTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMaterialUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureExtraQualifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsedPricingMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsToinenRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnusTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaStandQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "FacCompletionDeclarationReference")]
    pub fac_completion_declaration_reference: CompletionDeclarationReference,
    #[serde(rename = "FacFinancingActNumber")]
    pub fac_financing_act_number: FinancingActNumber,
    #[serde(rename = "FacFinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_reference: Option<FinancingActApplicationReference>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<CoBankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: CoYesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: CoYesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_declaration_text_information: Option<FinancingActCompletionDeclarationTextInformation>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: Language,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "FacSender")]
    pub fac_sender: Sender,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "FacStartDate")]
    pub fac_start_date: StartDate,
    #[serde(rename = "FacEndDate")]
    pub fac_end_date: EndDate,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "FacCompletionDeclarationActors")]
    pub fac_completion_declaration_actors: CompletionDeclarationActors,
    #[serde(rename = "FacWorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub fac_working_representatives: Option<WorkingRepresentatives>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAgeType {
    #[serde(flatten)]
    pub base: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtUserRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSamplePlotType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationServiceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuntaKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesType {
    #[serde(rename = "Language")]
    pub language: Vec<LanguageCode1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeSpecifierType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAmmattiKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FeatureCode")]
    pub feature_code: CoFeatureCodeType,
    #[serde(rename = "FeatureInfo")]
    pub feature_info: CoString500Type,
    #[serde(rename = "GdtPointAndLineGeometriesGroup")]
    pub gdt_point_and_line_geometries_group: PointAndLineGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessMessageTimeStampType {
    #[serde(flatten)]
    pub base: CoTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListType {
    #[serde(rename = "FeeBaseListItem")]
    pub fee_base_list_item: Vec<FeebaseListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: NotEmptyTreeSpeciesType,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "SawlogPercent", skip_serializing_if = "Option::is_none")]
    pub sawlog_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "TotalSawlogVolume", skip_serializing_if = "Option::is_none")]
    pub total_sawlog_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "TotalPulpwoodVolume", skip_serializing_if = "Option::is_none")]
    pub total_pulpwood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TotalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<CoVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoOfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<pointProperty>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOfficeType {
    #[serde(flatten)]
    pub base: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "CddDistributionModelGroup")]
    pub cdd_distribution_model_group: DistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType5 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: ReRealEstatesWithOwnersInformationType2,
    #[serde(rename = "ProcessingAreas")]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCasesType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<Vec<ForestDataUpdateUseCaseType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMainTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreaType {
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceType {
    #[serde(rename = "@message")]
    pub message: MessageTypeType,
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: String,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "BusinessMessageTimeStamp")]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
    #[serde(rename = "BusinessAcceptanceActor")]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
    #[serde(rename = "BusinessAcceptanceStatus")]
    pub business_acceptance_status: CoBusinessAcceptanceStatusType,
    #[serde(rename = "BusinessAcceptanceId", skip_serializing_if = "Option::is_none")]
    pub business_acceptance_id: Option<BusinessAcceptanceIdType>,
    #[serde(rename = "AdditionalInformation")]
    pub additional_information: AdditionalInformationType,
    #[serde(rename = "BusinessAcceptanceDate")]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTotalEstimationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaEvaluationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfLocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "GdtAlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: AlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTargetPartStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctDitchTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearAndOperationUrgencyGroup {
    #[serde(rename = "PlanningYear")]
    pub planning_year: PlanningYear,
    #[serde(rename = "OperationUrgency")]
    pub operation_urgency: OperationUrgency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassifiedType {
    #[serde(rename = "AssortmentVolumeUnclassified")]
    pub assortment_volume_unclassified: Vec<AssortmentVolumeUnclassifiedType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAcceptanceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString1000Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesGroup {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestActAreaType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientApplicationIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationsType {
    #[serde(rename = "DeclarationOtherOperation")]
    pub declaration_other_operation: Vec<CoDeclarationOtherOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSpareTreeCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaExceptionalPermitForHandlingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisValueType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowsType {
    #[serde(rename = "OperationRow")]
    pub operation_row: Vec<OperationRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TstTreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TreeStrata>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TssTreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TreeStandSummary>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowsType {
    #[serde(rename = "RoundWoodSalesRow")]
    pub round_wood_sales_row: Vec<RoundWoodSalesRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaCultivationMaterialType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPurchaseModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "DeadTreeType")]
    pub dead_tree_type: DeadTreeTypeType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesType {
    #[serde(rename = "Payee")]
    pub payee: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFinancingActWorkGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType4 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcTotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicDataType>,
    #[serde(rename = "ControlForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub control_forest_use_declaration: Option<ControlForestUseDeclarationType>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicDataType>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicDataType>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<DamageDataType>,
    #[serde(rename = "ControlDataSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub control_data_special_features: Option<ControlDataSpecialFeaturesType>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignDataType>,
    #[serde(rename = "ObjectOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub object_overall_evaluation_data: Option<ControlOverallEvaluationDataType>,
    #[serde(rename = "ControlEvaluations", skip_serializing_if = "Option::is_none")]
    pub control_evaluations: Option<ControlEvaluationsType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocationsType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceLocations", skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<ResourceLocationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionsType {
    #[serde(rename = "CaseAction")]
    pub case_action: Vec<CaseActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStorageDryingClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtScaleAssortmentType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeUnitType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoLeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonDataType {
    #[serde(rename = "DisQualificationReason")]
    pub dis_qualification_reason: String10Type,
    #[serde(rename = "DisQualificationReasonText")]
    pub dis_qualification_reason_text: String200Type,
    #[serde(rename = "DisQualificationPercentage")]
    pub dis_qualification_percentage: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStringType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal1FractionDigitType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType3 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifierType {
    #[serde(rename = "Type")]
    pub r#type: i32,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct toType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoPurchaseModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurementPlaceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoTotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTreeDecimalType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: String20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: PositiveIntegerType,
    #[serde(rename = "IsForestHaulageDistanceContinued")]
    pub is_forest_haulage_distance_continued: YesNoType,
    #[serde(rename = "StorageFinished")]
    pub storage_finished: YesNoType,
    #[serde(rename = "StorageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub storage_finished_date: Option<TimeStampType>,
    #[serde(rename = "WorkingSiteFinished")]
    pub working_site_finished: YesNoType,
    #[serde(rename = "WorkingSiteFinishedDate", skip_serializing_if = "Option::is_none")]
    pub working_site_finished_date: Option<TimeStampType>,
    #[serde(rename = "NotificationDate")]
    pub notification_date: TimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "AssortmentsChanges", skip_serializing_if = "Option::is_none")]
    pub assortments_changes: Option<AssortmentsChangesType>,
    #[serde(rename = "CommonMessages", skip_serializing_if = "Option::is_none")]
    pub common_messages: Option<CommonMessagesType>,
    #[serde(rename = "ScaleFactors", skip_serializing_if = "Option::is_none")]
    pub scale_factors: Option<ScaleFactorsType>,
    #[serde(rename = "ClientApplicationId", skip_serializing_if = "Option::is_none")]
    pub client_application_id: Option<ClientApplicationIdType>,
    #[serde(rename = "LoadRange", skip_serializing_if = "Option::is_none")]
    pub load_range: Option<LoadRangeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerType {
    #[serde(rename = "Owner")]
    pub owner: ContactInformationType,
    #[serde(rename = "OwnerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub owner_representative_person: Option<ContactInformationType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PEFCCertificationSystemSubTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeibullType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationCommitmentType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationUrgencyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeadTreeTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoRemovalClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditQuestionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSellerResponsible {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoProposalTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger4digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSukupuoliKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPointType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfoType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectBasicDataType {
    #[serde(rename = "ObjectReferenceType", skip_serializing_if = "Option::is_none")]
    pub object_reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<ReferenceType>,
    #[serde(rename = "ControlledOperationType", skip_serializing_if = "Option::is_none")]
    pub controlled_operation_type: Option<CostTypeNumberType>,
    #[serde(rename = "ControlledOperationDescription", skip_serializing_if = "Option::is_none")]
    pub controlled_operation_description: Option<String100Type>,
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctWorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnitType {
    #[serde(flatten)]
    pub base: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCallForOfferBusinessSenderRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDataSourceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMachineManufacturerType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCertificationSystemType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xmimebase64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItemType {
    #[serde(rename = "AverageStemVolume")]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: WtcoUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDataType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "PaymentType")]
    pub payment_type: PaymentTypeType,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "AdvanceTax")]
    pub advance_tax: AdvanceTaxType,
    #[serde(rename = "ForestFundPayment")]
    pub forest_fund_payment: ForestFundPaymentType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "DueDate")]
    pub due_date: DueDateType,
    #[serde(rename = "PaymentPermissionDate", skip_serializing_if = "Option::is_none")]
    pub payment_permission_date: Option<PaymentPermissionDateType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<PaymentDateType>,
    #[serde(rename = "Payee")]
    pub payee: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaProjectStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSiteStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreMessageReferenceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "RelatedCallForOffers", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offers: Option<RelatedCallForOffersType>,
    #[serde(rename = "AdditionalCode", skip_serializing_if = "Option::is_none")]
    pub additional_code: Option<AdditionalCodeType>,
    #[serde(rename = "CallForOfferBusinessSender")]
    pub call_for_offer_business_sender: WtcoCallForOfferBusinessSenderType,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<SellersType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "CallForOfferDate")]
    pub call_for_offer_date: CallForOfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<WtcoCallForOfferTextType>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
    #[serde(rename = "CallForOfferSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_silviculture_info: Option<CallForOfferSilvicultureInfoType>,
    #[serde(rename = "WsCallForOfferWorkingSites")]
    pub ws_call_for_offer_working_sites: CallForOfferWorkingSites,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationType {
    #[serde(rename = "OperationCode", skip_serializing_if = "Option::is_none")]
    pub operation_code: Option<ObjectProtectionOperationCodeType>,
    #[serde(rename = "OperationDescription", skip_serializing_if = "Option::is_none")]
    pub operation_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDataType {
    #[serde(rename = "LogKey")]
    pub log_key: String10Type,
    #[serde(rename = "ProductKey")]
    pub product_key: ERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: PositiveInteger4digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: PositiveInteger4digitsType,
    #[serde(rename = "CalibrationUseLog", skip_serializing_if = "Option::is_none")]
    pub calibration_use_log: Option<YesNoType>,
    #[serde(rename = "LogDiameterClass", skip_serializing_if = "Option::is_none")]
    pub log_diameter_class: Option<PositiveInteger3digitsType>,
    #[serde(rename = "LogLengthClass", skip_serializing_if = "Option::is_none")]
    pub log_length_class: Option<PositiveInteger4digitsType>,
    #[serde(rename = "LogMeasurements")]
    pub log_measurements: Vec<LogMeasurementsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonsType {
    #[serde(rename = "DisQualificationReason", skip_serializing_if = "Option::is_none")]
    pub dis_qualification_reason: Option<Vec<DisQualificationReasonDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMessageType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostilokeroTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorsType {
    #[serde(rename = "Actor")]
    pub actor: Vec<ActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsFaksinumeroTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStoreyType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostitoimipaikkaNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "MapSymbols")]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentreDecision")]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate")]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
    #[serde(rename = "FinancingActCompletionGeometries")]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLiftingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTimeType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType2 {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinklocatorType {
    #[serde(rename = "XlinklocatorModel")]
    pub xlinklocator_model: locatorModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtServiceNameofAPIType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignDataType {
    #[serde(rename = "RootDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub root_damage_percentage: Option<PositiveIntegerType>,
    #[serde(rename = "StemDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub stem_damage_percentage: Option<PositiveIntegerType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathMeanDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_mean_distance: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathMeanWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_mean_width: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathSubsidenceLength", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_length: Option<Decimal3_1Type>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<Decimal3_1Type>,
    #[serde(rename = "CuttingBy", skip_serializing_if = "Option::is_none")]
    pub cutting_by: Option<VirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<VirtaHarvestingSeasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<MeanAgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<CoDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoMeanHeightType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "TsTreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<TreeStandDataDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraMainGroupType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaGroundManipulationMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionType {
    #[serde(rename = "Tree")]
    pub tree: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationDataType {
    #[serde(rename = "CultivatedCropStemCount")]
    pub cultivated_crop_stem_count: StemCountType,
    #[serde(rename = "NaturalCropStemCount")]
    pub natural_crop_stem_count: StemCountType,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<StemCountType>,
    #[serde(rename = "CultivatedDeadStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_dead_stem_count: Option<StemCountType>,
    #[serde(rename = "StockingWIthSeedlings")]
    pub stocking_w_ith_seedlings: i32,
    #[serde(rename = "GroundManipulationMethod")]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<ThreeDigitPositiveIntegerType>,
    #[serde(rename = "CultivationMaterial")]
    pub cultivation_material: TwoDigitPositiveIntegerType,
    #[serde(rename = "PlantingWorkQuality")]
    pub planting_work_quality: i32,
    #[serde(rename = "SoilModificationEstimate")]
    pub soil_modification_estimate: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreasType {
    #[serde(rename = "WorkingArea", skip_serializing_if = "Option::is_none")]
    pub working_area: Option<Vec<WorkingAreaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStatisticsOperationType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessagesType {
    #[serde(rename = "CommonMessage", skip_serializing_if = "Option::is_none")]
    pub common_message: Option<Vec<CommonMessageDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<pointProperty>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString10Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatSurvivingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPercentWithFraction1Type {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3FractionDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccFinancingActNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKatuNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StRestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: RestrictionsMainGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActDataType {
    #[serde(rename = "WorksDueDate")]
    pub works_due_date: CoDateType,
    #[serde(rename = "CompletionDeclarationDeliveryDueDate")]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectsType {
    #[serde(rename = "GeometryObject")]
    pub geometry_object: Vec<GeometryObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringUseCaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonsType {
    #[serde(rename = "Reason")]
    pub reason: Vec<ReasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHeightClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSumTableAreaType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformationType {
    #[serde(rename = "FSFNumber", skip_serializing_if = "Option::is_none")]
    pub f_s_f_number: Option<String50Type>,
    #[serde(rename = "FSFValidity", skip_serializing_if = "Option::is_none")]
    pub f_s_f_validity: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Bulk", skip_serializing_if = "Option::is_none")]
    pub bulk: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityOfTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatusType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType1 {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@realEstateId")]
    pub real_estate_id: String,
    #[serde(rename = "@parcelId")]
    pub parcel_id: String,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataWithGeometryType,
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "OpPlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub op_planned_operation_chains: Option<PlannedOperationChains>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesSummaryType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "SpecialFeatures")]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoninessType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "Explanation")]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlPoint")]
    pub gml_point: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoProviderRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemCountOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub stem_count_other_tree_species: Option<PositiveInteger2digitsType>,
    #[serde(rename = "MeanHeightOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_height_other_tree_species: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub basal_area_other_tree_species: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanDiameterOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_other_tree_species: Option<PositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStratumNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaStandQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType1 {
    #[serde(rename = "base")]
    pub base: String,
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
pub struct BdtString100Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordTreeSpeciesType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String100Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger1digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType2 {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBetaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorType {
    #[serde(rename = "XlinklocatorModel")]
    pub xlinklocator_model: locatorModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleAssortmentType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorDataType {
    #[serde(rename = "ScaleAssortmentType")]
    pub scale_assortment_type: ScaleAssortmentType,
    #[serde(rename = "LoadCount")]
    pub load_count: PositiveIntegerType,
    #[serde(rename = "Area")]
    pub area: AreaCodeType,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "CodeName")]
    pub code_name: String50Type,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<ScaleFactorTreeSpeciesType>,
    #[serde(rename = "Month", skip_serializing_if = "Option::is_none")]
    pub month: Option<MonthType>,
    #[serde(rename = "HarvestingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub harvesting_finished_date: Option<TimeStampType>,
    #[serde(rename = "SemiDry", skip_serializing_if = "Option::is_none")]
    pub semi_dry: Option<YesNoType>,
    #[serde(rename = "SnowOrIce", skip_serializing_if = "Option::is_none")]
    pub snow_or_ice: Option<YesNoType>,
    #[serde(rename = "WeightClass", skip_serializing_if = "Option::is_none")]
    pub weight_class: Option<PositiveInteger1digitsType>,
    #[serde(rename = "Humidity", skip_serializing_if = "Option::is_none")]
    pub humidity: Option<Decimal1FractionDigitType>,
    #[serde(rename = "HumidityMeasured", skip_serializing_if = "Option::is_none")]
    pub humidity_measured: Option<YesNoType>,
    #[serde(rename = "ForestHaulageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_finished_date: Option<TimeStampType>,
    #[serde(rename = "CleanlinessClass", skip_serializing_if = "Option::is_none")]
    pub cleanliness_class: Option<CleanlinessClassType>,
    #[serde(rename = "Weight")]
    pub weight: Integer7digitsType,
    #[serde(rename = "Density")]
    pub density: Decimal3FractionDigitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "MeltedWater", skip_serializing_if = "Option::is_none")]
    pub melted_water: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6TotalDigitsType {
    #[serde(rename = "base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionRequirementsGroup {
    #[serde(rename = "DiameterMin")]
    pub diameter_min: DiameterMin,
    #[serde(rename = "DiameterMax")]
    pub diameter_max: DiameterMax,
    #[serde(rename = "LengthMin")]
    pub length_min: LengthMin,
    #[serde(rename = "LengthMax")]
    pub length_max: LengthMax,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationsType {
    #[serde(rename = "Organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<OrganizationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "SubsidyApplierReference")]
    pub subsidy_applier_reference: Vec<SubsidyApplierReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtSimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartType {
    #[serde(rename = "Status3", skip_serializing_if = "Option::is_none")]
    pub status3: Option<CoChangeStateType>,
    #[serde(rename = "PartNumber")]
    pub part_number: String,
    #[serde(rename = "PartsDetectedArea", skip_serializing_if = "Option::is_none")]
    pub parts_detected_area: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "CuttingMethod", skip_serializing_if = "Option::is_none")]
    pub cutting_method: Option<OpCuttingTypeType>,
    #[serde(rename = "RegenerationType", skip_serializing_if = "Option::is_none")]
    pub regeneration_type: Option<VirtaRegenerationType>,
    #[serde(rename = "TargetPartStatus", skip_serializing_if = "Option::is_none")]
    pub target_part_status: Option<VirtaTargetPartStatusType>,
    #[serde(rename = "OperationDate", skip_serializing_if = "Option::is_none")]
    pub operation_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "OperationYear", skip_serializing_if = "Option::is_none")]
    pub operation_year: Option<CoYearType>,
    #[serde(rename = "Classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<CoPositiveIntegerType>,
    #[serde(rename = "Review", skip_serializing_if = "Option::is_none")]
    pub review: Option<VirtaReviewType>,
    #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<VirtaReasonType>,
    #[serde(rename = "Review2", skip_serializing_if = "Option::is_none")]
    pub review2: Option<VirtaReviewType>,
    #[serde(rename = "Reason2", skip_serializing_if = "Option::is_none")]
    pub reason2: Option<VirtaReasonType>,
    #[serde(rename = "InspectionMethod", skip_serializing_if = "Option::is_none")]
    pub inspection_method: Option<VirtaInspectionMethodType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "SeedStemCount", skip_serializing_if = "Option::is_none")]
    pub seed_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StubStemCount", skip_serializing_if = "Option::is_none")]
    pub stub_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "CultivatedCropStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "NaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub natural_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "DeadStemCount", skip_serializing_if = "Option::is_none")]
    pub dead_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<decimal>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<VirtaStandQualityType>,
    #[serde(rename = "HabitatCode", skip_serializing_if = "Option::is_none")]
    pub habitat_code: Option<VirtaHabitatCodeType>,
    #[serde(rename = "HabitatType", skip_serializing_if = "Option::is_none")]
    pub habitat_type: Option<VirtaHabitatTypeType>,
    #[serde(rename = "HabitatSurviving", skip_serializing_if = "Option::is_none")]
    pub habitat_surviving: Option<VirtaHabitatSurvivingType>,
    #[serde(rename = "ExceptionalPermitForHandling", skip_serializing_if = "Option::is_none")]
    pub exceptional_permit_for_handling: Option<VirtaExceptionalPermitForHandlingType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoMainGroupType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<CoDrainageStateType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "PineDecimal", skip_serializing_if = "Option::is_none")]
    pub pine_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SpruceDecimal", skip_serializing_if = "Option::is_none")]
    pub spruce_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "BirchDecimal", skip_serializing_if = "Option::is_none")]
    pub birch_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "StockingWithSeedlings", skip_serializing_if = "Option::is_none")]
    pub stocking_with_seedlings: Option<CoVirtaYesNoType>,
    #[serde(rename = "GroundManipulationMethod", skip_serializing_if = "Option::is_none")]
    pub ground_manipulation_method: Option<VirtaGroundManipulationMethodType>,
    #[serde(rename = "SoilImprovementEvaluation", skip_serializing_if = "Option::is_none")]
    pub soil_improvement_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "ClearingEstimation", skip_serializing_if = "Option::is_none")]
    pub clearing_estimation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "DamageSource", skip_serializing_if = "Option::is_none")]
    pub damage_source: Option<CoFeatureTypeType>,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<OpSilvicultureTypeType>,
    #[serde(rename = "CultivationMaterial", skip_serializing_if = "Option::is_none")]
    pub cultivation_material: Option<VirtaCultivationMaterialType>,
    #[serde(rename = "PlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub plant_evaluation: Option<VirtaPlantEvaluationType>,
    #[serde(rename = "GrassControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub grass_control_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "SproutForestControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub sprout_forest_control_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "SeedPlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub seed_plant_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "RootRotControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub root_rot_control_evaluation: Option<VirtaRootRotControlEvaluationType>,
    #[serde(rename = "HarvestingClassifiation", skip_serializing_if = "Option::is_none")]
    pub harvesting_classifiation: Option<VirtaHarvestingClassificationType>,
    #[serde(rename = "RootDamageCount", skip_serializing_if = "Option::is_none")]
    pub root_damage_count: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "StemDamageCount", skip_serializing_if = "Option::is_none")]
    pub stem_damage_count: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "TotalEstimation", skip_serializing_if = "Option::is_none")]
    pub total_estimation: Option<VirtaTotalEstimationType>,
    #[serde(rename = "CuttingByMachine", skip_serializing_if = "Option::is_none")]
    pub cutting_by_machine: Option<VirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<VirtaHarvestingSeasonType>,
    #[serde(rename = "PartEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_easting_coordinate: Option<string>,
    #[serde(rename = "PartNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_northing_coordinate: Option<string>,
    #[serde(rename = "NotDamagedCount", skip_serializing_if = "Option::is_none")]
    pub not_damaged_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class1damageCount", skip_serializing_if = "Option::is_none")]
    pub class1damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class2damageCount", skip_serializing_if = "Option::is_none")]
    pub class2damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class3damageCount", skip_serializing_if = "Option::is_none")]
    pub class3damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class4damageCount", skip_serializing_if = "Option::is_none")]
    pub class4damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "SmallPeelDamage", skip_serializing_if = "Option::is_none")]
    pub small_peel_damage: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "LargePeelDamage", skip_serializing_if = "Option::is_none")]
    pub large_peel_damage: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "DamagedSeedlingCount", skip_serializing_if = "Option::is_none")]
    pub damaged_seedling_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "PineLog", skip_serializing_if = "Option::is_none")]
    pub pine_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "SpruceLog", skip_serializing_if = "Option::is_none")]
    pub spruce_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "BirchLog", skip_serializing_if = "Option::is_none")]
    pub birch_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "PinePulp", skip_serializing_if = "Option::is_none")]
    pub pine_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "LogM3sum", skip_serializing_if = "Option::is_none")]
    pub log_m3sum: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "PulpM3sum", skip_serializing_if = "Option::is_none")]
    pub pulp_m3sum: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "SprucePulp", skip_serializing_if = "Option::is_none")]
    pub spruce_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "BirchPulp", skip_serializing_if = "Option::is_none")]
    pub birch_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "OtherTimberValue", skip_serializing_if = "Option::is_none")]
    pub other_timber_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "EnergyTimberValue", skip_serializing_if = "Option::is_none")]
    pub energy_timber_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "CuttingValue", skip_serializing_if = "Option::is_none")]
    pub cutting_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "DestroyedCuttingValue", skip_serializing_if = "Option::is_none")]
    pub destroyed_cutting_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "ExpectedValueCoefficient", skip_serializing_if = "Option::is_none")]
    pub expected_value_coefficient: Option<CoPositiveDecimalMax1IntegralPartMax2FractionalPartType>,
    #[serde(rename = "ExpectedValueSurplus", skip_serializing_if = "Option::is_none")]
    pub expected_value_surplus: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Compensation", skip_serializing_if = "Option::is_none")]
    pub compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AlreadyPaidCompensation", skip_serializing_if = "Option::is_none")]
    pub already_paid_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "NeedForTreatment", skip_serializing_if = "Option::is_none")]
    pub need_for_treatment: Option<CoVirtaYesNoType>,
    #[serde(rename = "Suggestion", skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<VirtaSuggestionType>,
    #[serde(rename = "Phase2youngCropCount", skip_serializing_if = "Option::is_none")]
    pub phase2young_crop_count: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RecommendedDensity", skip_serializing_if = "Option::is_none")]
    pub recommended_density: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RepairPlantingCosts", skip_serializing_if = "Option::is_none")]
    pub repair_planting_costs: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
    #[serde(rename = "GeometryStatus", skip_serializing_if = "Option::is_none")]
    pub geometry_status: Option<string>,
    #[serde(rename = "GeometryId", skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<string>,
    #[serde(rename = "GmlPolygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<Polygon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: StemTypeType,
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoExtendedQuantityUnitType,
    #[serde(rename = "AssortmentInfo")]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: String20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger4digitsType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FacFinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_stands: Option<FinancingActCompletionStands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierBaseType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString5Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCategoryType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simple {
    #[serde(rename = "XlinksimpleModel")]
    pub xlinksimple_model: simpleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetailsType {
    #[serde(rename = "SilvicultureRestrictionGroup")]
    pub silviculture_restriction_group: SilvicultureRestrictionGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalTextType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNimiTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurposeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaDecrease>,
    #[serde(rename = "GdtPolygonGeometry")]
    pub gdt_polygon_geometry: PolygonGeometry,
    #[serde(rename = "GdtMultiPolygonGeometry")]
    pub gdt_multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsBICKoodiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType {
    #[serde(rename = "CostTypeNumber")]
    pub cost_type_number: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvmTyyppi {
    #[serde(rename = "base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionsType {
    #[serde(rename = "UsingRestriction")]
    pub using_restriction: Vec<UsingRestrictionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkinTyyppi {
    #[serde(rename = "base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeCuttingType {
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String10Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationType {
    #[serde(rename = "@assessorActorId")]
    pub assessor_actor_id: IdStringType,
    #[serde(rename = "@assessObjectActorId")]
    pub assess_object_actor_id: IdStringType,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<EvaluationCodeType>,
    #[serde(rename = "EvaluationText", skip_serializing_if = "Option::is_none")]
    pub evaluation_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<ERPIdType>,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Name")]
    pub name: String50Type,
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<StemTypeType>,
    #[serde(rename = "Quality")]
    pub quality: String5Type,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<WorkCodeUnitType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "MeasurementMethod")]
    pub measurement_method: MeasurementMethodType,
    #[serde(rename = "MeasurementPlace")]
    pub measurement_place: MeasurementPlaceType,
    #[serde(rename = "DiameterMin", skip_serializing_if = "Option::is_none")]
    pub diameter_min: Option<PositiveIntegerType>,
    #[serde(rename = "DiameterMax", skip_serializing_if = "Option::is_none")]
    pub diameter_max: Option<PositiveIntegerType>,
    #[serde(rename = "HeightMin", skip_serializing_if = "Option::is_none")]
    pub height_min: Option<Decimal1FractionDigitType>,
    #[serde(rename = "HeightMax", skip_serializing_if = "Option::is_none")]
    pub height_max: Option<Decimal1FractionDigitType>,
    #[serde(rename = "CanModify", skip_serializing_if = "Option::is_none")]
    pub can_modify: Option<YesNoType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<String200Type>,
    #[serde(rename = "DeliveryUserId", skip_serializing_if = "Option::is_none")]
    pub delivery_user_id: Option<String50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActorsType {
    #[serde(rename = "CompletionDeclarationActor")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceType {
    #[serde(rename = "XlinkresourceModel")]
    pub xlinkresource_model: resourceModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateRegisterUnitGroup {
    #[serde(rename = "MunicipalityName", skip_serializing_if = "Option::is_none")]
    pub municipality_name: Option<MunicipalityName>,
    #[serde(rename = "GroupNumber")]
    pub group_number: GroupNumber,
    #[serde(rename = "UnitNumber")]
    pub unit_number: UnitNumber,
    #[serde(rename = "AreaNumber")]
    pub area_number: AreaNumber,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumber,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedAmountUnitType {
    #[serde(flatten)]
    pub base: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString50Type {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSaveIncompleteType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TstTreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TreeStrata>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TssTreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TreeStandSummary>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicDataType {
    #[serde(rename = "ProjectNo", skip_serializing_if = "Option::is_none")]
    pub project_no: Option<ProjectNoType>,
    #[serde(rename = "SelfMonitoringType", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_type: Option<SelfMonitoringTypeType>,
    #[serde(rename = "SelfMonitoringDate", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_date: Option<DateType>,
    #[serde(rename = "FccForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_use_declaration_number: Option<ForestUseDeclarationNumber>,
    #[serde(rename = "FccFinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_number: Option<FinancingActNumber>,
    #[serde(rename = "FccCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<CustomerReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClassType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameBaseType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertisementDatingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExceptionalPermitForHandlingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperType {
    #[serde(rename = "CaliperId", skip_serializing_if = "Option::is_none")]
    pub caliper_id: Option<String200Type>,
    #[serde(rename = "CaliperApplication", skip_serializing_if = "Option::is_none")]
    pub caliper_application: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementsType {
    #[serde(rename = "LogDiameter")]
    pub log_diameter: PositiveInteger3digitsType,
    #[serde(rename = "ControlLogDiameter")]
    pub control_log_diameter: PositiveInteger3digitsType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometriesType {
    #[serde(rename = "DecisionGeometry")]
    pub decision_geometry: Vec<DecisionGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaWorkQualityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyModeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeBetweenProposalYearsGroup {
    #[serde(rename = "MinProposalYear")]
    pub min_proposal_year: MinProposalYear,
    #[serde(rename = "MaxProposalYear")]
    pub max_proposal_year: MaxProposalYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointType {
    #[serde(rename = "Diameter")]
    pub diameter: DiameterType,
    #[serde(rename = "CumulativeMass")]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "StartTime")]
    pub start_time: TimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratumType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "SeedlingBeginningCode", skip_serializing_if = "Option::is_none")]
    pub seedling_beginning_code: Option<CoSeedlingOriginType>,
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountInSpot", skip_serializing_if = "Option::is_none")]
    pub amount_in_spot: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountOutsideSpot", skip_serializing_if = "Option::is_none")]
    pub amount_outside_spot: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountUnit", skip_serializing_if = "Option::is_none")]
    pub amount_unit: Option<CoAmountUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SfLocatedSpecialFeature1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributesType {
    #[serde(rename = "TreeSpeciesAttribute", skip_serializing_if = "Option::is_none")]
    pub tree_species_attribute: Option<Vec<TreeSpeciesAttributeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "Payees")]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfoType {
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<WtcoIncludePaymentPlanType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessagesType {
    #[serde(rename = "ErrorMessageData")]
    pub error_message_data: Vec<ErrorMessageDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatAdvertisementType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: StbAreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoHeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<CoDiameterClassType>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication")]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<CoString2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationGroup {
    #[serde(rename = "WoodLotInformationType")]
    pub wood_lot_information_type: WoodLotInformationType,
    #[serde(rename = "WoodLotInformationValue")]
    pub wood_lot_information_value: WoodLotInformationValue,
    #[serde(rename = "WoodLotInformationTypeDescription", skip_serializing_if = "Option::is_none")]
    pub wood_lot_information_type_description: Option<WoodLotInformationTypeDescription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@versionNo")]
    pub version_no: VersionNoType,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: MeasurementDateType,
    #[serde(rename = "InsertDate")]
    pub insert_date: InsertDateType,
    #[serde(rename = "MeasurementCertificateType")]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<TextType>,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "PaidValue", skip_serializing_if = "Option::is_none")]
    pub paid_value: Option<PaidValueType>,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
    #[serde(rename = "PaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub payment_transactions: Option<WtcoPaymentTransactionsType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageType {
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "StorageDisplayId", skip_serializing_if = "Option::is_none")]
    pub storage_display_id: Option<String20Type>,
    #[serde(rename = "Geometry")]
    pub geometry: PointAndLineOrPolygonType,
    #[serde(rename = "GeometryModificationAllowed")]
    pub geometry_modification_allowed: YesNoType,
    #[serde(rename = "PlowingName", skip_serializing_if = "Option::is_none")]
    pub plowing_name: Option<String50Type>,
    #[serde(rename = "PlowingTelephone", skip_serializing_if = "Option::is_none")]
    pub plowing_telephone: Option<String20Type>,
    #[serde(rename = "PlowingEmail", skip_serializing_if = "Option::is_none")]
    pub plowing_email: Option<String50Type>,
    #[serde(rename = "PlowingArranged", skip_serializing_if = "Option::is_none")]
    pub plowing_arranged: Option<YesNoType>,
    #[serde(rename = "PlowingDate", skip_serializing_if = "Option::is_none")]
    pub plowing_date: Option<DateType>,
    #[serde(rename = "Accessibility")]
    pub accessibility: TransportAccessibilityType,
    #[serde(rename = "TurningPointClass", skip_serializing_if = "Option::is_none")]
    pub turning_point_class: Option<TurningPointClassType>,
    #[serde(rename = "StorageInfo", skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<String500Type>,
    #[serde(rename = "DeliveryRestriction", skip_serializing_if = "Option::is_none")]
    pub delivery_restriction: Option<YesNoType>,
    #[serde(rename = "StorageName", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String50Type>,
    #[serde(rename = "StorageAddress", skip_serializing_if = "Option::is_none")]
    pub storage_address: Option<String500Type>,
    #[serde(rename = "StorageDryingClass", skip_serializing_if = "Option::is_none")]
    pub storage_drying_class: Option<StorageDryingClassType>,
    #[serde(rename = "StorageLandOwner", skip_serializing_if = "Option::is_none")]
    pub storage_land_owner: Option<StorageLandOwnerType>,
    #[serde(rename = "StorageLandOwnerInformation", skip_serializing_if = "Option::is_none")]
    pub storage_land_owner_information: Option<ContactInformationType>,
    #[serde(rename = "StorageAdditionalRemarks", skip_serializing_if = "Option::is_none")]
    pub storage_additional_remarks: Option<String3000Type>,
    #[serde(rename = "StorageLinkedToWorkingSite", skip_serializing_if = "Option::is_none")]
    pub storage_linked_to_working_site: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaReasonType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLajiTekstiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFeatureTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: AssortmentIdentifierGroup,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: QuantityGroup,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<PricesAndCurrencyGroup>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeNameType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "GdtAlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: AlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationType {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: TimeStampType,
    #[serde(rename = "CalibrationAdjustment")]
    pub calibration_adjustment: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatSurvivingType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionType {
    #[serde(rename = "SceneryWorkPermissionNeeded")]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
    #[serde(rename = "SceneryWorkPermissionAcceptance", skip_serializing_if = "Option::is_none")]
    pub scenery_work_permission_acceptance: Option<CoDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType2 {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber")]
    pub location_municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<CoMunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimiTyyppi {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax5IntegralPartMax2FractionalPartType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummariesType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectKeys", skip_serializing_if = "Option::is_none")]
    pub object_keys: Option<ObjectKeysType>,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSitesType {
    #[serde(rename = "ContractWorkingSiteDetails")]
    pub contract_working_site_details: Vec<ContractWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulkType {
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "Bulk")]
    pub bulk: PositiveIntegerType,
}

