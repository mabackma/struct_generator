use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StormWorkingSite {
    #[serde(flatten)]
    pub storm_working_site: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute4 {
    #[serde(flatten)]
    pub attribute4: BdtWorkCodeQualifierType4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute5 {
    #[serde(flatten)]
    pub attribute5: BdtWorkCodeQualifierType5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreClearingInformation {
    #[serde(flatten)]
    pub pre_clearing_information: PreClearingInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendWorkingAloneNotification {
    #[serde(flatten)]
    pub send_working_alone_notification: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute3 {
    #[serde(flatten)]
    pub attribute3: BdtWorkCodeQualifierType3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizedToSend {
    #[serde(flatten)]
    pub authorized_to_send: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSFNumber {
    #[serde(flatten)]
    pub f_s_f_number: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute1 {
    #[serde(flatten)]
    pub attribute1: BdtWorkCodeQualifierType1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationContactPerson {
    #[serde(flatten)]
    pub notification_contact_person: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesAttributes {
    #[serde(flatten)]
    pub tree_species_attributes: TreeSpeciesAttributesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetDensity {
    #[serde(flatten)]
    pub target_density: BdtPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesCutting {
    #[serde(flatten)]
    pub stand_trees_cutting: StandTreesCuttingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: BdtString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherRemarks {
    #[serde(flatten)]
    pub other_remarks: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellersLogsInfo {
    #[serde(flatten)]
    pub sellers_logs_info: BdtString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerContactInformation {
    #[serde(flatten)]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductionFileSendFrequency {
    #[serde(flatten)]
    pub production_file_send_frequency: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellersLogs {
    #[serde(flatten)]
    pub sellers_logs: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryInfo {
    #[serde(flatten)]
    pub delivery_info: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelNo {
    #[serde(flatten)]
    pub parcel_no: BdtPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialDeliveryType {
    #[serde(flatten)]
    pub material_delivery_type: MaterialDeliveryTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItem {
    #[serde(flatten)]
    pub list_item: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeginNotificationAllowed {
    #[serde(flatten)]
    pub begin_notification_allowed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingFinishedDate {
    #[serde(flatten)]
    pub cutting_finished_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasisListItem {
    #[serde(flatten)]
    pub fee_basis_list_item: FeebasisListItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlannedForHarvestingDate {
    #[serde(flatten)]
    pub working_site_planned_for_harvesting_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificationHandlingInstructions {
    #[serde(flatten)]
    pub certification_handling_instructions: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerType {
    #[serde(flatten)]
    pub customer_type: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerRepresentativePerson {
    #[serde(flatten)]
    pub customer_representative_person: BdtContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MixedForestRegenarationMethods {
    #[serde(flatten)]
    pub mixed_forest_regenaration_methods: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpRaising {
    #[serde(flatten)]
    pub stump_raising: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueListItem {
    #[serde(flatten)]
    pub value_list_item: ValueListItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListId {
    #[serde(flatten)]
    pub list_id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeciduousTreeTargetDensityPercent {
    #[serde(flatten)]
    pub deciduous_tree_target_density_percent: BdtPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: BdtString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DegreeDays {
    #[serde(flatten)]
    pub degree_days: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(flatten)]
    pub description: BdtString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPlanStandId {
    #[serde(flatten)]
    pub forest_plan_stand_id: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    #[serde(flatten)]
    pub id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SodWorkingSite {
    #[serde(flatten)]
    pub sod_working_site: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: BdtWorkingSitePlanningStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeUnit {
    #[serde(flatten)]
    pub fee_unit: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemTypeBulks {
    #[serde(flatten)]
    pub stem_type_bulks: StemTypeBulksType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesCuttingVolume {
    #[serde(flatten)]
    pub stand_trees_cutting_volume: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportArea {
    #[serde(flatten)]
    pub transport_area: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceNameOfAPI {
    #[serde(flatten)]
    pub service_name_of_a_p_i: BdtServiceNameofAPIType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemTypeBulk {
    #[serde(flatten)]
    pub stem_type_bulk: StemTypeBulkType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: BdtContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IslandWorkingSite {
    #[serde(flatten)]
    pub island_working_site: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bulk {
    #[serde(flatten)]
    pub bulk: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinLength {
    #[serde(flatten)]
    pub min_length: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultsOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub results_of_accessibility_analysis: ResultsOfAccessibilityAnalysisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSpeciesAttribute {
    #[serde(flatten)]
    pub tree_species_attribute: TreeSpeciesAttributeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute2 {
    #[serde(flatten)]
    pub attribute2: BdtWorkCodeQualifierType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainPlanningDone {
    #[serde(flatten)]
    pub terrain_planning_done: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCertification {
    #[serde(flatten)]
    pub forest_certification: BdtCertificationSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryDate {
    #[serde(flatten)]
    pub delivery_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsValueForceWorkingSite {
    #[serde(flatten)]
    pub is_value_force_working_site: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTree {
    #[serde(flatten)]
    pub stand_tree: StandTreeCuttingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestOwners {
    #[serde(flatten)]
    pub forest_owners: ForestOwnersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAreaRequired {
    #[serde(flatten)]
    pub test_area_required: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationStandFellingPurpose {
    #[serde(flatten)]
    pub forest_use_declaration_stand_felling_purpose: BdtCuttingPurposeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delivery {
    #[serde(flatten)]
    pub delivery: DeliveryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinDiameter {
    #[serde(flatten)]
    pub min_diameter: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousBlock {
    #[serde(flatten)]
    pub previous_block: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialAreaId {
    #[serde(flatten)]
    pub material_area_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAreaMethod {
    #[serde(flatten)]
    pub test_area_method: BdtSamplePlotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delivered {
    #[serde(flatten)]
    pub delivered: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreNotificationAllowed {
    #[serde(flatten)]
    pub pre_notification_allowed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAreaNumber {
    #[serde(flatten)]
    pub test_area_number: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Altitude {
    #[serde(flatten)]
    pub altitude: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasSupport {
    #[serde(flatten)]
    pub has_support: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    #[serde(flatten)]
    pub owner: BdtContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchOrRoadPlanId {
    #[serde(flatten)]
    pub ditch_or_road_plan_id: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonRole {
    #[serde(flatten)]
    pub person_role: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeltingDate {
    #[serde(flatten)]
    pub melting_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation1 {
    #[serde(flatten)]
    pub organisation1: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyer {
    #[serde(flatten)]
    pub service_buyer: ServiceBuyerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation3 {
    #[serde(flatten)]
    pub organisation3: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesCurrent {
    #[serde(flatten)]
    pub stand_trees_current: StandTreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasisList {
    #[serde(flatten)]
    pub fee_basis_list: FeeBasisListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandTreesStratumLeaving {
    #[serde(flatten)]
    pub stand_trees_stratum_leaving: StandTreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueList {
    #[serde(flatten)]
    pub value_list: ValueListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeType {
    #[serde(flatten)]
    pub fee_type: BdtFeeBasisValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: BdtStorageLandOwnerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionOfOrigin {
    #[serde(flatten)]
    pub region_of_origin: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomassCollection {
    #[serde(flatten)]
    pub biomass_collection: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTimeEnd {
    #[serde(flatten)]
    pub operation_time_end: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganisationId {
    #[serde(flatten)]
    pub organisation_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterSections {
    #[serde(flatten)]
    pub diameter_sections: DiameterSectionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LargeSummaryReportRequired {
    #[serde(flatten)]
    pub large_summary_report_required: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterSection {
    #[serde(flatten)]
    pub diameter_section: SectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationNotNeeded {
    #[serde(flatten)]
    pub forest_use_declaration_not_needed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelPosition {
    #[serde(flatten)]
    pub label_position: GdtPointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultValue {
    #[serde(flatten)]
    pub default_value: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsibleOfPreClearing {
    #[serde(flatten)]
    pub responsible_of_pre_clearing: WctResponsibleOfPreClearingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuousCoverForestry {
    #[serde(flatten)]
    pub continuous_cover_forestry: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DryingClass {
    #[serde(flatten)]
    pub drying_class: BdtDryingClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub result_of_accessibility_analysis: ResultOfAccessibilityAnalysisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityAttachments {
    #[serde(flatten)]
    pub quality_attachments: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomassFinishedDate {
    #[serde(flatten)]
    pub biomass_finished_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportCompany {
    #[serde(flatten)]
    pub transport_company: BdtContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDisplayId {
    #[serde(flatten)]
    pub storage_display_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousBlocks {
    #[serde(flatten)]
    pub previous_blocks: PreviousBlockInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationsToSendWsoInformation {
    #[serde(flatten)]
    pub authorizations_to_send_wso_information: AuthorizationsToSendWsoInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainClass {
    #[serde(flatten)]
    pub terrain_class: BdtTerrainClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation2 {
    #[serde(flatten)]
    pub organisation2: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureContractNumber {
    #[serde(flatten)]
    pub silviculture_contract_number: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentIncrementAllowed {
    #[serde(flatten)]
    pub assortment_increment_allowed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deliveries {
    #[serde(flatten)]
    pub deliveries: DeliveriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingCode {
    #[serde(flatten)]
    pub working_code: BdtWorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingControlRequired {
    #[serde(flatten)]
    pub cutting_control_required: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerRepresentativePerson {
    #[serde(flatten)]
    pub owner_representative_person: BdtContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: BdtWorkingSiteStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HumidityPercentage {
    #[serde(flatten)]
    pub humidity_percentage: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Percentage {
    #[serde(flatten)]
    pub percentage: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationTimeStart {
    #[serde(flatten)]
    pub operation_time_start: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: BdtMeasurementPlaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationRestriction {
    #[serde(flatten)]
    pub operation_restriction: OperationRestrictionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommercialName {
    #[serde(flatten)]
    pub commercial_name: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSFInformation {
    #[serde(flatten)]
    pub f_s_f_information: FSFInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(flatten)]
    pub authorization_to_send_wso_information: AuthorizationToSendWsoInformation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteOperational {
    #[serde(flatten)]
    pub working_site_operational: WorkingSiteOperationalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryName {
    #[serde(flatten)]
    pub delivery_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelLabel {
    #[serde(flatten)]
    pub parcel_label: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDryingClass {
    #[serde(flatten)]
    pub storage_drying_class: BdtStorageDryingClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeText {
    #[serde(flatten)]
    pub fee_text: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendNotificationsAlways {
    #[serde(flatten)]
    pub send_notifications_always: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractValidDate {
    #[serde(flatten)]
    pub contract_valid_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCode {
    #[serde(flatten)]
    pub user_code: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessRightsInfo {
    #[serde(flatten)]
    pub access_rights_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyToDo {
    #[serde(flatten)]
    pub ready_to_do: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: WctQualityOfTreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: WorkingSitePriorityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationStandExtraInfo {
    #[serde(flatten)]
    pub forest_use_declaration_stand_extra_info: BdtString2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoTextMandatory {
    #[serde(flatten)]
    pub info_text_mandatory: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryNumber {
    #[serde(flatten)]
    pub delivery_number: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingSustainableForestry {
    #[serde(flatten)]
    pub financing_sustainable_forestry: FinancingSustainableForestryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplier {
    #[serde(flatten)]
    pub supplier: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousBlockState {
    #[serde(flatten)]
    pub previous_block_state: BdtPreviousBlockStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: BdtTurningPointClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSFValidity {
    #[serde(flatten)]
    pub f_s_f_validity: FSFValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockIsFSFBlock {
    #[serde(flatten)]
    pub block_is_f_s_f_block: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainWorkCode {
    #[serde(flatten)]
    pub main_work_code: BdtMainWorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingFinishedDate {
    #[serde(flatten)]
    pub stump_lifting_finished_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: BdtWorkingSitePlanningOperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organisation4 {
    #[serde(flatten)]
    pub organisation4: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreClearingExecutionTime {
    #[serde(flatten)]
    pub pre_clearing_execution_time: FSFValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendNotifications {
    #[serde(flatten)]
    pub send_notifications: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "@DeliveryUserId")]
    pub delivery_user_id: String50Type,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: Vec<ProductUserId>,
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceType {
    #[serde(rename = "StorageForestHaulageDistanceGroup")]
    pub storage_forest_haulage_distance_group: StorageForestHaulageDistanceGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeCuttingType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<Vec<MaterialType>>,
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
pub struct DeliveryType {
    #[serde(rename = "DeliveryUserId")]
    pub delivery_user_id: String50Type,
    #[serde(rename = "DeliveryInfo")]
    pub delivery_info: String50Type,
    #[serde(rename = "DeliveryName")]
    pub delivery_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformationType {
    #[serde(rename = "ResponsibleOfPreClearing", skip_serializing_if = "Option::is_none")]
    pub responsible_of_pre_clearing: Option<ResponsibleOfPreClearingType>,
    #[serde(rename = "PreClearingExecutionTime", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_execution_time: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockInfoType {
    #[serde(rename = "PreviousBlock", skip_serializing_if = "Option::is_none")]
    pub previous_block: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeCuttingType {
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<Vec<FeeBasisDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulksType {
    #[serde(rename = "StemTypeBulk", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulk: Option<Vec<StemTypeBulkType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryTypeType {
    pub base: String,
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
pub struct StoragesForestHaulageDistancesType {
    #[serde(rename = "StorageForestHaulageDistance")]
    pub storage_forest_haulage_distance: Vec<StorageForestHaulageDistanceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot", skip_serializing_if = "Option::is_none")]
    pub sample_plot: Option<Vec<SamplePlotType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListType {
    #[serde(rename = "ValueListItem")]
    pub value_list_item: Vec<ValueListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: String10Type,
    #[serde(rename = "FeeType")]
    pub fee_type: FeeBasisValueType,
    #[serde(rename = "FeeText")]
    pub fee_text: String50Type,
    #[serde(rename = "FeeUnit", skip_serializing_if = "Option::is_none")]
    pub fee_unit: Option<String10Type>,
    #[serde(rename = "InfoTextMandatory")]
    pub info_text_mandatory: YesNoType,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String50Type>,
    #[serde(rename = "ValueList", skip_serializing_if = "Option::is_none")]
    pub value_list: Option<ValueListType>,
    #[serde(rename = "FeeBasisList", skip_serializing_if = "Option::is_none")]
    pub fee_basis_list: Option<FeeBasisListType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysisType {
    #[serde(rename = "ResultOfAccessibilityAnalysis")]
    pub result_of_accessibility_analysis: Vec<ResultOfAccessibilityAnalysisType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeType>,
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
pub struct ResultOfAccessibilityAnalysisType {
    #[serde(rename = "Accessibility")]
    pub accessibility: AccessibilityType,
    #[serde(rename = "Percentage")]
    pub percentage: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<RealEstatesGroup>,
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
pub struct ValueListItemType {
    #[serde(rename = "ListId")]
    pub list_id: PositiveIntegerType,
    #[serde(rename = "ListItem")]
    pub list_item: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureType {
    #[serde(rename = "FeatureDataGroup")]
    pub sf_feature_data_group: FeatureDataGroup,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub sf_feature_info: Option<FeatureInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveriesType {
    #[serde(rename = "Delivery")]
    pub delivery: Vec<DeliveryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestryType {
    #[serde(rename = "BlockIsFSFBlock", skip_serializing_if = "Option::is_none")]
    pub block_is_f_s_f_block: Option<YesNoType>,
    #[serde(rename = "FSFInformation", skip_serializing_if = "Option::is_none")]
    pub f_s_f_information: Option<Vec<FSFInformationType>>,
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
pub struct WorkingSitePriorityType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeType {
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
pub struct TreeSpeciesAttributesType {
    #[serde(rename = "TreeSpeciesAttribute", skip_serializing_if = "Option::is_none")]
    pub tree_species_attribute: Option<Vec<TreeSpeciesAttributeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulkType {
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "Bulk")]
    pub bulk: PositiveIntegerType,
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
pub struct SamplePlotType {
    #[serde(rename = "TestAreaNumber")]
    pub test_area_number: PositiveIntegerType,
    #[serde(rename = "Geometry")]
    pub geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceGroup {
    #[serde(rename = "StorageId")]
    pub storage_id: StorageId,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: ForestHaulageDistance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionsType {
    #[serde(rename = "DiameterSection")]
    pub diameter_section: Vec<SectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformationType {
    #[serde(rename = "ServiceBuyer")]
    pub service_buyer: Vec<ServiceBuyerType>,
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
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SpecialFeatureType>,
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
pub struct AuthorizationToSendWsoInformation {
    #[serde(rename = "ServiceNameOfAPI")]
    pub service_name_of_a_p_i: ServiceNameofAPIType,
    #[serde(rename = "AuthorizedToSend")]
    pub authorized_to_send: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesType {
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<StorageType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<Vec<AssortmentType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformationType {
    #[serde(rename = "FSFNumber", skip_serializing_if = "Option::is_none")]
    pub f_s_f_number: Option<String50Type>,
    #[serde(rename = "FSFValidity", skip_serializing_if = "Option::is_none")]
    pub f_s_f_validity: Option<FSFValidityType>,
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
pub struct FSFValidityType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListType {
    #[serde(rename = "FeeBasisListItem")]
    pub fee_basis_list_item: Vec<FeebasisListItemType>,
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
pub struct ForestUseDeclarationType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
    #[serde(rename = "ForestUseDeclarationNotNeeded", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_not_needed: Option<YesNoType>,
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
pub struct TreeSpeciesAttributeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Bulk", skip_serializing_if = "Option::is_none")]
    pub bulk: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityOfTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensityGroup {
    #[serde(rename = "TargetDensity", skip_serializing_if = "Option::is_none")]
    pub target_density: Option<TargetDensity>,
    #[serde(rename = "DeciduousTreeTargetDensityPercent", skip_serializing_if = "Option::is_none")]
    pub deciduous_tree_target_density_percent: Option<DeciduousTreeTargetDensityPercent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesGroup {
    #[serde(rename = "RealEstate")]
    pub real_estate: RealEstate,
    #[serde(rename = "RealEstateName", skip_serializing_if = "Option::is_none")]
    pub real_estate_name: Option<RealEstateName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnersType {
    #[serde(rename = "ForestOwner")]
    pub forest_owner: Vec<ForestOwnerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformationType {
    #[serde(rename = "AuthorizationToSendWsoInformation")]
    pub authorization_to_send_wso_information: Vec<AuthorizationToSendWsoInformation>,
}

