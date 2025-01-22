use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: BdtContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: BdtMeasurementPlaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: BdtMeasurementMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: BdtString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageProposals {
    #[serde(flatten)]
    pub storage_proposals: StoragesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: BdtTransportAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageClass {
    #[serde(flatten)]
    pub storage_class: BdtStorageDryingClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelinationObjectOrderId {
    #[serde(flatten)]
    pub delination_object_order_id: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoragesProposalForestHaulageDistances {
    #[serde(flatten)]
    pub storages_proposal_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentID {
    #[serde(flatten)]
    pub assortment_i_d: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: BdtWorkingSitePlanningOperationStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: BdtStorageLandOwnerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: BdtTurningPointClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: BdtWorkingSitePlanningStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: BdtString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCount {
    #[serde(flatten)]
    pub image_count: BdtPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: BdtString3000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: BdtString5Type,
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
pub struct AssortmentType {
    #[serde(rename = "AssortmentID", skip_serializing_if = "Option::is_none")]
    pub assortment_i_d: Option<ERPIdType>,
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
pub struct StoragesForestHaulageDistancesType {
    #[serde(rename = "StorageForestHaulageDistance")]
    pub storage_forest_haulage_distance: Vec<StorageForestHaulageDistanceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<Vec<AssortmentType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceType {
    #[serde(rename = "StorageForestHaulageDistanceGroup")]
    pub storage_forest_haulage_distance_group: StorageForestHaulageDistanceGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "@Status")]
    pub status: AssortmentStatusType,
    #[serde(rename = "ProductUserId", skip_serializing_if = "Option::is_none")]
    pub product_user_id: Option<Vec<String>>,
    #[serde(flatten)]
    pub base: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageType {
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
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
    #[serde(rename = "StorageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<StorageDryingClassType>,
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
pub struct StoragesType {
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<StorageType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceGroup {
    #[serde(rename = "StorageId")]
    pub storage_id: StorageId,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: ForestHaulageDistance,
}

