#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSites {
    #[serde(flatten)]
    pub call_for_offer_working_sites: CallForOfferWorkingSitesType,
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
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: VoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: StoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalPriceType,
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
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: RestrictionCodeType,
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
pub struct BaseRealEstates {
    #[serde(flatten)]
    pub base_real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub used_pricing_method_type: UsedPricingMethodTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
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
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: VATStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: EtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: AmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: RestrictionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: DocumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandScapingText {
    #[serde(flatten)]
    pub land_scaping_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuttings {
    #[serde(flatten)]
    pub cuttings: CuttingsType,
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
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomass {
    #[serde(flatten)]
    pub stem_biomass: StemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: MainTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanStumpDiameter {
    #[serde(flatten)]
    pub mean_stump_diameter: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: DiameterType,
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
pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: StemCountType,
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
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StandBasicDataType,
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
pub struct Scale {
    #[serde(flatten)]
    pub scale: ScaleType,
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
pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: DateType,
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
pub struct ChangeTime {
    #[serde(flatten)]
    pub change_time: ChangeTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: JohnsonSBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: FeatureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
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
pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: TurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: ShapeDeltaType,
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
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: ShapeGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: SahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: OperationStatusType,
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
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureDrainageText {
    #[serde(flatten)]
    pub road_structure_drainage_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDate {
    #[serde(flatten)]
    pub document_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterProtectionAction {
    #[serde(flatten)]
    pub water_protection_action: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethod {
    #[serde(flatten)]
    pub pricing_method: UsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: AssortmentNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    #[serde(flatten)]
    pub tree: TreeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: LocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: PostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructureShapeText {
    #[serde(flatten)]
    pub road_structure_shape_text: String200Type,
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
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: KieliKoodiTyyppi,
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
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCode {
    #[serde(flatten)]
    pub assortment_code: AssortmentCodeType,
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
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
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
pub struct Variance {
    #[serde(flatten)]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: OrganizationNameType,
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
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: DistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: MachineCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: MeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsible {
    #[serde(flatten)]
    pub forest_use_declaration_responsible: ForestUseDeclarationResponsibleType,
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
pub struct DocumentDescription {
    #[serde(flatten)]
    pub document_description: DocumentDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeDescription {
    #[serde(flatten)]
    pub wood_lot_information_type_description: String500Type,
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
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
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
pub struct PaymentTransactionCategory {
    #[serde(flatten)]
    pub payment_transaction_category: MoneyTransactionCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: SellersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: SellerRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: ValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: DevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDate {
    #[serde(flatten)]
    pub v_a_t_registration_date: VATRegistrationDateType,
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
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: KansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: Decimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
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
pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: HarvestingAccessibilityType,
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
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: KolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: StandQualityType,
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
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: FinalAuditerTypeType,
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
pub struct PurchaseMode {
    #[serde(flatten)]
    pub purchase_mode: PurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TreeSpeciesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(flatten)]
    pub payment_transaction_type: MoneyTransactionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: MachineAccessoryCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferId {
    #[serde(flatten)]
    pub related_call_for_offer_id: string,
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
pub struct Value {
    #[serde(flatten)]
    pub value: MoneyType,
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
pub struct Biomass {
    #[serde(flatten)]
    pub biomass: BiomassType,
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
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitValue {
    #[serde(flatten)]
    pub unit_value: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StandBasicDataDateType,
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
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: WeibullType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: YearType,
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
pub struct GradeCode {
    #[serde(flatten)]
    pub grade_code: GradeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValue {
    #[serde(flatten)]
    pub wood_lot_information_value: WoodLotInformationValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErosionBlockingAction {
    #[serde(flatten)]
    pub erosion_blocking_action: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: PostOfficeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadStructure {
    #[serde(flatten)]
    pub road_structure: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: AlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiability {
    #[serde(flatten)]
    pub cutting_planner_liability: CuttingPlannerLiabilityType,
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
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: SahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackForPlanner {
    #[serde(flatten)]
    pub feedback_for_planner: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: base64Binary,
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
    pub stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: YritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: integer,
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
pub struct PipeInstallation {
    #[serde(flatten)]
    pub pipe_installation: WorkingQualityType,
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
pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: PostalCodeType,
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
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: DifficultyClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: ValidityType,
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
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: RoadUsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSites {
    #[serde(flatten)]
    pub offer_working_sites: OfferWorkingSitesType,
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
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: SpecialPermissionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: PositiveInteger5digitsType,
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
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: SiviilisaatyTekstiTyyppi,
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
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: ShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: EnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
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
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: WideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(flatten)]
    pub age: AgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: SilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BIC {
    #[serde(flatten)]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: DateType,
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
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    #[serde(flatten)]
    pub file_type: FileTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: ValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TreeStrata2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cutting {
    #[serde(flatten)]
    pub cutting: CuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
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
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
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
pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: AddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: String200Type,
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
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: KuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: HuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeState {
    #[serde(flatten)]
    pub change_state: ChangeStateType,
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
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: SukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: HarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: FeatureAdditionalInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPoint {
    #[serde(flatten)]
    pub supply_point: SupplyPointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: CallForOfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
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
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: MaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: PankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
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
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: RealEstateType,
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
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: BasalAreaType,
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
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: MeanHeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: ControlDataOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: DateType,
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
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: FinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BearingCapacityClassType,
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
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: NeljasRiviTekstiTyyppi,
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
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WorkingSiteGeometriesType,
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
pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: PaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometry {
    #[serde(flatten)]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: string,
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
    pub special_feature: BasicFeature2Type,
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
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeType {
    #[serde(flatten)]
    pub dead_tree_type: DeadTreeTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: DeclarationReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolumeAccounted {
    #[serde(flatten)]
    pub harvested_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: CareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffer {
    #[serde(flatten)]
    pub related_call_for_offer: RelatedCallForOfferType,
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
pub struct CallForOffer {
    #[serde(flatten)]
    pub call_for_offer: CallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: DiameterMinType,
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
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: CertificationSystemsType,
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
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: KatuNimiTyyppi,
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
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: EmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: KuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescription {
    #[serde(flatten)]
    pub payment_transaction_description: PaymentTransactionDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
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
pub struct Shape {
    #[serde(flatten)]
    pub shape: ShapeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CumulativePointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPayment {
    #[serde(flatten)]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
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
pub struct PriceMatrix {
    #[serde(flatten)]
    pub price_matrix: PriceMatrixType,
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
pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLots {
    #[serde(flatten)]
    pub wood_lots: WoodLotsType,
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
pub struct LeafBiomass {
    #[serde(flatten)]
    pub leaf_biomass: LeafBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: KuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ForestUseDeclarationType,
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
pub struct TreeIdentifiers {
    #[serde(flatten)]
    pub tree_identifiers: TreeIdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: BICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferDescription {
    #[serde(flatten)]
    pub related_call_for_offer_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: ShapeBetaType,
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
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WorkingSitePlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: PuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: SubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystem {
    #[serde(flatten)]
    pub certification_system: CertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountNotified {
    #[serde(flatten)]
    pub amount_notified: Decimal3FractionDigitsType,
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
    pub diameter_max: DiameterMaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: WorkingSiteNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: EndDateType,
}

