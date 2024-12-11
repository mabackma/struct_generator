#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Altitude {
    #[serde(flatten)]
    pub altitude: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingFinishedDate {
    #[serde(flatten)]
    pub cutting_finished_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractValidDate {
    #[serde(flatten)]
    pub contract_valid_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingFinishedDate {
    #[serde(flatten)]
    pub stump_lifting_finished_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformation {
    #[serde(flatten)]
    pub pre_clearing_information: PreClearingInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: WorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaRequired {
    #[serde(flatten)]
    pub test_area_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryNumber {
    #[serde(flatten)]
    pub delivery_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeUnit {
    #[serde(flatten)]
    pub fee_unit: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRepresentativePerson {
    #[serde(flatten)]
    pub owner_representative_person: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation2 {
    #[serde(flatten)]
    pub organisation2: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: StoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotifications {
    #[serde(flatten)]
    pub send_notifications: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidity {
    #[serde(flatten)]
    pub f_s_f_validity: FSFValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCode {
    #[serde(flatten)]
    pub user_code: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Percentage {
    #[serde(flatten)]
    pub percentage: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClass {
    #[serde(flatten)]
    pub drying_class: DryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: SpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRole {
    #[serde(flatten)]
    pub person_role: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryInfo {
    #[serde(flatten)]
    pub delivery_info: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedToSend {
    #[serde(flatten)]
    pub authorized_to_send: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Length {
    #[serde(flatten)]
    pub length: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSection {
    #[serde(flatten)]
    pub diameter_section: SectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethod {
    #[serde(flatten)]
    pub pricing_method: PricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargeSummaryReportRequired {
    #[serde(flatten)]
    pub large_summary_report_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: StorageLandOwnerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCutting {
    #[serde(flatten)]
    pub stand_trees_cutting: StandTreesCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationStandExtraInfo {
    #[serde(flatten)]
    pub forest_use_declaration_stand_extra_info: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryName {
    #[serde(flatten)]
    pub delivery_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute2 {
    #[serde(flatten)]
    pub attribute2: WorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCurrent {
    #[serde(flatten)]
    pub stand_trees_current: StandTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode {
    #[serde(flatten)]
    pub language_code: LanguageCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIncrementAllowed {
    #[serde(flatten)]
    pub assortment_increment_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoTextMandatory {
    #[serde(flatten)]
    pub info_text_mandatory: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClass {
    #[serde(flatten)]
    pub bearing_capacity_class: BearingCapacityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: MeasurementPlaceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingExecutionTime {
    #[serde(flatten)]
    pub pre_clearing_execution_time: FSFValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: MeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: WorkingSitePriorityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    #[serde(flatten)]
    pub stand: StandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionOfOrigin {
    #[serde(flatten)]
    pub region_of_origin: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformation {
    #[serde(flatten)]
    pub f_s_f_information: FSFInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClass {
    #[serde(flatten)]
    pub difficulty_class: DifficultyClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyToDo {
    #[serde(flatten)]
    pub ready_to_do: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationHandlingInstructions {
    #[serde(flatten)]
    pub certification_handling_instructions: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stands {
    #[serde(flatten)]
    pub stands: StandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: MaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersLogs {
    #[serde(flatten)]
    pub sellers_logs: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsValueForceWorkingSite {
    #[serde(flatten)]
    pub is_value_force_working_site: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDate {
    #[serde(flatten)]
    pub delivery_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesStratumLeaving {
    #[serde(flatten)]
    pub stand_trees_stratum_leaving: StandTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bulk {
    #[serde(flatten)]
    pub bulk: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: SpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersLogsInfo {
    #[serde(flatten)]
    pub sellers_logs_info: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyer {
    #[serde(flatten)]
    pub service_buyer: ServiceBuyerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeText {
    #[serde(flatten)]
    pub fee_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaNumber {
    #[serde(flatten)]
    pub test_area_number: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformation {
    #[serde(flatten)]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportArea {
    #[serde(flatten)]
    pub transport_area: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryType {
    #[serde(flatten)]
    pub material_delivery_type: MaterialDeliveryTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreNotificationAllowed {
    #[serde(flatten)]
    pub pre_notification_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: MaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendWorkingAloneNotification {
    #[serde(flatten)]
    pub send_working_alone_notification: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNotNeeded {
    #[serde(flatten)]
    pub forest_use_declaration_not_needed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(flatten)]
    pub list_item: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockIsFSFBlock {
    #[serde(flatten)]
    pub block_is_f_s_f_block: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    #[serde(flatten)]
    pub amount: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPlanStandId {
    #[serde(flatten)]
    pub forest_plan_stand_id: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeltingDate {
    #[serde(flatten)]
    pub melting_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StormWorkingSite {
    #[serde(flatten)]
    pub storm_working_site: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttribute {
    #[serde(flatten)]
    pub tree_species_attribute: TreeSpeciesAttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute3 {
    #[serde(flatten)]
    pub attribute3: WorkCodeQualifierType3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameOfAPI {
    #[serde(flatten)]
    pub service_name_of_a_p_i: ServiceNameofAPIType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTimeEnd {
    #[serde(flatten)]
    pub operation_time_end: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlock {
    #[serde(flatten)]
    pub previous_block: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwners {
    #[serde(flatten)]
    pub forest_owners: ForestOwnersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClass {
    #[serde(flatten)]
    pub terrain_class: TerrainClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNo {
    #[serde(flatten)]
    pub parcel_no: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaMethod {
    #[serde(flatten)]
    pub test_area_method: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainWorkCode {
    #[serde(flatten)]
    pub main_work_code: MainWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: WorkingSitePlanningOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainPlanningDone {
    #[serde(flatten)]
    pub terrain_planning_done: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlocks {
    #[serde(flatten)]
    pub previous_blocks: PreviousBlockInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeType {
    #[serde(flatten)]
    pub fee_type: FeeBasisValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationStandFellingPurpose {
    #[serde(flatten)]
    pub forest_use_declaration_stand_felling_purpose: CuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListId {
    #[serde(flatten)]
    pub list_id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulks {
    #[serde(flatten)]
    pub stem_type_bulks: StemTypeBulksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRestriction {
    #[serde(flatten)]
    pub operation_restriction: OperationRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PointAndLineOrPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachments {
    #[serde(flatten)]
    pub quality_attachments: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpRaising {
    #[serde(flatten)]
    pub stump_raising: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(flatten)]
    pub authorization_to_send_wso_information: AuthorizationToSendWsoInformation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributes {
    #[serde(flatten)]
    pub tree_species_attributes: TreeSpeciesAttributesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingCode {
    #[serde(flatten)]
    pub working_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinuousCoverForestry {
    #[serde(flatten)]
    pub continuous_cover_forestry: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: AccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueList {
    #[serde(flatten)]
    pub value_list: ValueListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: HarvestingStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(flatten)]
    pub owner: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation4 {
    #[serde(flatten)]
    pub organisation4: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommercialName {
    #[serde(flatten)]
    pub commercial_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelLabel {
    #[serde(flatten)]
    pub parcel_label: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportCompany {
    #[serde(flatten)]
    pub transport_company: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisList {
    #[serde(flatten)]
    pub fee_basis_list: FeeBasisListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulk {
    #[serde(flatten)]
    pub stem_type_bulk: StemTypeBulkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformation {
    #[serde(flatten)]
    pub authorizations_to_send_wso_information: AuthorizationsToSendWsoInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumidityPercentage {
    #[serde(flatten)]
    pub humidity_percentage: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingControlRequired {
    #[serde(flatten)]
    pub cutting_control_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanName {
    #[serde(flatten)]
    pub ditch_or_road_plan_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearing {
    #[serde(flatten)]
    pub responsible_of_pre_clearing: ResponsibleOfPreClearingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute4 {
    #[serde(flatten)]
    pub attribute4: WorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IslandWorkingSite {
    #[serde(flatten)]
    pub island_working_site: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSections {
    #[serde(flatten)]
    pub diameter_sections: DiameterSectionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanId {
    #[serde(flatten)]
    pub ditch_or_road_plan_id: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation1 {
    #[serde(flatten)]
    pub organisation1: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: WorkingSitePlanningStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockState {
    #[serde(flatten)]
    pub previous_block_state: PreviousBlockStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowingTreeSpecies {
    #[serde(flatten)]
    pub growing_tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(flatten)]
    pub age: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerRepresentativePerson {
    #[serde(flatten)]
    pub customer_representative_person: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestry {
    #[serde(flatten)]
    pub financing_sustainable_forestry: FinancingSustainableForestryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(flatten)]
    pub id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginNotificationAllowed {
    #[serde(flatten)]
    pub begin_notification_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MixedForestRegenarationMethods {
    #[serde(flatten)]
    pub mixed_forest_regenaration_methods: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deliveries {
    #[serde(flatten)]
    pub deliveries: DeliveriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinLength {
    #[serde(flatten)]
    pub min_length: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delivered {
    #[serde(flatten)]
    pub delivered: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganisationId {
    #[serde(flatten)]
    pub organisation_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: TurningPointClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WorkingSiteNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDisplayId {
    #[serde(flatten)]
    pub storage_display_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRightsInfo {
    #[serde(flatten)]
    pub access_rights_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supplier {
    #[serde(flatten)]
    pub supplier: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassFinishedDate {
    #[serde(flatten)]
    pub biomass_finished_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute1 {
    #[serde(flatten)]
    pub attribute1: WorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensity {
    #[serde(flatten)]
    pub target_density: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTimeStart {
    #[serde(flatten)]
    pub operation_time_start: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelPosition {
    #[serde(flatten)]
    pub label_position: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinDiameter {
    #[serde(flatten)]
    pub min_diameter: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: QualityOfTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTree {
    #[serde(flatten)]
    pub stand_tree: StandTreeCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultValue {
    #[serde(flatten)]
    pub default_value: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDays {
    #[serde(flatten)]
    pub degree_days: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerType {
    #[serde(flatten)]
    pub customer_type: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCertification {
    #[serde(flatten)]
    pub forest_certification: CertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFNumber {
    #[serde(flatten)]
    pub f_s_f_number: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationContactPerson {
    #[serde(flatten)]
    pub notification_contact_person: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeciduousTreeTargetDensityPercent {
    #[serde(flatten)]
    pub deciduous_tree_target_density_percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasSupport {
    #[serde(flatten)]
    pub has_support: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotificationsAlways {
    #[serde(flatten)]
    pub send_notifications_always: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: AssortmentGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatment {
    #[serde(flatten)]
    pub stump_treatment: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SodWorkingSite {
    #[serde(flatten)]
    pub sod_working_site: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItem {
    #[serde(flatten)]
    pub value_list_item: ValueListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRemarks {
    #[serde(flatten)]
    pub other_remarks: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassCollection {
    #[serde(flatten)]
    pub biomass_collection: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureContractNumber {
    #[serde(flatten)]
    pub silviculture_contract_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperational {
    #[serde(flatten)]
    pub working_site_operational: WorkingSiteOperationalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation3 {
    #[serde(flatten)]
    pub organisation3: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDryingClass {
    #[serde(flatten)]
    pub storage_drying_class: StorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAreaId {
    #[serde(flatten)]
    pub material_area_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClass {
    #[serde(flatten)]
    pub development_class: WideDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: ForestOwnerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingVolume {
    #[serde(flatten)]
    pub stand_trees_cutting_volume: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub result_of_accessibility_analysis: ResultOfAccessibilityAnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delivery {
    #[serde(flatten)]
    pub delivery: DeliveryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute5 {
    #[serde(flatten)]
    pub attribute5: WorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub results_of_accessibility_analysis: ResultsOfAccessibilityAnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: StorageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannedForHarvestingDate {
    #[serde(flatten)]
    pub working_site_planned_for_harvesting_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionFileSendFrequency {
    #[serde(flatten)]
    pub production_file_send_frequency: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListItem {
    #[serde(flatten)]
    pub fee_basis_list_item: FeebasisListItemType,
}

