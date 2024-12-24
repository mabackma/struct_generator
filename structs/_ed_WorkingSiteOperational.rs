#[derive(Debug, Serialize, Deserialize)]
pub struct ContractValidDate {
    #[serde(flatten)]
    pub contract_valid_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelPosition {
    #[serde(flatten)]
    pub label_position: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deliveries {
    #[serde(flatten)]
    pub deliveries: DeliveriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingCode {
    #[serde(flatten)]
    pub working_code: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIncrementAllowed {
    #[serde(flatten)]
    pub assortment_increment_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute2 {
    #[serde(flatten)]
    pub attribute2: BdtWorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationContactPerson {
    #[serde(flatten)]
    pub notification_contact_person: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Altitude {
    #[serde(flatten)]
    pub altitude: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation3 {
    #[serde(flatten)]
    pub organisation3: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub results_of_accessibility_analysis: ResultsOfAccessibilityAnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganisationId {
    #[serde(flatten)]
    pub organisation_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute5 {
    #[serde(flatten)]
    pub attribute5: BdtWorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttribute {
    #[serde(flatten)]
    pub tree_species_attribute: TreeSpeciesAttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(flatten)]
    pub owner: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpLiftingFinishedDate {
    #[serde(flatten)]
    pub stump_lifting_finished_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItem {
    #[serde(flatten)]
    pub value_list_item: ValueListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MixedForestRegenarationMethods {
    #[serde(flatten)]
    pub mixed_forest_regenaration_methods: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeText {
    #[serde(flatten)]
    pub fee_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlock {
    #[serde(flatten)]
    pub previous_block: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockIsFSFBlock {
    #[serde(flatten)]
    pub block_is_f_s_f_block: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreNotificationAllowed {
    #[serde(flatten)]
    pub pre_notification_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannedForHarvestingDate {
    #[serde(flatten)]
    pub working_site_planned_for_harvesting_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute4 {
    #[serde(flatten)]
    pub attribute4: BdtWorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensity {
    #[serde(flatten)]
    pub target_density: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supplier {
    #[serde(flatten)]
    pub supplier: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperational {
    #[serde(flatten)]
    pub working_site_operational: WorkingSiteOperationalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulks {
    #[serde(flatten)]
    pub stem_type_bulks: StemTypeBulksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulk {
    #[serde(flatten)]
    pub stem_type_bulk: StemTypeBulkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaNumber {
    #[serde(flatten)]
    pub test_area_number: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNotNeeded {
    #[serde(flatten)]
    pub forest_use_declaration_not_needed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportArea {
    #[serde(flatten)]
    pub transport_area: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingFinishedDate {
    #[serde(flatten)]
    pub cutting_finished_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaRequired {
    #[serde(flatten)]
    pub test_area_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwners {
    #[serde(flatten)]
    pub forest_owners: ForestOwnersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SodWorkingSite {
    #[serde(flatten)]
    pub sod_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionFileSendFrequency {
    #[serde(flatten)]
    pub production_file_send_frequency: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersLogs {
    #[serde(flatten)]
    pub sellers_logs: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialAreaId {
    #[serde(flatten)]
    pub material_area_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeUnit {
    #[serde(flatten)]
    pub fee_unit: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRemarks {
    #[serde(flatten)]
    pub other_remarks: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultValue {
    #[serde(flatten)]
    pub default_value: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureContractNumber {
    #[serde(flatten)]
    pub silviculture_contract_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformation {
    #[serde(flatten)]
    pub f_s_f_information: FSFInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributes {
    #[serde(flatten)]
    pub tree_species_attributes: TreeSpeciesAttributesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationHandlingInstructions {
    #[serde(flatten)]
    pub certification_handling_instructions: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRestriction {
    #[serde(flatten)]
    pub operation_restriction: OperationRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassCollection {
    #[serde(flatten)]
    pub biomass_collection: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidity {
    #[serde(flatten)]
    pub f_s_f_validity: FSFValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryName {
    #[serde(flatten)]
    pub delivery_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockState {
    #[serde(flatten)]
    pub previous_block_state: BdtPreviousBlockStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingExecutionTime {
    #[serde(flatten)]
    pub pre_clearing_execution_time: FSFValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTimeStart {
    #[serde(flatten)]
    pub operation_time_start: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDisplayId {
    #[serde(flatten)]
    pub storage_display_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRole {
    #[serde(flatten)]
    pub person_role: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestry {
    #[serde(flatten)]
    pub financing_sustainable_forestry: FinancingSustainableForestryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeciduousTreeTargetDensityPercent {
    #[serde(flatten)]
    pub deciduous_tree_target_density_percent: BdtPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeType {
    #[serde(flatten)]
    pub fee_type: BdtFeeBasisValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDate {
    #[serde(flatten)]
    pub delivery_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotifications {
    #[serde(flatten)]
    pub send_notifications: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFNumber {
    #[serde(flatten)]
    pub f_s_f_number: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinDiameter {
    #[serde(flatten)]
    pub min_diameter: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: WorkingSitePriorityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendWorkingAloneNotification {
    #[serde(flatten)]
    pub send_working_alone_notification: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationStandExtraInfo {
    #[serde(flatten)]
    pub forest_use_declaration_stand_extra_info: BdtString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommercialName {
    #[serde(flatten)]
    pub commercial_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation1 {
    #[serde(flatten)]
    pub organisation1: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerRepresentativePerson {
    #[serde(flatten)]
    pub customer_representative_person: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinLength {
    #[serde(flatten)]
    pub min_length: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryInfo {
    #[serde(flatten)]
    pub delivery_info: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersLogsInfo {
    #[serde(flatten)]
    pub sellers_logs_info: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformation {
    #[serde(flatten)]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation2 {
    #[serde(flatten)]
    pub organisation2: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCutting {
    #[serde(flatten)]
    pub stand_trees_cutting: StandTreesCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerType {
    #[serde(flatten)]
    pub customer_type: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsValueForceWorkingSite {
    #[serde(flatten)]
    pub is_value_force_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinuousCoverForestry {
    #[serde(flatten)]
    pub continuous_cover_forestry: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPlanStandId {
    #[serde(flatten)]
    pub forest_plan_stand_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute1 {
    #[serde(flatten)]
    pub attribute1: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionOfOrigin {
    #[serde(flatten)]
    pub region_of_origin: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyToDo {
    #[serde(flatten)]
    pub ready_to_do: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListId {
    #[serde(flatten)]
    pub list_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysis {
    #[serde(flatten)]
    pub result_of_accessibility_analysis: ResultOfAccessibilityAnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotificationsAlways {
    #[serde(flatten)]
    pub send_notifications_always: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IslandWorkingSite {
    #[serde(flatten)]
    pub island_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassFinishedDate {
    #[serde(flatten)]
    pub biomass_finished_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasSupport {
    #[serde(flatten)]
    pub has_support: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationStandFellingPurpose {
    #[serde(flatten)]
    pub forest_use_declaration_stand_felling_purpose: BdtCuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StormWorkingSite {
    #[serde(flatten)]
    pub storm_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(flatten)]
    pub authorization_to_send_wso_information: AuthorizationToSendWsoInformation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelLabel {
    #[serde(flatten)]
    pub parcel_label: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRepresentativePerson {
    #[serde(flatten)]
    pub owner_representative_person: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(flatten)]
    pub list_item: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachments {
    #[serde(flatten)]
    pub quality_attachments: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRightsInfo {
    #[serde(flatten)]
    pub access_rights_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisList {
    #[serde(flatten)]
    pub fee_basis_list: FeeBasisListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation4 {
    #[serde(flatten)]
    pub organisation4: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delivered {
    #[serde(flatten)]
    pub delivered: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyer {
    #[serde(flatten)]
    pub service_buyer: ServiceBuyerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchOrRoadPlanId {
    #[serde(flatten)]
    pub ditch_or_road_plan_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedToSend {
    #[serde(flatten)]
    pub authorized_to_send: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTimeEnd {
    #[serde(flatten)]
    pub operation_time_end: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClass {
    #[serde(flatten)]
    pub terrain_class: BdtTerrainClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformation {
    #[serde(flatten)]
    pub authorizations_to_send_wso_information: AuthorizationsToSendWsoInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeltingDate {
    #[serde(flatten)]
    pub melting_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportCompany {
    #[serde(flatten)]
    pub transport_company: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesStratumLeaving {
    #[serde(flatten)]
    pub stand_trees_stratum_leaving: StandTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpRaising {
    #[serde(flatten)]
    pub stump_raising: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClass {
    #[serde(flatten)]
    pub drying_class: BdtDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargeSummaryReportRequired {
    #[serde(flatten)]
    pub large_summary_report_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSections {
    #[serde(flatten)]
    pub diameter_sections: DiameterSectionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDays {
    #[serde(flatten)]
    pub degree_days: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainWorkCode {
    #[serde(flatten)]
    pub main_work_code: BdtMainWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainPlanningDone {
    #[serde(flatten)]
    pub terrain_planning_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNo {
    #[serde(flatten)]
    pub parcel_no: BdtPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformation {
    #[serde(flatten)]
    pub pre_clearing_information: PreClearingInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSection {
    #[serde(flatten)]
    pub diameter_section: SectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTree {
    #[serde(flatten)]
    pub stand_tree: StandTreeCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCurrent {
    #[serde(flatten)]
    pub stand_trees_current: StandTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueList {
    #[serde(flatten)]
    pub value_list: ValueListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlocks {
    #[serde(flatten)]
    pub previous_blocks: PreviousBlockInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryNumber {
    #[serde(flatten)]
    pub delivery_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListItem {
    #[serde(flatten)]
    pub fee_basis_list_item: FeebasisListItemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delivery {
    #[serde(flatten)]
    pub delivery: DeliveryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Percentage {
    #[serde(flatten)]
    pub percentage: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAreaMethod {
    #[serde(flatten)]
    pub test_area_method: BdtSamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: BdtWorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearing {
    #[serde(flatten)]
    pub responsible_of_pre_clearing: WctResponsibleOfPreClearingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCertification {
    #[serde(flatten)]
    pub forest_certification: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDryingClass {
    #[serde(flatten)]
    pub storage_drying_class: BdtStorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bulk {
    #[serde(flatten)]
    pub bulk: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingControlRequired {
    #[serde(flatten)]
    pub cutting_control_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryType {
    #[serde(flatten)]
    pub material_delivery_type: MaterialDeliveryTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingVolume {
    #[serde(flatten)]
    pub stand_trees_cutting_volume: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameOfAPI {
    #[serde(flatten)]
    pub service_name_of_a_p_i: BdtServiceNameofAPIType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCode {
    #[serde(flatten)]
    pub user_code: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginNotificationAllowed {
    #[serde(flatten)]
    pub begin_notification_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoTextMandatory {
    #[serde(flatten)]
    pub info_text_mandatory: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumidityPercentage {
    #[serde(flatten)]
    pub humidity_percentage: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute3 {
    #[serde(flatten)]
    pub attribute3: BdtWorkCodeQualifierType3,
}

