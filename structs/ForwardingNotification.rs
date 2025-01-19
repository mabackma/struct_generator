use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessageId {
    #[serde(flatten)]
    pub common_message_id: WctCommonMessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentsChanges {
    #[serde(flatten)]
    pub assortments_changes: AssortmentsChangesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeltedWater {
    #[serde(flatten)]
    pub melted_water: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleAssortmentType {
    #[serde(flatten)]
    pub scale_assortment_type: BdtScaleAssortmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinished {
    #[serde(flatten)]
    pub working_site_finished: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartLoadNumber {
    #[serde(flatten)]
    pub start_load_number: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeName {
    #[serde(flatten)]
    pub code_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientApplicationId {
    #[serde(flatten)]
    pub client_application_id: BdtClientApplicationIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingNotification {
    #[serde(flatten)]
    pub forwarding_notification: ForwardingNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadRange {
    #[serde(flatten)]
    pub load_range: LoadRangeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HumidityMeasured {
    #[serde(flatten)]
    pub humidity_measured: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentsChange {
    #[serde(flatten)]
    pub assortments_change: AssortmentChangeDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeVolume {
    #[serde(flatten)]
    pub change_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub is_forest_haulage_distance_continued: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HasAssortmentChanges {
    #[serde(flatten)]
    pub has_assortment_changes: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemiDry {
    #[serde(flatten)]
    pub semi_dry: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndLoadNumber {
    #[serde(flatten)]
    pub end_load_number: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleFactors {
    #[serde(flatten)]
    pub scale_factors: ScaleFactorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvesterVolume {
    #[serde(flatten)]
    pub harvester_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentStorageVolume {
    #[serde(flatten)]
    pub sent_storage_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessageFreeText {
    #[serde(flatten)]
    pub common_message_free_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleanlinessClass {
    #[serde(flatten)]
    pub cleanliness_class: BdtCleanlinessClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weight {
    #[serde(flatten)]
    pub weight: BdtInteger7digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Month {
    #[serde(flatten)]
    pub month: WctMonthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Finished {
    #[serde(flatten)]
    pub finished: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleFactor {
    #[serde(flatten)]
    pub scale_factor: ScaleFactorDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageFinished {
    #[serde(flatten)]
    pub storage_finished: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Humidity {
    #[serde(flatten)]
    pub humidity: BdtDecimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestHaulageFinishedDate {
    #[serde(flatten)]
    pub forest_haulage_finished_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinishedDate {
    #[serde(flatten)]
    pub working_site_finished_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Covered {
    #[serde(flatten)]
    pub covered: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldDestinationStorage {
    #[serde(flatten)]
    pub old_destination_storage: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentWorkingSiteVolume {
    #[serde(flatten)]
    pub sent_working_site_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldCode {
    #[serde(flatten)]
    pub old_code: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationDate {
    #[serde(flatten)]
    pub notification_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessage {
    #[serde(flatten)]
    pub common_message: CommonMessageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingFinishedDate {
    #[serde(flatten)]
    pub harvesting_finished_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Density {
    #[serde(flatten)]
    pub density: BdtDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScaleTransformation {
    #[serde(flatten)]
    pub scale_transformation: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCode {
    #[serde(flatten)]
    pub new_code: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonMessages {
    #[serde(flatten)]
    pub common_messages: CommonMessagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnowOrIce {
    #[serde(flatten)]
    pub snow_or_ice: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeightClass {
    #[serde(flatten)]
    pub weight_class: BdtPositiveInteger1digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageFinishedDate {
    #[serde(flatten)]
    pub storage_finished_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewDestinationStorage {
    #[serde(flatten)]
    pub new_destination_storage: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorsType {
    #[serde(rename = "ScaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<Vec<ScaleFactorDataType>>,
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
pub struct CommonMessagesType {
    #[serde(rename = "CommonMessage", skip_serializing_if = "Option::is_none")]
    pub common_message: Option<Vec<CommonMessageDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChangesType {
    #[serde(rename = "AssortmentsChange", skip_serializing_if = "Option::is_none")]
    pub assortments_change: Option<Vec<AssortmentChangeDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRangeType {
    #[serde(rename = "StartLoadNumber")]
    pub start_load_number: PositiveInteger4digitsType,
    #[serde(rename = "EndLoadNumber")]
    pub end_load_number: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<ERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<WorkCodeUnitType>,
    #[serde(rename = "SentStorageVolume")]
    pub sent_storage_volume: Decimal3FractionDigitsType,
    #[serde(rename = "SentWorkingSiteVolume")]
    pub sent_working_site_volume: Decimal3FractionDigitsType,
    #[serde(rename = "HarvesterVolume", skip_serializing_if = "Option::is_none")]
    pub harvester_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "ForestHaulageDistanceContinued", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance_continued: Option<PositiveIntegerType>,
    #[serde(rename = "Covered")]
    pub covered: YesNoType,
    #[serde(rename = "HasAssortmentChanges")]
    pub has_assortment_changes: YesNoType,
    #[serde(rename = "ScaleTransformation")]
    pub scale_transformation: YesNoType,
    #[serde(rename = "Finished")]
    pub finished: YesNoType,
    #[serde(rename = "DeliveryUserId", skip_serializing_if = "Option::is_none")]
    pub delivery_user_id: Option<String50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageDataType {
    #[serde(rename = "CommonMessageId", skip_serializing_if = "Option::is_none")]
    pub common_message_id: Option<CommonMessageType>,
    #[serde(rename = "CommonMessageFreeText", skip_serializing_if = "Option::is_none")]
    pub common_message_free_text: Option<String200Type>,
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

