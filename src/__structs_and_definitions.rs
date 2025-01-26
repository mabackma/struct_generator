use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListId {
    #[serde(flatten)]
    pub list_id: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationContactPerson {
    #[serde(flatten)]
    pub notification_contact_person: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductionFileSendFrequency {
    #[serde(flatten)]
    pub production_file_send_frequency: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeType {
    #[serde(flatten)]
    pub fee_type: FeeBasisValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantSealingErrorCount {
    #[serde(flatten)]
    pub plant_sealing_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataName {
    #[serde(flatten)]
    pub data_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfreezingDate {
    #[serde(flatten)]
    pub unfreezing_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainApplier {
    #[serde(flatten)]
    pub main_applier: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundWoodSalesRow {
    #[serde(flatten)]
    pub round_wood_sales_row: RoundWoodSalesRowType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleanlinessClass {
    #[serde(flatten)]
    pub cleanliness_class: CleanlinessClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: UlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadRating {
    #[serde(flatten)]
    pub load_rating: LoadRatingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(flatten)]
    pub location: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettlingBasins {
    #[serde(flatten)]
    pub settling_basins: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    #[serde(flatten)]
    pub attribute: AttributeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStand {
    #[serde(flatten)]
    pub declaration_stand: DeclarationStandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountNotified {
    #[serde(flatten)]
    pub amount_notified: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparatedParcel {
    #[serde(flatten)]
    pub unseparated_parcel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccCompletionDeclarationReference {
    #[serde(flatten)]
    pub fcc_completion_declaration_reference: CompletionDeclarationReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    #[serde(flatten)]
    pub route: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    #[serde(flatten)]
    pub owner: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: SukuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestType {
    #[serde(flatten)]
    pub forest_type: FertilityClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: SpareTreeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadVolume {
    #[serde(flatten)]
    pub load_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GdtMultiPolygonGeometry {
    #[serde(flatten)]
    pub gdt_multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: JohnsonSBType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PolygonOrMultiPolygon2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionCount {
    #[serde(flatten)]
    pub reduction_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reasons {
    #[serde(flatten)]
    pub reasons: ReasonsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Acknowledge {
    #[serde(flatten)]
    pub acknowledge: AcknowledgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cutting {
    #[serde(flatten)]
    pub cutting: CuttingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractors {
    #[serde(flatten)]
    pub sub_contractors: SubContractorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadStructureText {
    #[serde(flatten)]
    pub road_structure_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeIdentifiers {
    #[serde(flatten)]
    pub tree_identifiers: TreeIdentifiersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetExtraInfo {
    #[serde(flatten)]
    pub target_extra_info: VirtaExtraInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StanfordFile {
    #[serde(flatten)]
    pub stanford_file: StanfordFileType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludedInOffer {
    #[serde(flatten)]
    pub included_in_offer: IncludedInOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    #[serde(flatten)]
    pub machine: MachineTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KemeraId {
    #[serde(flatten)]
    pub kemera_id: VirtaIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferDate {
    #[serde(flatten)]
    pub call_for_offer_date: CallForOfferDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateOwners {
    #[serde(flatten)]
    pub real_estate_owners: RealEstateOwnersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerReference {
    #[serde(flatten)]
    pub customer_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvlEnvelope {
    #[serde(flatten)]
    pub envl_envelope: Envelope,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MixedForestRegenarationMethods {
    #[serde(flatten)]
    pub mixed_forest_regenaration_methods: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjectTypeSpecifier {
    #[serde(flatten)]
    pub child_object_type_specifier: ObjectTypeSpecifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibration {
    #[serde(flatten)]
    pub length_calibration: LengthCalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpectedValueCoefficient {
    #[serde(flatten)]
    pub expected_value_coefficient: PositiveDecimalMax1IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status3 {
    #[serde(flatten)]
    pub status3: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetNumber {
    #[serde(flatten)]
    pub target_number: PositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkApplication {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredContactingMethod {
    #[serde(flatten)]
    pub preferred_contacting_method: PreferredContactingMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    #[serde(flatten)]
    pub model: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubPriceArea {
    #[serde(flatten)]
    pub stub_price_area: VirtaSumTableAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetectedGroundWood {
    #[serde(flatten)]
    pub detected_ground_wood: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivationScope {
    #[serde(flatten)]
    pub cultivation_scope: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSource {
    #[serde(flatten)]
    pub data_source: DataSourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringStandArea {
    #[serde(flatten)]
    pub self_monitoring_stand_area: AreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionAnswerAsText {
    #[serde(flatten)]
    pub question_answer_as_text: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationApplicant {
    #[serde(flatten)]
    pub compensation_applicant: ContactInformationBankAccountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditBioMassForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentVolumeUnclassified {
    #[serde(flatten)]
    pub assortment_volume_unclassified: AssortmentVolumeUnclassifiedType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: CertificationSystemsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapSymbolName {
    #[serde(flatten)]
    pub map_symbol_name: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearCutting {
    #[serde(flatten)]
    pub clear_cutting: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Envelopes {
    #[serde(flatten)]
    pub envelopes: EnvelopesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantedSeedlingCountInCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_cultivated_spots: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeName {
    #[serde(flatten)]
    pub code_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialDeliveryType {
    #[serde(flatten)]
    pub material_delivery_type: MaterialDeliveryTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionAvgDiameterSummary {
    #[serde(flatten)]
    pub reduction_avg_diameter_summary: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadStructure {
    #[serde(flatten)]
    pub road_structure: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyAmount {
    #[serde(flatten)]
    pub subsidy_amount: MoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitialLoadId {
    #[serde(flatten)]
    pub partitial_load_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: OsoiteNumeroTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(flatten)]
    pub product: ProductType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: HenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentMonth {
    #[serde(flatten)]
    pub deployment_month: String5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSFInformation {
    #[serde(flatten)]
    pub f_s_f_information: FSFInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionStand {
    #[serde(flatten)]
    pub financing_act_completion_stand: FinancingActCompletionStandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadCount {
    #[serde(flatten)]
    pub load_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettlingBasinsText {
    #[serde(flatten)]
    pub settling_basins_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestedVolumeAccounted {
    #[serde(flatten)]
    pub harvested_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionData {
    #[serde(flatten)]
    pub completion_data: ExtendedCompletionDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audition {
    #[serde(flatten)]
    pub audition: AuditionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentClass {
    #[serde(flatten)]
    pub document_class: DocumentClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendWorkingAloneNotification {
    #[serde(flatten)]
    pub send_working_alone_notification: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionSender {
    #[serde(flatten)]
    pub transmission_sender: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAcceptanceActor {
    #[serde(flatten)]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportAccessibility {
    #[serde(flatten)]
    pub transport_accessibility: TransportAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingBeginningCode {
    #[serde(flatten)]
    pub seedling_beginning_code: SeedlingOriginType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionData {
    #[serde(flatten)]
    pub inspection_data: InspectionDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: FeatureTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectReference {
    #[serde(flatten)]
    pub object_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AckAcknowledge {
    #[serde(flatten)]
    pub ack_acknowledge: Acknowledge,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployerRegister {
    #[serde(flatten)]
    pub employer_register: EmployerRegisterType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationPolygonReference {
    #[serde(flatten)]
    pub declaration_polygon_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreinformDate {
    #[serde(flatten)]
    pub preinform_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeWrittenAgreement {
    #[serde(flatten)]
    pub employee_written_agreement: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: MunicipalityNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: SpecialFeatureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationCode {
    #[serde(flatten)]
    pub evaluation_code: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: FeatureCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplierId {
    #[serde(flatten)]
    pub subsidy_applier_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIsAuthorizedForEstate {
    #[serde(flatten)]
    pub project_is_authorized_for_estate: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommunicationType {
    #[serde(flatten)]
    pub communication_type: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalTreeSpecies {
    #[serde(flatten)]
    pub goal_tree_species: TreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErosionBlockingAction {
    #[serde(flatten)]
    pub erosion_blocking_action: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalArea {
    #[serde(flatten)]
    pub proposal_area: ProposalAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCode {
    #[serde(flatten)]
    pub new_code: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasureDeviceCheckRequired {
    #[serde(flatten)]
    pub measure_device_check_required: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectiveAgreement {
    #[serde(flatten)]
    pub collective_agreement: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedStartDate {
    #[serde(flatten)]
    pub estimated_start_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorResource {
    #[serde(flatten)]
    pub sub_contractor_resource: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GradeCode {
    #[serde(flatten)]
    pub grade_code: GradeCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccFinancingActNumber {
    #[serde(flatten)]
    pub fcc_financing_act_number: FinancingActNumber,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FudrForestUseDeclarationReference {
    #[serde(flatten)]
    pub fudr_forest_use_declaration_reference: ForestUseDeclarationReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayeesAndRealEstates {
    #[serde(flatten)]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationByInsurance {
    #[serde(flatten)]
    pub compensation_by_insurance: CompensationByInsuranceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineAccessoryDescription {
    #[serde(flatten)]
    pub machine_accessory_description: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationDate {
    #[serde(flatten)]
    pub diameter_calibration_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    #[serde(flatten)]
    pub object: ForestCentreControlObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaserRepresentativePerson {
    #[serde(flatten)]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwarderId {
    #[serde(flatten)]
    pub forwarder_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    #[serde(flatten)]
    pub role: OrganizationRoleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivatedDeadStemCount {
    #[serde(flatten)]
    pub cultivated_dead_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: WorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(flatten)]
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOffer {
    #[serde(flatten)]
    pub call_for_offer: CallForOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationUrgency {
    #[serde(flatten)]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: WorkingSitePlanningOperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaAndMapEvaluation {
    #[serde(flatten)]
    pub area_and_map_evaluation: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandWoodTradeInfo {
    #[serde(flatten)]
    pub stand_wood_trade_info: StandWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootDamageCount {
    #[serde(flatten)]
    pub root_damage_count: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectProtectionOperation {
    #[serde(flatten)]
    pub object_protection_operation: ObjectProtectionOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comments {
    #[serde(flatten)]
    pub comments: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationGeometry {
    #[serde(flatten)]
    pub financing_act_application_geometry: FinancingActGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoFeatureCodeType {
    #[serde(flatten)]
    pub co_feature_code_type: FeatureCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreNotificationAllowed {
    #[serde(flatten)]
    pub pre_notification_allowed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdTrees {
    #[serde(flatten)]
    pub sd_trees: Trees,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    #[serde(flatten)]
    pub value: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub contact_person: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionId {
    #[serde(flatten)]
    pub question_id: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalSender {
    #[serde(flatten)]
    pub original_sender: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageHeight {
    #[serde(flatten)]
    pub average_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestHaulageFinishedDate {
    #[serde(flatten)]
    pub forest_haulage_finished_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Law {
    #[serde(flatten)]
    pub law: VirtaLawType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthClass {
    #[serde(flatten)]
    pub length_class: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalAreaSpruce {
    #[serde(flatten)]
    pub basal_area_spruce: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation2 {
    #[serde(flatten)]
    pub organisation2: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveIncomplete {
    #[serde(flatten)]
    pub save_incomplete: VirtaSaveIncompleteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditDynamic {
    #[serde(flatten)]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreClearing {
    #[serde(flatten)]
    pub pre_clearing: YesNoNotNeededType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchMoundsCount {
    #[serde(flatten)]
    pub ditch_mounds_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingFreeText {
    #[serde(flatten)]
    pub training_free_text: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReasonCode {
    #[serde(flatten)]
    pub reason_code: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationTotalSubsidy {
    #[serde(flatten)]
    pub application_total_subsidy: MoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Humidity {
    #[serde(flatten)]
    pub humidity: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Depth {
    #[serde(flatten)]
    pub depth: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionStumpHeight {
    #[serde(flatten)]
    pub reduction_stump_height: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetPart {
    #[serde(flatten)]
    pub target_part: TargetPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDescription {
    #[serde(flatten)]
    pub action_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBaseListItem {
    #[serde(flatten)]
    pub fee_base_list_item: FeebaseListItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoTextMandatory {
    #[serde(flatten)]
    pub info_text_mandatory: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAccounting {
    #[serde(flatten)]
    pub final_accounting: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractValidDate {
    #[serde(flatten)]
    pub contract_valid_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasAssortmentChanges {
    #[serde(flatten)]
    pub has_assortment_changes: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyArgumentText {
    #[serde(flatten)]
    pub subsidy_argument_text: String5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettlingPits {
    #[serde(flatten)]
    pub settling_pits: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceLocations {
    #[serde(flatten)]
    pub resource_locations: ResourceLocationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludeForestFundPayment {
    #[serde(flatten)]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationTextInformation {
    #[serde(flatten)]
    pub declaration_text_information: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OOffer {
    #[serde(flatten)]
    pub o_offer: Offer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductGroupName {
    #[serde(flatten)]
    pub product_group_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    #[serde(flatten)]
    pub offer: OfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpruceLog {
    #[serde(flatten)]
    pub spruce_log: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CorrectHeightStumps {
    #[serde(flatten)]
    pub correct_height_stumps: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionAnswerText {
    #[serde(flatten)]
    pub question_answer_text: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoPreferredContactingMethodType {
    #[serde(flatten)]
    pub co_preferred_contacting_method_type: PreferredContactingMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: MunicipalityNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldCode {
    #[serde(flatten)]
    pub old_code: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleaningBreaksDone {
    #[serde(flatten)]
    pub cleaning_breaks_done: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeVolume {
    #[serde(flatten)]
    pub change_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalFile {
    #[serde(flatten)]
    pub external_file: ExternalFileType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpOperations {
    #[serde(flatten)]
    pub op_operations: Operations,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockingWithSeedlings {
    #[serde(flatten)]
    pub stocking_with_seedlings: VirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoMachineCodeType {
    #[serde(flatten)]
    pub co_machine_code_type: MachineCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueList {
    #[serde(flatten)]
    pub value_list: ValueListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Month {
    #[serde(flatten)]
    pub month: MonthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountSpruce {
    #[serde(flatten)]
    pub stem_count_spruce: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandDataDate {
    #[serde(flatten)]
    pub ts_tree_stand_data_date: TreeStandDataDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trainings {
    #[serde(flatten)]
    pub trainings: TrainingsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: AgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociationEvaluationApproval {
    #[serde(flatten)]
    pub association_evaluation_approval: VirtaApprovalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationByLegislation {
    #[serde(flatten)]
    pub compensation_by_legislation: CompensationByLegislationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationCategory {
    #[serde(flatten)]
    pub evaluation_category: EvaluationSubjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoRestrictionType {
    #[serde(flatten)]
    pub co_restriction_type: RestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCertification {
    #[serde(flatten)]
    pub forest_certification: CertificationSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: FaksinumeroTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationAmount {
    #[serde(flatten)]
    pub compensation_amount: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTexts {
    #[serde(flatten)]
    pub payment_texts: PaymentTextsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleFactor {
    #[serde(flatten)]
    pub scale_factor: ScaleFactorDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationMode {
    #[serde(flatten)]
    pub operation_mode: OperationModeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoEvaluationCodeType {
    #[serde(flatten)]
    pub co_evaluation_code_type: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAcceptance {
    #[serde(flatten)]
    pub business_acceptance: BusinessAcceptanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvertisementDating {
    #[serde(flatten)]
    pub advertisement_dating: VirtaAdvertisementDatingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccAttorney {
    #[serde(flatten)]
    pub fcc_attorney: Attorney,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationStandExtraInfo {
    #[serde(flatten)]
    pub forest_use_declaration_stand_extra_info: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilImprovementEvaluation {
    #[serde(flatten)]
    pub soil_improvement_evaluation: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: DifficultyClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSignControlClassifier {
    #[serde(flatten)]
    pub harvesting_sign_control_classifier: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerOfAttorneyDate {
    #[serde(flatten)]
    pub power_of_attorney_date: PowerOfAttorneyDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTransactionCategory {
    #[serde(flatten)]
    pub payment_transaction_category: MoneyTransactionCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: SahkopostiosoiteTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentIncrementAllowed {
    #[serde(flatten)]
    pub assortment_increment_allowed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbol {
    #[serde(flatten)]
    pub symbol: MapSymbolDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTransactions {
    #[serde(flatten)]
    pub payment_transactions: PaymentTransactionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: TwoDigitPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyType {
    #[serde(flatten)]
    pub company_type: CompanyTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestSeedlingPlantingDepth {
    #[serde(flatten)]
    pub nearest_seedling_planting_depth: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalpedMoundsCount {
    #[serde(flatten)]
    pub scalped_mounds_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccArrivalDate {
    #[serde(flatten)]
    pub fcc_arrival_date: ArrivalDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActingDate {
    #[serde(flatten)]
    pub acting_date: ActingDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataRegeneration {
    #[serde(flatten)]
    pub control_data_regeneration: RegenerationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedBasalArea {
    #[serde(flatten)]
    pub planned_basal_area: BasalAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionGeometry {
    #[serde(flatten)]
    pub decision_geometry: DecisionGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: FirstNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingsRows {
    #[serde(flatten)]
    pub loggings_rows: LoggingsRowsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOfferId {
    #[serde(flatten)]
    pub related_call_for_offer_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub offer_working_site_silviculture_info: OfferWorkingSiteSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureMethodCorrectText {
    #[serde(flatten)]
    pub silviculture_method_correct_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacWorkingRepresentatives {
    #[serde(flatten)]
    pub fac_working_representatives: WorkingRepresentatives,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElevatorCertificate {
    #[serde(flatten)]
    pub elevator_certificate: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingRealizationPractice {
    #[serde(flatten)]
    pub cutting_realization_practice: CuttingTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreDebtCollectionRegister {
    #[serde(flatten)]
    pub pre_debt_collection_register: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SittingMoneyKilometers {
    #[serde(flatten)]
    pub sitting_money_kilometers: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationTimestamp {
    #[serde(flatten)]
    pub location_timestamp: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparatedParcelNumber {
    #[serde(flatten)]
    pub unseparated_parcel_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kilometers {
    #[serde(flatten)]
    pub kilometers: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkSafetyRisks {
    #[serde(flatten)]
    pub work_safety_risks: WorkSafetyRisksType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeState {
    #[serde(flatten)]
    pub change_state: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerShipTypeCode {
    #[serde(flatten)]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyArgument {
    #[serde(flatten)]
    pub subsidy_argument: SubsidyArgumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier1 {
    #[serde(flatten)]
    pub work_code_qualifier1: WorkCodeQualifierType1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsQuantities {
    #[serde(flatten)]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingCode {
    #[serde(flatten)]
    pub working_code: WorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCode {
    #[serde(flatten)]
    pub assortment_code: AssortmentCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManipulationMethod {
    #[serde(flatten)]
    pub manipulation_method: WorkCodeQualifierType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaBusinessAcceptance {
    #[serde(flatten)]
    pub ba_business_acceptance: BusinessAcceptance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompartmentId {
    #[serde(flatten)]
    pub compartment_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCompact {
    #[serde(flatten)]
    pub assortment_compact: AssortmentCompactType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerResourceId {
    #[serde(flatten)]
    pub service_buyer_resource_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TechnicalContactPerson {
    #[serde(flatten)]
    pub technical_contact_person: TechnicalContactPersonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisQualificationPercentageTotal {
    #[serde(flatten)]
    pub dis_qualification_percentage_total: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status1 {
    #[serde(flatten)]
    pub status1: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectReferenceType {
    #[serde(flatten)]
    pub object_reference_type: ForestCentreMessageReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalEstimation {
    #[serde(flatten)]
    pub total_estimation: VirtaTotalEstimationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorCode {
    #[serde(flatten)]
    pub error_code: String25Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoReplyCodeType {
    #[serde(flatten)]
    pub co_reply_code_type: ReplyCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingIncome {
    #[serde(flatten)]
    pub cutting_income: CuttingIncomeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedResource {
    #[serde(flatten)]
    pub planned_resource: PlannedResourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    #[serde(flatten)]
    pub position: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDamageQualifier {
    #[serde(flatten)]
    pub forest_damage_qualifier: ForestDamageQualifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureRestrictions {
    #[serde(flatten)]
    pub silviculture_restrictions: SilvicultureRestrictionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XsanyURI {
    #[serde(flatten)]
    pub xsany_u_r_i: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentrePayments {
    #[serde(flatten)]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Population {
    #[serde(flatten)]
    pub population: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchType {
    #[serde(flatten)]
    pub ditch_type: DitchTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    #[serde(flatten)]
    pub vehicle: VehicleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionActor {
    #[serde(flatten)]
    pub completion_actor: CompletionActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingOrder {
    #[serde(flatten)]
    pub harvesting_order: HarvestingOrderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandSummary {
    #[serde(flatten)]
    pub tree_stand_summary: TreeStandSummary2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SawlogPercent {
    #[serde(flatten)]
    pub sawlog_percent: SawLogPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationUnitPrice {
    #[serde(flatten)]
    pub application_unit_price: MoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpStemCount {
    #[serde(flatten)]
    pub stump_stem_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: NeljasRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearingDone {
    #[serde(flatten)]
    pub clearing_done: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: MunicipalityNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationCost {
    #[serde(flatten)]
    pub evaluation_cost: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditSoilConditioning {
    #[serde(flatten)]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: FinalAuditerTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestEnergySuitable {
    #[serde(flatten)]
    pub forest_energy_suitable: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machines {
    #[serde(flatten)]
    pub machines: MachinesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingHoursSaturday {
    #[serde(flatten)]
    pub working_hours_saturday: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacEndDate {
    #[serde(flatten)]
    pub fac_end_date: EndDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDate {
    #[serde(flatten)]
    pub control_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkApplicationRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FellingRightDuration {
    #[serde(flatten)]
    pub felling_right_duration: FellingRightDurationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlNo {
    #[serde(flatten)]
    pub control_no: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ParcelNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentStorageVolume {
    #[serde(flatten)]
    pub sent_storage_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: MinimumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Removed {
    #[serde(flatten)]
    pub removed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: StorageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsClassifier {
    #[serde(flatten)]
    pub statistics_classifier: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItem {
    #[serde(flatten)]
    pub list_item: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionTime {
    #[serde(flatten)]
    pub transmission_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseActions {
    #[serde(flatten)]
    pub case_actions: CaseActionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactRequest {
    #[serde(flatten)]
    pub contact_request: ContactRequestType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpirationDate {
    #[serde(flatten)]
    pub expiration_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingDistance {
    #[serde(flatten)]
    pub forwarding_distance: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountPine {
    #[serde(flatten)]
    pub stem_count_pine: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FinancingActApplicationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeripheralCode {
    #[serde(flatten)]
    pub peripheral_code: PeripheralCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccForestCentreMessageReference {
    #[serde(flatten)]
    pub fcc_forest_centre_message_reference: ForestCentreMessageReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WideCertificationSystem {
    #[serde(flatten)]
    pub wide_certification_system: WideCertificationSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalAreaPine {
    #[serde(flatten)]
    pub basal_area_pine: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitCostArea {
    #[serde(flatten)]
    pub unit_cost_area: VirtaSumTableAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: IdentifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareGroupOfTrees {
    #[serde(flatten)]
    pub spare_group_of_trees: SpareTreesByCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasisListItem {
    #[serde(flatten)]
    pub fee_basis_list_item: FeebasisListItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationDate {
    #[serde(flatten)]
    pub notification_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellersLogs {
    #[serde(flatten)]
    pub sellers_logs: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentsChanges {
    #[serde(flatten)]
    pub assortments_changes: AssortmentsChangesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: LoppuHetkiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasuringDeviceVersion {
    #[serde(flatten)]
    pub measuring_device_version: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    #[serde(flatten)]
    pub organization: OrganizationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandVehiclePathTooDeepPercentage {
    #[serde(flatten)]
    pub stand_vehicle_path_too_deep_percentage: PercentWithFraction1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureContractNumber {
    #[serde(flatten)]
    pub silviculture_contract_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizationTotalAuditText {
    #[serde(flatten)]
    pub fertilization_total_audit_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantPositionErrorCount {
    #[serde(flatten)]
    pub plant_position_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accepted {
    #[serde(flatten)]
    pub accepted: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    #[serde(flatten)]
    pub date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsMapSymbol {
    #[serde(flatten)]
    pub ms_map_symbol: MapSymbol,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleTransformation {
    #[serde(flatten)]
    pub scale_transformation: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalInformation {
    #[serde(flatten)]
    pub additional_information: AdditionalInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemsNoticed {
    #[serde(flatten)]
    pub water_economy_systems_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacUpdatePreviousMessage {
    #[serde(flatten)]
    pub fac_update_previous_message: UpdatePreviousMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeDescription {
    #[serde(flatten)]
    pub cost_type_description: CostTypeDescriptionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessage {
    #[serde(flatten)]
    pub status_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: VirtaHarvestingSeasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttorneyReceivesPayment {
    #[serde(flatten)]
    pub attorney_receives_payment: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjects {
    #[serde(flatten)]
    pub parent_objects: ParentObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedWorkAmount {
    #[serde(flatten)]
    pub completed_work_amount: AmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_wood_trade_info: CallForOfferWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootDamages {
    #[serde(flatten)]
    pub root_damages: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommercialName {
    #[serde(flatten)]
    pub commercial_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetBasalArea {
    #[serde(flatten)]
    pub target_basal_area: BasalAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentPermissionDate {
    #[serde(flatten)]
    pub payment_permission_date: PaymentPermissionDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtinguisherVerificationDate {
    #[serde(flatten)]
    pub extinguisher_verification_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxDebt {
    #[serde(flatten)]
    pub tax_debt: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {
    #[serde(flatten)]
    pub tree: TreeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionBasedOnStoniness {
    #[serde(flatten)]
    pub restriction_based_on_stoniness: RestrictionBasedOnStoninessType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSignData {
    #[serde(flatten)]
    pub harvesting_sign_data: HarvestingSignDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: DiameterType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactPersonInFinland {
    #[serde(flatten)]
    pub contact_person_in_finland: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: RestrictionCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionHandler {
    #[serde(flatten)]
    pub decision_handler: DecisionHandlerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalTidinessNoticed {
    #[serde(flatten)]
    pub environmental_tidiness_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestSeedlingSoilCompressed {
    #[serde(flatten)]
    pub nearest_seedling_soil_compressed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalMessageType {
    #[serde(flatten)]
    pub original_message_type: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessSender {
    #[serde(flatten)]
    pub business_sender: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationDate {
    #[serde(flatten)]
    pub operation_date: DateMmDdYyyyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificationSystem {
    #[serde(flatten)]
    pub certification_system: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status2 {
    #[serde(flatten)]
    pub status2: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinProposalYear {
    #[serde(flatten)]
    pub min_proposal_year: MinProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: AreaDecreaseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareOfOwnerShip {
    #[serde(flatten)]
    pub share_of_owner_ship: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchDepthWidthAndDrainageEffectText {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorId {
    #[serde(flatten)]
    pub operator_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterSilverBirch {
    #[serde(flatten)]
    pub mean_diameter_silver_birch: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassCollectionDoneText {
    #[serde(flatten)]
    pub bio_mass_collection_done_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalSawlogVolume {
    #[serde(flatten)]
    pub total_sawlog_volume: SawLogVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Photographer {
    #[serde(flatten)]
    pub photographer: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundWoodSalesRows {
    #[serde(flatten)]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryCode {
    #[serde(flatten)]
    pub country_code: ISO3166char2CountryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteGeometry {
    #[serde(flatten)]
    pub working_site_geometry: LocatedSpecialFeature2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacElectronicNotification {
    #[serde(flatten)]
    pub fac_electronic_notification: ElectronicNotification,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetectedEnergyWood {
    #[serde(flatten)]
    pub detected_energy_wood: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalibrationUseLog {
    #[serde(flatten)]
    pub calibration_use_log: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationOtherSubjects {
    #[serde(flatten)]
    pub financing_act_application_other_subjects: FinancingActApplicationOtherSubjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PricingMethod {
    #[serde(flatten)]
    pub pricing_method: PricingMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingArea {
    #[serde(flatten)]
    pub processing_area: ProcessingAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmimecontentType {
    #[serde(flatten)]
    pub xmimecontent_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsQuantity {
    #[serde(flatten)]
    pub statistics_quantity: StatisticsQuantityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityAttachments {
    #[serde(flatten)]
    pub quality_attachments: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadStructureDrainageText {
    #[serde(flatten)]
    pub road_structure_drainage_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: HarvestingStemTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditsList {
    #[serde(flatten)]
    pub audits_list: AuditsListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogM3sum {
    #[serde(flatten)]
    pub log_m3sum: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterSections {
    #[serde(flatten)]
    pub diameter_sections: DiameterSectionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearingEstimation {
    #[serde(flatten)]
    pub clearing_estimation: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyElementNS {
    #[serde(flatten)]
    pub key_element_n_s: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootDamagePercentage {
    #[serde(flatten)]
    pub root_damage_percentage: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingAccuracySign {
    #[serde(flatten)]
    pub cutting_accuracy_sign: String5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase {
    #[serde(flatten)]
    pub phase: VirtaPhaseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionData {
    #[serde(flatten)]
    pub restriction_data: RestrictionDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadRange {
    #[serde(flatten)]
    pub load_range: LoadRangeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ICEName {
    #[serde(flatten)]
    pub i_c_e_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionOtherSubjects {
    #[serde(flatten)]
    pub financing_act_completion_other_subjects: FinancingActCompletionOtherSubjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalMessage {
    #[serde(flatten)]
    pub original_message: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActApplicationStands {
    #[serde(flatten)]
    pub fac_financing_act_application_stands: FinancingActApplicationStands,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedStemCount {
    #[serde(flatten)]
    pub seed_stem_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetAmount {
    #[serde(flatten)]
    pub target_amount: AmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoadUnitInvoiced {
    #[serde(flatten)]
    pub work_load_unit_invoiced: WorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleData {
    #[serde(flatten)]
    pub scale_data: ScaleDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemNoticedText {
    #[serde(flatten)]
    pub water_economy_system_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReRealEstatesWithOwnersInformationType2 {
    #[serde(flatten)]
    pub re_real_estates_with_owners_information_type2: RealEstatesWithOwnersInformationType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendNotificationsAlways {
    #[serde(flatten)]
    pub send_notifications_always: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingAccessibility {
    #[serde(flatten)]
    pub logging_accessibility: HarvestingAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsOfProject {
    #[serde(flatten)]
    pub parts_of_project: PartsOfProjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccCompletionDeclarationNumber {
    #[serde(flatten)]
    pub fcc_completion_declaration_number: CompletionDeclarationNumber,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson2 {
    #[serde(flatten)]
    pub extra_person2: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationDeliveryDueDate {
    #[serde(flatten)]
    pub completion_declaration_delivery_due_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: IBANTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Services {
    #[serde(flatten)]
    pub services: ServicesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: StanfordTreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacEstimatedEndDate {
    #[serde(flatten)]
    pub fac_estimated_end_date: EstimatedEndDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreventionSubstance {
    #[serde(flatten)]
    pub prevention_substance: PreventionSubstanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyer {
    #[serde(flatten)]
    pub service_buyer: ServiceBuyerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: TurningPointClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFileName {
    #[serde(flatten)]
    pub document_file_name: DocumentFileNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalPrice {
    #[serde(flatten)]
    pub total_price: TotalPriceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwsOWorkingSite {
    #[serde(flatten)]
    pub ows_o_working_site: OWorkingSite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoOtherHabitatCodeType {
    #[serde(flatten)]
    pub co_other_habitat_code_type: OtherHabitatCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingRelated {
    #[serde(flatten)]
    pub cutting_related: CuttingRelatedType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkGrouMember {
    #[serde(flatten)]
    pub work_grou_member: WorkGrouMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubDiameter {
    #[serde(flatten)]
    pub stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XsgYear {
    #[serde(flatten)]
    pub xsg_year: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute1 {
    #[serde(flatten)]
    pub attribute1: WorkCodeQualifierType1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetPartStatus {
    #[serde(flatten)]
    pub target_part_status: VirtaTargetPartStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsedPricingMethod {
    #[serde(flatten)]
    pub used_pricing_method: UsedPricingMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessageFreeText {
    #[serde(flatten)]
    pub common_message_free_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentMatrixVolumes {
    #[serde(flatten)]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subsidies {
    #[serde(flatten)]
    pub subsidies: SubsidiesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestCultivatedSpotLength {
    #[serde(flatten)]
    pub nearest_cultivated_spot_length: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Justifications {
    #[serde(flatten)]
    pub justifications: JustificationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: ViidesRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyCode {
    #[serde(flatten)]
    pub reply_code: ReplyCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsibleActor {
    #[serde(flatten)]
    pub responsible_actor: ResponsibleActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionDate {
    #[serde(flatten)]
    pub decision_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    #[serde(flatten)]
    pub name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: TreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasePartNumber {
    #[serde(flatten)]
    pub base_part_number: VirtaPartNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBH {
    #[serde(flatten)]
    pub dbh: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoTimeStamp {
    #[serde(flatten)]
    pub co_time_stamp: TimeStamp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseNumber {
    #[serde(flatten)]
    pub case_number: FinancingActNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisQualificationPercentage {
    #[serde(flatten)]
    pub dis_qualification_percentage: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccDocuments {
    #[serde(flatten)]
    pub fcc_documents: Documents,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDiameter {
    #[serde(flatten)]
    pub tree_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteForwardedProduction {
    #[serde(flatten)]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightClass {
    #[serde(flatten)]
    pub height_class: HeightClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandRootDamagesPercentage {
    #[serde(flatten)]
    pub stand_root_damages_percentage: PercentWithFraction1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActNumber {
    #[serde(flatten)]
    pub fac_financing_act_number: FinancingActNumber,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlCutting {
    #[serde(flatten)]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacCustomerReference {
    #[serde(flatten)]
    pub fac_customer_reference: CustomerReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageClass {
    #[serde(flatten)]
    pub storage_class: StorageDryingClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientApplicationId {
    #[serde(flatten)]
    pub client_application_id: ClientApplicationIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelYear {
    #[serde(flatten)]
    pub model_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    #[serde(flatten)]
    pub language: LanguageCode1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Load {
    #[serde(flatten)]
    pub load: LoadType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningQualityText {
    #[serde(flatten)]
    pub soil_conditioning_quality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrainStorageAsInstructed {
    #[serde(flatten)]
    pub drain_storage_as_instructed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecificationText {
    #[serde(flatten)]
    pub specification_text: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: BaseCompactStandBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditionResource {
    #[serde(flatten)]
    pub audition_resource: AuditionResourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: MeanType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Questions {
    #[serde(flatten)]
    pub questions: AuditsListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemId {
    #[serde(flatten)]
    pub stem_id: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDataUpdate {
    #[serde(flatten)]
    pub forest_data_update: ForestDataUpdateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeHealthCare {
    #[serde(flatten)]
    pub employee_health_care: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Envelope {
    #[serde(flatten)]
    pub envelope: EnvelopeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandLocatedOnIsland {
    #[serde(flatten)]
    pub stand_located_on_island: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchingBreaks {
    #[serde(flatten)]
    pub ditching_breaks: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotDamagedCount {
    #[serde(flatten)]
    pub not_damaged_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interrupted {
    #[serde(flatten)]
    pub interrupted: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionAvgDiameter {
    #[serde(flatten)]
    pub reduction_avg_diameter: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinSizeShapeAndFunctionalityText {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalCode {
    #[serde(flatten)]
    pub additional_code: AdditionalCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchCleaningBreakLocation {
    #[serde(flatten)]
    pub ditch_cleaning_break_location: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingsRow {
    #[serde(flatten)]
    pub loggings_row: LoggingsRowType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActorId {
    #[serde(flatten)]
    pub actor_id: IdStringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDate {
    #[serde(flatten)]
    pub completion_date: CompletionDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendTimestamp {
    #[serde(flatten)]
    pub send_timestamp: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDataAndSubsidy {
    #[serde(flatten)]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfulfilledArea {
    #[serde(flatten)]
    pub unfulfilled_area: PolygonOrMultiPolygon2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDescription {
    #[serde(flatten)]
    pub document_description: DocumentDescriptionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationSoilPreparationOperation {
    #[serde(flatten)]
    pub declaration_soil_preparation_operation: DeclarationSoilPreparationOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class2DamageCount {
    #[serde(flatten)]
    pub class2_damage_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCount {
    #[serde(flatten)]
    pub image_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TssTreeStandSummary {
    #[serde(flatten)]
    pub tss_tree_stand_summary: TreeStandSummary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainDamageOutsideStand {
    #[serde(flatten)]
    pub terrain_damage_outside_stand: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasisList {
    #[serde(flatten)]
    pub fee_basis_list: FeeBasisListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataId {
    #[serde(flatten)]
    pub data_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status5 {
    #[serde(flatten)]
    pub status5: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Days {
    #[serde(flatten)]
    pub days: DaysType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RadiusReduction {
    #[serde(flatten)]
    pub radius_reduction: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Publicity {
    #[serde(flatten)]
    pub publicity: PublicityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationDeliveringEvaluation {
    #[serde(flatten)]
    pub declaration_delivering_evaluation: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class1DamageCount {
    #[serde(flatten)]
    pub class1_damage_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase2youngCropCount {
    #[serde(flatten)]
    pub phase2young_crop_count: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoGradeCodeType {
    #[serde(flatten)]
    pub co_grade_code_type: GradeCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendDate {
    #[serde(flatten)]
    pub send_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandBasedData {
    #[serde(flatten)]
    pub tree_stand_based_data: TreeStandBasedDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationStandFellingPurpose {
    #[serde(flatten)]
    pub forest_use_declaration_stand_felling_purpose: CuttingPurposeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperativeData {
    #[serde(flatten)]
    pub operative_data: OperativeDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatOperations {
    #[serde(flatten)]
    pub habitat_operations: HabitatOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SumTableArea {
    #[serde(flatten)]
    pub sum_table_area: VirtaSumTableAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Objects {
    #[serde(flatten)]
    pub objects: ControlObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpointProperty {
    #[serde(flatten)]
    pub gmlpoint_property: PointProperty,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorization {
    #[serde(flatten)]
    pub authorization: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDistributionStratum {
    #[serde(flatten)]
    pub stem_distribution_stratum: StemDistributionStratumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    #[serde(flatten)]
    pub id: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingClassifiation {
    #[serde(flatten)]
    pub harvesting_classifiation: VirtaHarvestingClassificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousSameAreaMooseDamageCompensationYear {
    #[serde(flatten)]
    pub previous_same_area_moose_damage_compensation_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemiDry {
    #[serde(flatten)]
    pub semi_dry: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCode {
    #[serde(flatten)]
    pub user_code: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delivered {
    #[serde(flatten)]
    pub delivered: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactMunicipality {
    #[serde(flatten)]
    pub contact_municipality: MunicipalityNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    pub action: ActionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: SyntymaPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemTypeBulks {
    #[serde(flatten)]
    pub stem_type_bulks: StemTypeBulksType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Finished {
    #[serde(flatten)]
    pub finished: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: EdellinenSukuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageSourceCode {
    #[serde(flatten)]
    pub damage_source_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xstoken {
    #[serde(flatten)]
    pub xstoken: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoCompleteStateType {
    #[serde(flatten)]
    pub co_complete_state_type: CompleteStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDiameterClass {
    #[serde(flatten)]
    pub log_diameter_class: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkGrouMembers {
    #[serde(flatten)]
    pub work_grou_members: WorkGrouMembersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDistributionStrata {
    #[serde(flatten)]
    pub stem_distribution_strata: StemDistributionStrataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review2 {
    #[serde(flatten)]
    pub review2: VirtaReviewType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlOverallEvaluationData {
    #[serde(flatten)]
    pub control_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferBusinessSender {
    #[serde(flatten)]
    pub offer_business_sender: OfferBusinessSenderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualitySystems {
    #[serde(flatten)]
    pub quality_systems: QualitySystemsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PulpM3sum {
    #[serde(flatten)]
    pub pulp_m3sum: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BearingCapacityClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionGeometry {
    #[serde(flatten)]
    pub financing_act_completion_geometry: FinancingActGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionAnswerAdditionalText {
    #[serde(flatten)]
    pub question_answer_additional_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: HabitatTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpsLiftedText {
    #[serde(flatten)]
    pub stumps_lifted_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationDescription {
    #[serde(flatten)]
    pub length_calibration_description: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payees {
    #[serde(flatten)]
    pub payees: PayeesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceLocation {
    #[serde(flatten)]
    pub resource_location: ResourceLocationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingBiomass {
    #[serde(flatten)]
    pub remaining_biomass: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoadId {
    #[serde(flatten)]
    pub work_load_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacSentDate {
    #[serde(flatten)]
    pub fac_sent_date: SentDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentDate {
    #[serde(flatten)]
    pub sent_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandFinished {
    #[serde(flatten)]
    pub stand_finished: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyTo {
    #[serde(flatten)]
    pub reply_to: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationMainTreeSpecies {
    #[serde(flatten)]
    pub declaration_main_tree_species: TreeSpeciesConciseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactLocationInformation {
    #[serde(flatten)]
    pub contact_location_information: AlternativeGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xsdate {
    #[serde(flatten)]
    pub xsdate: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VAT {
    #[serde(flatten)]
    pub vat: VATType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningMethodCorrect {
    #[serde(flatten)]
    pub soil_conditioning_method_correct: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvesterModel {
    #[serde(flatten)]
    pub harvester_model: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryObject {
    #[serde(flatten)]
    pub geometry_object: GeometryObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteWorkTime {
    #[serde(flatten)]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayeeAndRealEstate {
    #[serde(flatten)]
    pub payee_and_real_estate: PayeeAndRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizerVolumeOrdered {
    #[serde(flatten)]
    pub fertilizer_volume_ordered: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectNo {
    #[serde(flatten)]
    pub project_no: ProjectNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: ExtendedHabitatCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelNo {
    #[serde(flatten)]
    pub parcel_no: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestReference {
    #[serde(flatten)]
    pub request_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicityOrganizations {
    #[serde(flatten)]
    pub publicity_organizations: OrganizationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisQualificationSign {
    #[serde(flatten)]
    pub dis_qualification_sign: String5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentsChange {
    #[serde(flatten)]
    pub assortments_change: AssortmentChangeDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlledOperationDescription {
    #[serde(flatten)]
    pub controlled_operation_description: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSites {
    #[serde(flatten)]
    pub working_sites: WorkingSitesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SproutForestControlEvaluation {
    #[serde(flatten)]
    pub sprout_forest_control_evaluation: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlLogLength {
    #[serde(flatten)]
    pub control_log_length: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlObjectBasicData {
    #[serde(flatten)]
    pub control_object_basic_data: ControlObjectBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStands {
    #[serde(flatten)]
    pub st_stands: Stands,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDescription {
    #[serde(flatten)]
    pub work_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PilotName {
    #[serde(flatten)]
    pub pilot_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestRealizationData {
    #[serde(flatten)]
    pub forest_realization_data: ForestRealizationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    #[serde(flatten)]
    pub header: HeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestedTrees {
    #[serde(flatten)]
    pub harvested_trees: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: YesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStrata {
    #[serde(flatten)]
    pub tree_strata: TreeStrata2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpBiomass {
    #[serde(flatten)]
    pub stump_biomass: StumpBiomassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LandScapingText {
    #[serde(flatten)]
    pub land_scaping_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: WorkCodeQualifierType1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkCompletion {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    #[serde(flatten)]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTimeStart {
    #[serde(flatten)]
    pub operation_time_start: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: NormalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockingWIthSeedlings {
    #[serde(flatten)]
    pub stocking_w_ith_seedlings: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseMode {
    #[serde(flatten)]
    pub purchase_mode: PurchaseModeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsuOperations {
    #[serde(flatten)]
    pub osu_operations: Operations,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingAccessibility {
    #[serde(flatten)]
    pub harvesting_accessibility: HarvestingAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceIdMJ {
    #[serde(flatten)]
    pub resource_id_m_j: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: RoadUsingRightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestHaulageAccessibility {
    #[serde(flatten)]
    pub forest_haulage_accessibility: HarvestingAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingType {
    #[serde(flatten)]
    pub financing_type: FinancingActFinancingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: KuolemaPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootRotControlEvaluation {
    #[serde(flatten)]
    pub root_rot_control_evaluation: VirtaRootRotControlEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: PaayksikkoNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoChangeTime {
    #[serde(flatten)]
    pub co_change_time: ChangeTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsActive {
    #[serde(flatten)]
    pub is_active: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoTreeSpeciesConciseType {
    #[serde(flatten)]
    pub co_tree_species_concise_type: TreeSpeciesConciseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPersonText {
    #[serde(flatten)]
    pub extra_person_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjectId {
    #[serde(flatten)]
    pub parent_object_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionType {
    #[serde(flatten)]
    pub selection_type: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandSilvicultureInfo {
    #[serde(flatten)]
    pub stand_silviculture_info: StandSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserName {
    #[serde(flatten)]
    pub user_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeHeight {
    #[serde(flatten)]
    pub tree_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlLogVolume {
    #[serde(flatten)]
    pub control_log_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FudrForestUseDeclarationReferences {
    #[serde(flatten)]
    pub fudr_forest_use_declaration_references: ForestUseDeclarationReferences,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manufacturer {
    #[serde(flatten)]
    pub manufacturer: MachineManufacturerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: SpecialPermissionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organizations {
    #[serde(flatten)]
    pub organizations: OrganizationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreventionCompleted {
    #[serde(flatten)]
    pub prevention_completed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingBiomassText {
    #[serde(flatten)]
    pub remaining_biomass_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InTerrain {
    #[serde(flatten)]
    pub in_terrain: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XsNCName {
    #[serde(flatten)]
    pub xs_n_c_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayeeId {
    #[serde(flatten)]
    pub payee_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeUnit {
    #[serde(flatten)]
    pub fee_unit: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: PankkitiliTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditIdentifierVersion {
    #[serde(flatten)]
    pub final_audit_identifier_version: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteHarvestedProduction {
    #[serde(flatten)]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingRepresentative {
    #[serde(flatten)]
    pub working_representative: WorkingRepresentativeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RwsRoundWoodSalesRows {
    #[serde(flatten)]
    pub rws_round_wood_sales_rows: RoundWoodSalesRows,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemTypeVolume {
    #[serde(flatten)]
    pub stem_type_volume: StemTypeVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestStatisticsData {
    #[serde(flatten)]
    pub forest_statistics_data: ForestStatisticsDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: MeasurementMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasuringDeviceLastControl {
    #[serde(flatten)]
    pub measuring_device_last_control: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: BufferDistanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderScaleModelYear {
    #[serde(flatten)]
    pub loader_scale_model_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingSustainableForestry {
    #[serde(flatten)]
    pub financing_sustainable_forestry: FinancingSustainableForestryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestObjectData {
    #[serde(flatten)]
    pub forest_object_data: ForestObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    #[serde(flatten)]
    pub attributes: AttributesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentreControlDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectBasicData {
    #[serde(flatten)]
    pub object_basic_data: ObjectBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silviculture {
    #[serde(flatten)]
    pub silviculture: SilvicultureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute3 {
    #[serde(flatten)]
    pub attribute3: WorkCodeQualifierType3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportCompany {
    #[serde(flatten)]
    pub transport_company: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: HuoltosuhdeTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThinningTooExcessiveCount {
    #[serde(flatten)]
    pub thinning_too_excessive_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteHarvestingQualityControlManual {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentType {
    #[serde(flatten)]
    pub payment_type: PaymentTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingAccuracy {
    #[serde(flatten)]
    pub cutting_accuracy: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirchDecimal {
    #[serde(flatten)]
    pub birch_decimal: VirtaTreeDecimalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamage {
    #[serde(flatten)]
    pub previous_moose_damage: PreviousMooseDamageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: ShapeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XsgMonthDay {
    #[serde(flatten)]
    pub xsg_month_day: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadTreeStratum {
    #[serde(flatten)]
    pub dead_tree_stratum: DeadTreeStratumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDeclarationReference {
    #[serde(flatten)]
    pub moose_damage_declaration_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier3 {
    #[serde(flatten)]
    pub work_code_qualifier3: WorkCodeQualifierType3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad1 {
    #[serde(flatten)]
    pub work_load1: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantEvaluation {
    #[serde(flatten)]
    pub plant_evaluation: VirtaPlantEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessRightsInfo {
    #[serde(flatten)]
    pub access_rights_info: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub tree_damage_outside_stand_evaluation: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDisplayId {
    #[serde(flatten)]
    pub storage_display_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceItem {
    #[serde(flatten)]
    pub price_item: PriceItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringObjectProtectionOperationsData {
    #[serde(flatten)]
    pub self_monitoring_object_protection_operations_data: SelfMonitoringObjectProtectionOperationsDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: ScaleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelinationObjectOrderId {
    #[serde(flatten)]
    pub delination_object_order_id: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreeGroup {
    #[serde(flatten)]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: WorkCodeGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSFValidity {
    #[serde(flatten)]
    pub f_s_f_validity: FSFValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationCode {
    #[serde(flatten)]
    pub operation_code: ObjectProtectionOperationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingVolume {
    #[serde(flatten)]
    pub cutting_volume: CuttingVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandStemDamagesPercentage {
    #[serde(flatten)]
    pub stand_stem_damages_percentage: PercentWithFraction1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountTarget {
    #[serde(flatten)]
    pub stem_count_target: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeafBiomass {
    #[serde(flatten)]
    pub leaf_biomass: LeafBiomassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDryingClass {
    #[serde(flatten)]
    pub storage_drying_class: StorageDryingClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldDestinationStorage {
    #[serde(flatten)]
    pub old_destination_storage: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeText {
    #[serde(flatten)]
    pub fee_text: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccCustomerReference {
    #[serde(flatten)]
    pub fcc_customer_reference: CustomerReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: CallForOfferTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantSiteCountSummary {
    #[serde(flatten)]
    pub plant_site_count_summary: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgDominantHeight {
    #[serde(flatten)]
    pub stand_avg_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReRealEstates {
    #[serde(flatten)]
    pub re_real_estates: RealEstates,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationAdjustmentButtLog {
    #[serde(flatten)]
    pub diameter_calibration_adjustment_butt_log: Integer3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalVolume {
    #[serde(flatten)]
    pub total_volume: VolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesSummary {
    #[serde(flatten)]
    pub tree_species_summary: TreeSpeciesSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class3damageCount {
    #[serde(flatten)]
    pub class3damage_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerRepresentativePerson {
    #[serde(flatten)]
    pub owner_representative_person: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationYear {
    #[serde(flatten)]
    pub operation_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncedLength {
    #[serde(flatten)]
    pub announced_length: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlReferenceType {
    #[serde(flatten)]
    pub control_reference_type: ForestCentreMessageReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassCollectionDone {
    #[serde(flatten)]
    pub bio_mass_collection_done: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    #[serde(flatten)]
    pub log: LogDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreinformDetails {
    #[serde(flatten)]
    pub preinform_details: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountOtherTreeSpecies {
    #[serde(flatten)]
    pub stem_count_other_tree_species: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HopperNumber {
    #[serde(flatten)]
    pub hopper_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: ValtiotunnusKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: MunicipalityNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateSeedlingsToWorkingSite {
    #[serde(flatten)]
    pub date_seedlings_to_working_site: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: ThreeDigitPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(flatten)]
    pub document: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialSupplier {
    #[serde(flatten)]
    pub material_supplier: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: WideDevelopmentClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataSpecialFeature {
    #[serde(flatten)]
    pub control_data_special_feature: ControlDataSpecialFeatureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkCompletionRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingSuitable {
    #[serde(flatten)]
    pub stump_lifting_suitable: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubMeanDiameter {
    #[serde(flatten)]
    pub stub_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogCount {
    #[serde(flatten)]
    pub log_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingPlannerLiability {
    #[serde(flatten)]
    pub cutting_planner_liability: CuttingPlannerLiabilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardedVolume {
    #[serde(flatten)]
    pub forwarded_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLotInformationTypeDescription {
    #[serde(flatten)]
    pub wood_lot_information_type_description: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamages {
    #[serde(flatten)]
    pub previous_moose_damages: PreviousMooseDamagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterSection {
    #[serde(flatten)]
    pub diameter_section: SectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedOperationChains {
    #[serde(flatten)]
    pub planned_operation_chains: PlannedOperationChainsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationRows {
    #[serde(flatten)]
    pub operation_rows: OperationRowsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson4 {
    #[serde(flatten)]
    pub extra_person4: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerationType {
    #[serde(flatten)]
    pub regeneration_type: VirtaRegenerationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingRightsOwner {
    #[serde(flatten)]
    pub cutting_rights_owner: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageData {
    #[serde(flatten)]
    pub damage_data: DamageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainWorkCode {
    #[serde(flatten)]
    pub main_work_code: MainWorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanner {
    #[serde(flatten)]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderConfirmation {
    #[serde(flatten)]
    pub order_confirmation: OrderConfirmationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamName {
    #[serde(flatten)]
    pub team_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTransactionDescription {
    #[serde(flatten)]
    pub payment_transaction_description: PaymentTransactionDescriptionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringObjectData {
    #[serde(flatten)]
    pub self_monitoring_object_data: SelfMonitoringObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInfo {
    #[serde(flatten)]
    pub contract_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogKey {
    #[serde(flatten)]
    pub log_key: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: IdStringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetParts {
    #[serde(flatten)]
    pub target_parts: TargetPartsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingHoursBusinessDay {
    #[serde(flatten)]
    pub working_hours_business_day: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAreaRequired {
    #[serde(flatten)]
    pub test_area_required: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_diameter_other_tree_species: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurfaceRunoffFieldGroundIsUnbrokenText {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseRealEstates {
    #[serde(flatten)]
    pub base_real_estates: BaseRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeePensionCertificate {
    #[serde(flatten)]
    pub employee_pension_certificate: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterHardWood {
    #[serde(flatten)]
    pub mean_diameter_hard_wood: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirthDate {
    #[serde(flatten)]
    pub birth_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeNumber {
    #[serde(flatten)]
    pub tree_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: KuudesRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsCallForOfferWorkingSites {
    #[serde(flatten)]
    pub ws_call_for_offer_working_sites: CallForOfferWorkingSites,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnWeight {
    #[serde(flatten)]
    pub own_weight: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceMatrix {
    #[serde(flatten)]
    pub price_matrix: PriceMatrixType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationStand {
    #[serde(flatten)]
    pub financing_act_application_stand: FinancingActApplicationStandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureRestrictionEnds {
    #[serde(flatten)]
    pub silviculture_restriction_ends: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operations {
    #[serde(flatten)]
    pub operations: OperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: BICKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsAssortmentCompactClasses {
    #[serde(flatten)]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentCleanlinessNoticedText {
    #[serde(flatten)]
    pub environment_cleanliness_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivedCompensation {
    #[serde(flatten)]
    pub received_compensation: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    #[serde(flatten)]
    pub target: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: SukupuoliKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreDecision {
    #[serde(flatten)]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthOfDitchDiggedDuringSoilPreparation {
    #[serde(flatten)]
    pub length_of_ditch_digged_during_soil_preparation: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAcceptanceId {
    #[serde(flatten)]
    pub business_acceptance_id: BusinessAcceptanceIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlEvaluation {
    #[serde(flatten)]
    pub control_evaluation: ControlEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub offer_working_site_wood_trade_info: OfferWorkingSiteWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: KieliKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Density {
    #[serde(flatten)]
    pub density: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: PostilokerolyhenneTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurningPlaceText {
    #[serde(flatten)]
    pub turning_place_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingNotification {
    #[serde(flatten)]
    pub forwarding_notification: ForwardingNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    #[serde(flatten)]
    pub products: ProductsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyDataSet {
    #[serde(flatten)]
    pub forest_property_data_set: ForestPropertyDataSetType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthMin {
    #[serde(flatten)]
    pub length_min: LengthMinType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub goal_amount_of_soil_preparation_spot: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryId {
    #[serde(flatten)]
    pub geometry_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClodMoistureErrorCount {
    #[serde(flatten)]
    pub clod_moisture_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellersLogsInfo {
    #[serde(flatten)]
    pub sellers_logs_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedWorkingTimeConsumption {
    #[serde(flatten)]
    pub estimated_working_time_consumption: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    #[serde(flatten)]
    pub service: OrganizationServiceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstateOwner {
    #[serde(flatten)]
    pub estate_owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: SellersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    #[serde(flatten)]
    pub resources: ResourcesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantLocationErrorCount {
    #[serde(flatten)]
    pub plant_location_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedAmountUnit {
    #[serde(flatten)]
    pub decided_amount_unit: DecidedAmountUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractor {
    #[serde(flatten)]
    pub sub_contractor: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantStorageManagement {
    #[serde(flatten)]
    pub plant_storage_management: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActApplicationReference {
    #[serde(flatten)]
    pub fac_financing_act_application_reference: FinancingActApplicationReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlObjectData {
    #[serde(flatten)]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecificationCode {
    #[serde(flatten)]
    pub specification_code: SpecificationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RectificationDemand {
    #[serde(flatten)]
    pub rectification_demand: String5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TravelStartTime {
    #[serde(flatten)]
    pub travel_start_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestOwnerGroup {
    #[serde(flatten)]
    pub forest_owner_group: ForestOwnerGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallWoodRemovalClass {
    #[serde(flatten)]
    pub small_wood_removal_class: SmallWoodRemovalClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteSilvicultureText {
    #[serde(flatten)]
    pub offer_working_site_silviculture_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    #[serde(flatten)]
    pub operation: OperationDefType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccForestUseDeclarationNumber {
    #[serde(flatten)]
    pub fcc_forest_use_declaration_number: ForestUseDeclarationNumber,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_status: OrderStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationDescription {
    #[serde(flatten)]
    pub diameter_calibration_description: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncementId {
    #[serde(flatten)]
    pub announcement_id: AnnouncementIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActRealEstates {
    #[serde(flatten)]
    pub financing_act_real_estates: FinancingActRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacEstimatedStartDate {
    #[serde(flatten)]
    pub fac_estimated_start_date: EstimatedStartDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeparateSpareTrees {
    #[serde(flatten)]
    pub separate_spare_trees: SpareTreesByCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreClearingExecutionTime {
    #[serde(flatten)]
    pub pre_clearing_execution_time: FSFValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad1Unit {
    #[serde(flatten)]
    pub work_load1_unit: WorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesCurrent {
    #[serde(flatten)]
    pub stand_trees_current: StandTreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipationPercentage {
    #[serde(flatten)]
    pub participation_percentage: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightSilverBirch {
    #[serde(flatten)]
    pub mean_height_silver_birch: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Targets {
    #[serde(flatten)]
    pub targets: TargetsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    #[serde(flatten)]
    pub label: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payments {
    #[serde(flatten)]
    pub payments: PaymentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: PostilokeroTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyMode {
    #[serde(flatten)]
    pub company_mode: CompanyModeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageVolume {
    #[serde(flatten)]
    pub average_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SprucePulp {
    #[serde(flatten)]
    pub spruce_pulp: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeDitchesNotDiggedAsMentionedInOrderText {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryName {
    #[serde(flatten)]
    pub delivery_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TravelEndTime {
    #[serde(flatten)]
    pub travel_end_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantedSeedlingCountInNotCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_not_cultivated_spots: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTreeReduction {
    #[serde(flatten)]
    pub operation_tree_reduction: OperationTreeReductionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionDate {
    #[serde(flatten)]
    pub inspection_date: DateMmDdYyyyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: WeibullType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerContactInformation {
    #[serde(flatten)]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suggestion {
    #[serde(flatten)]
    pub suggestion: VirtaSuggestionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_height_other_tree_species: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoragesProposalForestHaulageDistances {
    #[serde(flatten)]
    pub storages_proposal_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentClassCode {
    #[serde(flatten)]
    pub assortment_class_code: AssortmentClassCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StSpecialFeatures {
    #[serde(flatten)]
    pub st_special_features: SpecialFeatures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct District {
    #[serde(flatten)]
    pub district: ThinningDistrictType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoChangeState {
    #[serde(flatten)]
    pub co_change_state: ChangeState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeName {
    #[serde(flatten)]
    pub attribute_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CurrencyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinDiameter {
    #[serde(flatten)]
    pub min_diameter: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentifierValue {
    #[serde(flatten)]
    pub identifier_value: IdentifierValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonPersonificationId {
    #[serde(flatten)]
    pub non_personification_id: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contractors {
    #[serde(flatten)]
    pub contractors: ContractorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(flatten)]
    pub version: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CfowsCFOWorkingSite {
    #[serde(flatten)]
    pub cfows_c_f_o_working_site: CFOWorkingSite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedUnitPrice {
    #[serde(flatten)]
    pub decided_unit_price: DecidedUnitPriceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDistribution {
    #[serde(flatten)]
    pub stem_distribution: StemDistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlSilviculture {
    #[serde(flatten)]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: WorkingSitePlanningStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: ShapeGammaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cuttings {
    #[serde(flatten)]
    pub cuttings: CuttingsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: SceneryWorkPermissionNeededType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BulgeHeight {
    #[serde(flatten)]
    pub bulge_height: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousBlockState {
    #[serde(flatten)]
    pub previous_block_state: PreviousBlockStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: WorkingSitePriorityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamageEvaluationMunicipality {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_municipality: MunicipalityNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureMethodCorrect {
    #[serde(flatten)]
    pub silviculture_method_correct: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xsinteger {
    #[serde(flatten)]
    pub xsinteger: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationActor {
    #[serde(flatten)]
    pub completion_declaration_actor: PayeeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeIntervalForMeasuringSamplePlot {
    #[serde(flatten)]
    pub time_interval_for_measuring_sample_plot: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileType {
    #[serde(flatten)]
    pub file_type: FileTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerOfAttorneyDocument {
    #[serde(flatten)]
    pub power_of_attorney_document: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadTreeType {
    #[serde(flatten)]
    pub dead_tree_type: DeadTreeTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestCultivatedSpotWidth {
    #[serde(flatten)]
    pub nearest_cultivated_spot_width: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksDueDate {
    #[serde(flatten)]
    pub works_due_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavatorMovingAndTreeDamagesText {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HavesterModelYear {
    #[serde(flatten)]
    pub havester_model_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationActors {
    #[serde(flatten)]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringEvaluation {
    #[serde(flatten)]
    pub self_monitoring_evaluation: SelfMonitoringEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CumulativePointType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacCompletionDeclarationReference {
    #[serde(flatten)]
    pub fac_completion_declaration_reference: CompletionDeclarationReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsstStands {
    #[serde(flatten)]
    pub wsst_stands: Stands,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UseCases {
    #[serde(flatten)]
    pub use_cases: ForestDataUpdateUseCasesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationNumber {
    #[serde(flatten)]
    pub completion_declaration_number: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentReference {
    #[serde(flatten)]
    pub payment_reference: PaymentsReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StratumOrigin {
    #[serde(flatten)]
    pub stratum_origin: SeedlingOriginType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationId {
    #[serde(flatten)]
    pub registration_id: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizationType {
    #[serde(flatten)]
    pub fertilization_type: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathMeanWidth {
    #[serde(flatten)]
    pub vehicle_path_mean_width: Decimal5_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDamagePercentage {
    #[serde(flatten)]
    pub stem_damage_percentage: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurfaceDrainingAreasText {
    #[serde(flatten)]
    pub surface_draining_areas_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataWaterSystemProtection {
    #[serde(flatten)]
    pub control_data_water_system_protection: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplierReferenceList {
    #[serde(flatten)]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoHabitatCodeType {
    #[serde(flatten)]
    pub co_habitat_code_type: HabitatCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceOrOtherCompensation {
    #[serde(flatten)]
    pub insurance_or_other_compensation: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsDetectedArea {
    #[serde(flatten)]
    pub parts_detected_area: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteDetails {
    #[serde(flatten)]
    pub call_for_offer_working_site_details: CallForOfferWorkingSiteDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamagedPlantsErrorCount {
    #[serde(flatten)]
    pub damaged_plants_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationDescription {
    #[serde(flatten)]
    pub operation_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sawinghours {
    #[serde(flatten)]
    pub sawinghours: SawinghoursDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessages {
    #[serde(flatten)]
    pub error_messages: ErrorMessagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PineDecimal {
    #[serde(flatten)]
    pub pine_decimal: VirtaTreeDecimalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandVehiclePathWidth {
    #[serde(flatten)]
    pub stand_vehicle_path_width: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipientType {
    #[serde(flatten)]
    pub recipient_type: RecipientTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseDate {
    #[serde(flatten)]
    pub case_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandBasalAreaSummary {
    #[serde(flatten)]
    pub stand_basal_area_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractText {
    #[serde(flatten)]
    pub contract_text: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceNumber {
    #[serde(flatten)]
    pub insurance_number: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataSwampForestManagement {
    #[serde(flatten)]
    pub control_data_swamp_forest_management: ControlDataSwampForestManagementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBaseList {
    #[serde(flatten)]
    pub fee_base_list: FeeBaseListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringEvaluations {
    #[serde(flatten)]
    pub self_monitoring_evaluations: SelfMonitoringEvaluationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueGrowthPercent {
    #[serde(flatten)]
    pub value_growth_percent: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: FinalAuditSpareTreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOffer {
    #[serde(flatten)]
    pub related_call_for_offer: RelatedCallForOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SfFeatureInfo {
    #[serde(flatten)]
    pub sf_feature_info: FeatureInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentYear {
    #[serde(flatten)]
    pub deployment_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestEnergySuitableText {
    #[serde(flatten)]
    pub forest_energy_suitable_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDate {
    #[serde(flatten)]
    pub remove_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsedPricingMethods {
    #[serde(flatten)]
    pub used_pricing_methods: UsedPricingMethodsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATRegistrationDate {
    #[serde(flatten)]
    pub v_a_t_registration_date: VATRegistrationDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notices {
    #[serde(flatten)]
    pub notices: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSite {
    #[serde(flatten)]
    pub working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteKey {
    #[serde(flatten)]
    pub working_site_key: WorkingSiteKeyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsOfferWorkingSites {
    #[serde(flatten)]
    pub ws_offer_working_sites: OfferWorkingSites,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearingDoneText {
    #[serde(flatten)]
    pub clearing_done_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationsToSendWsoInformation {
    #[serde(flatten)]
    pub authorizations_to_send_wso_information: AuthorizationsToSendWsoInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRegenerationOperation {
    #[serde(flatten)]
    pub declaration_regeneration_operation: DeclarationRegenerationOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccContactPerson {
    #[serde(flatten)]
    pub fcc_contact_person: ContactPerson,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerShipType {
    #[serde(flatten)]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightPine {
    #[serde(flatten)]
    pub mean_height_pine: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(flatten)]
    pub message: PayloadType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FinancingActCompletionDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartOfProject {
    #[serde(flatten)]
    pub part_of_project: PartOfProjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaliperId {
    #[serde(flatten)]
    pub caliper_id: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesStratumLeaving {
    #[serde(flatten)]
    pub stand_trees_stratum_leaving: StandTreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoMachineAccessoryCodeType {
    #[serde(flatten)]
    pub co_machine_accessory_code_type: MachineAccessoryCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityData {
    #[serde(flatten)]
    pub accessibility_data: AccessibilityDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XspositiveInteger {
    #[serde(flatten)]
    pub xspositive_integer: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingStumpCount {
    #[serde(flatten)]
    pub remaining_stump_count: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesCuttingVolume {
    #[serde(flatten)]
    pub stand_trees_cutting_volume: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringType {
    #[serde(flatten)]
    pub self_monitoring_type: SelfMonitoringTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: Decimal5_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: SilvicultureRestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasSupport {
    #[serde(flatten)]
    pub has_support: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectKeys {
    #[serde(flatten)]
    pub object_keys: ObjectKeysType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherRemarks {
    #[serde(flatten)]
    pub other_remarks: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationAdjustment {
    #[serde(flatten)]
    pub diameter_calibration_adjustment: Integer3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetId {
    #[serde(flatten)]
    pub target_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplyPoint {
    #[serde(flatten)]
    pub supply_point: SupplyPointType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestroyedCuttingValue {
    #[serde(flatten)]
    pub destroyed_cutting_value: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WtcoDocuments {
    #[serde(flatten)]
    pub wtco_documents: Documents,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: ShapeBetaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvesterId {
    #[serde(flatten)]
    pub harvester_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectNumber {
    #[serde(flatten)]
    pub object_number: ObjectNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: SahkoinenAsiointiTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class4damageCount {
    #[serde(flatten)]
    pub class4damage_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRegenerationCommitment {
    #[serde(flatten)]
    pub declaration_regeneration_commitment: RegenerationCommitmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStands {
    #[serde(flatten)]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountAccounted {
    #[serde(flatten)]
    pub amount_accounted: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanStumpDiameter {
    #[serde(flatten)]
    pub mean_stump_diameter: DiameterType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: LajiTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingControlRequired {
    #[serde(flatten)]
    pub cutting_control_required: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlMultiPolygon {
    #[serde(flatten)]
    pub gml_multi_polygon: MultiPolygon,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockIsFSFBlock {
    #[serde(flatten)]
    pub block_is_f_s_f_block: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathTooDeep {
    #[serde(flatten)]
    pub vehicle_path_too_deep: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementCertificate {
    #[serde(flatten)]
    pub measurement_certificate: MeasurementCertificateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalibrationAdjustment {
    #[serde(flatten)]
    pub calibration_adjustment: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: SamplePlotTreesSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightSpruce {
    #[serde(flatten)]
    pub mean_height_spruce: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetStemCount {
    #[serde(flatten)]
    pub target_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class3DamageCount {
    #[serde(flatten)]
    pub class3_damage_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTree {
    #[serde(flatten)]
    pub stand_tree: StandTreeCuttingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestOwners {
    #[serde(flatten)]
    pub forest_owners: ForestOwnersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurnoverMoundsCount {
    #[serde(flatten)]
    pub turnover_mounds_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialAreaId {
    #[serde(flatten)]
    pub material_area_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachments {
    #[serde(flatten)]
    pub attachments: AttachmentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedEndDate {
    #[serde(flatten)]
    pub estimated_end_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionStands {
    #[serde(flatten)]
    pub financing_act_completion_stands: FinancingActCompletionStandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: HuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandExtraInfo {
    #[serde(flatten)]
    pub stand_extra_info: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentClasses {
    #[serde(flatten)]
    pub assortment_classes: AssortmentClassesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesCutting {
    #[serde(flatten)]
    pub stand_trees_cutting: StandTreesCuttingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HgPointGeometry {
    #[serde(flatten)]
    pub hg_point_geometry: PointGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkQuality {
    #[serde(flatten)]
    pub work_quality: VirtaWorkQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoDataSource {
    #[serde(flatten)]
    pub co_data_source: DataSource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreClearingText {
    #[serde(flatten)]
    pub pre_clearing_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentVolumesUnclassified {
    #[serde(flatten)]
    pub assortment_volumes_unclassified: AssortmentVolumesUnclassifiedType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateOwner {
    #[serde(flatten)]
    pub real_estate_owner: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandVolumeSummary {
    #[serde(flatten)]
    pub stand_volume_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: AlayksikkoNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterProtectionActionText {
    #[serde(flatten)]
    pub water_protection_action_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: BetaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTimeEnd {
    #[serde(flatten)]
    pub operation_time_end: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeDitchesNotDiggedAsMentionedInOrder {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalYear {
    #[serde(flatten)]
    pub proposal_year: ProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingAreaReference {
    #[serde(flatten)]
    pub processing_area_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveTreesLeft {
    #[serde(flatten)]
    pub save_trees_left: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludePaymentPlan {
    #[serde(flatten)]
    pub include_payment_plan: IncludePaymentPlanType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractWorkingSiteDetails {
    #[serde(flatten)]
    pub contract_working_site_details: ContractWorkingSiteDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialInfoText {
    #[serde(flatten)]
    pub material_info_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferExpirationDate {
    #[serde(flatten)]
    pub offer_expiration_date: OfferExpirationDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inspection {
    #[serde(flatten)]
    pub inspection: InspectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentCleanlinessNoticed {
    #[serde(flatten)]
    pub environment_cleanliness_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogLengthClass {
    #[serde(flatten)]
    pub log_length_class: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationDate {
    #[serde(flatten)]
    pub length_calibration_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderScaleModel {
    #[serde(flatten)]
    pub loader_scale_model: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoRestrictionCode {
    #[serde(flatten)]
    pub co_restriction_code: RestrictionCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemTypeVolumes {
    #[serde(flatten)]
    pub stem_type_volumes: StemTypeVolumesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpTreatment {
    #[serde(flatten)]
    pub stump_treatment: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchOrRoadPlanId {
    #[serde(flatten)]
    pub ditch_or_road_plan_id: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathMeanDistance {
    #[serde(flatten)]
    pub vehicle_path_mean_distance: Decimal5_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: AlkuHetkiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalTidinessNoticedText {
    #[serde(flatten)]
    pub environmental_tidiness_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filename {
    #[serde(flatten)]
    pub filename: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOffers {
    #[serde(flatten)]
    pub related_call_for_offers: RelatedCallForOffersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Minutes {
    #[serde(flatten)]
    pub minutes: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataForestRoadConstruction {
    #[serde(flatten)]
    pub control_data_forest_road_construction: ControlDataForestRoadConstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BranchBiomass {
    #[serde(flatten)]
    pub branch_biomass: BranchBiomassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationActor {
    #[serde(flatten)]
    pub application_actor: ApplicationActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityControlDate {
    #[serde(flatten)]
    pub quality_control_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryStatus {
    #[serde(flatten)]
    pub geometry_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SfUsingRestrictions {
    #[serde(flatten)]
    pub sf_using_restrictions: UsingRestrictions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xsboolean {
    #[serde(flatten)]
    pub xsboolean: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationGeometries {
    #[serde(flatten)]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccFinancingActApplicationReference {
    #[serde(flatten)]
    pub fcc_financing_act_application_reference: FinancingActApplicationReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Covered {
    #[serde(flatten)]
    pub covered: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Advertiser {
    #[serde(flatten)]
    pub advertiser: VirtaAdvertiserType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthMax {
    #[serde(flatten)]
    pub length_max: LengthMaxType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cost {
    #[serde(flatten)]
    pub cost: CostType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson3 {
    #[serde(flatten)]
    pub extra_person3: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathSubsidenceLength {
    #[serde(flatten)]
    pub vehicle_path_subsidence_length: Decimal3_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetDensity {
    #[serde(flatten)]
    pub target_density: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub financing_act_completion_declaration_text_information: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainPlanningDone {
    #[serde(flatten)]
    pub terrain_planning_done: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: ShapeDeltaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadStructureDrainage {
    #[serde(flatten)]
    pub road_structure_drainage: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: TurvakieltoKytkinTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageProposals {
    #[serde(flatten)]
    pub storage_proposals: StoragesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingAreas {
    #[serde(flatten)]
    pub working_areas: WorkingAreasType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KemeraMunicipalityId {
    #[serde(flatten)]
    pub kemera_municipality_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibration {
    #[serde(flatten)]
    pub diameter_calibration: DiameterCalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyToDo {
    #[serde(flatten)]
    pub ready_to_do: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureQualityText {
    #[serde(flatten)]
    pub silviculture_quality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreReply {
    #[serde(flatten)]
    pub forest_centre_reply: ForestCentreReplyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerResourceLocations {
    #[serde(flatten)]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation1 {
    #[serde(flatten)]
    pub organisation1: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialFeatureData {
    #[serde(flatten)]
    pub special_feature_data: SpecialFeatureDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndLoadNumber {
    #[serde(flatten)]
    pub end_load_number: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub fac_financing_act_application_text_information: FinancingActApplicationTextInformation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActWorkCode {
    #[serde(flatten)]
    pub fac_financing_act_work_code: FinancingActWorkCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonObjectData {
    #[serde(flatten)]
    pub common_object_data: CommonObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionOfOrigin {
    #[serde(flatten)]
    pub region_of_origin: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurfaceRunoffFieldGroundIsUnbroken {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken: YesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delivery {
    #[serde(flatten)]
    pub delivery: DeliveryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineGeometry {
    #[serde(flatten)]
    pub line_geometry: LineGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteOperational {
    #[serde(flatten)]
    pub working_site_operational: WorkingSiteOperationalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlLogCount {
    #[serde(flatten)]
    pub control_log_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionNumber {
    #[serde(flatten)]
    pub decision_number: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier5 {
    #[serde(flatten)]
    pub work_code_qualifier5: WorkCodeQualifierType5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: VATInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemBiomass {
    #[serde(flatten)]
    pub stem_biomass: StemBiomassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDamages {
    #[serde(flatten)]
    pub stem_damages: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WorkingSiteGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleFactors {
    #[serde(flatten)]
    pub scale_factors: ScaleFactorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentVolumes {
    #[serde(flatten)]
    pub assortment_volumes: AssortmentVolumesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCoordinates {
    #[serde(flatten)]
    pub stem_coordinates: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UseCase {
    #[serde(flatten)]
    pub use_case: ForestDataUpdateUseCaseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreventionFungusOfTheGenusData {
    #[serde(flatten)]
    pub prevention_fungus_of_the_genus_data: PreventionFungusOfTheGenusDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute4 {
    #[serde(flatten)]
    pub attribute4: WorkCodeQualifierType4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteHarvestingQualityControl {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsAssortmentClasses {
    #[serde(flatten)]
    pub as_assortment_classes: AssortmentClasses,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlForestUseDeclaration {
    #[serde(flatten)]
    pub control_forest_use_declaration: ControlForestUseDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditionResources {
    #[serde(flatten)]
    pub audition_resources: AuditionResourcesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRole {
    #[serde(flatten)]
    pub user_role: UserRoleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripherals {
    #[serde(flatten)]
    pub peripherals: PeripheralsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationDescription {
    #[serde(flatten)]
    pub compensation_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: HuoneistotunnisteNumeroTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingWeight {
    #[serde(flatten)]
    pub working_weight: WorkingWeightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A1 {
    #[serde(flatten)]
    pub a1: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Telephone {
    #[serde(flatten)]
    pub telephone: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainDamage {
    #[serde(flatten)]
    pub main_damage: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingHoursSunday {
    #[serde(flatten)]
    pub working_hours_sunday: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemNoticed {
    #[serde(flatten)]
    pub water_economy_system_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchingBreaksText {
    #[serde(flatten)]
    pub ditching_breaks_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelCount {
    #[serde(flatten)]
    pub level_count: PositiveInteger1digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DegreeDays {
    #[serde(flatten)]
    pub degree_days: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: CallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoadInvoiced {
    #[serde(flatten)]
    pub work_load_invoiced: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettlingPitsText {
    #[serde(flatten)]
    pub settling_pits_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalName {
    #[serde(flatten)]
    pub additional_name: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinAndDamDimensioningIsFollowed {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: YesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingsRightsOwnerRepresentative {
    #[serde(flatten)]
    pub cuttings_rights_owner_representative: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {
    #[serde(flatten)]
    pub measurement: MeasurementDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payee {
    #[serde(flatten)]
    pub payee: PayeeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: FeatureAdditionalCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessages {
    #[serde(flatten)]
    pub common_messages: CommonMessagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Damage {
    #[serde(flatten)]
    pub damage: DamageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: SubsidyApplierBaseContactAndEstateInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub offer_working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    #[serde(flatten)]
    pub day: DayType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandCorrectHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_correct_height_stumps_percentage: PercentWithFraction1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: KansalaisuusKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad2 {
    #[serde(flatten)]
    pub work_load2: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraFinancingApplication {
    #[serde(flatten)]
    pub extra_financing_application: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogVolume {
    #[serde(flatten)]
    pub log_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditSilviculture {
    #[serde(flatten)]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialAmountUnit {
    #[serde(flatten)]
    pub material_amount_unit: MaterialUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoCostTypeNumberType {
    #[serde(flatten)]
    pub co_cost_type_number_type: CostTypeNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: SubGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements {
    #[serde(flatten)]
    pub measurements: MeasurementsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Enddate {
    #[serde(flatten)]
    pub enddate: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchDepthWidthAndDrainageEffect {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeltedWater {
    #[serde(flatten)]
    pub melted_water: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinePulp {
    #[serde(flatten)]
    pub pine_pulp: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    #[serde(flatten)]
    pub question: AuditQuestionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionReceiver {
    #[serde(flatten)]
    pub decision_receiver: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendedDensity {
    #[serde(flatten)]
    pub recommended_density: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Training {
    #[serde(flatten)]
    pub training: TrainingDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingAreaNumber {
    #[serde(flatten)]
    pub processing_area_number: ProcessingAreaNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainClass {
    #[serde(flatten)]
    pub terrain_class: TerrainClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaidValue {
    #[serde(flatten)]
    pub paid_value: PaidValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActNumber {
    #[serde(flatten)]
    pub financing_act_number: FinancingActNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapSymbols {
    #[serde(flatten)]
    pub map_symbols: MapSymbolsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureValidity {
    #[serde(flatten)]
    pub silviculture_validity: SilvicultureValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classification {
    #[serde(flatten)]
    pub classification: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegalAccidentInsurance {
    #[serde(flatten)]
    pub legal_accident_insurance: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier4 {
    #[serde(flatten)]
    pub work_code_qualifier4: WorkCodeQualifierType4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrdererResponsibilityDocumentsChecked {
    #[serde(flatten)]
    pub orderer_responsibility_documents_checked: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationNumber {
    #[serde(flatten)]
    pub forest_use_declaration_number: ForestUseDeclarationNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassQuality {
    #[serde(flatten)]
    pub bio_mass_quality: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WppPayment {
    #[serde(flatten)]
    pub wpp_payment: Payment,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialVolume {
    #[serde(flatten)]
    pub material_volume: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationInfo {
    #[serde(flatten)]
    pub operation_info: OperationInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectGeometry {
    #[serde(flatten)]
    pub object_geometry: ObjectGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAcceptanceStatus {
    #[serde(flatten)]
    pub business_acceptance_status: BusinessAcceptanceStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureRestrictionDetails {
    #[serde(flatten)]
    pub silviculture_restriction_details: SilvicultureRestrictionDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingContract {
    #[serde(flatten)]
    pub working_contract: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionAnswer {
    #[serde(flatten)]
    pub question_answer: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstablishedPartNumber {
    #[serde(flatten)]
    pub established_part_number: VirtaPartNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamageEvaluationDate {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacApplicationActors {
    #[serde(flatten)]
    pub fac_application_actors: ApplicationActors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallWoodRemovalVolume {
    #[serde(flatten)]
    pub small_wood_removal_volume: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisQualificationReason {
    #[serde(flatten)]
    pub dis_qualification_reason: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: LoppuPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoControlDataOperationStatusType {
    #[serde(flatten)]
    pub co_control_data_operation_status_type: ControlDataOperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BIC {
    #[serde(flatten)]
    pub bic: BICType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessageData {
    #[serde(flatten)]
    pub error_message_data: ErrorMessageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryNumber {
    #[serde(flatten)]
    pub delivery_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceStartDate {
    #[serde(flatten)]
    pub service_start_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatOperation {
    #[serde(flatten)]
    pub habitat_operation: HabitatOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationAmount {
    #[serde(flatten)]
    pub application_amount: Decimal7And2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Specification {
    #[serde(flatten)]
    pub specification: SpecificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeGrowth {
    #[serde(flatten)]
    pub volume_growth: VolumeGrowthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateSeedlingsPlanted {
    #[serde(flatten)]
    pub date_seedlings_planted: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantingDepthErrorCount {
    #[serde(flatten)]
    pub planting_depth_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: LocationEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipeInstallationText {
    #[serde(flatten)]
    pub pipe_installation_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiPolygonGeometry {
    #[serde(flatten)]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDate {
    #[serde(flatten)]
    pub action_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MineralSoilLayer {
    #[serde(flatten)]
    pub mineral_soil_layer: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingSuitableText {
    #[serde(flatten)]
    pub stump_lifting_suitable_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingByMachine {
    #[serde(flatten)]
    pub cutting_by_machine: VirtaCuttingByMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredContactingMethods {
    #[serde(flatten)]
    pub preferred_contacting_methods: PreferredContactingMethodsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsedMachine {
    #[serde(flatten)]
    pub used_machine: UsedMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountSilverBirch {
    #[serde(flatten)]
    pub stem_count_silver_birch: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoStemCountType {
    #[serde(flatten)]
    pub co_stem_count_type: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSFNumber {
    #[serde(flatten)]
    pub f_s_f_number: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessReceiver {
    #[serde(flatten)]
    pub business_receiver: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerSampleAreaApproval {
    #[serde(flatten)]
    pub owner_sample_area_approval: VirtaApprovalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesData {
    #[serde(flatten)]
    pub tree_species_data: TreeSpeciesDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: ToinenRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: AccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditRoadMaking {
    #[serde(flatten)]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetAmountUnit {
    #[serde(flatten)]
    pub target_amount_unit: ExtendedWideUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizationTotalAudit {
    #[serde(flatten)]
    pub fertilization_total_audit: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FellingRightValidityDate {
    #[serde(flatten)]
    pub felling_right_validity_date: FellingRightValidityDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageStemVolume {
    #[serde(flatten)]
    pub average_stem_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusCode {
    #[serde(flatten)]
    pub status_code: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ForestUseDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpGeometryType {
    #[serde(flatten)]
    pub help_geometry_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationAdjustment {
    #[serde(flatten)]
    pub length_calibration_adjustment: Integer3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDate {
    #[serde(flatten)]
    pub moose_damage_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlEvaluations {
    #[serde(flatten)]
    pub control_evaluations: ControlEvaluationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationRow {
    #[serde(flatten)]
    pub operation_row: OperationRowType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogLength {
    #[serde(flatten)]
    pub log_length: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Work {
    #[serde(flatten)]
    pub work: WorkType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingLotIdentifier {
    #[serde(flatten)]
    pub seedling_lot_identifier: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpGeometries {
    #[serde(flatten)]
    pub help_geometries: HelpGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrProductsType {
    #[serde(flatten)]
    pub pr_products_type: ProductsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: SiviilisaatyTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditStumpForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousBlocks {
    #[serde(flatten)]
    pub previous_blocks: PreviousBlockInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountLeft {
    #[serde(flatten)]
    pub amount_left: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantingWorkQuality {
    #[serde(flatten)]
    pub planting_work_quality: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SawLogPercent {
    #[serde(flatten)]
    pub saw_log_percent: SawLogPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStandReference {
    #[serde(flatten)]
    pub declaration_stand_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpCuttingAsInstructed {
    #[serde(flatten)]
    pub stump_cutting_as_instructed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpsLifted {
    #[serde(flatten)]
    pub stumps_lifted: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SodWorkingSite {
    #[serde(flatten)]
    pub sod_working_site: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationActors {
    #[serde(flatten)]
    pub application_actors: ApplicationActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Damages {
    #[serde(flatten)]
    pub damages: DamageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Locator {
    #[serde(flatten)]
    pub locator: LocatorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: NimiTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjects {
    #[serde(flatten)]
    pub child_objects: ChildObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: DrainageStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerActionApproval {
    #[serde(flatten)]
    pub owner_action_approval: VirtaApprovalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hours {
    #[serde(flatten)]
    pub hours: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OverallTotalSubsidy {
    #[serde(flatten)]
    pub overall_total_subsidy: MoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavationSoilLocation {
    #[serde(flatten)]
    pub excavation_soil_location: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    #[serde(flatten)]
    pub error_message: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactorId {
    #[serde(flatten)]
    pub contactor_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidityDateEnd {
    #[serde(flatten)]
    pub validity_date_end: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchMeanWidth {
    #[serde(flatten)]
    pub ditch_mean_width: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlledOperationType {
    #[serde(flatten)]
    pub controlled_operation_type: CostTypeNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FulfilledArea {
    #[serde(flatten)]
    pub fulfilled_area: FulfilledAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: GammaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnergyTimberValue {
    #[serde(flatten)]
    pub energy_timber_value: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccBankAccount {
    #[serde(flatten)]
    pub fcc_bank_account: BankAccount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageFinishedDate {
    #[serde(flatten)]
    pub storage_finished_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingFinishedDate {
    #[serde(flatten)]
    pub stump_lifting_finished_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveTreesLeftText {
    #[serde(flatten)]
    pub save_trees_left_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ICETelephone {
    #[serde(flatten)]
    pub i_c_e_telephone: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialAmount {
    #[serde(flatten)]
    pub material_amount: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppliedLength {
    #[serde(flatten)]
    pub applied_length: Decimal6_2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpTidiness {
    #[serde(flatten)]
    pub stump_tidiness: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplierReference {
    #[serde(flatten)]
    pub subsidy_applier_reference: SubsidyApplierReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteState {
    #[serde(flatten)]
    pub complete_state: CompleteStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Active {
    #[serde(flatten)]
    pub active: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizerVolumeMeasured {
    #[serde(flatten)]
    pub fertilizer_volume_measured: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    #[serde(flatten)]
    pub review: VirtaReviewType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeClass {
    #[serde(flatten)]
    pub tree_class: TreeClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalDate {
    #[serde(flatten)]
    pub proposal_date: ProposalDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationType {
    #[serde(flatten)]
    pub notification_type: NotificationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaxProposalYear {
    #[serde(flatten)]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingAreas {
    #[serde(flatten)]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalType {
    #[serde(flatten)]
    pub proposal_type: ProposalTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningDepth {
    #[serde(flatten)]
    pub soil_conditioning_depth: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadStructureShape {
    #[serde(flatten)]
    pub road_structure_shape: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertedMapSymbolId {
    #[serde(flatten)]
    pub inserted_map_symbol_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvesterVolume {
    #[serde(flatten)]
    pub harvester_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductKey {
    #[serde(flatten)]
    pub product_key: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: InspectionMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NationalityCode {
    #[serde(flatten)]
    pub nationality_code: String5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceSchedule {
    #[serde(flatten)]
    pub resource_schedule: ResourceScheduleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdStemDistribution {
    #[serde(flatten)]
    pub sd_stem_distribution: StemDistribution,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: IkaluokkaTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeTime {
    #[serde(flatten)]
    pub change_time: ChangeTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: QuantityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: PostinumeroKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyPercent {
    #[serde(flatten)]
    pub subsidy_percent: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentClass {
    #[serde(flatten)]
    pub assortment_class: AssortmentClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterCalibrationReason {
    #[serde(flatten)]
    pub diameter_calibration_reason: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumplingLevelingErrorCount {
    #[serde(flatten)]
    pub dumpling_leveling_error_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryDate {
    #[serde(flatten)]
    pub delivery_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopOperationProject {
    #[serde(flatten)]
    pub cop_operation_project: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StandBasicDataDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacSender {
    #[serde(flatten)]
    pub fac_sender: Sender,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurningPlace {
    #[serde(flatten)]
    pub turning_place: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: MeasurementPlaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingData {
    #[serde(flatten)]
    pub seedling_data: SeedlingDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAreaMethod {
    #[serde(flatten)]
    pub test_area_method: SamplePlotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapSymbolId {
    #[serde(flatten)]
    pub map_symbol_id: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantStorageManagementText {
    #[serde(flatten)]
    pub plant_storage_management_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterSpruce {
    #[serde(flatten)]
    pub mean_diameter_spruce: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Age {
    #[serde(flatten)]
    pub age: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stem {
    #[serde(flatten)]
    pub stem: StemDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arc {
    #[serde(flatten)]
    pub arc: ArcType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionType {
    #[serde(flatten)]
    pub decision_type: DecisionTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Roles {
    #[serde(flatten)]
    pub roles: RolesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsValueForceWorkingSite {
    #[serde(flatten)]
    pub is_value_force_working_site: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlStandArea {
    #[serde(flatten)]
    pub control_stand_area: AreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    #[serde(flatten)]
    pub area: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirchLog {
    #[serde(flatten)]
    pub birch_log: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartLoadNumber {
    #[serde(flatten)]
    pub start_load_number: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacAreaNo {
    #[serde(flatten)]
    pub fac_area_no: AreaNo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingStemCount {
    #[serde(flatten)]
    pub cutting_stem_count: CuttingStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteEndNotification {
    #[serde(flatten)]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KilometersWithCaravan {
    #[serde(flatten)]
    pub kilometers_with_caravan: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WstoOffer {
    #[serde(flatten)]
    pub wsto_offer: Offer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewDestinationStorage {
    #[serde(flatten)]
    pub new_destination_storage: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalCompensations {
    #[serde(flatten)]
    pub total_compensations: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(flatten)]
    pub image: ImageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganisationId {
    #[serde(flatten)]
    pub organisation_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingConditionAndQualityDescription {
    #[serde(flatten)]
    pub seedling_condition_and_quality_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetAnnouncedAmount {
    #[serde(flatten)]
    pub target_announced_amount: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacLanguage {
    #[serde(flatten)]
    pub fac_language: Language,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomControlStemRejectedReason {
    #[serde(flatten)]
    pub random_control_stem_rejected_reason: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyElementName {
    #[serde(flatten)]
    pub key_element_name: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalStemCount {
    #[serde(flatten)]
    pub goal_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingQualityText {
    #[serde(flatten)]
    pub stump_lifting_quality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActWorkGroup {
    #[serde(flatten)]
    pub financing_act_work_group: FinancingActWorkGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KilometersWithBreakHouse {
    #[serde(flatten)]
    pub kilometers_with_break_house: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacDocuments {
    #[serde(flatten)]
    pub fac_documents: Documents,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: StatusryhmaTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeNumber {
    #[serde(flatten)]
    pub cost_type_number: CostTypeNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: TreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectOverallEvaluationData {
    #[serde(flatten)]
    pub object_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutOfServiceStartDate {
    #[serde(flatten)]
    pub out_of_service_start_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActRealEstates {
    #[serde(flatten)]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlternativeName {
    #[serde(flatten)]
    pub alternative_name: AlternativeNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceCompany {
    #[serde(flatten)]
    pub insurance_company: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Works {
    #[serde(flatten)]
    pub works: WorksType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: WorkingSiteStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreClearingInformation {
    #[serde(flatten)]
    pub pre_clearing_information: PreClearingInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status: OperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantingSiteCount {
    #[serde(flatten)]
    pub planting_site_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reason {
    #[serde(flatten)]
    pub reason: ReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryObjects {
    #[serde(flatten)]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: UlkomaaHenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub real_amount_of_soil_preparation_spot: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingStratum {
    #[serde(flatten)]
    pub seedling_stratum: SeedlingStratumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlPolygon {
    #[serde(flatten)]
    pub gml_polygon: Polygon,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Productivity {
    #[serde(flatten)]
    pub productivity: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub result_of_accessibility_analysis: ResultOfAccessibilityAnalysisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CfoCallForOffer {
    #[serde(flatten)]
    pub cfo_call_for_offer: CallForOffer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSites {
    #[serde(flatten)]
    pub call_for_offer_working_sites: CallForOfferWorkingSitesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IslandWorkingSite {
    #[serde(flatten)]
    pub island_working_site: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: Decimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpruceDecimal {
    #[serde(flatten)]
    pub spruce_decimal: VirtaTreeDecimalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HumidityPercentage {
    #[serde(flatten)]
    pub humidity_percentage: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountOutsideSpot {
    #[serde(flatten)]
    pub amount_outside_spot: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: ShapeAlfaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeSummary {
    #[serde(flatten)]
    pub volume_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElectronicNotification {
    #[serde(flatten)]
    pub electronic_notification: ElectronicNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkSafetyRiskDescription {
    #[serde(flatten)]
    pub work_safety_risk_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reason2 {
    #[serde(flatten)]
    pub reason2: VirtaReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: YritysTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LandScaping {
    #[serde(flatten)]
    pub land_scaping: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentPercent {
    #[serde(flatten)]
    pub assortment_percent: AssortmentPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPlanStandId {
    #[serde(flatten)]
    pub forest_plan_stand_id: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class4DamageCount {
    #[serde(flatten)]
    pub class4_damage_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StTreeStandBasedDataType {
    #[serde(flatten)]
    pub st_tree_stand_based_data_type: TreeStandBasedDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NationalityFreeText {
    #[serde(flatten)]
    pub nationality_free_text: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WorkingSiteNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerType {
    #[serde(flatten)]
    pub customer_type: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractCode {
    #[serde(flatten)]
    pub contract_code: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightHardWood {
    #[serde(flatten)]
    pub mean_height_hard_wood: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActCompletionStands {
    #[serde(flatten)]
    pub fac_financing_act_completion_stands: FinancingActCompletionStands,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessageId {
    #[serde(flatten)]
    pub common_message_id: CommonMessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hoppers {
    #[serde(flatten)]
    pub hoppers: HoppersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestCultivatedSpotHeight {
    #[serde(flatten)]
    pub nearest_cultivated_spot_height: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRoles {
    #[serde(flatten)]
    pub user_roles: UserRolesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    #[serde(flatten)]
    pub text: TextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedOperationChain {
    #[serde(flatten)]
    pub planned_operation_chain: PlannedOperationChainType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: VarianceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageSourceDescription {
    #[serde(flatten)]
    pub damage_source_description: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeIdentifier {
    #[serde(flatten)]
    pub tree_identifier: TreeIdentifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionReceiver {
    #[serde(flatten)]
    pub transmission_receiver: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportDistanceUnit {
    #[serde(flatten)]
    pub transport_distance_unit: DistanceUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsOperation {
    #[serde(flatten)]
    pub statistics_operation: StatisticsOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncedEnergyWood {
    #[serde(flatten)]
    pub announced_energy_wood: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitialLoad {
    #[serde(flatten)]
    pub partitial_load: PartitialLoadType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class1damageCount {
    #[serde(flatten)]
    pub class1damage_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestInfo {
    #[serde(flatten)]
    pub request_info: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier2 {
    #[serde(flatten)]
    pub work_code_qualifier2: WorkCodeQualifierType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operator {
    #[serde(flatten)]
    pub operator: VirtaAdvertiserType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrassControlEvaluation {
    #[serde(flatten)]
    pub grass_control_evaluation: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PassingPlace {
    #[serde(flatten)]
    pub passing_place: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationOtherOperation {
    #[serde(flatten)]
    pub declaration_other_operation: DeclarationOtherOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorderingWithWaterAreaOrStream {
    #[serde(flatten)]
    pub bordering_with_water_area_or_stream: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDeclarationRealEstates {
    #[serde(flatten)]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: KatuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantCount {
    #[serde(flatten)]
    pub plant_count: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatAdvertisement {
    #[serde(flatten)]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    #[serde(flatten)]
    pub temperature: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalpsCount {
    #[serde(flatten)]
    pub scalps_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalProposalYear {
    #[serde(flatten)]
    pub original_proposal_year: OriginalProposalYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandDataDate {
    #[serde(flatten)]
    pub tree_stand_data_date: TreeStandDataDate2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: MaatunnusKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageClass {
    #[serde(flatten)]
    pub damage_class: VirtaDamageClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodAdditionalDetails {
    #[serde(flatten)]
    pub cod_additional_details: AdditionalDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelId {
    #[serde(flatten)]
    pub parcel_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlternativeIdentifier {
    #[serde(flatten)]
    pub alternative_identifier: AlternativeIdentifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActWorkGroup {
    #[serde(flatten)]
    pub fac_financing_act_work_group: FinancingActWorkGroup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Height {
    #[serde(flatten)]
    pub height: HeightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Startdate {
    #[serde(flatten)]
    pub startdate: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotDamagedSeedlingCount {
    #[serde(flatten)]
    pub not_damaged_seedling_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotsSummaries {
    #[serde(flatten)]
    pub sample_plots_summaries: SamplePlotSummariesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    #[serde(flatten)]
    pub attachment: AttachmentDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    #[serde(flatten)]
    pub r#type: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterPine {
    #[serde(flatten)]
    pub mean_diameter_pine: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectorName {
    #[serde(flatten)]
    pub inspector_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineDescription {
    #[serde(flatten)]
    pub machine_description: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityAttachment {
    #[serde(flatten)]
    pub quality_attachment: QualityAttachmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDiameter {
    #[serde(flatten)]
    pub log_diameter: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataInformation {
    #[serde(flatten)]
    pub data_information: DataInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountSeedlingsToPlant {
    #[serde(flatten)]
    pub amount_seedlings_to_plant: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OWorkingSite {
    #[serde(flatten)]
    pub o_working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTooHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_too_height_stumps_percentage: PercentWithFraction1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedlingConditionAndQuality {
    #[serde(flatten)]
    pub seedling_condition_and_quality: SeedlingConditionAndQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStandTextInformation {
    #[serde(flatten)]
    pub declaration_stand_text_information: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineAccessoryCode {
    #[serde(flatten)]
    pub machine_accessory_code: MachineAccessoryCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingValue {
    #[serde(flatten)]
    pub cutting_value: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipeInstallation {
    #[serde(flatten)]
    pub pipe_installation: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub financing_act_application_text_information: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReasonDescription {
    #[serde(flatten)]
    pub reason_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiabilityInsurance {
    #[serde(flatten)]
    pub liability_insurance: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessages {
    #[serde(flatten)]
    pub status_messages: StatusMessageLanguageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: Decimal3_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferText {
    #[serde(flatten)]
    pub offer_text: OfferTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: DeclarationReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeValue {
    #[serde(flatten)]
    pub attribute_value: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: HuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaledMass {
    #[serde(flatten)]
    pub scaled_mass: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DryingClass {
    #[serde(flatten)]
    pub drying_class: DryingClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleaningBreaksDoneText {
    #[serde(flatten)]
    pub cleaning_breaks_done_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreventionSubstanceProductName {
    #[serde(flatten)]
    pub prevention_substance_product_name: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingAreaPreclearingNeed {
    #[serde(flatten)]
    pub cutting_area_preclearing_need: CuttingAreaPreclearingNeedType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizerVolumeMeasuredText {
    #[serde(flatten)]
    pub fertilizer_volume_measured_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherEvaluation {
    #[serde(flatten)]
    pub other_evaluation: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomassCollection {
    #[serde(flatten)]
    pub biomass_collection: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorVerified {
    #[serde(flatten)]
    pub sub_contractor_verified: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Distribution {
    #[serde(flatten)]
    pub distribution: DistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: StorageLandOwnerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSizeTreeReduction {
    #[serde(flatten)]
    pub sample_plot_size_tree_reduction: SamplePlotSizeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: KuntaNumeroTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavatorMovingAndTreeDamages {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthCalibrationReason {
    #[serde(flatten)]
    pub length_calibration_reason: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherPublicSubstitute {
    #[serde(flatten)]
    pub other_public_substitute: OtherPublicSubstituteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xsfloat {
    #[serde(flatten)]
    pub xsfloat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrainStorageAsInstructedText {
    #[serde(flatten)]
    pub drain_storage_as_instructed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: PostitoimipaikkaNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeLeft {
    #[serde(flatten)]
    pub volume_left: Decimal6TotalDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StratumNumber {
    #[serde(flatten)]
    pub stratum_number: StratumNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalAreaSilverBirch {
    #[serde(flatten)]
    pub basal_area_silver_birch: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RightToSpecifyBankAccountsOfPaymentTransactions {
    #[serde(flatten)]
    pub right_to_specify_bank_accounts_of_payment_transactions: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherTimberValue {
    #[serde(flatten)]
    pub other_timber_value: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: Decimal5_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousBlock {
    #[serde(flatten)]
    pub previous_block: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Length {
    #[serde(flatten)]
    pub length: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: AmmattiKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportArea {
    #[serde(flatten)]
    pub transport_area: String10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingMethod {
    #[serde(flatten)]
    pub cutting_method: CuttingTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualitySystem {
    #[serde(flatten)]
    pub quality_system: QualitySystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelPosition {
    #[serde(flatten)]
    pub label_position: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectProtectionOperations {
    #[serde(flatten)]
    pub object_protection_operations: ObjectProtectionOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: VolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionGeometries {
    #[serde(flatten)]
    pub decision_geometries: DecisionGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentWorkingSiteVolume {
    #[serde(flatten)]
    pub sent_working_site_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurerId {
    #[serde(flatten)]
    pub measurer_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccOverallTotalSubsidy {
    #[serde(flatten)]
    pub fcc_overall_total_subsidy: OverallTotalSubsidy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLotInformationType {
    #[serde(flatten)]
    pub wood_lot_information_type: WoodLotInformationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificationHandlingInstructions {
    #[serde(flatten)]
    pub certification_handling_instructions: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamageOutsideStand {
    #[serde(flatten)]
    pub tree_damage_outside_stand: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: EnsimmainenRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: ValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SawLogVolume {
    #[serde(flatten)]
    pub saw_log_volume: SawLogVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacFinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub fac_financing_act_completion_declaration_text_information: FinancingActCompletionDeclarationTextInformation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActRealEstate {
    #[serde(flatten)]
    pub financing_act_real_estate: FinancingActRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    #[serde(flatten)]
    pub payment: ForestCentrePaymentDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlLogDiameter {
    #[serde(flatten)]
    pub control_log_diameter: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalAreaOtherTreeSpecies {
    #[serde(flatten)]
    pub basal_area_other_tree_species: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassQualityText {
    #[serde(flatten)]
    pub bio_mass_quality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PassingPlaceText {
    #[serde(flatten)]
    pub passing_place_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class2damageCount {
    #[serde(flatten)]
    pub class2damage_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryInfo {
    #[serde(flatten)]
    pub delivery_info: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: SellerRepresentativePersonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedWorkAmount {
    #[serde(flatten)]
    pub planned_work_amount: AmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: KutsumaNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentMainGroup {
    #[serde(flatten)]
    pub assortment_main_group: AssortmentMainGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TstTreeStrata {
    #[serde(flatten)]
    pub tst_tree_strata: TreeStrata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacCompletionDeclarationActors {
    #[serde(flatten)]
    pub fac_completion_declaration_actors: CompletionDeclarationActors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumptionUnit {
    #[serde(flatten)]
    pub consumption_unit: ConsumptionUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrowthPlaceDataSource {
    #[serde(flatten)]
    pub growth_place_data_source: DataSourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAreaNumber {
    #[serde(flatten)]
    pub test_area_number: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionAsText {
    #[serde(flatten)]
    pub question_as_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Orientation {
    #[serde(flatten)]
    pub orientation: OrientationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePreviousDeclaration {
    #[serde(flatten)]
    pub update_previous_declaration: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalData {
    #[serde(flatten)]
    pub proposal_data: ProposalDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: KolmasRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountHardWood {
    #[serde(flatten)]
    pub stem_count_hard_wood: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestedVolume {
    #[serde(flatten)]
    pub harvested_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xsdecimal {
    #[serde(flatten)]
    pub xsdecimal: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirdromeAsInstructedText {
    #[serde(flatten)]
    pub airdrome_as_instructed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: AlkuPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuousCoverForestry {
    #[serde(flatten)]
    pub continuous_cover_forestry: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackForPlanner {
    #[serde(flatten)]
    pub feedback_for_planner: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferDate {
    #[serde(flatten)]
    pub offer_date: OfferDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CuttingRestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlReference {
    #[serde(flatten)]
    pub control_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActProjectCompleted {
    #[serde(flatten)]
    pub financing_act_project_completed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageDate {
    #[serde(flatten)]
    pub image_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubStemCount {
    #[serde(flatten)]
    pub stub_stem_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemTypeBulk {
    #[serde(flatten)]
    pub stem_type_bulk: StemTypeBulkType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subsidy {
    #[serde(flatten)]
    pub subsidy: SubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchMeanDepth {
    #[serde(flatten)]
    pub ditch_mean_depth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoutTimberClassifier {
    #[serde(flatten)]
    pub stout_timber_classifier: StoutTimberClassifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultValue {
    #[serde(flatten)]
    pub default_value: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnowOrIce {
    #[serde(flatten)]
    pub snow_or_ice: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registered {
    #[serde(flatten)]
    pub registered: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubCategory {
    #[serde(flatten)]
    pub sub_category: ImageSubCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingFinishedDate {
    #[serde(flatten)]
    pub cutting_finished_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueListItem {
    #[serde(flatten)]
    pub value_list_item: ValueListItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreMessage {
    #[serde(flatten)]
    pub forest_centre_message: ForestCentreMessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HgLineGeometry {
    #[serde(flatten)]
    pub hg_line_geometry: LineGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceTypes {
    #[serde(flatten)]
    pub service_types: ServiceTypesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeekCalendar {
    #[serde(flatten)]
    pub week_calendar: WeekCalendarType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Altitude {
    #[serde(flatten)]
    pub altitude: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartNumber {
    #[serde(flatten)]
    pub part_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerInvolvement {
    #[serde(flatten)]
    pub owner_involvement: VirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountingDate {
    #[serde(flatten)]
    pub accounting_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadGreenMass {
    #[serde(flatten)]
    pub load_green_mass: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Woodlot {
    #[serde(flatten)]
    pub woodlot: WoodLotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalibrationDate {
    #[serde(flatten)]
    pub calibration_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardedVolumeAccounted {
    #[serde(flatten)]
    pub forwarded_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PineLog {
    #[serde(flatten)]
    pub pine_log: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationDevelopmentClass {
    #[serde(flatten)]
    pub declaration_development_class: DeclarationDevelopmentClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingPurpose {
    #[serde(flatten)]
    pub cutting_purpose: CuttingPurposeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: MaterialCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Percent {
    #[serde(flatten)]
    pub percent: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCode {
    #[serde(flatten)]
    pub request_code: RequestCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgVehiclePathDistance {
    #[serde(flatten)]
    pub stand_avg_vehicle_path_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: RestrictionTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalInspectionId {
    #[serde(flatten)]
    pub internal_inspection_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceName {
    #[serde(flatten)]
    pub resource_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteDetails {
    #[serde(flatten)]
    pub offer_working_site_details: OfferWorkingSiteDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WpmcMeasurementCertificate {
    #[serde(flatten)]
    pub wpmc_measurement_certificate: MeasurementCertificate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditQuestion {
    #[serde(flatten)]
    pub audit_question: AuditQuestionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemsNoticedText {
    #[serde(flatten)]
    pub water_economy_systems_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson1 {
    #[serde(flatten)]
    pub extra_person1: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesAttribute {
    #[serde(flatten)]
    pub tree_species_attribute: TreeSpeciesAttributeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HopperLocationFromGPS {
    #[serde(flatten)]
    pub hopper_location_from_g_p_s: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchOrRoadPlanName {
    #[serde(flatten)]
    pub ditch_or_road_plan_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectiveAgreements {
    #[serde(flatten)]
    pub collective_agreements: CollectiveAgreementsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeltingDate {
    #[serde(flatten)]
    pub melting_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjectId {
    #[serde(flatten)]
    pub child_object_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuspensionHoleDensitySizeAndLocationText {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IBAN {
    #[serde(flatten)]
    pub iban: IBANType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedPlantEvaluation {
    #[serde(flatten)]
    pub seed_plant_evaluation: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: MaximumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: LajiKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCompletionDate {
    #[serde(flatten)]
    pub work_completion_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinSizeShapeAndFunctionality {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantCountSummary {
    #[serde(flatten)]
    pub plant_count_summary: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceNameOfAPI {
    #[serde(flatten)]
    pub service_name_of_a_p_i: ServiceNameofAPIType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HumidityMeasured {
    #[serde(flatten)]
    pub humidity_measured: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonObjectDataReference {
    #[serde(flatten)]
    pub common_object_data_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbsoluteQuantity {
    #[serde(flatten)]
    pub absolute_quantity: AbsoluteQuantityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadStemCount {
    #[serde(flatten)]
    pub dead_stem_count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Count {
    #[serde(flatten)]
    pub count: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: Decimal7And2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccForestUseDeclarationReference {
    #[serde(flatten)]
    pub fcc_forest_use_declaration_reference: ForestUseDeclarationReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplier {
    #[serde(flatten)]
    pub supplier: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisQualificationReasons {
    #[serde(flatten)]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GdtPointLineAndPolygonGeometriesGroup {
    #[serde(flatten)]
    pub gdt_point_line_and_polygon_geometries_group: PointLineAndPolygonGeometriesGroup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuspensionHoleDensitySizeAndLocation {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlBasicData {
    #[serde(flatten)]
    pub control_basic_data: ControlBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinAndDamDimensioningIsFollowedText {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub is_forest_haulage_distance_continued: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObject {
    #[serde(flatten)]
    pub parent_object: ParentObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgtTarget {
    #[serde(flatten)]
    pub tgt_target: Target,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataSpecialFeatures {
    #[serde(flatten)]
    pub control_data_special_features: ControlDataSpecialFeaturesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditSpareTrees {
    #[serde(flatten)]
    pub final_audit_spare_trees: FinalAuditSpareTreesByCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductInstructionProductInstruction {
    #[serde(flatten)]
    pub product_instruction_product_instruction: ProductInstruction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageCode {
    #[serde(flatten)]
    pub language_code: LanguageCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    #[serde(flatten)]
    pub actor: ActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherTreeSpecies {
    #[serde(flatten)]
    pub other_tree_species: OtherTreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionReceivers {
    #[serde(flatten)]
    pub decision_receivers: DecisionReceiversType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadStructureShapeText {
    #[serde(flatten)]
    pub road_structure_shape_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationRestriction {
    #[serde(flatten)]
    pub operation_restriction: OperationRestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedStemCount {
    #[serde(flatten)]
    pub planned_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpectedValueSurplus {
    #[serde(flatten)]
    pub expected_value_surplus: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherConservationMethods {
    #[serde(flatten)]
    pub other_conservation_methods: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    #[serde(flatten)]
    pub title: TitleEltType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: FinalAuditTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: VakinainenKytkinTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Acceptance {
    #[serde(flatten)]
    pub acceptance: AcceptanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute2 {
    #[serde(flatten)]
    pub attribute2: WorkCodeQualifierType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationReference {
    #[serde(flatten)]
    pub financing_act_application_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TooHeightStumps {
    #[serde(flatten)]
    pub too_height_stumps: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorsAllowed {
    #[serde(flatten)]
    pub sub_contractors_allowed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationsAndImprovements {
    #[serde(flatten)]
    pub notifications_and_improvements: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeginDate {
    #[serde(flatten)]
    pub begin_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDate {
    #[serde(flatten)]
    pub create_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditIdentifier {
    #[serde(flatten)]
    pub final_audit_identifier: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSystemInUse {
    #[serde(flatten)]
    pub external_system_in_use: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureQuality {
    #[serde(flatten)]
    pub silviculture_quality: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: KuntaKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestExtraInfo {
    #[serde(flatten)]
    pub harvest_extra_info: VirtaExtraInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentID {
    #[serde(flatten)]
    pub assortment_i_d: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmllineStringProperty {
    #[serde(flatten)]
    pub gmlline_string_property: LineStringProperty,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingDate {
    #[serde(flatten)]
    pub training_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirdromeAsInstructed {
    #[serde(flatten)]
    pub airdrome_as_instructed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseAction {
    #[serde(flatten)]
    pub case_action: CaseActionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomassFinishedDate {
    #[serde(flatten)]
    pub biomass_finished_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: StatisticsPurchaseModeCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeightClass {
    #[serde(flatten)]
    pub weight_class: PositiveInteger1digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpRaising {
    #[serde(flatten)]
    pub stump_raising: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalendarDay {
    #[serde(flatten)]
    pub calendar_day: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDate {
    #[serde(flatten)]
    pub document_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDamageCount {
    #[serde(flatten)]
    pub stem_damage_count: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingBy {
    #[serde(flatten)]
    pub cutting_by: VirtaCuttingByMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationDescription {
    #[serde(flatten)]
    pub evaluation_description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedStatisticsOperationType {
    #[serde(flatten)]
    pub reported_statistics_operation_type: ReportedStatisticsOperationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccPaymentDate {
    #[serde(flatten)]
    pub fcc_payment_date: PaymentDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreclearingEvaluation {
    #[serde(flatten)]
    pub preclearing_evaluation: PreclearingEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingId {
    #[serde(flatten)]
    pub training_id: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtsDeadTreeStrata {
    #[serde(flatten)]
    pub dts_dead_tree_strata: DeadTreeStrata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationText {
    #[serde(flatten)]
    pub evaluation_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOfferDescription {
    #[serde(flatten)]
    pub related_call_for_offer_description: String1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackForPlannerText {
    #[serde(flatten)]
    pub feedback_for_planner_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerContractorId {
    #[serde(flatten)]
    pub owner_contractor_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HgPolygonGeometry {
    #[serde(flatten)]
    pub hg_polygon_geometry: PolygonGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalAreaSummary {
    #[serde(flatten)]
    pub basal_area_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwarderDelay {
    #[serde(flatten)]
    pub forwarder_delay: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationPolygons {
    #[serde(flatten)]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanDate {
    #[serde(flatten)]
    pub working_site_plan_date: WorkingSitePlanDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: AssortmentGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeGroups {
    #[serde(flatten)]
    pub work_code_groups: WorkCodeGroupsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(flatten)]
    pub address: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpTidinessText {
    #[serde(flatten)]
    pub stump_tidiness_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinLength {
    #[serde(flatten)]
    pub min_length: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: EtunimetNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KilometersWithTrailer {
    #[serde(flatten)]
    pub kilometers_with_trailer: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseContractId {
    #[serde(flatten)]
    pub purchase_contract_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeginNotificationAllowed {
    #[serde(flatten)]
    pub begin_notification_allowed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionCountSummary {
    #[serde(flatten)]
    pub reduction_count_summary: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalAreaPercent {
    #[serde(flatten)]
    pub proposal_area_percent: ProposalAreaPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeCount {
    #[serde(flatten)]
    pub tree_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesAttributes {
    #[serde(flatten)]
    pub tree_species_attributes: TreeSpeciesAttributesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageSource {
    #[serde(flatten)]
    pub damage_source: FeatureTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningQuality {
    #[serde(flatten)]
    pub soil_conditioning_quality: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAcceptances {
    #[serde(flatten)]
    pub business_acceptances: BusinessAcceptancesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTreeSpeciesSummary {
    #[serde(flatten)]
    pub operation_tree_species_summary: OperationTreeSpeciesSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Caliper {
    #[serde(flatten)]
    pub caliper: CaliperType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataText {
    #[serde(flatten)]
    pub metadata_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingStumpCountText {
    #[serde(flatten)]
    pub remaining_stump_count_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchesInAdditionToCultivation {
    #[serde(flatten)]
    pub ditches_in_addition_to_cultivation: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotBasicData {
    #[serde(flatten)]
    pub sample_plot_basic_data: SamplePlotBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonRole {
    #[serde(flatten)]
    pub person_role: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageFinished {
    #[serde(flatten)]
    pub storage_finished: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundWoodSalesData {
    #[serde(flatten)]
    pub round_wood_sales_data: RoundWoodSalesDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportDistance {
    #[serde(flatten)]
    pub transport_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: NimilajiKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActData {
    #[serde(flatten)]
    pub financing_act_data: FinancingActDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountInSpot {
    #[serde(flatten)]
    pub amount_in_spot: PositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoRegenerationCommitmentType {
    #[serde(flatten)]
    pub co_regeneration_commitment_type: RegenerationCommitmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingQuality {
    #[serde(flatten)]
    pub stump_lifting_quality: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weight {
    #[serde(flatten)]
    pub weight: Integer7digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaliperApplication {
    #[serde(flatten)]
    pub caliper_application: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeListItem {
    #[serde(flatten)]
    pub tree_list_item: TreeListItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusTimestamp {
    #[serde(flatten)]
    pub status_timestamp: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterProtectionAction {
    #[serde(flatten)]
    pub water_protection_action: WorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirchPulp {
    #[serde(flatten)]
    pub birch_pulp: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningMethod {
    #[serde(flatten)]
    pub soil_conditioning_method: WorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingArea {
    #[serde(flatten)]
    pub working_area: WorkingAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationStands {
    #[serde(flatten)]
    pub financing_act_application_stands: FinancingActApplicationStandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRealEstates {
    #[serde(flatten)]
    pub declaration_real_estates: DeclarationRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calibration {
    #[serde(flatten)]
    pub calibration: CalibrationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankCode {
    #[serde(flatten)]
    pub bank_code: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyZone {
    #[serde(flatten)]
    pub subsidy_zone: ForestActAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MooseDamageDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherConservationMethodsText {
    #[serde(flatten)]
    pub other_conservation_methods_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StanfordResourceId {
    #[serde(flatten)]
    pub stanford_resource_id: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureOrder {
    #[serde(flatten)]
    pub silviculture_order: SilvicultureOrderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XsdateTime {
    #[serde(flatten)]
    pub xsdate_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityTag {
    #[serde(flatten)]
    pub entity_tag: EntityTagType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadTreeStrata {
    #[serde(flatten)]
    pub dead_tree_strata: DeadTreeStrataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetSelection {
    #[serde(flatten)]
    pub target_selection: TargetSelectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: ExtendedQuantityUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperativeTreeSpeciesData {
    #[serde(flatten)]
    pub operative_tree_species_data: TreeSpeciesDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvanceTax {
    #[serde(flatten)]
    pub advance_tax: AdvanceTaxType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidityDateBegin {
    #[serde(flatten)]
    pub validity_date_begin: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionGeometries {
    #[serde(flatten)]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationResponsible {
    #[serde(flatten)]
    pub forest_use_declaration_responsible: ForestUseDeclarationResponsibleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpPlannedOperationChains {
    #[serde(flatten)]
    pub op_planned_operation_chains: PlannedOperationChains,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Specifications {
    #[serde(flatten)]
    pub specifications: SpecificationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerRepresentativePerson {
    #[serde(flatten)]
    pub customer_representative_person: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlAdditionalInformation {
    #[serde(flatten)]
    pub control_additional_information: ControlAdditionalInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    #[serde(flatten)]
    pub category: ImageCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchCleaningBreakLocationText {
    #[serde(flatten)]
    pub ditch_cleaning_break_location_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileFormat {
    #[serde(flatten)]
    pub file_format: String5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisQualificationReasonText {
    #[serde(flatten)]
    pub dis_qualification_reason_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedVolume {
    #[serde(flatten)]
    pub planned_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurfaceDrainingAreas {
    #[serde(flatten)]
    pub surface_draining_areas: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingTrees {
    #[serde(flatten)]
    pub remaining_trees: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendNotifications {
    #[serde(flatten)]
    pub send_notifications: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataMooseDamageData {
    #[serde(flatten)]
    pub control_data_moose_damage_data: MooseDamageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectsRealizationData {
    #[serde(flatten)]
    pub objects_realization_data: ObjectsRealizationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReBaseRealEstateType2 {
    #[serde(flatten)]
    pub re_base_real_estate_type2: BaseRealEstateType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: CareOfTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MastoInspection {
    #[serde(flatten)]
    pub masto_inspection: VirtaMastoInspectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlPoint {
    #[serde(flatten)]
    pub gml_point: Point,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: VirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialFeaturesControl {
    #[serde(flatten)]
    pub special_features_control: ControlDataSpecialFeatureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Loads {
    #[serde(flatten)]
    pub loads: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SdsStemDistributionStrata {
    #[serde(flatten)]
    pub sds_stem_distribution_strata: StemDistributionStrata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyElementId {
    #[serde(flatten)]
    pub key_element_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultsOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub results_of_accessibility_analysis: ResultsOfAccessibilityAnalysisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialShipment {
    #[serde(flatten)]
    pub material_shipment: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinished {
    #[serde(flatten)]
    pub working_site_finished: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighStumps {
    #[serde(flatten)]
    pub high_stumps: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductivityUnit {
    #[serde(flatten)]
    pub productivity_unit: WorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStatus {
    #[serde(flatten)]
    pub project_status: VirtaProjectStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WstcrContract {
    #[serde(flatten)]
    pub wstcr_contract: Contract,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoBankAccount {
    #[serde(flatten)]
    pub co_bank_account: BankAccount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingRepresentatives {
    #[serde(flatten)]
    pub working_representatives: WorkingRepresentativesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateSeedlingsExitFromTreeNursery {
    #[serde(flatten)]
    pub date_seedlings_exit_from_tree_nursery: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertDate {
    #[serde(flatten)]
    pub insert_date: InsertDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreeGroups {
    #[serde(flatten)]
    pub spare_tree_groups: SpareTreeGroupsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsibleOfPreClearing {
    #[serde(flatten)]
    pub responsible_of_pre_clearing: ResponsibleOfPreClearingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionId {
    #[serde(flatten)]
    pub transmission_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentMatrixVolume {
    #[serde(flatten)]
    pub assortment_matrix_volume: AssortmentMatrixVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: EtuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlannedForHarvestingDate {
    #[serde(flatten)]
    pub working_site_planned_for_harvesting_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLotInformationValue {
    #[serde(flatten)]
    pub wood_lot_information_value: WoodLotInformationValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErosionBlockingActionText {
    #[serde(flatten)]
    pub erosion_blocking_action_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpolygonProperty {
    #[serde(flatten)]
    pub gmlpolygon_property: PolygonProperty,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actors {
    #[serde(flatten)]
    pub actors: ActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: KuvausTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilData {
    #[serde(flatten)]
    pub soil_data: BaseSoilDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionYear {
    #[serde(flatten)]
    pub completion_year: YearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectRealization {
    #[serde(flatten)]
    pub object_realization: ObjectRealizationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlStandBasicData {
    #[serde(flatten)]
    pub control_stand_basic_data: ControlStandBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeHealthCareInfo {
    #[serde(flatten)]
    pub employee_health_care_info: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemovalClass {
    #[serde(flatten)]
    pub removal_class: RemovalClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfoAsText {
    #[serde(flatten)]
    pub key_info_as_text: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TpTargetPart {
    #[serde(flatten)]
    pub tp_target_part: TargetPart,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationReference {
    #[serde(flatten)]
    pub completion_declaration_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Seller {
    #[serde(flatten)]
    pub seller: SellerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThinningTooExcessive {
    #[serde(flatten)]
    pub thinning_too_excessive: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: BankAccountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PulpWoodVolume {
    #[serde(flatten)]
    pub pulp_wood_volume: PulpWoodVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacAttorney {
    #[serde(flatten)]
    pub fac_attorney: Attorney,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStratum {
    #[serde(flatten)]
    pub tree_stratum: TreeStratum2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatLocation {
    #[serde(flatten)]
    pub habitat_location: HabitatLocationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GdtPointAndLineGeometriesGroup {
    #[serde(flatten)]
    pub gdt_point_and_line_geometries_group: PointAndLineGeometriesGroup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanAndSubsidy {
    #[serde(flatten)]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineCode {
    #[serde(flatten)]
    pub machine_code: MachineCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Languages {
    #[serde(flatten)]
    pub languages: LanguagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WholeName {
    #[serde(flatten)]
    pub whole_name: WholeNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GdtPolygonGeometry {
    #[serde(flatten)]
    pub gdt_polygon_geometry: PolygonGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalPulpwoodVolume {
    #[serde(flatten)]
    pub total_pulpwood_volume: PulpWoodVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct E101 {
    #[serde(flatten)]
    pub e101: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StormWorkingSite {
    #[serde(flatten)]
    pub storm_working_site: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessage {
    #[serde(flatten)]
    pub common_message: CommonMessageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCompactClasses {
    #[serde(flatten)]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilPreparationSpotsAreEnough {
    #[serde(flatten)]
    pub soil_preparation_spots_are_enough: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurerName {
    #[serde(flatten)]
    pub measurer_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialUnit {
    #[serde(flatten)]
    pub material_unit: MaterialUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedTotalSubsidy {
    #[serde(flatten)]
    pub decided_total_subsidy: DecidedTotalSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrContactRequest {
    #[serde(flatten)]
    pub cr_contact_request: ContactRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractWorkingSites {
    #[serde(flatten)]
    pub contract_working_sites: ContractWorkingSitesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: StoreyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActWorkCode {
    #[serde(flatten)]
    pub financing_act_work_code: FinancingActWorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: ExtendedMainGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasalAreaHardWood {
    #[serde(flatten)]
    pub basal_area_hard_wood: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementId {
    #[serde(flatten)]
    pub measurement_id: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlLineString {
    #[serde(flatten)]
    pub gml_line_string: LineString,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VirtaId {
    #[serde(flatten)]
    pub virta_id: VirtaIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationNotNeeded {
    #[serde(flatten)]
    pub forest_use_declaration_not_needed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: SamplePlotSizeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandData {
    #[serde(flatten)]
    pub ts_tree_stand_data: TreeStandData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavationSoilLocationText {
    #[serde(flatten)]
    pub excavation_soil_location_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpMeanDiameter {
    #[serde(flatten)]
    pub stump_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationAmountUnit {
    #[serde(flatten)]
    pub application_amount_unit: ForestCentreUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanVolume {
    #[serde(flatten)]
    pub mean_volume: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: StemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepresentativePerson {
    #[serde(flatten)]
    pub representative_person: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelLabel {
    #[serde(flatten)]
    pub parcel_label: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: PuhelinnumeroTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Biomass {
    #[serde(flatten)]
    pub biomass: BiomassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeOrGroundDamages {
    #[serde(flatten)]
    pub tree_or_ground_damages: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLots {
    #[serde(flatten)]
    pub wood_lots: WoodLotsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationsAndImprovementsText {
    #[serde(flatten)]
    pub notifications_and_improvements_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    #[serde(flatten)]
    pub resource: ResourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Percentage {
    #[serde(flatten)]
    pub percentage: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: StandQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionActors {
    #[serde(flatten)]
    pub completion_actors: CompletionActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(flatten)]
    pub description: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlReferenceMass {
    #[serde(flatten)]
    pub control_reference_mass: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeOrGroundDamagesText {
    #[serde(flatten)]
    pub tree_or_ground_damages_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncedArea {
    #[serde(flatten)]
    pub announced_area: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePreviousMessage {
    #[serde(flatten)]
    pub update_previous_message: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpCuttingAsInstructedText {
    #[serde(flatten)]
    pub stump_cutting_as_instructed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: VoimassaoloKytkinTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunningMeters {
    #[serde(flatten)]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad2Unit {
    #[serde(flatten)]
    pub work_load2_unit: WorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAcceptanceDate {
    #[serde(flatten)]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedAmount {
    #[serde(flatten)]
    pub decided_amount: DecidedAmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: String5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeRegistration {
    #[serde(flatten)]
    pub trade_registration: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    #[serde(flatten)]
    pub status: WorkingSiteStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaNo {
    #[serde(flatten)]
    pub area_no: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObject {
    #[serde(flatten)]
    pub child_object: ChildObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanningYear {
    #[serde(flatten)]
    pub planning_year: PlanningYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EarliestInspectionDate {
    #[serde(flatten)]
    pub earliest_inspection_date: DateMmDdYyyyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutOfServiceEndDate {
    #[serde(flatten)]
    pub out_of_service_end_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacCopOperationProject {
    #[serde(flatten)]
    pub fac_cop_operation_project: CopOperationProject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinishedDate {
    #[serde(flatten)]
    pub working_site_finished_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LargeSummaryReportRequired {
    #[serde(flatten)]
    pub large_summary_report_required: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerationData {
    #[serde(flatten)]
    pub regeneration_data: RegenerationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkPointCount {
    #[serde(flatten)]
    pub work_point_count: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditHarvesting {
    #[serde(flatten)]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_silviculture_info: CallForOfferSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestFundPayment {
    #[serde(flatten)]
    pub forest_fund_payment: ForestFundPaymentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: ValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestSeedlingDistance {
    #[serde(flatten)]
    pub nearest_seedling_distance: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WorkingSitePlanType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: HeightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorWrittenAgreement {
    #[serde(flatten)]
    pub sub_contractor_written_agreement: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecisionHandlers {
    #[serde(flatten)]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyData {
    #[serde(flatten)]
    pub forest_property_data: ForestPropertyDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingFinishedDate {
    #[serde(flatten)]
    pub harvesting_finished_date: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighStumpsText {
    #[serde(flatten)]
    pub high_stumps_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizedToSend {
    #[serde(flatten)]
    pub authorized_to_send: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderUserId {
    #[serde(flatten)]
    pub sender_user_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSystemName {
    #[serde(flatten)]
    pub external_system_name: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: YesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringDate {
    #[serde(flatten)]
    pub self_monitoring_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Width {
    #[serde(flatten)]
    pub width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractEndingDate {
    #[serde(flatten)]
    pub contract_ending_date: ContractEndingDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferBusinessSender {
    #[serde(flatten)]
    pub call_for_offer_business_sender: CallForOfferBusinessSenderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainReason {
    #[serde(flatten)]
    pub main_reason: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileBinary {
    #[serde(flatten)]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditName {
    #[serde(flatten)]
    pub final_audit_name: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Justification {
    #[serde(flatten)]
    pub justification: String5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeciduousTreeTargetDensityPercent {
    #[serde(flatten)]
    pub deciduous_tree_target_density_percent: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation3 {
    #[serde(flatten)]
    pub organisation3: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringBasicData {
    #[serde(flatten)]
    pub self_monitoring_basic_data: SelfMonitoringBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMeasurements {
    #[serde(flatten)]
    pub log_measurements: LogMeasurementsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractBeginningDate {
    #[serde(flatten)]
    pub contract_beginning_date: ContractBeginningDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociationInvolvement {
    #[serde(flatten)]
    pub association_involvement: VirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bulk {
    #[serde(flatten)]
    pub bulk: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationPolygon {
    #[serde(flatten)]
    pub declaration_polygon: DeclarationPolygonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute5 {
    #[serde(flatten)]
    pub attribute5: WorkCodeQualifierType5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hopper {
    #[serde(flatten)]
    pub hopper: HopperType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FccPaymentReference {
    #[serde(flatten)]
    pub fcc_payment_reference: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContarctorId {
    #[serde(flatten)]
    pub contarctor_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSites {
    #[serde(flatten)]
    pub offer_working_sites: OfferWorkingSitesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderEmail {
    #[serde(flatten)]
    pub sender_email: EmailAddressType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: String3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoosePercentage {
    #[serde(flatten)]
    pub moose_percentage: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlMultiSurface {
    #[serde(flatten)]
    pub gml_multi_surface: MultiSurface,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceNumber {
    #[serde(flatten)]
    pub sequence_number: SequenceNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacStartDate {
    #[serde(flatten)]
    pub fac_start_date: StartDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deliveries {
    #[serde(flatten)]
    pub deliveries: DeliveriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpTreatmentText {
    #[serde(flatten)]
    pub stump_treatment_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountUnit {
    #[serde(flatten)]
    pub amount_unit: AmountUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation4 {
    #[serde(flatten)]
    pub organisation4: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrTree {
    #[serde(flatten)]
    pub tr_tree: Tree,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    #[serde(flatten)]
    pub email: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub terrain_damage_outside_stand_evaluation: EvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xsstring {
    #[serde(flatten)]
    pub xsstring: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherHabitatCode {
    #[serde(flatten)]
    pub other_habitat_code: OtherHabitatCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertileType {
    #[serde(flatten)]
    pub fertile_type: MaterialCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentText {
    #[serde(flatten)]
    pub payment_text: String5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringData {
    #[serde(flatten)]
    pub self_monitoring_data: ForestCentreSelfMonitoringDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationOtherOperations {
    #[serde(flatten)]
    pub declaration_other_operations: DeclarationOtherOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitValue {
    #[serde(flatten)]
    pub unit_value: UnitPriceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacSubsidyAmount {
    #[serde(flatten)]
    pub fac_subsidy_amount: SubsidyAmount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareTreesFromMapSymbols {
    #[serde(flatten)]
    pub spare_trees_from_map_symbols: SpareTreesByCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessMessageTimeStamp {
    #[serde(flatten)]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningMethodCorrectText {
    #[serde(flatten)]
    pub soil_conditioning_method_correct_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: SpareTreesByCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Employment {
    #[serde(flatten)]
    pub employment: EmploymentDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacPayeesAndRealEstates {
    #[serde(flatten)]
    pub fac_payees_and_real_estates: PayeesAndRealEstates,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    #[serde(flatten)]
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeNameType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMainGroupType {
    #[serde(rename = "MainGroupType")]
    pub main_group_type: MainGroupType,
    #[serde(rename = "ExtraMainGroupType")]
    pub extra_main_group_type: ExtraMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeadTreeTypeType {
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
pub struct JhsSahkopostiosoiteTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccCostTypeDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBusinessAcceptanceStatusType {
    pub base: String,
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
pub struct DeliveriesType {
    #[serde(rename = "Delivery")]
    pub delivery: Vec<DeliveryType>,
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
pub struct CoMeanHeightType {
    #[serde(rename = "HeightType")]
    pub height_type: HeightType,
    #[serde(rename = "EmptyStringType")]
    pub empty_string_type: EmptyStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateYYYYMMOrYYYYMMDDType {
    #[serde(rename = "gMonthDay")]
    pub xsg_month_day: NaiveDate,
    #[serde(rename = "DateType")]
    pub date_type: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger5digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String100Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<StratumNumberType>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
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
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandsType {
    #[serde(rename = "FinancingActApplicationStand")]
    pub financing_act_application_stand: Vec<FinancingActApplicationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsCareOfTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectsType {
    #[serde(rename = "GeometryObject")]
    pub geometry_object: Vec<GeometryObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "Payments")]
    pub payments: PaymentsType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAreaTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodTypeType {
    pub base: String,
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
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<PointProperty>,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateType {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionGroup {
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<DateType>,
    #[serde(rename = "SilvicultureRestriction")]
    pub silviculture_restriction: SilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationGroup {
    #[serde(rename = "WoodLotInformationValue")]
    pub wood_lot_information_value: WoodLotInformationValueType,
    #[serde(rename = "WoodLotInformationType")]
    pub wood_lot_information_type: WoodLotInformationTypeType,
    #[serde(rename = "WoodLotInformationTypeDescription", skip_serializing_if = "Option::is_none")]
    pub wood_lot_information_type_description: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtClientApplicationIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<YesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<String200Type>,
    #[serde(rename = "LimitsToWaterSystem", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system: Option<YesNoType>,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub water_system_protection: Option<YesNoType>,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed: Option<YesNoType>,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient: Option<YesNoType>,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
    #[serde(rename = "StumpLiftingSuitable")]
    pub stump_lifting_suitable: YesNoType,
    #[serde(rename = "StumpLiftingSuitableText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_suitable_text: Option<String200Type>,
    #[serde(rename = "StumpCuttingAsInstructed")]
    pub stump_cutting_as_instructed: YesNoType,
    #[serde(rename = "StumpCuttingAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub stump_cutting_as_instructed_text: Option<String200Type>,
    #[serde(rename = "StumpTidiness", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness: Option<YesNoType>,
    #[serde(rename = "StumpTidinessText", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness_text: Option<String200Type>,
    #[serde(rename = "RemainingStumpCount", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count: Option<YesNoType>,
    #[serde(rename = "RemainingStumpCountText", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count_text: Option<String200Type>,
    #[serde(rename = "StumpLiftingQuality", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality: Option<WorkingQualityType>,
    #[serde(rename = "StumpLiftingQualityText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTerrainClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatePrecipionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<WorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDateType {
    #[serde(rename = "@type")]
    pub r#type: DatePrecipionType,
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreMessageReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingRealizationPracticeType {
    pub base: String,
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
pub struct ActingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandKeyGroup1 {
    #[serde(rename = "RealEstateId", skip_serializing_if = "Option::is_none")]
    pub real_estate_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "ParcelId", skip_serializing_if = "Option::is_none")]
    pub parcel_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "CompartmentId", skip_serializing_if = "Option::is_none")]
    pub compartment_id: Option<CoIdStringNotEmptyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMachineTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStringType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSamplePlotType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: PositiveInteger3digitsType,
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "MeasureDate")]
    pub measure_date: DateType,
    #[serde(rename = "Measurable")]
    pub measurable: YesNoType,
    #[serde(rename = "MeasurerId")]
    pub measurer_id: String20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "MeasurerName")]
    pub measurer_name: String50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: PointGeometryType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: YesNoType,
    #[serde(rename = "PilotName", skip_serializing_if = "Option::is_none")]
    pub pilot_name: Option<String50Type>,
    #[serde(rename = "FertileType", skip_serializing_if = "Option::is_none")]
    pub fertile_type: Option<MaterialCodeType>,
    #[serde(rename = "MeanVolume", skip_serializing_if = "Option::is_none")]
    pub mean_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Hoppers")]
    pub hoppers: HoppersType,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
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
pub struct CoMainGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDifficultyClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: StatisticsPurchaseModeCodeType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "StatisticsAssortmentCompactClasses")]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlkuPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSaveIncompleteType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary2Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<MeanAgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
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
pub struct SfFeatureDataGroup {
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaYesNoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnitType {
    #[serde(flatten)]
    pub base: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroupType {
    #[serde(flatten)]
    pub base: CoForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureExtraQualifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString2000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "Point")]
    pub gml_point: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiNameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgencyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageStateType {
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
pub struct ConsumptionUnitType {
    #[serde(flatten)]
    pub base: CoUnitPerHectareType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPreviousBlockStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "AverageVolume")]
    pub average_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVATStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSaveIncompleteType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoProposalTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarianceType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationRegenerationOperationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureRestrictionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDataType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
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
pub struct MaterialUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTimeStampType {
    pub base: NaiveDateTime,
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
pub struct EnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
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
pub struct WtcoMessageTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: OwsWorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPracticeType {
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
pub struct CddDistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: DistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHarvestingClassificationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaYesNoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListType {
    #[serde(rename = "ValueListItem")]
    pub value_list_item: Vec<ValueListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoLeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuvausTekstiTyyppi {
    pub base: String,
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
pub struct CoSoilTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionsType {
    #[serde(rename = "SilvicultureRestrictionDetails")]
    pub silviculture_restriction_details: Vec<SilvicultureRestrictionDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtVehicleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: DistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimumType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoNotNeededType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamagesType {
    #[serde(rename = "PreviousMooseDamage")]
    pub previous_moose_damage: Vec<PreviousMooseDamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysisType {
    #[serde(rename = "ResultOfAccessibilityAnalysis")]
    pub result_of_accessibility_analysis: Vec<ResultOfAccessibilityAnalysisType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainsType {
    #[serde(rename = "PlannedOperationChain")]
    pub planned_operation_chain: Vec<PlannedOperationChainType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XshexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger3digitsType {
    pub base: i32,
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
pub struct SellerGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
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
pub struct CoDecimal3And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctLoadRatingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "OfferWorkingSiteDetails")]
    pub offer_working_site_details: Vec<OfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawinghoursDataType {
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
    #[serde(rename = "Minutes")]
    pub minutes: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoWoodLotInformationTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsibleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHeightClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSubCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsingRightResponsibleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "Offer", skip_serializing_if = "Option::is_none")]
    pub o_offer: Option<Vec<Offer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionsMainGroup {
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction: Option<CuttingRestrictionType>,
    #[serde(rename = "CuttingRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction_ends: Option<DateType>,
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
pub struct LoggingAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCodeType {
    #[serde(rename = "CuttingRestrictionType")]
    pub cutting_restriction_type: CuttingRestrictionType,
    #[serde(rename = "SilvicultureRestrictionType")]
    pub silviculture_restriction_type: SilvicultureRestrictionType,
    #[serde(rename = "OtherRestrictionsType")]
    pub other_restrictions_type: OtherRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMachineManufacturerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EbEnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingPurposeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideDevelopmentClassType {
    #[serde(rename = "DevelopmentClassType")]
    pub development_class_type: DevelopmentClassType,
    #[serde(rename = "DevelopmentClassExtensionsType")]
    pub development_class_extensions_type: DevelopmentClassExtensionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPriorityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSmallWoodRemovalClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Name")]
    pub name: String50Type,
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Volume")]
    pub volume: Decimal6TotalDigitsType,
    #[serde(rename = "VolumeLeft")]
    pub volume_left: Decimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDigitPositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctMonthType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaDamageClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuntaNumeroTyyppi {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrataType {
    #[serde(rename = "StemDistributionStratum")]
    pub stem_distribution_stratum: Vec<StemDistributionStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPartNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaAdvertiserType {
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
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "PlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub op_planned_operation_chains: Option<PlannedOperationChains>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
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
pub struct JhsKuntaKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequests {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "ContactRequest")]
    pub cr_contact_request: Vec<ContactRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChangesType {
    #[serde(rename = "AssortmentsChange", skip_serializing_if = "Option::is_none")]
    pub assortments_change: Option<Vec<AssortmentChangeDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityType {
    #[serde(flatten)]
    pub base: EvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationNatureManagementSpecifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMainWorkCodeType {
    #[serde(rename = "WorkCodeType")]
    pub work_code_type: WorkCodeType,
    #[serde(rename = "WorkCodeGroupType")]
    pub work_code_group_type: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<BaseSoilDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType2 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "IncludedInOffer")]
    pub included_in_offer: IncludedInOfferType,
    #[serde(rename = "PurchaserRepresentativePerson")]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "OfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_wood_trade_info: Option<OfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_info: Option<OfferWorkingSiteSilvicultureInfoType>,
    #[serde(rename = "OfferWorkingSitePaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_payment_transactions: Option<WtcoOfferWorkingSitePaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<WorkingSiteNameType>,
    #[serde(rename = "SellerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub seller_representative_person: Option<WtcoSellerRepresentativePersonType>,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<WtcoSellersType>,
    #[serde(rename = "VATInfo", skip_serializing_if = "Option::is_none")]
    pub v_a_t_info: Option<WtcoVATInfoType>,
    #[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
    pub real_estate: Option<RealEstateType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<WtcoRoadUsingRightType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<WtcoCallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<Stands>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WtcoWorkingSiteGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCurrencyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessagesType {
    #[serde(rename = "ErrorMessageData")]
    pub error_message_data: Vec<ErrorMessageDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSahkoinenAsiointiTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger1digitsType {
    pub base: i32,
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
pub struct ValtiotunnusKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax2IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurerTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPaayksikkoNimiTyyppi {
    pub base: String,
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
pub struct PayeesType {
    #[serde(rename = "Payee")]
    pub payee: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistancesType {
    #[serde(rename = "StorageForestHaulageDistance")]
    pub storage_forest_haulage_distance: Vec<StorageForestHaulageDistanceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageDataType {
    #[serde(rename = "ReferenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceType>,
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
pub struct JhsMaatunnusKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPreclearingEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleAttrType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesType {
    #[serde(rename = "Machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<Vec<MachineTypeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
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
pub struct PricesAndCurrencyGroup {
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice")]
    pub total_price: TotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString2000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType2 {
    #[serde(flatten)]
    pub base: CoPositiveInteger5digitsType,
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
pub struct BdtImageSubCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesSummaryType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkinTyyppi {
    pub base: bool,
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
pub struct BearingCapacityClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationsType {
    #[serde(rename = "Organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<OrganizationType>>,
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
    pub estate_owner: Option<String>,
    #[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub municipality_number: Option<KuntaKoodiTyyppi>,
    #[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<String>,
    #[serde(rename = "UnseparatedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel_number: Option<String>,
    #[serde(rename = "UnseparatedParcel", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel: Option<String>,
    #[serde(rename = "KemeraMunicipalityId", skip_serializing_if = "Option::is_none")]
    pub kemera_municipality_id: Option<String>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<DateMmDdYyyyType>,
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
    pub extra_info: Option<VirtaExtraInfoType>,
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
pub struct BdtPositiveInteger5digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeType {
    #[serde(rename = "OriginalFeatureCodeType")]
    pub original_feature_code_type: OriginalFeatureCodeType,
    #[serde(rename = "FeatureCodeExtensionsType")]
    pub feature_code_extensions_type: FeatureCodeExtensionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperationsType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal4And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierBaseContactAndEstateInfoType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<PaymentsRealEstatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurposeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowsType {
    #[serde(rename = "RoundWoodSalesRow")]
    pub round_wood_sales_row: Vec<RoundWoodSalesRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<FinalAuditSpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityOfTreeSpeciesType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCuttingByMachineType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcType {
    #[serde(rename = "arcModel")]
    pub xlinkarc_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryObjectType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingSeasonType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstatesType {
    #[serde(rename = "FinancingActRealEstate")]
    pub financing_act_real_estate: Vec<FinancingActRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCleanlinessClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String3000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesType {
    #[serde(rename = "Trees")]
    pub trees: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtUserRoleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateRegisterUnitGroup {
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumberType,
    #[serde(rename = "MunicipalityName", skip_serializing_if = "Option::is_none")]
    pub municipality_name: Option<MunicipalityNameType>,
    #[serde(rename = "GroupNumber")]
    pub group_number: String,
    #[serde(rename = "AreaNumber")]
    pub area_number: String,
    #[serde(rename = "UnitNumber")]
    pub unit_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingDirectingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString5000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub cod_additional_details: Option<AdditionalDetails>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
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
pub struct CoForestCentreUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4FractionDigitsType {
    pub base: f64,
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
pub struct CoVirtaApprovalType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFeatureTypeType {
    pub base: String,
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
pub struct AssortmentMainGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectsType {
    #[serde(rename = "ChildObject")]
    pub child_object: Vec<ChildObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSumTableAreaType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelatedType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode1Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceGroup {
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteStateType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAgeType {
    #[serde(flatten)]
    pub base: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationsType {
    #[serde(rename = "SilviculturalOperation")]
    pub silvicultural_operation: Vec<SilviculturalOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkextendedModel {
    #[serde(rename = "Xlinkresource")]
    pub xlinkresource: String,
    #[serde(rename = "Xlinkarc")]
    pub xlinkarc: String,
    #[serde(rename = "Xlinklocator")]
    pub xlinklocator: String,
    #[serde(rename = "Xlinktitle")]
    pub xlinktitle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCasesType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<Vec<ForestDataUpdateUseCaseType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleAssortmentType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPercentWithFraction1Type {
    pub base: f64,
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
pub struct JhsHuoneistotunnisteNumeroTyyppi {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTotalEstimationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: Decimal1FractionDigitType,
    #[serde(rename = "Orientation")]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TgtVirtaExtraInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTextsType {
    #[serde(rename = "PaymentText")]
    pub payment_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeSpeciesType {
    #[serde(rename = "TreeSpeciesType")]
    pub co_tree_species_type: String,
    #[serde(rename = "EmptyStringType")]
    pub co_empty_string_type: CoEmptyStringType,
    #[serde(rename = "ExtraTreeSpeciesType")]
    pub co_extra_tree_species_type: CoExtraTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumberType {
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
pub struct DaysType {
    #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
    pub day: Option<Vec<DayType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO639char2LanguageType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
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
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: CallForOffer,
    #[serde(rename = "CFOWorkingSite")]
    pub cfows_c_f_o_working_site: CFOWorkingSite,
    #[serde(rename = "BusinessAcceptance")]
    pub ba_business_acceptance: BusinessAcceptance,
    #[serde(rename = "Payment")]
    pub wpp_payment: Payment,
    #[serde(rename = "MeasurementCertificate")]
    pub wpmc_measurement_certificate: MeasurementCertificate,
    #[serde(rename = "ForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: ForestUseDeclarationReferences,
    #[serde(rename = "Contract")]
    pub wstcr_contract: Contract,
    #[serde(rename = "Offer")]
    pub wsto_offer: Offer,
    #[serde(rename = "OWorkingSite")]
    pub ows_o_working_site: OWorkingSite,
    #[serde(rename = "Operations")]
    pub osu_operations: Operations,
    #[serde(rename = "ContactRequest")]
    pub cr_contact_request: ContactRequest,
    #[serde(rename = "MapSymbol")]
    pub ms_map_symbol: MapSymbol,
    #[serde(rename = "Acknowledge")]
    pub ack_acknowledge: Acknowledge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointType {
    #[serde(rename = "Diameter")]
    pub diameter: DiameterType,
    #[serde(rename = "CumulativeMass")]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTekstiTyyppi {
    pub base: String,
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
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: CfowsWorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesType {
    #[serde(rename = "Service")]
    pub service: Vec<OrganizationServiceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertisementDatingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStampType {
    pub base: NaiveDateTime,
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
pub struct CoVirtaStandQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDamageQualifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceType {
    #[serde(rename = "@message")]
    pub message: MessageTypeType,
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: String,
    #[serde(rename = "TimeStamp")]
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
pub struct SubGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatOperationsType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: SpecificationCodeType,
    #[serde(rename = "SpecificationText", skip_serializing_if = "Option::is_none")]
    pub specification_text: Option<CoString2000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotNeededType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrataType {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometries2Group {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<Vec<MaterialType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "SubsidyApplierReference")]
    pub subsidy_applier_reference: Vec<SubsidyApplierReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassExtensionsType {
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
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
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
pub struct GdtAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurementPlaceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationServiceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String10Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
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
pub struct CoMoneyTransactionCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReferenceType {
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
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: MessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<CiEmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassesType {
    #[serde(rename = "AssortmentClass")]
    pub assortment_class: Vec<AssortmentClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EbEnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingRestrictionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompanyModeType {
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
pub struct SahkopostiosoiteTekstiTyyppi {
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
pub struct UsedPricingMethodsType {
    #[serde(rename = "UsedPricingMethod")]
    pub used_pricing_method: Vec<UsedPricingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMaterialUnitType {
    pub base: String,
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
pub struct CoIdentifierTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDataGroup {
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "SelfMonitoringEvaluations", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_evaluations: Option<SelfMonitoringEvaluationsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicDataType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "ControlDataWaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub control_data_water_system_protection: Option<String>,
    #[serde(rename = "WorkingSiteFinalAuditRoadMaking", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_road_making: Option<WorkingSiteFinalAuditRoadMakingType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<BaseSoilDataType>,
    #[serde(rename = "SelfMonitoringBasicData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_basic_data: Option<SelfMonitoringBasicDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<RestrictionDataType>,
    #[serde(rename = "ControlEvaluations", skip_serializing_if = "Option::is_none")]
    pub control_evaluations: Option<ControlEvaluationsType>,
    #[serde(rename = "WorkingSiteFinalAuditDraining", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_draining: Option<WorkingSiteFinalAuditDrainingType>,
    #[serde(rename = "ControlDataSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub control_data_special_features: Option<ControlDataSpecialFeaturesType>,
    #[serde(rename = "MapSymbol", skip_serializing_if = "Option::is_none")]
    pub map_symbol: Option<MapSymbolType>,
    #[serde(rename = "SelfMonitoringObjectData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_object_data: Option<SelfMonitoringObjectDataType>,
    #[serde(rename = "ControlDataMooseDamageData", skip_serializing_if = "Option::is_none")]
    pub control_data_moose_damage_data: Option<MooseDamageDataType>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicDataType>,
    #[serde(rename = "ControlDataSwampForestManagement", skip_serializing_if = "Option::is_none")]
    pub control_data_swamp_forest_management: Option<ControlDataSwampForestManagementType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignDataType>,
    #[serde(rename = "DamageData", skip_serializing_if = "Option::is_none")]
    pub damage_data: Option<DamageDataType>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicDataType>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicDataType>,
    #[serde(rename = "SelfMonitoringObjectProtectionOperationsData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_object_protection_operations_data: Option<SelfMonitoringObjectProtectionOperationsDataType>,
    #[serde(rename = "ControlDataForestRoadConstruction", skip_serializing_if = "Option::is_none")]
    pub control_data_forest_road_construction: Option<ControlDataForestRoadConstructionType>,
    #[serde(rename = "ControlOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub control_overall_evaluation_data: Option<ControlOverallEvaluationDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "ControlDataRegeneration", skip_serializing_if = "Option::is_none")]
    pub control_data_regeneration: Option<RegenerationDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYearType {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<DocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationType {
    #[serde(rename = "OperationCode", skip_serializing_if = "Option::is_none")]
    pub operation_code: Option<ObjectProtectionOperationCodeType>,
    #[serde(rename = "OperationDescription", skip_serializing_if = "Option::is_none")]
    pub operation_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "FinancingActCompletionGeometry")]
    pub financing_act_completion_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLajiKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockInfoType {
    #[serde(rename = "PreviousBlock", skip_serializing_if = "Option::is_none")]
    pub previous_block: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoExtraTreeSpeciesType {
    pub base: String,
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
pub struct BdtAssortmentGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger2digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeType {
    pub base: String,
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
pub struct StorageDryingClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortERPIdType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditAnswerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreForestDataUpdateObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKatuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "Question")]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
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
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TreeStandSummary2Type>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub sd_trees: Option<Trees>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAmmattiKoodiTyyppi {
    pub base: String,
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
    pub working_site_text: Option<String1500Type>,
    #[serde(rename = "AssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolsType {
    #[serde(rename = "Symbol")]
    pub symbol: Vec<MapSymbolDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCodeType {
    #[serde(flatten)]
    pub base: String50Type,
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
pub struct EndDateType {
    #[serde(flatten)]
    pub base: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccForestCentreMessageReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItemType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TreeNumber", skip_serializing_if = "Option::is_none")]
    pub tree_number: Option<i32>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<DiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<MeanHeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "TreeIdentifiers", skip_serializing_if = "Option::is_none")]
    pub tree_identifiers: Option<TreeIdentifiersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileNameType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger6digitsType {
    pub base: i32,
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
pub struct KuntaKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotKnownType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalTextType {
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
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "OperationStatus")]
    pub operation_status: OperationStatusType,
    #[serde(rename = "ActingDate")]
    pub acting_date: ActingDateType,
    #[serde(rename = "ResponsibleActor", skip_serializing_if = "Option::is_none")]
    pub responsible_actor: Option<ResponsibleActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "DeclarationStandReference")]
    pub declaration_stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "CuttingPurpose")]
    pub cutting_purpose: CoCuttingPurposeType,
    #[serde(rename = "CuttingRealizationPractice")]
    pub cutting_realization_practice: CoCuttingRealizationPracticeType,
    #[serde(rename = "ForestDamageQualifier", skip_serializing_if = "Option::is_none")]
    pub forest_damage_qualifier: Option<CoForestDamageQualifierType>,
    #[serde(rename = "HabitatCode")]
    pub habitat_code: HabitatCodeType,
    #[serde(rename = "HabitatOperations", skip_serializing_if = "Option::is_none")]
    pub habitat_operations: Option<HabitatOperationsType>,
    #[serde(rename = "OtherHabitatCode", skip_serializing_if = "Option::is_none")]
    pub other_habitat_code: Option<OtherHabitatCodeType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "DeclarationMainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub declaration_main_tree_species: Option<TreeSpeciesConciseType>,
    #[serde(rename = "DeclarationDevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub declaration_development_class: Option<CoDeclarationDevelopmentClassType>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<AgeType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "DeclarationRegenerationCommitment", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_commitment: Option<RegenerationCommitmentType>,
    #[serde(rename = "DeclarationRegenerationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_operation: Option<CoDeclarationRegenerationOperationType>,
    #[serde(rename = "DeclarationSoilPreparationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_soil_preparation_operation: Option<CoDeclarationSoilPreparationOperationType>,
    #[serde(rename = "DeclarationOtherOperations", skip_serializing_if = "Option::is_none")]
    pub declaration_other_operations: Option<DeclarationOtherOperationsType>,
    #[serde(rename = "DeclarationStandTextInformation", skip_serializing_if = "Option::is_none")]
    pub declaration_stand_text_information: Option<CoString2000Type>,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
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
pub struct DistributionModelGroup {
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSBType,
    #[serde(rename = "Weibull")]
    pub weibull: WeibullType,
    #[serde(rename = "Normal")]
    pub normal: NormalType,
    #[serde(rename = "Gamma")]
    pub gamma: GammaType,
    #[serde(rename = "Beta")]
    pub beta: BetaType,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoPurchaseModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingStemTypeType {
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
pub struct CaliperType {
    #[serde(rename = "CaliperId", skip_serializing_if = "Option::is_none")]
    pub caliper_id: Option<String200Type>,
    #[serde(rename = "CaliperApplication", skip_serializing_if = "Option::is_none")]
    pub caliper_application: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: Vec<CallForOffer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNameType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesConciseType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoActionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeatureCodeType {
    #[serde(rename = "OriginalFeatureCodeType")]
    pub original_feature_code_type: OriginalFeatureCodeType,
    #[serde(rename = "FeatureCodeExtensionsType")]
    pub feature_code_extensions_type: FeatureCodeExtensionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyTransactionTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedModel {
    #[serde(rename = "Xlinkresource")]
    pub xlinkresource: String,
    #[serde(rename = "Xlinkarc")]
    pub xlinkarc: String,
    #[serde(rename = "Xlinklocator")]
    pub xlinklocator: String,
    #[serde(rename = "Xlinktitle")]
    pub xlinktitle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItemType {
    #[serde(rename = "AverageStemVolume")]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodiTyyppi {
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
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TreeStrata>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TreeStandSummary>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditAnswerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType1 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiEmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHarvestingAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString3000Type {
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
pub struct WideUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal1FractionDigitType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPlantEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentType {
    #[serde(rename = "SubsidyArgumentText")]
    pub subsidy_argument_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTargetPartStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtInteger7digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "MultiPolygon")]
    pub gml_multi_polygon: MultiPolygon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
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
pub struct SilvicultureRestrictionMainGroup {
    #[serde(rename = "SilvicultureRestrictions", skip_serializing_if = "Option::is_none")]
    pub silviculture_restrictions: Option<SilvicultureRestrictionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger4digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAmountUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationCommitmentType {
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
pub struct YesNoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMaxType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStandQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecisionTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpCuttingTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedHabitatCodeType {
    #[serde(rename = "HabitatCodeType")]
    pub habitat_code_type: HabitatCodeType,
    #[serde(rename = "VirtaHabitatCodeType")]
    pub virta_habitat_code_type: VirtaHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRatingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdStringType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionType {
    #[serde(rename = "Tree")]
    pub tree: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinklocatorType {
    #[serde(rename = "locatorModel")]
    pub xlinklocator_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumesType {
    #[serde(rename = "StemTypeVolume")]
    pub stem_type_volume: Vec<StemTypeVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType2 {
    pub base: String,
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
    pub power_of_attorney: Option<PowerOfAttorneyType>,
    #[serde(rename = "PowerOfAttorneyDate", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_date: Option<FccPowerOfAttorneyDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowsType {
    #[serde(rename = "LoggingsRow")]
    pub loggings_row: Vec<LoggingsRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityType {
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
pub struct JhsYritysTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMinType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeCuttingType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString50Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax5IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentreDecision")]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionType {
    #[serde(rename = "SceneryWorkPermissionNeeded")]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
    #[serde(rename = "SceneryWorkPermissionAcceptance", skip_serializing_if = "Option::is_none")]
    pub scenery_work_permission_acceptance: Option<CoDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPerHectareType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleEltType {
    #[serde(rename = "titleModel")]
    pub xlinktitle_model: String,
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
    #[serde(rename = "RoundWoodSalesRows", skip_serializing_if = "Option::is_none")]
    pub rws_round_wood_sales_rows: Option<RoundWoodSalesRows>,
    #[serde(rename = "OperationRows", skip_serializing_if = "Option::is_none")]
    pub operation_rows: Option<OperationRowsType>,
    #[serde(rename = "LoggingsRows", skip_serializing_if = "Option::is_none")]
    pub loggings_rows: Option<LoggingsRowsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeType {
    #[serde(rename = "WorkingCode")]
    pub working_code: WorkCodeType,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<DifficultyClassType>,
    #[serde(rename = "Attribute1", skip_serializing_if = "Option::is_none")]
    pub attribute1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "Attribute2", skip_serializing_if = "Option::is_none")]
    pub attribute2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "Attribute3", skip_serializing_if = "Option::is_none")]
    pub attribute3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "Attribute4", skip_serializing_if = "Option::is_none")]
    pub attribute4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "Attribute5", skip_serializing_if = "Option::is_none")]
    pub attribute5: Option<Vec<WorkCodeQualifierType5>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType4 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureDataGroup {
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPhaseType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentDetailsType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CaseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<FccFinancingActNumberType>,
    #[serde(rename = "ForestCentreMessageReferenceType", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference_type: Option<FccForestCentreMessageReferenceType>,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference: Option<ForestCentreMessageReference>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<CompletionDeclarationReference>,
    #[serde(rename = "CompletionDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_number: Option<CompletionDeclarationNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<CustomerReference>,
    #[serde(rename = "ArrivalDate")]
    pub fcc_arrival_date: ArrivalDate,
    #[serde(rename = "PaymentReference")]
    pub fcc_payment_reference: PaymentReference,
    #[serde(rename = "PaymentDate")]
    pub fcc_payment_date: PaymentDate,
    #[serde(rename = "BankAccount")]
    pub fcc_bank_account: BankAccount,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<ContactInformationType>,
    #[serde(rename = "ContactPerson")]
    pub fcc_contact_person: ContactPerson,
    #[serde(rename = "PaymentTexts", skip_serializing_if = "Option::is_none")]
    pub payment_texts: Option<PaymentTextsType>,
    #[serde(rename = "OverallTotalSubsidy")]
    pub fcc_overall_total_subsidy: OverallTotalSubsidy,
    #[serde(rename = "SubsidyAppliers")]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate")]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
    #[serde(rename = "FinancingActCompletionGeometries")]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSCCertificationSystemSubTypeType {
    pub base: String,
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
pub struct DeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: RealEstatesWithOwnersInformationType2,
    #[serde(rename = "ProcessingAreas")]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoRemovalClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsStatusryhmaTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteJakokirjainTekstiTyyppi {
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
pub struct AreaNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningDistrictType {
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
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
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
pub struct ControlEvaluationsType {
    #[serde(rename = "ControlEvaluation")]
    pub control_evaluation: Vec<ControlEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: PointProperty,
    #[serde(rename = "lineStringProperty")]
    pub gmlline_string_property: LineStringProperty,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSuggestionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoReportedStatisticsOperationTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordTreeSpeciesType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: RealEstatesWithOwnersInformationType2,
    #[serde(rename = "DeclarationPolygons")]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertiserType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSiviilisaatyTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfIdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataGroup {
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<WideDevelopmentClassType>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<StandQualityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGammaType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistanceType {
    #[serde(flatten)]
    pub base: CoDecimal4And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
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
pub struct SfLocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "AlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: GdtAlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostilokerolyhenneTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendarType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSellerResponsible {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingOriginType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extended {
    #[serde(rename = "extendedModel", skip_serializing_if = "Option::is_none")]
    pub xlinkextended_model: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReasonType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: CoIdStringNotEmptyType,
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
pub struct CoPositiveDecimalMax5IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativesType {
    #[serde(rename = "WorkingRepresentative")]
    pub working_representative: Vec<WorkingRepresentativeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMaterialUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureInfo")]
    pub feature_info: String500Type,
    #[serde(rename = "PointAndLineGeometriesGroup")]
    pub gdt_point_and_line_geometries_group: PointAndLineGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataType {
    #[serde(rename = "SoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub soil_data_group: Option<SoilDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1500Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSitePlanningOperationStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3TotalDigitsType {
    pub base: f64,
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
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: AlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xsbase64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestActAreaType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupsType {
    #[serde(rename = "SpareTreeGroup")]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestUseDeclarationResponsibleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionTypeType {
    pub base: String,
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
pub struct BdtString1000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
    #[serde(rename = "StemTypeType")]
    pub co_stem_type_type: CoStemTypeType,
    #[serde(rename = "ExtraStemTypeType")]
    pub co_extra_stem_type_type: CoExtraStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoNotKnownType {
    #[serde(rename = "YesNoType")]
    pub yes_no_type: YesNoType,
    #[serde(rename = "NotKnownType")]
    pub not_known_type: NotKnownType,
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
pub struct LoadRangeType {
    #[serde(rename = "StartLoadNumber")]
    pub start_load_number: PositiveInteger4digitsType,
    #[serde(rename = "EndLoadNumber")]
    pub end_load_number: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    pub base: String,
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
pub struct WctCommonMessageType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierBaseType {
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
pub struct VATRegistrationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsType {
    #[serde(rename = "Product")]
    pub product: Vec<ProductType>,
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
pub struct ControlEvaluationType {
    #[serde(rename = "EvaluationCategory", skip_serializing_if = "Option::is_none")]
    pub evaluation_category: Option<EvaluationSubjectType>,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<i32>,
    #[serde(rename = "EvaluationDescription", skip_serializing_if = "Option::is_none")]
    pub evaluation_description: Option<String1000Type>,
    #[serde(rename = "MainReason", skip_serializing_if = "Option::is_none")]
    pub main_reason: Option<YesNoType>,
    #[serde(rename = "ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<i32>,
    #[serde(rename = "ReasonDescription", skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiersType {
    #[serde(rename = "TreeIdentifier", skip_serializing_if = "Option::is_none")]
    pub tree_identifier: Option<Vec<TreeIdentifierType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString1500Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "FinancingActApplicationGeometry")]
    pub financing_act_application_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumeroTyyppi {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub fac_area_no: AreaNo,
    #[serde(rename = "FinancingActWorkCode")]
    pub fac_financing_act_work_code: FinancingActWorkCode,
    #[serde(rename = "PayeesAndRealEstates")]
    pub fac_payees_and_real_estates: PayeesAndRealEstates,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureIdentifierExtensionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger1digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemSelectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreementsType {
    #[serde(rename = "CollectiveAgreement")]
    pub collective_agreement: Vec<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoTotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger6digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstituteType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListType {
    #[serde(rename = "FeeBaseListItem")]
    pub fee_base_list_item: Vec<FeebaseListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetkiTyyppi {
    pub base: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationSoilPreparationOperationType {
    pub base: String,
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
pub struct ProposalAreaType {
    #[serde(flatten)]
    pub base: CoDecimal4And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSubGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeType {
    #[serde(rename = "OwnerShipTypeCode")]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatLocationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceType {
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
pub struct JhsFaksinumeroTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandsType {
    #[serde(rename = "FinancingActCompletionStand")]
    pub financing_act_completion_stand: Vec<FinancingActCompletionStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType1 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Description")]
    pub description: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Objects")]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionsType {
    #[serde(rename = "UsingRestriction")]
    pub using_restriction: Vec<UsingRestrictionType>,
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
    #[serde(rename = "ProductInstruction")]
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
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "StandNumber")]
    pub stand_number: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaInspectionTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceIdType {
    pub base: String,
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
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "Works", skip_serializing_if = "Option::is_none")]
    pub works: Option<WorksType>,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_plant_management: Option<SelfMonitoringWorkingSiteFinalAuditPlantManagementType>,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_plant_management: Option<SelfMonitoringWorkingSiteQualityControlPlantManagementType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<SelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<SelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<SelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<SelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<SelfMonitoringImageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditQuestionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesType {
    #[serde(rename = "OtherTreeSpecies")]
    pub other_tree_species: Vec<OtherTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoQuantityUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger1digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal1FractionDigitType {
    pub base: f64,
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
    pub grade_code: Option<GradeCodeType>,
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
pub struct BdtHeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsUlkomaaPostitoimipaikkaNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger2digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtInteger3digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReductionType {
    #[serde(rename = "StumpStemCount", skip_serializing_if = "Option::is_none")]
    pub stump_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StumpMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stump_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionsType {
    #[serde(rename = "CaseAction")]
    pub case_action: Vec<CaseActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeType {
    pub base: String,
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
pub struct StbStandNumberExtensionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType5 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteKirjainTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<CoReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType2,
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
pub struct CompletionActorType {
    #[serde(rename = "ActorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<CoIdStringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislationType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "CompensationDescription", skip_serializing_if = "Option::is_none")]
    pub compensation_description: Option<String>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValueType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(flatten)]
    pub base: CoMainTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "SoilDataGroup")]
    pub soil_data_group: SoilDataGroup,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<YearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: TreeStandBasedDataGroup,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: RestrictionsMainGroup,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaDecreaseType>,
    #[serde(rename = "PolygonGeometry")]
    pub gdt_polygon_geometry: PolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub gdt_multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagementType {
    #[serde(rename = "DitchMeanDepth")]
    pub ditch_mean_depth: String,
    #[serde(rename = "DitchMeanWidth")]
    pub ditch_mean_width: String,
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
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: String,
    #[serde(flatten)]
    pub base: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetPartStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString200Type {
    pub base: String,
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
pub struct ExtendedWideUnitType {
    #[serde(rename = "WideUnitType")]
    pub co_wide_unit_type: CoWideUnitType,
    #[serde(rename = "UnitPerHectareType")]
    pub co_unit_per_hectare_type: CoUnitPerHectareType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaReasonType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearingType {
    pub base: String,
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
    pub stem_count: Option<PositiveInteger6digitsType>,
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
    pub volume: Option<f64>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<PositiveInteger6digitsType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<PositiveInteger3digitsType>,
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
    pub main_group: Option<MainGroupType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<CoDrainageStateType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesType>,
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
    pub part_easting_coordinate: Option<String>,
    #[serde(rename = "PartNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_northing_coordinate: Option<String>,
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
    pub geometry_status: Option<String>,
    #[serde(rename = "GeometryId", skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<String>,
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<Polygon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsValiaikainenHenkiloTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreSelfMonitoringObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActorsType {
    #[serde(rename = "CompletionDeclarationActor")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlkuHetkiTyyppi {
    pub base: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesType {
    #[serde(rename = "Language")]
    pub language: Vec<LanguageCode1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaPlantEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate")]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
    #[serde(rename = "FinancingActApplicationGeometries")]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachineType {
    #[serde(rename = "MachineCode", skip_serializing_if = "Option::is_none")]
    pub machine_code: Option<MachineCodeType>,
    #[serde(rename = "MachineDescription", skip_serializing_if = "Option::is_none")]
    pub machine_description: Option<CoString500Type>,
    #[serde(rename = "MachineAccessoryCode", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_code: Option<MachineAccessoryCodeType>,
    #[serde(rename = "MachineAccessoryDescription", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_description: Option<CoString500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaLawType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType2 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActFinancingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceType {
    #[serde(rename = "StorageForestHaulageDistanceGroup")]
    pub storage_forest_haulage_distance_group: StorageForestHaulageDistanceGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString50Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaximumType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal2FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionContactInformationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FirstName")]
    pub first_name: FirstNameType,
    #[serde(rename = "LastName")]
    pub last_name: LastNameType,
    #[serde(rename = "PersonOrganizationName")]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostilokeroTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientTypeType {
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
pub struct BICType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPuhelinnumeroTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger4digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumberType {
    pub base: String,
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
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<Polygon>,
    #[serde(rename = "TargetParts", skip_serializing_if = "Option::is_none")]
    pub target_parts: Option<TargetPartsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPankkitiliTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkresourceType {
    #[serde(rename = "resourceModel")]
    pub xlinkresource_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaInspectionMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationsType {
    #[serde(rename = "DeclarationOtherOperation")]
    pub declaration_other_operation: Vec<CoDeclarationOtherOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsedPricingMethodTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayType {
    #[serde(rename = "CalendarDay")]
    pub calendar_day: DateType,
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
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
pub struct WorkingAreaType {
    #[serde(rename = "OperationalRegion")]
    pub operational_region: String50Type,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCertificationSystemType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString20Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateMmDdYyyyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotType {
    #[serde(rename = "WoodLotInformationGroup")]
    pub wood_lot_information_group: WoodLotInformationGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatorModel {
    #[serde(rename = "Xlinktitle", skip_serializing_if = "Option::is_none")]
    pub xlinktitle: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperationType {
    #[serde(flatten)]
    pub base: CoStatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDataType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditerTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6_2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrixType {
    #[serde(rename = "PriceItem")]
    pub price_item: Vec<PriceItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNimiTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal6TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsType {
    #[serde(rename = "SubContractor", skip_serializing_if = "Option::is_none")]
    pub sub_contractor: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDevelopmentClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StRestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: RestrictionsMainGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeeBasisValueType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YearType {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystemType {
    #[serde(rename = "CertificationSystemType")]
    pub certification_system_type: CertificationSystemType,
    #[serde(rename = "PEFCCertificationSystemSubTypeType")]
    pub p_e_f_c_certification_system_sub_type_type: PEFCCertificationSystemSubTypeType,
    #[serde(rename = "FSCCertificationSystemSubTypeType")]
    pub f_s_c_certification_system_sub_type_type: FSCCertificationSystemSubTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotKnownType {
    #[serde(rename = "YesNoType")]
    pub yes_no_type: YesNoType,
    #[serde(rename = "NotKnownType")]
    pub not_known_type: NotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsTurvakieltoKytkinTyyppi {
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRootRotControlEvaluationType {
    pub base: String,
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
pub struct PostinumeroKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUnitPerHectareType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsVoimassaoloKytkinTyyppi {
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMinType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneAndTelefaxGroup {
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "MobilePhoneNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<MobilePhoneNumberType>,
    #[serde(rename = "TelefaxNumber", skip_serializing_if = "Option::is_none")]
    pub telefax_number: Option<TelefaxNumberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(rename = "TreeSpeciesType")]
    pub co_tree_species_type: CoTreeSpeciesType,
    #[serde(rename = "EmptyStringType")]
    pub co_empty_string_type: CoEmptyStringType,
    #[serde(rename = "ExtraTreeSpeciesType")]
    pub co_extra_tree_species_type: CoExtraTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtThinningDistrictType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPublicityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: UseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfowsWorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "IncludedInOffer")]
    pub included_in_offer: IncludedInOfferType,
    #[serde(rename = "PurchaserRepresentativePerson")]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "OfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_wood_trade_info: Option<OfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_info: Option<OfferWorkingSiteSilvicultureInfoType>,
    #[serde(rename = "OfferWorkingSitePaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_payment_transactions: Option<WtcoOfferWorkingSitePaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<WorkingSiteNameType>,
    #[serde(rename = "SellerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub seller_representative_person: Option<WtcoSellerRepresentativePersonType>,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<WtcoSellersType>,
    #[serde(rename = "VATInfo", skip_serializing_if = "Option::is_none")]
    pub v_a_t_info: Option<WtcoVATInfoType>,
    #[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
    pub real_estate: Option<RealEstateType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<WtcoRoadUsingRightType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<WtcoCallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<Stands>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WtcoWorkingSiteGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityType {
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: ExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtServiceNameofAPIType {
    pub base: String,
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
pub struct SeedlingDataType {
    #[serde(rename = "SeedlingStratum")]
    pub seedling_stratum: Vec<SeedlingStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax1IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHenkiloTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumberType {
    pub base: String,
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
    pub fcc_customer_reference: Option<CustomerReference>,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_application_reference: Option<FinancingActApplicationReference>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<CompletionDeclarationReference>,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
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
    pub attorney: Option<ContactInformationType>,
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
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TreeStrata>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TreeStandSummary>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<StemDistribution>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<StemDistributionStrata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatAdvertisementType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoWideUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO3166char2CountryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactSoilTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHarvestingSeasonType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCaseType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
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
pub struct ScaleFactorTreeSpeciesType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTargetSelectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@versionNo")]
    pub version_no: VersionNoType,
    #[serde(rename = "TimeStamp")]
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
    #[serde(rename = "AssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
    #[serde(rename = "PaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub payment_transactions: Option<PaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsType {
    #[serde(rename = "Cutting")]
    pub cutting: Vec<CuttingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPurchaseModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRestrictionsType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType {
    #[serde(rename = "CostTypeNumber")]
    pub cost_type_number: CostTypeNumberType,
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
pub struct PaayksikkoNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActuateType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeedType {
    #[serde(flatten)]
    pub base: YesNoSellerResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationsType {
    #[serde(rename = "SelfMonitoringEvaluation")]
    pub self_monitoring_evaluation: Vec<SelfMonitoringEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandsType {
    #[serde(rename = "DeclarationStand")]
    pub declaration_stand: Vec<DeclarationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoDigitPositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistributionType {
    #[serde(rename = "CumulativePoint")]
    pub cumulative_point: Vec<CumulativePointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
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
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: ForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSyntymaPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPaymentType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
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
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString200Type {
    pub base: String,
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
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAddressGroup {
    #[serde(rename = "CountryText", skip_serializing_if = "Option::is_none")]
    pub country_text: Option<CountryTextType>,
    #[serde(rename = "StateText", skip_serializing_if = "Option::is_none")]
    pub state_text: Option<String200Type>,
    #[serde(rename = "PostalCode")]
    pub postal_code: String10Type,
    #[serde(rename = "PostOffice")]
    pub post_office: String50Type,
    #[serde(rename = "Address")]
    pub address: String50Type,
    #[serde(rename = "StateCode", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<StateCodeType>,
    #[serde(rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<ISO3166char2CountryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyAndPercentType {
    #[serde(rename = "AbsoluteQuantity")]
    pub absolute_quantity: AbsoluteQuantityType,
    #[serde(rename = "Percent")]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StemCount")]
    pub stem_count: PositiveInteger2digitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: Decimal1FractionDigitType,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCodeType {
    #[serde(rename = "CuttingDirectingType")]
    pub cutting_directing_type: CuttingDirectingType,
    #[serde(rename = "CuttingExtraQualifierType")]
    pub cutting_extra_qualifier_type: CuttingExtraQualifierType,
    #[serde(rename = "SilvicultureExtraQualifierType")]
    pub silviculture_extra_qualifier_type: SilvicultureExtraQualifierType,
    #[serde(rename = "CommonOperationExtraQualifierType")]
    pub common_operation_extra_qualifier_type: CommonOperationExtraQualifierType,
    #[serde(rename = "OperationNatureManagementSpecifierType")]
    pub operation_nature_management_specifier_type: OperationNatureManagementSpecifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTimeType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString100Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonOperationExtraQualifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeysType {
    #[serde(rename = "StandKeyGroup1")]
    pub stand_key_group1: StandKeyGroup1,
    #[serde(rename = "StandKeyGroup2")]
    pub stand_key_group2: StandKeyGroup2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaAdvertisementDatingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestObjectDataObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidityType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
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
pub struct ControlDataSpecialFeaturesType {
    #[serde(rename = "ControlDataSpecialFeature")]
    pub control_data_special_feature: Vec<ControlDataSpecialFeatureType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xmimebase64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    pub base: Vec<u8>,
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
pub struct ElectronicNotificationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<StratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "DistributionModelGroup")]
    pub cdd_distribution_model_group: CddDistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub co_bank_account: Option<BankAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString500Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: StbStandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HrefType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoChangeStateType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: GdtSimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString5Type {
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
pub struct BaseSoilDataGroup {
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageStateType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<ExtendedMainGroupType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclarationType {
    #[serde(rename = "CuttingRealizationPractice", skip_serializing_if = "Option::is_none")]
    pub cutting_realization_practice: Option<CuttingTypeType>,
    #[serde(rename = "HarvestingSignControlClassifier", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_control_classifier: Option<EvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeCharType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationNatureManagementSpecifierType {
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
pub struct TimeBetweenProposalYearsGroup {
    #[serde(rename = "MinProposalYear")]
    pub min_proposal_year: MinProposalYearType,
    #[serde(rename = "MaxProposalYear")]
    pub max_proposal_year: MaxProposalYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSukuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoObjectTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtQualitySystemType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncomeType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoExtraStemTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
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
pub struct TreeSummariesType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal2FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNimilajiKoodiTyyppi {
    pub base: String,
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
pub struct DamageDataType {
    #[serde(rename = "Damage")]
    pub damage: Vec<DamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedAmountType {
    #[serde(flatten)]
    pub base: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMaxType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageSubversionType {
    pub base: String,
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
pub struct VirtaHabitatAdvertisementType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStorageLandOwnerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBearingCapacityClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionAsUnitGroup {
    #[serde(rename = "QuantityUnit", skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<ExtendedQuantityUnitType>,
    #[serde(rename = "UnitValue", skip_serializing_if = "Option::is_none")]
    pub unit_value: Option<UnitPriceType>,
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<QuantityType>,
    #[serde(rename = "TotalValue", skip_serializing_if = "Option::is_none")]
    pub total_value: Option<TotalValueType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisksType {
    #[serde(rename = "WorkSafetyRiskDescription")]
    pub work_safety_risk_description: Vec<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtSimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataTreeStandDataDateType {
    #[serde(flatten)]
    pub base: TsTreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtScaleAssortmentType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummaryType {
    #[serde(rename = "OperativeTreeSpeciesData")]
    pub operative_tree_species_data: Vec<TsTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FourDigitPositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineManufacturerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax4IntegralPartMax2FractionalPartType {
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
pub struct ChildObjectType {
    #[serde(rename = "ChildObjectType")]
    pub child_object_type: CoObjectTypeType,
    #[serde(rename = "ChildObjectTypeSpecifier", skip_serializing_if = "Option::is_none")]
    pub child_object_type_specifier: Option<ObjectTypeSpecifierType>,
    #[serde(rename = "ChildObjectId")]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<PreferredContactingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralsType {
    #[serde(rename = "PeripheralCode", skip_serializing_if = "Option::is_none")]
    pub peripheral_code: Option<Vec<PeripheralCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaGroup {
    #[serde(rename = "ProposalAreaPercent", skip_serializing_if = "Option::is_none")]
    pub proposal_area_percent: Option<ProposalAreaPercentType>,
    #[serde(rename = "ProposalArea", skip_serializing_if = "Option::is_none")]
    pub proposal_area: Option<ProposalAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTimeStampType {
    pub base: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExceptionalPermitForHandlingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlersType {
    #[serde(rename = "DecisionHandler")]
    pub decision_handler: Vec<DecisionHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: Date,
    #[serde(rename = "EndDate")]
    pub end_date: Date,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsOsoiteNumeroTyyppi {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateType {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEmptyStringType {
    pub base: String,
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
pub struct ControlBasicDataType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<UseCaseType>,
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
pub struct ParcelType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: ParcelNumberType,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformationType {
    #[serde(rename = "ResponsibleOfPreClearing", skip_serializing_if = "Option::is_none")]
    pub responsible_of_pre_clearing: Option<ResponsibleOfPreClearingType>,
    #[serde(rename = "PreClearingExecutionTime", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_execution_time: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaLawType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeStandDataMomentType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourcesType {
    #[serde(rename = "AuditionResource", skip_serializing_if = "Option::is_none")]
    pub audition_resource: Option<AuditionResourceType>,
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
    pub planned_stem_count: Option<StemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType5 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOtherPublicSubstituteType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringUseCaseType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorsType {
    #[serde(rename = "Actor")]
    pub actor: Vec<ActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: PointProperty,
    #[serde(rename = "lineStringProperty")]
    pub gmlline_string_property: LineStringProperty,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcModel {
    #[serde(rename = "Xlinktitle", skip_serializing_if = "Option::is_none")]
    pub xlinktitle: Option<Vec<String>>,
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
pub struct UlkomaaPostitoimipaikkaNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPointType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSukupuoliKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureExtraQualifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSitePlanningStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecisionGeometryObjectType {
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
pub struct PreferredContactingMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer3digitsType {
    pub base: i32,
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
    #[serde(rename = "ResourceName")]
    pub resource_name: String50Type,
    #[serde(rename = "WorkingHoursBusinessDay")]
    pub working_hours_business_day: PositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSaturday")]
    pub working_hours_saturday: PositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSunday")]
    pub working_hours_sunday: PositiveInteger2digitsType,
    #[serde(rename = "Days")]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccPowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLocationType {
    pub base: String,
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
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<CoDateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
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
    #[serde(rename = "CallForOfferWorkingSites")]
    pub ws_call_for_offer_working_sites: CallForOfferWorkingSites,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStemTypeType {
    #[serde(rename = "StemTypeType")]
    pub co_stem_type_type: String,
    #[serde(rename = "ExtraStemTypeType")]
    pub co_extra_stem_type_type: CoExtraStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateMmDdYyyyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger3digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTurningPointClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonType {
    #[serde(rename = "ReasonCode")]
    pub reason_code: String10Type,
    #[serde(rename = "ReasonDescription")]
    pub reason_description: CoString1000Type,
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
pub struct CoDecimal7And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationSubjectType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometriesType {
    #[serde(rename = "DecisionGeometry")]
    pub decision_geometry: Vec<DecisionGeometryType>,
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
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataGroup {
    #[serde(rename = "MainGroup")]
    pub main_group: ExtendedMainGroupType,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageStateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwnersType {
    #[serde(rename = "RealEstateOwner")]
    pub real_estate_owner: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPaymentTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentWithFraction2Type {
    pub base: f64,
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
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: ForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StBaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<BaseSoilDataGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger2digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFertilityClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
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
pub struct CoElectronicNotificationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetSelectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
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
    pub planned_stem_count: Option<StemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationType {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: TimeStampType,
    #[serde(rename = "CalibrationAdjustment")]
    pub calibration_adjustment: PositiveInteger3digitsType,
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
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger2digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: PositiveInteger2digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    #[serde(flatten)]
    pub base: StbStandNumberExtensionType,
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
pub struct HenkiloTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYearType {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDensityClassType {
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
pub struct BusinessAcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: ForestCentreWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: ForestCentreWorkCodeType,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
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
pub struct NeljasRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYesNoNotKnownType {
    #[serde(rename = "YesNoType")]
    pub yes_no_type: YesNoType,
    #[serde(rename = "NotKnownType")]
    pub not_known_type: NotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePriorityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperationType {
    pub base: String,
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
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDateType {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsIkaluokkaTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionType {
    #[serde(rename = "RestrictionType")]
    pub co_restriction_type: RestrictionType,
    #[serde(rename = "RestrictionCode")]
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
pub struct PersonNameGroup {
    #[serde(rename = "WholeName")]
    pub whole_name: WholeNameType,
    #[serde(rename = "PersonOrganizationName", skip_serializing_if = "Option::is_none")]
    pub person_organization_name: Option<PersonOrganizationNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "FinancingActApplicationReference")]
    pub fac_financing_act_application_reference: FinancingActApplicationReference,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<FinancingActNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<FinancingActApplicationTextInformation>,
    #[serde(rename = "Language")]
    pub fac_language: Language,
    #[serde(rename = "Sender")]
    pub fac_sender: Sender,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "SentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "ElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumberType,
    #[serde(rename = "EstimatedStartDate")]
    pub fac_estimated_start_date: EstimatedStartDate,
    #[serde(rename = "EstimatedEndDate")]
    pub fac_estimated_end_date: EstimatedEndDate,
    #[serde(rename = "SubsidyAmount")]
    pub fac_subsidy_amount: SubsidyAmount,
    #[serde(rename = "FinancingActWorkGroup")]
    pub fac_financing_act_work_group: FinancingActWorkGroup,
    #[serde(rename = "CopOperationProject")]
    pub fac_cop_operation_project: CopOperationProject,
    #[serde(rename = "FinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "ApplicationActors")]
    pub fac_application_actors: ApplicationActors,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPreventionSubstanceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: JhsAlkuPvmTyyppi,
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
pub struct String200Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTotalEstimationType {
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
pub struct FinancingActWorkGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebaseListItemType {
    #[serde(rename = "Id")]
    pub id: PositiveIntegerType,
    #[serde(rename = "FeeValue")]
    pub fee_value: String10Type,
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
pub struct JhsBICKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestryType {
    #[serde(rename = "BlockIsFSFBlock", skip_serializing_if = "Option::is_none")]
    pub block_is_f_s_f_block: Option<YesNoType>,
    #[serde(rename = "FSFInformation", skip_serializing_if = "Option::is_none")]
    pub f_s_f_information: Option<Vec<FSFInformationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpSilvicultureTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcroleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedAmountUnitType {
    #[serde(flatten)]
    pub base: CoForestCentreUnitType,
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
pub struct ValiaikainenHenkiloTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFertilityClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTreeDecimalType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType3 {
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
pub struct CostTypeNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityGroup {
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: ExtendedQuantityUnitType,
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsValtiotunnusKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<LocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionsType {
    #[serde(rename = "DiameterSection")]
    pub diameter_section: Vec<SectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItemType {
    #[serde(rename = "ListId")]
    pub list_id: PositiveIntegerType,
    #[serde(rename = "ListItem")]
    pub list_item: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysisType {
    #[serde(rename = "Accessibility")]
    pub accessibility: AccessibilityType,
    #[serde(rename = "Percentage")]
    pub percentage: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliersType {
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<SubsidyApplierBaseContactAndEstateInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactStandBasicDataType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<YearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaMastoInspectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorsType {
    #[serde(rename = "ScaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<Vec<ScaleFactorDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromType {
    pub base: String,
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
pub struct FeeBasisListType {
    #[serde(rename = "FeeBasisListItem")]
    pub fee_basis_list_item: Vec<FeebasisListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeasurementCertificateTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumeroTyyppi {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
    #[serde(flatten)]
    pub base: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingWeightType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingClassificationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsViidesRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCertificationSystemType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferTextType {
    #[serde(flatten)]
    pub base: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreaType {
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsType {
    #[serde(rename = "Payment")]
    pub payment: Vec<ForestCentrePaymentDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<FinancingActApplicationStands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPercentType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SpecialFeatureType>,
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
pub struct MunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPeripheralCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisValueType {
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
pub struct JhsLajiTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExtraInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpVirtaEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDeltaType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageVersionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoRoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKeyType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
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
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
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
pub struct ToType {
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
pub struct StandInfoType {
    #[serde(flatten)]
    pub base: StbStandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoltosuhdeTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedCompletionDataType {
    #[serde(rename = "OperationStatus", skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<ControlDataOperationStatusType>,
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResourceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString500Type {
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
pub struct WctHopperTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "TreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<TreeStandDataDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesType {
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<StorageType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesConciseType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<CoHeightClassType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<HeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<CoDiameterClassType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<DiameterType>,
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
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMainTypeType {
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
pub struct StatisticsPurchaseModeCodeType {
    #[serde(rename = "PurchaseModeType")]
    pub purchase_mode_type: PurchaseModeType,
    #[serde(rename = "StatisticsPurchaseModeType")]
    pub statistics_purchase_mode_type: StatisticsPurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSellerGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDaysAreaType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitesType {
    #[serde(rename = "WorkingSite", skip_serializing_if = "Option::is_none")]
    pub working_site: Option<Vec<WorkingSiteType>>,
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
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeType {
    #[serde(rename = "WorkingCode")]
    pub working_code: String,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<DifficultyClassType>,
    #[serde(rename = "Attribute1", skip_serializing_if = "Option::is_none")]
    pub attribute1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "Attribute2", skip_serializing_if = "Option::is_none")]
    pub attribute2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "Attribute3", skip_serializing_if = "Option::is_none")]
    pub attribute3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "Attribute4", skip_serializing_if = "Option::is_none")]
    pub attribute4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "Attribute5", skip_serializing_if = "Option::is_none")]
    pub attribute5: Option<Vec<WorkCodeQualifierType5>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermissionType {
    #[serde(flatten)]
    pub base: CoYesNoType,
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
pub struct WtcoQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "AlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: GdtAlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightResponsibleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperationStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeGroupType {
    pub base: String,
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
pub struct ResourceTypeType {
    #[serde(rename = "PersonResourceType")]
    pub person_resource_type: PersonResourceType,
    #[serde(rename = "MachineTypeType")]
    pub machine_type_type: MachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbAreaDecreaseType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummaryType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "MeanAge")]
    pub mean_age: MeanAgeType,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
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
pub struct CostTypeDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaPhaseType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRegenerationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType2 {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensityGroup {
    #[serde(rename = "TargetDensity", skip_serializing_if = "Option::is_none")]
    pub target_density: Option<PositiveInteger5digitsType>,
    #[serde(rename = "DeciduousTreeTargetDensityPercent", skip_serializing_if = "Option::is_none")]
    pub deciduous_tree_target_density_percent: Option<PercentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtOrderStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: SimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfaType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanlinessClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoppersType {
    #[serde(rename = "Hopper", skip_serializing_if = "Option::is_none")]
    pub hopper: Option<Vec<HopperType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationRoleType {
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
    pub document: Option<Vec<u8>>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<u8>>,
    #[serde(rename = "DisQualificationReasons")]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwsWorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "IncludedInOffer")]
    pub included_in_offer: IncludedInOfferType,
    #[serde(rename = "PurchaserRepresentativePerson")]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "OfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_wood_trade_info: Option<OfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_info: Option<OfferWorkingSiteSilvicultureInfoType>,
    #[serde(rename = "OfferWorkingSitePaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_payment_transactions: Option<WtcoOfferWorkingSitePaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<WorkingSiteNameType>,
    #[serde(rename = "SellerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub seller_representative_person: Option<WtcoSellerRepresentativePersonType>,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<WtcoSellersType>,
    #[serde(rename = "VATInfo", skip_serializing_if = "Option::is_none")]
    pub v_a_t_info: Option<WtcoVATInfoType>,
    #[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
    pub real_estate: Option<RealEstateType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<WtcoRoadUsingRightType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<WtcoCallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<Stands>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WtcoWorkingSiteGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsPurchaseModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocationsType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceLocations", skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<ResourceLocationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassifiedType {
    #[serde(rename = "AssortmentVolumeUnclassified")]
    pub assortment_volume_unclassified: Vec<AssortmentVolumeUnclassifiedType>,
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
pub struct HelpGeometriesType {
    #[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_polygon_geometry: Option<Vec<PolygonGeometry>>,
    #[serde(rename = "LineGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_line_geometry: Option<Vec<LineGeometry>>,
    #[serde(rename = "PointGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_point_geometry: Option<Vec<PointGeometry>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "MultiPolygon")]
    pub gml_multi_polygon: MultiPolygon,
    #[serde(rename = "MultiSurface")]
    pub gml_multi_surface: MultiSurface,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferType {
    #[serde(rename = "RelatedCallForOfferId")]
    pub related_call_for_offer_id: String,
    #[serde(rename = "RelatedCallForOfferDescription", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offer_description: Option<String1500Type>,
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
pub struct TransactionQuantityUnitType {
    #[serde(rename = "WideUnitType")]
    pub wide_unit_type: WideUnitType,
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
pub struct StatusryhmaTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementsType {
    #[serde(rename = "Measurement")]
    pub measurement: Vec<MeasurementDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCuttingPurposeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationOtherOperationType {
    pub base: String,
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
pub struct HabitatTypeType {
    pub base: String,
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
pub struct WtcoVATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessagesType {
    #[serde(rename = "CommonMessage", skip_serializing_if = "Option::is_none")]
    pub common_message: Option<Vec<CommonMessageDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumberType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger4digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctResponsibleOfPreClearingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsedPricingMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotsType {
    #[serde(rename = "Woodlot")]
    pub woodlot: Vec<WoodLotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorsType {
    #[serde(rename = "ApplicationActor")]
    pub application_actor: Vec<ApplicationActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataMomentType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonsType {
    #[serde(rename = "Reason")]
    pub reason: Vec<ReasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseCompactStandBasicDataType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<StandNumberType>,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<YearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data_date: Option<StandBasicDataDateType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderRoleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClassType {
    pub base: String,
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
    pub product_key_group: KeyGroup,
    #[serde(rename = "ProductName")]
    pub product_name: CoString500Type,
    #[serde(rename = "Quantity")]
    pub quantity: Decimal2FractionDigitsType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: WideUnitType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<TotalPriceType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "Consumption", skip_serializing_if = "Option::is_none")]
    pub consumption: Option<ConsumptionType>,
    #[serde(rename = "ConsumptionUnit", skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<ConsumptionUnitType>,
    #[serde(rename = "PlannedResource", skip_serializing_if = "Option::is_none")]
    pub planned_resource: Option<PlannedResourceType>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String20Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaProjectStatusType {
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
pub struct HuoneistotunnisteNumeroTyyppi {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlerType {
    #[serde(rename = "@role")]
    pub role: String100Type,
    #[serde(flatten)]
    pub base: CiContactInformationType,
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
pub struct JhsKuolemaPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationUrgencyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameofAPIType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYesNoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    pub base: String,
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
pub struct ChangeStateType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowsType {
    #[serde(rename = "OperationRow")]
    pub operation_row: Vec<OperationRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "Tree")]
    pub tr_tree: Vec<Tree>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotEmptyTreeSpeciesType {
    #[serde(rename = "TreeSpeciesType")]
    pub co_tree_species_type: CoTreeSpeciesType,
    #[serde(rename = "ExtraTreeSpeciesType")]
    pub co_extra_tree_species_type: CoExtraTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureType {
    #[serde(rename = "FeatureDataGroup")]
    pub sf_feature_data_group: SfFeatureDataGroup,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub sf_feature_info: Option<FeatureInfo>,
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
pub struct LoppuHetkiTyyppi {
    pub base: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaRegenerationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStatisticsOperationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageType {
    #[serde(rename = "MainDamage", skip_serializing_if = "Option::is_none")]
    pub main_damage: Option<YesNoType>,
    #[serde(rename = "DamageSourceCode", skip_serializing_if = "Option::is_none")]
    pub damage_source_code: Option<String>,
    #[serde(rename = "DamageSourceDescription", skip_serializing_if = "Option::is_none")]
    pub damage_source_description: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaMastoInspectionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCallForOfferBusinessSenderRoleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeSpecifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraStemTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoninessType {
    pub base: String,
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
pub struct CoQuantityUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandBasicDataDateType {
    #[serde(flatten)]
    pub base: Date,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulksType {
    #[serde(rename = "StemTypeBulk", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulk: Option<Vec<StemTypeBulkType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstructionType {
    #[serde(rename = "AppliedLength")]
    pub applied_length: Vec<Decimal6_2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(flatten)]
    pub base: CoAreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSpecialFeatureIdentifierExtensionType {
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
pub struct CoAgeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompanyTypeType {
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
pub struct PaidValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax1IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetailsType {
    #[serde(rename = "SilvicultureRestrictionGroup")]
    pub silviculture_restriction_group: SilvicultureRestrictionGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsGPSlocationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientApplicationIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbAreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrataType {
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Vec<DeadTreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoProviderRoleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub fac_update_previous_message: UpdatePreviousMessage,
    #[serde(rename = "CompletionDeclarationReference")]
    pub fac_completion_declaration_reference: CompletionDeclarationReference,
    #[serde(rename = "FinancingActNumber")]
    pub fac_financing_act_number: FinancingActNumber,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_reference: Option<FinancingActApplicationReference>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<CustomerReference>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<BankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: CoYesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: CoYesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_declaration_text_information: Option<FinancingActCompletionDeclarationTextInformation>,
    #[serde(rename = "Language")]
    pub fac_language: Language,
    #[serde(rename = "ElectronicNotification")]
    pub fac_electronic_notification: ElectronicNotification,
    #[serde(rename = "Sender")]
    pub fac_sender: Sender,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<Attorney>,
    #[serde(rename = "SentDate")]
    pub fac_sent_date: SentDate,
    #[serde(rename = "StartDate")]
    pub fac_start_date: StartDate,
    #[serde(rename = "EndDate")]
    pub fac_end_date: EndDate,
    #[serde(rename = "FinancingActRealEstates")]
    pub fac_financing_act_real_estates: FinancingActRealEstates,
    #[serde(rename = "CompletionDeclarationActors")]
    pub fac_completion_declaration_actors: CompletionDeclarationActors,
    #[serde(rename = "WorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub fac_working_representatives: Option<WorkingRepresentatives>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
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
pub struct FertilityClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3_1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestActAreaType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataOperationStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStorageDryingClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostinumeroKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeasurementMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<ForestUseDeclarationReference>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<AssortmentClasses>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CuttingPlannerLiability", skip_serializing_if = "Option::is_none")]
    pub cutting_planner_liability: Option<CuttingPlannerLiabilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaStandQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger3digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvmTyyppi {
    pub base: NaiveDate,
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
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: PositiveInteger3digitsType,
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "MeasureDate")]
    pub measure_date: DateType,
    #[serde(rename = "Measurable")]
    pub measurable: YesNoType,
    #[serde(rename = "MeasurerId")]
    pub measurer_id: String20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "MeasurerName")]
    pub measurer_name: String50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: PointGeometryType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: YesNoType,
    #[serde(rename = "PilotName", skip_serializing_if = "Option::is_none")]
    pub pilot_name: Option<String50Type>,
    #[serde(rename = "FertileType", skip_serializing_if = "Option::is_none")]
    pub fertile_type: Option<MaterialCodeType>,
    #[serde(rename = "MeanVolume", skip_serializing_if = "Option::is_none")]
    pub mean_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Hoppers")]
    pub hoppers: HoppersType,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<RealEstates>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreasType {
    #[serde(rename = "FulfilledArea")]
    pub fulfilled_area: Vec<FulfilledAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoRestrictionBasedOnStoninessType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSelfMonitoringTypeType {
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
    pub stand_basic_data: Option<StandBasicDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectingMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsToinenRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Group {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationStatusType {
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
pub struct CallForOfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaExceptionalPermitForHandlingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<ForestUseDeclarationReference>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<AssortmentClasses>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreasType {
    #[serde(rename = "WorkingArea", skip_serializing_if = "Option::is_none")]
    pub working_area: Option<Vec<WorkingAreaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSeedlingOriginType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
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
pub struct BdtDryingClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PEFCCertificationSystemSubTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstatesType {
    #[serde(rename = "PayeeAndRealEstate")]
    pub payee_and_real_estate: Vec<PayeeAndRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "SubsidyApplierReferenceList")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "AlternativeIdentifier")]
    pub alternative_identifier: AlternativeIdentifierType,
    #[serde(rename = "AlternativeName", skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<AlternativeNameType>,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType4 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString25Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbIdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationSpecificationType {
    #[serde(rename = "SilvicultureExtraQualifierType")]
    pub silviculture_extra_qualifier_type: SilvicultureExtraQualifierType,
    #[serde(rename = "CommonOperationExtraQualifierType")]
    pub common_operation_extra_qualifier_type: CommonOperationExtraQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiversType {
    #[serde(rename = "DecisionReceiver")]
    pub decision_receiver: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingExtraQualifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaCuttingByMachineType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTypeType {
    #[serde(rename = "CuttingTypeType")]
    pub co_cutting_type_type: CoCuttingTypeType,
    #[serde(rename = "SilvicultureTypeType")]
    pub co_silviculture_type_type: CoSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTekstiTyyppi {
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
pub struct OperationStateType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedQuantityUnitType {
    #[serde(rename = "QuantityUnitType")]
    pub quantity_unit_type: QuantityUnitType,
    #[serde(rename = "StatisticsQuantityUnitType")]
    pub statistics_quantity_unit_type: StatisticsQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
    #[serde(rename = "Message")]
    pub message: PayloadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "Payees")]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSumTableAreaType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkarcType {
    #[serde(rename = "arcModel")]
    pub xlinkarc_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimiTyyppi {
    pub base: String,
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
pub struct CuttingDirectingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSpareTreeCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiabilityType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: GdtAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDiameterClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPaymentType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemType {
    pub base: String,
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
    pub sender: ContactInformationType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fcc_attorney: Option<Attorney>,
    #[serde(rename = "MooseDamageDeclarationRealEstates")]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
    #[serde(rename = "CompensationByInsurance")]
    pub compensation_by_insurance: CompensationByInsuranceType,
    #[serde(rename = "CompensationByLegislation")]
    pub compensation_by_legislation: CompensationByLegislationType,
    #[serde(rename = "PreviousMooseDamages", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damages: Option<PreviousMooseDamagesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformationType {
    #[serde(rename = "AuthorizationToSendWsoInformation")]
    pub authorization_to_send_wso_information: Vec<AuthorizationToSendWsoInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionTypeType {
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
pub struct PlanningYearAndOperationUrgencyGroup {
    #[serde(rename = "PlanningYear")]
    pub planning_year: PlanningYearType,
    #[serde(rename = "OperationUrgency")]
    pub operation_urgency: OperationUrgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFinancingActFinancingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMassType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal3FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlanType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestOwnerGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIdentifierGroup {
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<AssortmentCodeType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "AssortmentMainGroup", skip_serializing_if = "Option::is_none")]
    pub assortment_main_group: Option<AssortmentMainGroupType>,
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<String50Type>,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "Envelope")]
    pub envl_envelope: Vec<Envelope>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal3FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5Type {
    pub base: String,
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
pub struct StandTreeCuttingType {
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonsType {
    #[serde(rename = "DisQualificationReason", skip_serializing_if = "Option::is_none")]
    pub dis_qualification_reason: Option<Vec<DisQualificationReasonDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicity {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: Vec<CallForOffer>,
    #[serde(rename = "Publicity")]
    pub publicity: PublicityType,
    #[serde(rename = "PublicityOrganizations", skip_serializing_if = "Option::is_none")]
    pub publicity_organizations: Option<OrganizationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEvaluationSubjectType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesGroup {
    #[serde(rename = "RealEstate")]
    pub real_estate: String50Type,
    #[serde(rename = "RealEstateName", skip_serializing_if = "Option::is_none")]
    pub real_estate_name: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaApprovalType {
    pub base: String,
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
pub struct StatusCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoOfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingsType {
    #[serde(rename = "Training", skip_serializing_if = "Option::is_none")]
    pub training: Option<Vec<TrainingDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkarcModel {
    #[serde(rename = "Xlinktitle", skip_serializing_if = "Option::is_none")]
    pub xlinktitle: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingExtraQualifierType {
    pub base: String,
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
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<RealEstatesGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightType {
    #[serde(rename = "HeightType")]
    pub height_type: HeightType,
    #[serde(rename = "EmptyStringType")]
    pub empty_string_type: EmptyStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeededType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax2IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumesType {
    #[serde(rename = "AssortmentMatrixVolume")]
    pub assortment_matrix_volume: Vec<AssortmentMatrixVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemsType {
    #[serde(rename = "QualitySystem", skip_serializing_if = "Option::is_none")]
    pub quality_system: Option<Vec<QualitySystemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEtuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributesType {
    #[serde(rename = "TreeSpeciesAttribute", skip_serializing_if = "Option::is_none")]
    pub tree_species_attribute: Option<Vec<TreeSpeciesAttributeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<ForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerTypeType {
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
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
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
    pub bytes: Option<Vec<u8>>,
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
pub struct OperationModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInformationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger6digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTekstiTyyppi {
    pub base: String,
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
pub struct ConsumptionType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandKeyGroup2 {
    #[serde(rename = "ParcelNumber", skip_serializing_if = "Option::is_none")]
    pub parcel_number: Option<ParcelNumberType>,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<StbStandNumberType>,
    #[serde(rename = "RegisterUnitId", skip_serializing_if = "Option::is_none")]
    pub register_unit_id: Option<RegisterUnitIdType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartsType {
    #[serde(rename = "TargetPart")]
    pub tp_target_part: Vec<TargetPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "Point")]
    pub gml_point: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummaryType {
    #[serde(rename = "TreeSpeciesData")]
    pub tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctTaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationsType {
    #[serde(rename = "Specification")]
    pub specification: Vec<SpecificationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreUnitType {
    pub base: String,
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
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: CoForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<CoReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String500Type {
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
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: CoForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<CoReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentWithFraction1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScheduleType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKieliKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKolmasRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageDataType {
    #[serde(rename = "CommonMessageId", skip_serializing_if = "Option::is_none")]
    pub common_message_id: Option<CommonMessageType>,
    #[serde(rename = "CommonMessageFreeText", skip_serializing_if = "Option::is_none")]
    pub common_message_free_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActDataType {
    #[serde(rename = "WorksDueDate")]
    pub works_due_date: CoDateType,
    #[serde(rename = "CompletionDeclarationDeliveryDueDate")]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(rename = "ServiceNameOfAPI")]
    pub service_name_of_a_p_i: ServiceNameofAPIType,
    #[serde(rename = "AuthorizedToSend")]
    pub authorized_to_send: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSuggestionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionNoType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuudesRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinktitleEltType {
    #[serde(rename = "titleModel")]
    pub xlinktitle_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: StratumNumberType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "Age")]
    pub age: AgeType,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
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
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocumentClassType {
    pub base: String,
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
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
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
pub struct ImageCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLoppuPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleType {
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
    #[serde(rename = "TimeStamp")]
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
    #[serde(rename = "OfferWorkingSites")]
    pub ws_offer_working_sites: OfferWorkingSites,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
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
pub struct ObjectGeometryType {
    #[serde(rename = "@collectingMethod")]
    pub collecting_method: CollectingMethodType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<Decimal1FractionDigitType>,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: GdtAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDateType {
    #[serde(flatten)]
    pub base: CoDateType,
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
    pub stand_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "PointLineAndPolygonGeometriesGroup")]
    pub gdt_point_line_and_polygon_geometries_group: PointLineAndPolygonGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType3 {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCodeType {
    #[serde(flatten)]
    pub base: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembersType {
    #[serde(rename = "WorkGrouMember")]
    pub work_grou_member: Vec<WorkGrouMemberType>,
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
    pub quantity_unit: ExtendedQuantityUnitType,
    #[serde(rename = "AssortmentInfo")]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simple {
    #[serde(rename = "simpleModel")]
    pub xlinksimple_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String25Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDepotAccessibilityType {
    pub base: String,
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
pub struct BdtYesNoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<PointProperty>,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "Polygon")]
    pub gml_polygon: Polygon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMessageTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKansalaisuusKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5_1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoSellerResponsibleType {
    #[serde(rename = "YesNoType")]
    pub co_yes_no_type: CoYesNoType,
    #[serde(rename = "SellerResponsible")]
    pub seller_responsible: SellerResponsible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonsType {
    #[serde(rename = "DeclarationPolygon")]
    pub declaration_polygon: Vec<DeclarationPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkinTyyppi {
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinklocatorModel {
    #[serde(rename = "Xlinktitle", skip_serializing_if = "Option::is_none")]
    pub xlinktitle: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDefinedType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessMessageTimeStampType {
    #[serde(flatten)]
    pub base: CoTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolumeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAssortmentMainGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTekstiTyyppi {
    pub base: String,
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
pub struct CompletionDataType {
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<PaymentsRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesType {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger5digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkinTyyppi {
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreControlObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String50Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2Type {
    pub base: f64,
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
    pub file_binary: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "ContactInformation")]
    pub contact_information: ContactInformationType,
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
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer7digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsIBANTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString100Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountType {
    #[serde(flatten)]
    pub base: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoSellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
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
    pub power_of_attorney_document: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<HeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<CoDiameterClassType>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication")]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<CoString2000Type>,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
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
pub struct WorkCodeGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPolygonOrMultiPolygon2Type {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<PointProperty>,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataMessageType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: RestrictionsMainGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCaseType {
    #[serde(rename = "ControlUseCaseType")]
    pub control_use_case_type: ControlUseCaseType,
    #[serde(rename = "SelfMonitoringUseCaseType")]
    pub self_monitoring_use_case_type: SelfMonitoringUseCaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreDataMessageType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstateType {
    #[serde(flatten)]
    pub base: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffersType {
    #[serde(rename = "RelatedCallForOffer")]
    pub related_call_for_offer: Vec<RelatedCallForOfferType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostitoimipaikkaNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctWorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    pub base: f64,
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
pub struct ProcessingAreasType {
    #[serde(rename = "ProcessingArea")]
    pub processing_area: Vec<ProcessingAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCultivationMaterialType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEnsimmainenRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAcceptanceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaProjectStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem", skip_serializing_if = "Option::is_none")]
    pub certification_system: Option<Vec<String>>,
    #[serde(flatten)]
    pub base: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameBaseType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal1FractionDigitType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctDitchTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaReviewType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditQuestionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAndOriginalYearGroup {
    #[serde(rename = "ProposalYear")]
    pub proposal_year: ProposalYearType,
    #[serde(rename = "OriginalProposalYear", skip_serializing_if = "Option::is_none")]
    pub original_proposal_year: Option<OriginalProposalYearType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvmTyyppi {
    pub base: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSitesType {
    #[serde(rename = "ContractWorkingSiteDetails")]
    pub contract_working_site_details: Vec<ContractWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicDataType {
    #[serde(rename = "ProjectNo", skip_serializing_if = "Option::is_none")]
    pub project_no: Option<ProjectNoType>,
    #[serde(rename = "SelfMonitoringType", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_type: Option<SelfMonitoringTypeType>,
    #[serde(rename = "SelfMonitoringDate", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_date: Option<DateType>,
    #[serde(rename = "ForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_use_declaration_number: Option<ForestUseDeclarationNumber>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_number: Option<FinancingActNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<CustomerReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoISO639char2LanguageType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaGroundManipulationMethodType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsUlkomaaHenkiloTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnusTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateAssortmentRowType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulkType {
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "Bulk")]
    pub bulk: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateType>,
    #[serde(rename = "SilvicultureValidity", skip_serializing_if = "Option::is_none")]
    pub silviculture_validity: Option<SilvicultureValidityType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureText", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_text: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOfficeType {
    #[serde(flatten)]
    pub base: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraTreeSpeciesType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeOfForestObjectType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKutsumaNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatSurvivingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNoType {
    #[serde(flatten)]
    pub base: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsType {
    #[serde(rename = "Attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaGroundManipulationMethodType {
    pub base: String,
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
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
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
pub struct OperationStatusType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesType {
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: Vec<AssortmentVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_stands: Option<FinancingActCompletionStands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStoreyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainWorkCodeType {
    #[serde(rename = "WorkCodeType")]
    pub work_code_type: WorkCodeType,
    #[serde(rename = "WorkCodeGroupType")]
    pub work_code_group_type: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentrePayments")]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdentifierValueType {
    pub base: String,
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
pub struct PhoneNumberType {
    #[serde(flatten)]
    pub base: JhsPuhelinnumeroTekstiTyyppi,
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
pub struct ExtraMainGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsVakinainenKytkinTyyppi {
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstAndLastNameGroup {
    #[serde(rename = "LastName")]
    pub last_name: LastNameType,
    #[serde(rename = "FirstName")]
    pub first_name: FirstNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSceneryWorkPermissionNeededType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
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
    pub target_stem_count: Option<StemCountType>,
    #[serde(rename = "TargetBasalArea", skip_serializing_if = "Option::is_none")]
    pub target_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "TargetAmount", skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<AmountType>,
    #[serde(rename = "TargetAmountUnit", skip_serializing_if = "Option::is_none")]
    pub target_amount_unit: Option<ExtendedWideUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal4And2Type {
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
pub struct OperationInfoType {
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
pub struct WorkingWeightType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTransportAccessibilityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesType {
    #[serde(rename = "Role")]
    pub role: Vec<OrganizationRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDrainageStateType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDistanceUnitType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaCultivationMaterialType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlayksikkoNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: String,
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActorsType {
    #[serde(rename = "InformedActor")]
    pub informed_actor: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaWorkQualityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtImageCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "DeclarationPolygonReference")]
    pub declaration_polygon_reference: CoReferenceType,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: PolygonProperty,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfBasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: FeatureDataGroup,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<UsingRestrictions>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwnerType {
    #[serde(rename = "NameAndOrganizationGroup")]
    pub ci_name_and_organization_group: Vec<CiNameAndOrganizationGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEdellinenSukuNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantitiesType {
    #[serde(rename = "StatisticsQuantity")]
    pub statistics_quantity: Vec<StatisticsQuantityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String2000Type {
    pub base: String,
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
pub struct CallForOfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "CallForOfferWorkingSiteDetails")]
    pub call_for_offer_working_site_details: Vec<CallForOfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationsType {
    #[serde(rename = "ResourceLocation")]
    pub resource_location: Vec<ResourceLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "LineString")]
    pub gml_line_string: LineString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifierType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
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
pub struct BdtResourceTypeType {
    #[serde(rename = "PersonResourceType")]
    pub person_resource_type: PersonResourceType,
    #[serde(rename = "MachineTypeType")]
    pub machine_type_type: MachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: StbAreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetsType {
    #[serde(rename = "Target")]
    pub tgt_target: Vec<Target>,
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
    pub assortment_compact_classes: AssortmentCompactClassesType,
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
    pub time_stamp: TimeStampType,
    #[serde(rename = "Acceptance")]
    pub acceptance: CoAcceptanceType,
    #[serde(rename = "ReplyCode")]
    pub reply_code: ReplyCodeType,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<CoReferenceType>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<DateType>,
    #[serde(rename = "RegistrationId", skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<CoReferenceType>,
    #[serde(rename = "ErrorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<ErrorMessagesType>,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustificationsType {
    #[serde(rename = "Justification")]
    pub justification: Vec<CoString5000Type>,
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
pub struct IdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
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
pub struct AssortmentClassCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTaxType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformationType {
    #[serde(rename = "ServiceBuyer")]
    pub service_buyer: Vec<ServiceBuyerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesType {
    #[serde(rename = "UserRole")]
    pub user_role: Vec<UserRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlaceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString1000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptancesType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "BusinessAcceptance", skip_serializing_if = "Option::is_none")]
    pub ba_business_acceptance: Option<Vec<BusinessAcceptance>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax4IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMaterialCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatSurvivingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatorType {
    #[serde(rename = "locatorModel")]
    pub xlinklocator_model: String,
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
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
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
    pub working_site_final_audit_silviculture: Option<SelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<SelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<SelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<SelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccFinancingActNumberType {
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
pub struct DimensionRequirementsGroup {
    #[serde(rename = "LengthMin")]
    pub length_min: LengthMinType,
    #[serde(rename = "LengthMax")]
    pub length_max: LengthMaxType,
    #[serde(rename = "DiameterMax")]
    pub diameter_max: PositiveIntegerType,
    #[serde(rename = "DiameterMin")]
    pub diameter_min: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    pub base: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationBankAccountType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "BankAccount")]
    pub co_bank_account: BankAccount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeSpecifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorsType {
    #[serde(rename = "CompletionActor")]
    pub completion_actor: Vec<CompletionActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalType {
    #[serde(rename = "Mean")]
    pub mean: MeanType,
    #[serde(rename = "Variance")]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: ERPIdType,
    #[serde(rename = "QuestionAsText", skip_serializing_if = "Option::is_none")]
    pub question_as_text: Option<String200Type>,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: String50Type,
    #[serde(rename = "QuestionAnswerAsText", skip_serializing_if = "Option::is_none")]
    pub question_answer_as_text: Option<String50Type>,
    #[serde(rename = "QuestionAnswerAdditionalText", skip_serializing_if = "Option::is_none")]
    pub question_answer_additional_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategoryType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal7And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference14Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerResponsible {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDurationType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlUseCaseType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNeljasRiviTekstiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationsType {
    #[serde(rename = "ObjectProtectionOperation")]
    pub object_protection_operation: Vec<ObjectProtectionOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "ProductUserId")]
    pub product_user_id: String100Type,
    #[serde(rename = "ModificationDate")]
    pub modification_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEtunimetNimiTyyppi {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDataSourceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoExtendedMainGroupType {
    #[serde(rename = "MainGroupType")]
    pub main_group_type: MainGroupType,
    #[serde(rename = "ExtraMainGroupType")]
    pub extra_main_group_type: ExtraMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfoType {
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReviewType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFinancingActWorkGroupType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString10Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLoppuHetkiTyyppi {
    pub base: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTypeType {
    pub base: String,
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
pub struct ShapeBetaType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnersType {
    #[serde(rename = "ForestOwner")]
    pub forest_owner: Vec<ForestOwnerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIDType {
    #[serde(flatten)]
    pub base: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationDevelopmentClassType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CddDistributionModelGroup {
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSBType,
    #[serde(rename = "Weibull")]
    pub weibull: WeibullType,
    #[serde(rename = "Normal")]
    pub normal: NormalType,
    #[serde(rename = "Gamma")]
    pub gamma: GammaType,
    #[serde(rename = "Beta")]
    pub beta: BetaType,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "SilviculturalOperations", skip_serializing_if = "Option::is_none")]
    pub silvicultural_operations: Option<SilviculturalOperationsType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateTypeType {
    pub base: String,
}

