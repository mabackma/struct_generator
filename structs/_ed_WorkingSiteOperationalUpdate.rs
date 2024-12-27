#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: BdtWorkingSitePlanningStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProposals {
    #[serde(flatten)]
    pub storage_proposals: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentID {
    #[serde(flatten)]
    pub assortment_i_d: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: GdtPointAndLineOrPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCount {
    #[serde(flatten)]
    pub image_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: BdtWorkingSitePlanningOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageClass {
    #[serde(flatten)]
    pub storage_class: BdtStorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesProposalForestHaulageDistances {
    #[serde(flatten)]
    pub storages_proposal_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: StorageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: BdtString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: BdtMeasurementPlaceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: BdtAssortmentGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelinationObjectOrderId {
    #[serde(flatten)]
    pub delination_object_order_id: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: BdtString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: BdtMeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: BdtStorageLandOwnerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: BdtTurningPointClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: BdtString3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: BdtTransportAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: BdtString50Type,
}

