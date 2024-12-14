#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: JhsAlkuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedWorkingTimeConsumption {
    #[serde(flatten)]
    pub estimated_working_time_consumption: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary {
    #[serde(flatten)]
    pub tree_stand_summary: TssTreeStandSummary2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: CoMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystem {
    #[serde(flatten)]
    pub wide_certification_system: WideCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(flatten)]
    pub work: WorkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethod {
    #[serde(flatten)]
    pub pricing_method: CoUsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActors {
    #[serde(flatten)]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PilotName {
    #[serde(flatten)]
    pub pilot_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomass {
    #[serde(flatten)]
    pub branch_biomass: BranchBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Height {
    #[serde(flatten)]
    pub height: CoMeanHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesFromMapSymbols {
    #[serde(flatten)]
    pub spare_trees_from_map_symbols: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WtcoWorkingSiteGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceProductName {
    #[serde(flatten)]
    pub prevention_substance_product_name: CoString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuttings {
    #[serde(flatten)]
    pub cuttings: CuttingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainReason {
    #[serde(flatten)]
    pub main_reason: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpecies {
    #[serde(flatten)]
    pub other_tree_species: OtherTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: Decimal3_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjects {
    #[serde(flatten)]
    pub financing_act_application_other_subjects: FinancingActApplicationOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotHeight {
    #[serde(flatten)]
    pub nearest_cultivated_spot_height: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathMeanDistance {
    #[serde(flatten)]
    pub vehicle_path_mean_distance: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCode {
    #[serde(flatten)]
    pub other_habitat_code: CoOtherHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationType {
    #[serde(flatten)]
    pub fertilization_type: CoString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: JhsToinenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    #[serde(flatten)]
    pub cutting_stem_count_type: CuttingStemCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomass {
    #[serde(flatten)]
    pub leaf_biomass: LeafBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: TwoDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifier {
    #[serde(flatten)]
    pub stout_timber_classifier: StoutTimberClassifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameter {
    #[serde(flatten)]
    pub reduction_avg_diameter: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectData {
    #[serde(flatten)]
    pub forest_object_data: ForestObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedPlantsErrorCount {
    #[serde(flatten)]
    pub damaged_plants_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescription {
    #[serde(flatten)]
    pub payment_transaction_description: PaymentTransactionDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photographer {
    #[serde(flatten)]
    pub photographer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstates {
    #[serde(flatten)]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamagePercentage {
    #[serde(flatten)]
    pub root_damage_percentage: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(flatten)]
    pub payment_transaction_type: CoMoneyTransactionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: CoString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalSawlogVolume {
    #[serde(flatten)]
    pub total_sawlog_volume: CoSawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: WtcoWorkingSiteNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlNo {
    #[serde(flatten)]
    pub control_no: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareOfOwnerShip {
    #[serde(flatten)]
    pub share_of_owner_ship: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometry {
    #[serde(flatten)]
    pub line_geometry: LineGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocationText {
    #[serde(flatten)]
    pub excavation_soil_location_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSender {
    #[serde(flatten)]
    pub call_for_offer_business_sender: WtcoCallForOfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestType {
    #[serde(flatten)]
    pub forest_type: BdtFertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStands {
    #[serde(flatten)]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocation {
    #[serde(flatten)]
    pub excavation_soil_location: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNo {
    #[serde(flatten)]
    pub area_no: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: BdtServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: CddNormalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercent {
    #[serde(flatten)]
    pub saw_log_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMoundsCount {
    #[serde(flatten)]
    pub ditch_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataRegeneration {
    #[serde(flatten)]
    pub control_data_regeneration: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstruction {
    #[serde(flatten)]
    pub control_data_forest_road_construction: ControlDataForestRoadConstructionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub contact_person: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: CoHabitatTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffectText {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceOrOtherCompensation {
    #[serde(flatten)]
    pub insurance_or_other_compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometry {
    #[serde(flatten)]
    pub object_geometry: ObjectGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratum {
    #[serde(flatten)]
    pub dead_tree_stratum: DeadTreeStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: BdtHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulgeHeight {
    #[serde(flatten)]
    pub bulge_height: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(flatten)]
    pub count: CountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cutting {
    #[serde(flatten)]
    pub cutting: CuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningQualityText {
    #[serde(flatten)]
    pub soil_conditioning_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpMeanDiameter {
    #[serde(flatten)]
    pub stump_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub data_source: DataSourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierId {
    #[serde(flatten)]
    pub subsidy_applier_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSilverBirch {
    #[serde(flatten)]
    pub mean_diameter_silver_birch: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathMeanWidth {
    #[serde(flatten)]
    pub vehicle_path_mean_width: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowth {
    #[serde(flatten)]
    pub volume_growth: VolumeGrowthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3DamageCount {
    #[serde(flatten)]
    pub class3_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClasses {
    #[serde(flatten)]
    pub assortment_classes: AssortmentClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusData {
    #[serde(flatten)]
    pub prevention_fungus_of_the_genus_data: PreventionFungusOfTheGenusDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCategory {
    #[serde(flatten)]
    pub evaluation_category: CoEvaluationSubjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethodsText {
    #[serde(flatten)]
    pub other_conservation_methods_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainDamage {
    #[serde(flatten)]
    pub main_damage: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActor {
    #[serde(flatten)]
    pub application_actor: ApplicationActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Works {
    #[serde(flatten)]
    pub works: WorksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: JhsViidesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertileType {
    #[serde(flatten)]
    pub fertile_type: BdtMaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4DamageCount {
    #[serde(flatten)]
    pub class4_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelated {
    #[serde(flatten)]
    pub cutting_related: CuttingRelatedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: JhsVakinainenKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedEndDate {
    #[serde(flatten)]
    pub estimated_end_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolName {
    #[serde(flatten)]
    pub map_symbol_name: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeId {
    #[serde(flatten)]
    pub payee_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2DamageCount {
    #[serde(flatten)]
    pub class2_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: CoUsingRightResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingDistance {
    #[serde(flatten)]
    pub nearest_seedling_distance: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescription {
    #[serde(flatten)]
    pub document_description: DocumentDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate {
    #[serde(flatten)]
    pub tree_stand_data_date: TsTreeStandDataDate2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: BdtFeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithTrailer {
    #[serde(flatten)]
    pub kilometers_with_trailer: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: CddVarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicData {
    #[serde(flatten)]
    pub object_basic_data: ObjectBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: ThreeDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Length {
    #[serde(flatten)]
    pub length: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethod {
    #[serde(flatten)]
    pub soil_conditioning_method: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReferenceType {
    #[serde(flatten)]
    pub object_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeName {
    #[serde(flatten)]
    pub whole_name: WholeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(flatten)]
    pub depth: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowedText {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: JhsAmmattiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsible {
    #[serde(flatten)]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingActionText {
    #[serde(flatten)]
    pub erosion_blocking_action_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionData {
    #[serde(flatten)]
    pub restriction_data: StRestrictionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: DiameterMaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    #[serde(flatten)]
    pub tree: TreeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: JhsPostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: JhsSiviilisaatyTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceIdMJ {
    #[serde(flatten)]
    pub resource_id_m_j: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystem {
    #[serde(flatten)]
    pub certification_system: WtcoCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCount {
    #[serde(flatten)]
    pub reduction_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson4 {
    #[serde(flatten)]
    pub extra_person4: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: JhsCareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperation {
    #[serde(flatten)]
    pub declaration_soil_preparation_operation: CoDeclarationSoilPreparationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalVolume {
    #[serde(flatten)]
    pub small_wood_removal_volume: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluation {
    #[serde(flatten)]
    pub self_monitoring_evaluation: SelfMonitoringEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumber {
    #[serde(flatten)]
    pub forest_use_declaration_number: FccForestUseDeclarationNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingText {
    #[serde(flatten)]
    pub pre_clearing_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherConservationMethods {
    #[serde(flatten)]
    pub other_conservation_methods: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumps {
    #[serde(flatten)]
    pub high_stumps: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantedSeedlingCountInNotCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_not_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingSiteCount {
    #[serde(flatten)]
    pub planting_site_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceType {
    #[serde(flatten)]
    pub control_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeCount {
    #[serde(flatten)]
    pub tree_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstates {
    #[serde(flatten)]
    pub declaration_real_estates: DeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    #[serde(flatten)]
    pub height_class_type: HeightClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantPositionErrorCount {
    #[serde(flatten)]
    pub plant_position_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineDescription {
    #[serde(flatten)]
    pub machine_description: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScapingText {
    #[serde(flatten)]
    pub land_scaping_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedSeedlingCount {
    #[serde(flatten)]
    pub not_damaged_seedling_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: CddShapeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurpose {
    #[serde(flatten)]
    pub cutting_purpose: CoCuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperations {
    #[serde(flatten)]
    pub object_protection_operations: ObjectProtectionOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwner {
    #[serde(flatten)]
    pub real_estate_owner: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCode {
    #[serde(flatten)]
    pub financing_act_work_code: CoFinancingActWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: JhsPuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: CddJohnsonSBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandCorrectHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_correct_height_stumps_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStands {
    #[serde(flatten)]
    pub financing_act_completion_stands: FinancingActCompletionStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: CoOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocationText {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: JhsTurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    #[serde(flatten)]
    pub seller: SellerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactions {
    #[serde(flatten)]
    pub payment_transactions: PaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
    #[serde(flatten)]
    pub removal_class_type: RemovalClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: SfBasicFeature2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverallTotalSubsidy {
    #[serde(flatten)]
    pub overall_total_subsidy: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: WtcoRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeDescription {
    #[serde(flatten)]
    pub wood_lot_information_type_description: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDoneText {
    #[serde(flatten)]
    pub clearing_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibility {
    #[serde(flatten)]
    pub harvesting_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RightToSpecifyBankAccountsOfPaymentTransactions {
    #[serde(flatten)]
    pub right_to_specify_bank_accounts_of_payment_transactions: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: DrainageStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandReference {
    #[serde(flatten)]
    pub declaration_stand_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedStartDate {
    #[serde(flatten)]
    pub estimated_start_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperNumber {
    #[serde(flatten)]
    pub hopper_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDocument {
    #[serde(flatten)]
    pub power_of_attorney_document: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectBasicData {
    #[serde(flatten)]
    pub control_object_basic_data: ControlObjectBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_height_other_tree_species: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeft {
    #[serde(flatten)]
    pub save_trees_left: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBAN {
    #[serde(flatten)]
    pub iban: IBANType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareGroupOfTrees {
    #[serde(flatten)]
    pub spare_group_of_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrix {
    #[serde(flatten)]
    pub price_matrix: PriceMatrixType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLots {
    #[serde(flatten)]
    pub wood_lots: WoodLotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: ReRegisterUnitIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: AreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandTextInformation {
    #[serde(flatten)]
    pub declaration_stand_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreinformDetails {
    #[serde(flatten)]
    pub preinform_details: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionStumpHeight {
    #[serde(flatten)]
    pub reduction_stump_height: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationInvolvement {
    #[serde(flatten)]
    pub association_involvement: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: CddShapeDeltaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: JhsSukupuoliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClass {
    #[serde(flatten)]
    pub document_class: DocumentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstate {
    #[serde(flatten)]
    pub payee_and_real_estate: PayeeAndRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    #[serde(flatten)]
    pub r#type: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(flatten)]
    pub language: CoISO639char2LanguageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracySign {
    #[serde(flatten)]
    pub cutting_accuracy_sign: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDate {
    #[serde(flatten)]
    pub image_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplication {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionality {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClass {
    #[serde(flatten)]
    pub height_class: CoHeightClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureMethodCorrectText {
    #[serde(flatten)]
    pub silviculture_method_correct_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActor {
    #[serde(flatten)]
    pub completion_declaration_actor: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(flatten)]
    pub position: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainage {
    #[serde(flatten)]
    pub road_structure_drainage: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelection {
    #[serde(flatten)]
    pub target_selection: CoTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReference {
    #[serde(flatten)]
    pub object_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPayment {
    #[serde(flatten)]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCode {
    #[serde(flatten)]
    pub specification_code: SpecificationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgVehiclePathDistance {
    #[serde(flatten)]
    pub stand_avg_vehicle_path_distance: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: JhsFaksinumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TsTreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocation {
    #[serde(flatten)]
    pub habitat_location: CoHabitatLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Width {
    #[serde(flatten)]
    pub width: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMax {
    #[serde(flatten)]
    pub length_max: LengthMaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSourceCode {
    #[serde(flatten)]
    pub damage_source_code: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: JhsPostilokeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYear {
    #[serde(flatten)]
    pub min_proposal_year: MinProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: JhsHuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelEndTime {
    #[serde(flatten)]
    pub travel_end_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: CoSilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActors {
    #[serde(flatten)]
    pub application_actors: ApplicationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRiskDescription {
    #[serde(flatten)]
    pub work_safety_risk_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: AttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDate {
    #[serde(flatten)]
    pub offer_expiration_date: OfferExpirationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeatures {
    #[serde(flatten)]
    pub control_data_special_features: ControlDataSpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlledOperationType {
    #[serde(flatten)]
    pub controlled_operation_type: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    #[serde(flatten)]
    pub small_wood_removal_class_type: SmallWoodRemovalClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: FirstNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: JhsUlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountOtherTreeSpecies {
    #[serde(flatten)]
    pub stem_count_other_tree_species: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDominantHeight {
    #[serde(flatten)]
    pub stand_avg_dominant_height: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingDepthErrorCount {
    #[serde(flatten)]
    pub planting_depth_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CoCurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: JhsKutsumaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignControlClassifier {
    #[serde(flatten)]
    pub harvesting_sign_control_classifier: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentative {
    #[serde(flatten)]
    pub working_representative: WorkingRepresentativeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedStemCount {
    #[serde(flatten)]
    pub planned_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasons {
    #[serde(flatten)]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hoppers {
    #[serde(flatten)]
    pub hoppers: HoppersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: CoIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biomass {
    #[serde(flatten)]
    pub biomass: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSpruce {
    #[serde(flatten)]
    pub mean_height_spruce: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoosePercentage {
    #[serde(flatten)]
    pub moose_percentage: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationText {
    #[serde(flatten)]
    pub evaluation_text: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDate {
    #[serde(flatten)]
    pub working_site_plan_date: WorkingSitePlanDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: CoBankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: JhsPaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopOperationProject {
    #[serde(flatten)]
    pub cop_operation_project: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountTarget {
    #[serde(flatten)]
    pub stem_count_target: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    #[serde(flatten)]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreasText {
    #[serde(flatten)]
    pub surface_draining_areas_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: CoTreeClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearing {
    #[serde(flatten)]
    pub pre_clearing: BdtYesNoNotNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringStandArea {
    #[serde(flatten)]
    pub self_monitoring_stand_area: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: CddDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: JhsKieliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummary {
    #[serde(flatten)]
    pub tree_species_summary: TreeSpeciesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDiameter {
    #[serde(flatten)]
    pub tree_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovementsText {
    #[serde(flatten)]
    pub notifications_and_improvements_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjects {
    #[serde(flatten)]
    pub child_objects: ChildObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: WtcoSellerRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationDescription {
    #[serde(flatten)]
    pub evaluation_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationType {
    #[serde(flatten)]
    pub wood_lot_information_type: WoodLotInformationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: JhsValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationPercentage {
    #[serde(flatten)]
    pub participation_percentage: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: BdtSpareTreeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson3 {
    #[serde(flatten)]
    pub extra_person3: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTime {
    #[serde(flatten)]
    pub change_time: ChangeTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubDiameter {
    #[serde(flatten)]
    pub stub_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPersonText {
    #[serde(flatten)]
    pub extra_person_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightHardWood {
    #[serde(flatten)]
    pub mean_height_hard_wood: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: CoMaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: AreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSpruce {
    #[serde(flatten)]
    pub basal_area_spruce: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actors {
    #[serde(flatten)]
    pub actors: ActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectProtectionOperationsData {
    #[serde(flatten)]
    pub self_monitoring_object_protection_operations_data: SelfMonitoringObjectProtectionOperationsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: BdtFeatureAdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDate {
    #[serde(flatten)]
    pub control_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrata {
    #[serde(flatten)]
    pub dead_tree_strata: DeadTreeStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: DeclarationReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterId {
    #[serde(flatten)]
    pub harvester_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformation {
    #[serde(flatten)]
    pub control_additional_information: ControlAdditionalInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChains {
    #[serde(flatten)]
    pub planned_operation_chains: PlannedOperationChainsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeed {
    #[serde(flatten)]
    pub cutting_area_preclearing_need: CuttingAreaPreclearingNeedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassCollectionDone {
    #[serde(flatten)]
    pub bio_mass_collection_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumber {
    #[serde(flatten)]
    pub cost_type_number: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureMethodCorrect {
    #[serde(flatten)]
    pub silviculture_method_correct: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocation {
    #[serde(flatten)]
    pub ditch_cleaning_break_location: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: JhsValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaHardWood {
    #[serde(flatten)]
    pub basal_area_hard_wood: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManipulationMethod {
    #[serde(flatten)]
    pub manipulation_method: BdtWorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeIntervalForMeasuringSamplePlot {
    #[serde(flatten)]
    pub time_interval_for_measuring_sample_plot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: UnitNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletion {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNo {
    #[serde(flatten)]
    pub project_no: ProjectNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: WtcoSellersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerId {
    #[serde(flatten)]
    pub measurer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: SfValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: JhsKuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherEvaluation {
    #[serde(flatten)]
    pub other_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CddCumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiability {
    #[serde(flatten)]
    pub cutting_planner_liability: CuttingPlannerLiabilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Woodlot {
    #[serde(flatten)]
    pub woodlot: WoodLotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDone {
    #[serde(flatten)]
    pub cleaning_breaks_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVolumeSummary {
    #[serde(flatten)]
    pub stand_volume_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    #[serde(flatten)]
    pub date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlan {
    #[serde(flatten)]
    pub include_payment_plan: IncludePaymentPlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRightsOwner {
    #[serde(flatten)]
    pub cutting_rights_owner: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: PostalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountInSpot {
    #[serde(flatten)]
    pub amount_in_spot: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectTypeSpecifier {
    #[serde(flatten)]
    pub child_object_type_specifier: ObjectTypeSpecifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantCountSummary {
    #[serde(flatten)]
    pub plant_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeTreeReduction {
    #[serde(flatten)]
    pub sample_plot_size_tree_reduction: SamplePlotSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppliedLength {
    #[serde(flatten)]
    pub applied_length: Decimal6_2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSites {
    #[serde(flatten)]
    pub call_for_offer_working_sites: CallForOfferWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetails {
    #[serde(flatten)]
    pub offer_working_site_details: OfferWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: JhsMaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaReference {
    #[serde(flatten)]
    pub processing_area_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeparateSpareTrees {
    #[serde(flatten)]
    pub separate_spare_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: JhsAlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainApplier {
    #[serde(flatten)]
    pub main_applier: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMainGroup {
    #[serde(flatten)]
    pub assortment_main_group: CoAssortmentMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: WtcoUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolume {
    #[serde(flatten)]
    pub cutting_volume: CuttingVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatment {
    #[serde(flatten)]
    pub stump_treatment: BdtYesNoNotNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumber {
    #[serde(flatten)]
    pub object_number: ObjectNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamagePercentage {
    #[serde(flatten)]
    pub stem_damage_percentage: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(flatten)]
    pub vehicle: BdtVehicleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeData {
    #[serde(flatten)]
    pub operative_data: OperativeDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: JhsEdellinenSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Silviculture {
    #[serde(flatten)]
    pub silviculture: SilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlace {
    #[serde(flatten)]
    pub turning_place: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: TwoDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStands {
    #[serde(flatten)]
    pub financing_act_application_stands: FinancingActApplicationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnoverMoundsCount {
    #[serde(flatten)]
    pub turnover_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShape {
    #[serde(flatten)]
    pub road_structure_shape: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationReference {
    #[serde(flatten)]
    pub completion_declaration_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(flatten)]
    pub actor: ActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: BdtDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeys {
    #[serde(flatten)]
    pub object_keys: ObjectKeysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChain {
    #[serde(flatten)]
    pub planned_operation_chain: PlannedOperationChainType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObject {
    #[serde(flatten)]
    pub child_object: ChildObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReference {
    #[serde(flatten)]
    pub control_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamages {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCount {
    #[serde(flatten)]
    pub cutting_stem_count: CuttingStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    #[serde(flatten)]
    pub district: BdtThinningDistrictType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessiveCount {
    #[serde(flatten)]
    pub thinning_too_excessive_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandExtraInfo {
    #[serde(flatten)]
    pub stand_extra_info: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalTreeSpecies {
    #[serde(flatten)]
    pub goal_tree_species: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompact {
    #[serde(flatten)]
    pub assortment_compact: AssortmentCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantLocationErrorCount {
    #[serde(flatten)]
    pub plant_location_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InTerrain {
    #[serde(flatten)]
    pub in_terrain: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousMessage {
    #[serde(flatten)]
    pub update_previous_message: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: CoMachineAccessoryCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: RealEstateNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumber {
    #[serde(flatten)]
    pub stratum_number: CoStratumNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: DevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: WtcoCallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: JhsLajiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: CoDiameterClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilData {
    #[serde(flatten)]
    pub soil_data: StBaseSoilDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectData {
    #[serde(flatten)]
    pub common_object_data: CommonObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: JhsKatuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileName {
    #[serde(flatten)]
    pub document_file_name: DocumentFileNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaks {
    #[serde(flatten)]
    pub ditching_breaks: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TstTreeStrata2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedWorkAmount {
    #[serde(flatten)]
    pub planned_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaPine {
    #[serde(flatten)]
    pub basal_area_pine: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanStumpDiameter {
    #[serde(flatten)]
    pub mean_stump_diameter: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCase {
    #[serde(flatten)]
    pub use_case: CoForestDataUpdateUseCaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: CddScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(flatten)]
    pub child_object_type: CoObjectTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDoneText {
    #[serde(flatten)]
    pub cleaning_breaks_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingWorkQuality {
    #[serde(flatten)]
    pub planting_work_quality: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCode {
    #[serde(flatten)]
    pub country_code: CoISO3166char2CountryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaAndMapEvaluation {
    #[serde(flatten)]
    pub area_and_map_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperation {
    #[serde(flatten)]
    pub habitat_operation: CoHabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMeanDepth {
    #[serde(flatten)]
    pub ditch_mean_depth: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanName {
    #[serde(flatten)]
    pub ditch_or_road_plan_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: CoSawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStand {
    #[serde(flatten)]
    pub declaration_stand: DeclarationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsAndImprovements {
    #[serde(flatten)]
    pub notifications_and_improvements: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionActionText {
    #[serde(flatten)]
    pub water_protection_action_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MineralSoilLayer {
    #[serde(flatten)]
    pub mineral_soil_layer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluations {
    #[serde(flatten)]
    pub self_monitoring_evaluations: SelfMonitoringEvaluationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerReference {
    #[serde(flatten)]
    pub customer_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidenceLength {
    #[serde(flatten)]
    pub vehicle_path_subsidence_length: Decimal3_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectData {
    #[serde(flatten)]
    pub self_monitoring_object_data: SelfMonitoringObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: BdtFinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: JhsEtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: SfBufferDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSealingErrorCount {
    #[serde(flatten)]
    pub plant_sealing_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: SpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSourceDescription {
    #[serde(flatten)]
    pub damage_source_description: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(flatten)]
    pub id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathTooDeepPercentage {
    #[serde(flatten)]
    pub stand_vehicle_path_too_deep_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: JhsLajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: CoControlDataOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: CoWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonPersonificationId {
    #[serde(flatten)]
    pub non_personification_id: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(flatten)]
    pub hopper_type: WctHopperTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandArea {
    #[serde(flatten)]
    pub control_stand_area: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPoint {
    #[serde(flatten)]
    pub supply_point: SupplyPointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson1 {
    #[serde(flatten)]
    pub extra_person1: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: PostOfficeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerInvolvement {
    #[serde(flatten)]
    pub owner_involvement: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperations {
    #[serde(flatten)]
    pub habitat_operations: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclaration {
    #[serde(flatten)]
    pub control_forest_use_declaration: ControlForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: CoForestCentreDataMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlledOperationDescription {
    #[serde(flatten)]
    pub controlled_operation_description: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSpruce {
    #[serde(flatten)]
    pub mean_diameter_spruce: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: JhsLoppuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamages {
    #[serde(flatten)]
    pub stem_damages: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLifted {
    #[serde(flatten)]
    pub stumps_lifted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSilverBirch {
    #[serde(flatten)]
    pub basal_area_silver_birch: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: JhsIBANTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: CddBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumber {
    #[serde(flatten)]
    pub financing_act_number: FccFinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDescription {
    #[serde(flatten)]
    pub operation_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(flatten)]
    pub value: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperations {
    #[serde(flatten)]
    pub declaration_other_operations: DeclarationOtherOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvesting {
    #[serde(flatten)]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingSelfMonitoringFinalAuditHarvestingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringType {
    #[serde(flatten)]
    pub self_monitoring_type: CoSelfMonitoringTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluation {
    #[serde(flatten)]
    pub preclearing_evaluation: CoPreclearingEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    #[serde(flatten)]
    pub products: PrProductsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyReceivesPayment {
    #[serde(flatten)]
    pub attorney_receives_payment: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: JhsPankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: JhsAlayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerName {
    #[serde(flatten)]
    pub measurer_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessive {
    #[serde(flatten)]
    pub thinning_too_excessive: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceList {
    #[serde(flatten)]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousDeclaration {
    #[serde(flatten)]
    pub update_previous_declaration: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeftText {
    #[serde(flatten)]
    pub save_trees_left_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValue {
    #[serde(flatten)]
    pub wood_lot_information_value: WoodLotInformationValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: WtcoVATInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: JhsUlkomaaHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeHeight {
    #[serde(flatten)]
    pub tree_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaOtherTreeSpecies {
    #[serde(flatten)]
    pub basal_area_other_tree_species: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingPlantingDepth {
    #[serde(flatten)]
    pub nearest_seedling_planting_depth: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocationText {
    #[serde(flatten)]
    pub ditch_cleaning_break_location_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlannerText {
    #[serde(flatten)]
    pub feedback_for_planner_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: JhsHuoneistotunnisteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbols {
    #[serde(flatten)]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstate {
    #[serde(flatten)]
    pub financing_act_real_estate: FinancingActRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataReference {
    #[serde(flatten)]
    pub common_object_data_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagement {
    #[serde(flatten)]
    pub control_data_swamp_forest_management: ControlDataSwampForestManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: MainTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: CddShapeGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCode {
    #[serde(flatten)]
    pub grade_code: CoGradeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalTidinessNoticed {
    #[serde(flatten)]
    pub environmental_tidiness_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: BdtFinalAuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataText {
    #[serde(flatten)]
    pub metadata_text: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: JhsKolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: WtcoVATStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1DamageCount {
    #[serde(flatten)]
    pub class1_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomass {
    #[serde(flatten)]
    pub stump_biomass: StumpBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damage {
    #[serde(flatten)]
    pub damage: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: CoTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: WtcoCallForOfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonDescription {
    #[serde(flatten)]
    pub reason_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethods {
    #[serde(flatten)]
    pub used_pricing_methods: UsedPricingMethodsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: CddMeanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbroken {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken: BdtYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub goal_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: JhsNeljasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSender {
    #[serde(flatten)]
    pub offer_business_sender: OfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrectHeightStumps {
    #[serde(flatten)]
    pub correct_height_stumps: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub forest_centre_message_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingTrees {
    #[serde(flatten)]
    pub remaining_trees: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureText {
    #[serde(flatten)]
    pub road_structure_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestReference {
    #[serde(flatten)]
    pub request_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassCollectionDoneText {
    #[serde(flatten)]
    pub bio_mass_collection_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrder {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperation {
    #[serde(flatten)]
    pub declaration_other_operation: CoDeclarationOtherOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstates {
    #[serde(flatten)]
    pub base_real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: JhsStatusryhmaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReference {
    #[serde(flatten)]
    pub subsidy_applier_reference: SubsidyApplierReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: BdtVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClasses {
    #[serde(flatten)]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: CddDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TooHeightStumps {
    #[serde(flatten)]
    pub too_height_stumps: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringDate {
    #[serde(flatten)]
    pub self_monitoring_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageVolume {
    #[serde(flatten)]
    pub average_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: BdtFeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedTrees {
    #[serde(flatten)]
    pub harvested_trees: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationMainTreeSpecies {
    #[serde(flatten)]
    pub declaration_main_tree_species: CoTreeSpeciesConciseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalTidinessNoticedText {
    #[serde(flatten)]
    pub environmental_tidiness_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCountSummary {
    #[serde(flatten)]
    pub reduction_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDate {
    #[serde(flatten)]
    pub offer_date: OfferDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemsNoticedText {
    #[serde(flatten)]
    pub water_economy_systems_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlace {
    #[serde(flatten)]
    pub passing_place: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPlaceText {
    #[serde(flatten)]
    pub turning_place_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluation {
    #[serde(flatten)]
    pub control_evaluation: ControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantedSeedlingCountInCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesData {
    #[serde(flatten)]
    pub tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: JhsHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: FccPowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentReference {
    #[serde(flatten)]
    pub payment_reference: PaymentsReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: SamplePlotSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStand {
    #[serde(flatten)]
    pub financing_act_application_stand: FinancingActApplicationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingBeginningCode {
    #[serde(flatten)]
    pub seedling_beginning_code: CoSeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsRightsOwnerRepresentative {
    #[serde(flatten)]
    pub cuttings_rights_owner_representative: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingArea {
    #[serde(flatten)]
    pub processing_area: ProcessingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: CoExtendedHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementSelfMonitoringWorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    #[serde(flatten)]
    pub comments: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureQualityText {
    #[serde(flatten)]
    pub silviculture_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: CddShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSummary {
    #[serde(flatten)]
    pub basal_area_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderId {
    #[serde(flatten)]
    pub forwarder_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasins {
    #[serde(flatten)]
    pub settling_basins: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentDate {
    #[serde(flatten)]
    pub sent_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: CddLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanner {
    #[serde(flatten)]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentageTotal {
    #[serde(flatten)]
    pub dis_qualification_percentage_total: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotLength {
    #[serde(flatten)]
    pub nearest_cultivated_spot_length: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreas {
    #[serde(flatten)]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionEnds {
    #[serde(flatten)]
    pub silviculture_restriction_ends: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: CoMeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: CoDocumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinary {
    #[serde(flatten)]
    pub file_binary: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantStorageManagement {
    #[serde(flatten)]
    pub plant_storage_management: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolume {
    #[serde(flatten)]
    pub saw_log_volume: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: FccLocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompartmentId {
    #[serde(flatten)]
    pub compartment_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonCode {
    #[serde(flatten)]
    pub reason_code: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiers {
    #[serde(flatten)]
    pub tree_identifiers: TreeIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometries {
    #[serde(flatten)]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeState {
    #[serde(flatten)]
    pub change_state: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub financing_act_completion_declaration_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperation {
    #[serde(flatten)]
    pub declaration_regeneration_operation: CoDeclarationRegenerationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSilverBirch {
    #[serde(flatten)]
    pub mean_height_silver_birch: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPitsText {
    #[serde(flatten)]
    pub settling_pits_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPrice {
    #[serde(flatten)]
    pub total_price: WtcoTotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometry {
    #[serde(flatten)]
    pub working_site_geometry: SfLocatedSpecialFeature2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCutting {
    #[serde(flatten)]
    pub clear_cutting: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSites {
    #[serde(flatten)]
    pub offer_working_sites: OfferWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status5 {
    #[serde(flatten)]
    pub status5: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentage {
    #[serde(flatten)]
    pub dis_qualification_percentage: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstates {
    #[serde(flatten)]
    pub financing_act_real_estates: FinancingActRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: CddGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifier {
    #[serde(flatten)]
    pub forest_damage_qualifier: CoForestDamageQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: CddWeibullType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSpruce {
    #[serde(flatten)]
    pub stem_count_spruce: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItem {
    #[serde(flatten)]
    pub tree_list_item: TliTreeListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationReference {
    #[serde(flatten)]
    pub financing_act_application_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetails {
    #[serde(flatten)]
    pub call_for_offer_working_site_details: CallForOfferWorkingSiteDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: JhsSyntymaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingBy {
    #[serde(flatten)]
    pub cutting_by: CoVirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClass {
    #[serde(flatten)]
    pub declaration_development_class: CoDeclarationDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCompletionDate {
    #[serde(flatten)]
    pub work_completion_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocation {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: StandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMaking {
    #[serde(flatten)]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonText {
    #[serde(flatten)]
    pub dis_qualification_reason_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingSoilCompressed {
    #[serde(flatten)]
    pub nearest_seedling_soil_compressed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageClass {
    #[serde(flatten)]
    pub damage_class: VirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwners {
    #[serde(flatten)]
    pub real_estate_owners: RealEstateOwnersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: CddMaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanVolume {
    #[serde(flatten)]
    pub mean_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelId {
    #[serde(flatten)]
    pub parcel_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallation {
    #[serde(flatten)]
    pub pipe_installation: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithBreakHouse {
    #[serde(flatten)]
    pub kilometers_with_break_house: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferText {
    #[serde(flatten)]
    pub offer_text: OfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisks {
    #[serde(flatten)]
    pub work_safety_risks: WorkSafetyRisksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVolume {
    #[serde(flatten)]
    pub total_volume: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: JhsKuudesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeInstallationText {
    #[serde(flatten)]
    pub pipe_installation_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: CddMinimumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInfoText {
    #[serde(flatten)]
    pub material_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperation {
    #[serde(flatten)]
    pub object_protection_operation: ObjectProtectionOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: WctTaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damages {
    #[serde(flatten)]
    pub damages: DamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: ForestHaulageDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: EmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode {
    #[serde(flatten)]
    pub language_code: LanguageCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalData {
    #[serde(flatten)]
    pub proposal_data: ProposalDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: CoDifficultyClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: JhsHuoltosuhdeTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsSummaries {
    #[serde(flatten)]
    pub sample_plots_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandRootDamagesPercentage {
    #[serde(flatten)]
    pub stand_root_damages_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometries {
    #[serde(flatten)]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentatives {
    #[serde(flatten)]
    pub working_representatives: WorkingRepresentativesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityData {
    #[serde(flatten)]
    pub accessibility_data: AccessibilityDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroup {
    #[serde(flatten)]
    pub financing_act_work_group: CoFinancingActWorkGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignData {
    #[serde(flatten)]
    pub harvesting_sign_data: HarvestingSignDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluations {
    #[serde(flatten)]
    pub control_evaluations: ControlEvaluationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkQuality {
    #[serde(flatten)]
    pub work_quality: VirtaWorkQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson2 {
    #[serde(flatten)]
    pub extra_person2: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedWorkAmount {
    #[serde(flatten)]
    pub completed_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BIC {
    #[serde(flatten)]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCode {
    #[serde(flatten)]
    pub owner_ship_type_code: CoOwnerShipTypeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfo {
    #[serde(flatten)]
    pub stand_silviculture_info: StandSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingDone {
    #[serde(flatten)]
    pub clearing_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceDrainingAreas {
    #[serde(flatten)]
    pub surface_draining_areas: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_diameter_other_tree_species: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreinformDate {
    #[serde(flatten)]
    pub preinform_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitValue {
    #[serde(flatten)]
    pub unit_value: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYear {
    #[serde(flatten)]
    pub planning_year: PlanningYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListItem {
    #[serde(flatten)]
    pub fee_base_list_item: FeebaseListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilPreparationSpotsAreEnough {
    #[serde(flatten)]
    pub soil_preparation_spots_are_enough: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum {
    #[serde(flatten)]
    pub tree_stratum: TreeStratum2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometry {
    #[serde(flatten)]
    pub financing_act_application_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    #[serde(flatten)]
    pub offer: OfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItem {
    #[serde(flatten)]
    pub price_item: PriceItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WtcoWorkingSitePlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometry {
    #[serde(flatten)]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: CoAmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: JhsKuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfo {
    #[serde(flatten)]
    pub stand_wood_trade_info: StandWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelStartTime {
    #[serde(flatten)]
    pub travel_start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjects {
    #[serde(flatten)]
    pub financing_act_completion_other_subjects: FinancingActCompletionOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilviculture {
    #[serde(flatten)]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilviculture {
    #[serde(flatten)]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationCode {
    #[serde(flatten)]
    pub operation_code: CoObjectProtectionOperationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatmentText {
    #[serde(flatten)]
    pub stump_treatment_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKey {
    #[serde(flatten)]
    pub working_site_key: WorkingSiteKeyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: CoRestrictionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingPits {
    #[serde(flatten)]
    pub settling_pits: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: OrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseList {
    #[serde(flatten)]
    pub fee_base_list: FeeBaseListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningQuality {
    #[serde(flatten)]
    pub soil_conditioning_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrderText {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(flatten)]
    pub map_symbol_type: BdtFeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPractice {
    #[serde(flatten)]
    pub cutting_realization_practice: CoCuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationNumber {
    #[serde(flatten)]
    pub completion_declaration_number: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: CoForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometryType {
    #[serde(flatten)]
    pub help_geometry_type: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SittingMoneyKilometers {
    #[serde(flatten)]
    pub sitting_money_kilometers: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: CoAssortmentNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: CoSubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: GdtAlternativeGeometries2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: CoTransactionQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffect {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotWidth {
    #[serde(flatten)]
    pub nearest_cultivated_spot_width: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrata {
    #[serde(flatten)]
    pub stem_distribution_strata: StemDistributionStrataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoniness {
    #[serde(flatten)]
    pub restriction_based_on_stoniness: CoRestrictionBasedOnStoninessType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperLocationFromGPS {
    #[serde(flatten)]
    pub hopper_location_from_g_p_s: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: JhsHuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmountUnit {
    #[serde(flatten)]
    pub material_amount_unit: CoMaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub target: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityControlDate {
    #[serde(flatten)]
    pub quality_control_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationScope {
    #[serde(flatten)]
    pub cultivation_scope: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShapeText {
    #[serde(flatten)]
    pub road_structure_shape_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedData {
    #[serde(flatten)]
    pub tree_stand_based_data: StTreeStandBasedDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionalityText {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractId {
    #[serde(flatten)]
    pub purchase_contract_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumplingLevelingErrorCount {
    #[serde(flatten)]
    pub dumpling_leveling_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub tree_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: CoIdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubCategory {
    #[serde(flatten)]
    pub sub_category: BdtImageSubCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: BdtStoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorderingWithWaterAreaOrStream {
    #[serde(flatten)]
    pub bordering_with_water_area_or_stream: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbrokenText {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notices {
    #[serde(flatten)]
    pub notices: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: StemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioning {
    #[serde(flatten)]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainageText {
    #[serde(flatten)]
    pub road_structure_drainage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemsNoticed {
    #[serde(flatten)]
    pub water_economy_systems_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: JhsIkaluokkaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionData {
    #[serde(flatten)]
    pub completion_data: ExtendedCompletionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: JhsVoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchesInAdditionToCultivation {
    #[serde(flatten)]
    pub ditches_in_addition_to_cultivation: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(flatten)]
    pub identifier_type: IdentifierTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: GroupNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAmount {
    #[serde(flatten)]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: AddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantStorageManagementText {
    #[serde(flatten)]
    pub plant_storage_management_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCode {
    #[serde(flatten)]
    pub assortment_code: CoAssortmentCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMin {
    #[serde(flatten)]
    pub length_min: LengthMinType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSiteCountSummary {
    #[serde(flatten)]
    pub plant_site_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathWidth {
    #[serde(flatten)]
    pub stand_vehicle_path_width: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibility {
    #[serde(flatten)]
    pub transport_accessibility: CoTransportAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipType {
    #[serde(flatten)]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountPine {
    #[serde(flatten)]
    pub stem_count_pine: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructure {
    #[serde(flatten)]
    pub road_structure: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningDepth {
    #[serde(flatten)]
    pub soil_conditioning_depth: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: JhsBICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationTextInformation {
    #[serde(flatten)]
    pub declaration_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTooHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_too_height_stumps_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValue {
    #[serde(flatten)]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpStemCount {
    #[serde(flatten)]
    pub stump_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClodMoistureErrorCount {
    #[serde(flatten)]
    pub clod_moisture_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: ImageSelfMonitoringImageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: BdtFinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScaping {
    #[serde(flatten)]
    pub land_scaping: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CddCumulativePointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandStemDamagesPercentage {
    #[serde(flatten)]
    pub stand_stem_damages_percentage: BdtPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPulpwoodVolume {
    #[serde(flatten)]
    pub total_pulpwood_volume: CoPulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeType {
    #[serde(flatten)]
    pub dead_tree_type: DeadTreeTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthOfDitchDiggedDuringSoilPreparation {
    #[serde(flatten)]
    pub length_of_ditch_digged_during_soil_preparation: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDeliveringEvaluation {
    #[serde(flatten)]
    pub declaration_delivering_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(flatten)]
    pub authorization: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusReduction {
    #[serde(flatten)]
    pub radius_reduction: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithCaravan {
    #[serde(flatten)]
    pub kilometers_with_caravan: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOutsideSpot {
    #[serde(flatten)]
    pub amount_outside_spot: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowed {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: BdtYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: JhsEnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCases {
    #[serde(flatten)]
    pub use_cases: ForestDataUpdateUseCasesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReason {
    #[serde(flatten)]
    pub dis_qualification_reason: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandBasicData {
    #[serde(flatten)]
    pub control_stand_basic_data: ControlStandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratum {
    #[serde(flatten)]
    pub stem_distribution_stratum: StemDistributionStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hopper {
    #[serde(flatten)]
    pub hopper: HopperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamages {
    #[serde(flatten)]
    pub root_damages: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathTooDeep {
    #[serde(flatten)]
    pub vehicle_path_too_deep: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    #[serde(flatten)]
    pub file_type: FileTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: JhsNimilajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotification {
    #[serde(flatten)]
    pub electronic_notification: CoElectronicNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionCompleted {
    #[serde(flatten)]
    pub prevention_completed: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpsLiftedText {
    #[serde(flatten)]
    pub stumps_lifted_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageData {
    #[serde(flatten)]
    pub damage_data: DamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: SamplePlotTreesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolume {
    #[serde(flatten)]
    pub pulp_wood_volume: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureQuality {
    #[serde(flatten)]
    pub silviculture_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlingBasinsText {
    #[serde(flatten)]
    pub settling_basins_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingAction {
    #[serde(flatten)]
    pub erosion_blocking_action: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethodCorrect {
    #[serde(flatten)]
    pub soil_conditioning_method_correct: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightPine {
    #[serde(flatten)]
    pub mean_height_pine: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDate {
    #[serde(flatten)]
    pub document_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: DiameterMinType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: CddShapeBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(flatten)]
    pub age: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountHardWood {
    #[serde(flatten)]
    pub stem_count_hard_wood: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeSummary {
    #[serde(flatten)]
    pub volume_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClass {
    #[serde(flatten)]
    pub removal_class: CoRemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalStemCount {
    #[serde(flatten)]
    pub goal_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClass {
    #[serde(flatten)]
    pub small_wood_removal_class: CoSmallWoodRemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CddCumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpedMoundsCount {
    #[serde(flatten)]
    pub scalped_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_wood_trade_info: WtcoCallForOfferWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: SpecialPermissionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumOrigin {
    #[serde(flatten)]
    pub stratum_origin: CoSeedlingOriginType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCutting {
    #[serde(flatten)]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingSelfMonitoringWorkingSiteQualityControlCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCode {
    #[serde(flatten)]
    pub assortment_class_code: AssortmentClassCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResource {
    #[serde(flatten)]
    pub planned_resource: PlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeature {
    #[serde(flatten)]
    pub control_data_special_feature: ControlDataSpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionAction {
    #[serde(flatten)]
    pub water_protection_action: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSilverBirch {
    #[serde(flatten)]
    pub stem_count_silver_birch: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterPine {
    #[serde(flatten)]
    pub mean_diameter_pine: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandLocatedOnIsland {
    #[serde(flatten)]
    pub stand_located_on_island: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityDescription {
    #[serde(flatten)]
    pub seedling_condition_and_quality_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesControl {
    #[serde(flatten)]
    pub special_features_control: ControlDataSpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(flatten)]
    pub category: BdtImageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingBreaksText {
    #[serde(flatten)]
    pub ditching_breaks_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterHardWood {
    #[serde(flatten)]
    pub mean_diameter_hard_wood: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: CoInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationCommitment {
    #[serde(flatten)]
    pub declaration_regeneration_commitment: CoRegenerationCommitmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: CoVirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethod {
    #[serde(flatten)]
    pub used_pricing_method: UsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub terrain_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: CoMachineCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: JhsHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometry {
    #[serde(flatten)]
    pub financing_act_completion_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(flatten)]
    pub symbol: MapSymbolDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicData {
    #[serde(flatten)]
    pub self_monitoring_basic_data: SelfMonitoringBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPerson {
    #[serde(flatten)]
    pub technical_contact_person: TechnicalContactPersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCost {
    #[serde(flatten)]
    pub evaluation_cost: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomass {
    #[serde(flatten)]
    pub stem_biomass: StemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasicData {
    #[serde(flatten)]
    pub sample_plot_basic_data: SamplePlotBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: JhsKuolemaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectId {
    #[serde(flatten)]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationSign {
    #[serde(flatten)]
    pub dis_qualification_sign: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCode {
    #[serde(flatten)]
    pub evaluation_code: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: WtcoCertificationSystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumber {
    #[serde(flatten)]
    pub processing_area_number: ProcessingAreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstance {
    #[serde(flatten)]
    pub prevention_substance: CoPreventionSubstanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassingPlaceText {
    #[serde(flatten)]
    pub passing_place_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectOverallEvaluationData {
    #[serde(flatten)]
    pub object_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedDeadStemCount {
    #[serde(flatten)]
    pub cultivated_dead_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantCount {
    #[serde(flatten)]
    pub plant_count: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchType {
    #[serde(flatten)]
    pub ditch_type: WctDitchTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracy {
    #[serde(flatten)]
    pub cutting_accuracy: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: BdtMeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(flatten)]
    pub route: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementSelfMonitoringWorkingSiteQualityControlPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: JhsOsoiteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIsAuthorizedForEstate {
    #[serde(flatten)]
    pub project_is_authorized_for_estate: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamagesText {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStand {
    #[serde(flatten)]
    pub financing_act_completion_stand: FinancingActCompletionStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionCategory {
    #[serde(flatten)]
    pub payment_transaction_category: CoMoneyTransactionCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub real_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub financing_act_application_text_information: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataMooseDamageData {
    #[serde(flatten)]
    pub control_data_moose_damage_data: MooseDamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifier {
    #[serde(flatten)]
    pub tree_identifier: TreeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDate {
    #[serde(flatten)]
    pub v_a_t_registration_date: WtcoVATRegistrationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameterSummary {
    #[serde(flatten)]
    pub reduction_avg_diameter_summary: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: BdtWorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasalAreaSummary {
    #[serde(flatten)]
    pub stand_basal_area_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: WtcoRoadUsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensations {
    #[serde(flatten)]
    pub total_compensations: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: DitchingYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseMode {
    #[serde(flatten)]
    pub purchase_mode: PurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: JhsSahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumpsText {
    #[serde(flatten)]
    pub high_stumps_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBasicData {
    #[serde(flatten)]
    pub control_basic_data: ControlBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDate {
    #[serde(flatten)]
    pub felling_right_validity_date: FellingRightValidityDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: JhsKansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDate {
    #[serde(flatten)]
    pub power_of_attorney_date: FccPowerOfAttorneyDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpsCount {
    #[serde(flatten)]
    pub scalps_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsToWorkingSite {
    #[serde(flatten)]
    pub date_seedlings_to_working_site: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(flatten)]
    pub object: CodForestObjectDataObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWIthSeedlings {
    #[serde(flatten)]
    pub stocking_w_ith_seedlings: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: QuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClass {
    #[serde(flatten)]
    pub assortment_class: AssortmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kilometers {
    #[serde(flatten)]
    pub kilometers: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ReParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingLotIdentifier {
    #[serde(flatten)]
    pub seedling_lot_identifier: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: CoRestrictionCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlOverallEvaluationData {
    #[serde(flatten)]
    pub control_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: StandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethodCorrectText {
    #[serde(flatten)]
    pub soil_conditioning_method_correct_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMeanWidth {
    #[serde(flatten)]
    pub ditch_mean_width: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlanner {
    #[serde(flatten)]
    pub feedback_for_planner: BdtWorkingQualityType,
}

