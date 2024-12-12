#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessages {
    #[serde(flatten)]
    pub common_messages: CommonMessagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartLoadNumber {
    #[serde(flatten)]
    pub start_load_number: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndLoadNumber {
    #[serde(flatten)]
    pub end_load_number: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleAssortmentType {
    #[serde(flatten)]
    pub scale_assortment_type: ScaleAssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinishedDate {
    #[serde(flatten)]
    pub working_site_finished_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageFreeText {
    #[serde(flatten)]
    pub common_message_free_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeVolume {
    #[serde(flatten)]
    pub change_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadCount {
    #[serde(flatten)]
    pub load_count: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageFinished {
    #[serde(flatten)]
    pub storage_finished: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnowOrIce {
    #[serde(flatten)]
    pub snow_or_ice: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChange {
    #[serde(flatten)]
    pub assortments_change: AssortmentChangeDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotification {
    #[serde(flatten)]
    pub forwarding_notification: ForwardingNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChanges {
    #[serde(flatten)]
    pub assortments_changes: AssortmentsChangesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub is_forest_haulage_distance_continued: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageFinishedDate {
    #[serde(flatten)]
    pub storage_finished_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCode {
    #[serde(flatten)]
    pub new_code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Month {
    #[serde(flatten)]
    pub month: MonthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationDate {
    #[serde(flatten)]
    pub notification_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactor {
    #[serde(flatten)]
    pub scale_factor: ScaleFactorDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentStorageVolume {
    #[serde(flatten)]
    pub sent_storage_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleTransformation {
    #[serde(flatten)]
    pub scale_transformation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeltedWater {
    #[serde(flatten)]
    pub melted_water: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientApplicationId {
    #[serde(flatten)]
    pub client_application_id: ClientApplicationIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactors {
    #[serde(flatten)]
    pub scale_factors: ScaleFactorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingFinishedDate {
    #[serde(flatten)]
    pub harvesting_finished_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Covered {
    #[serde(flatten)]
    pub covered: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDestinationStorage {
    #[serde(flatten)]
    pub new_destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OldCode {
    #[serde(flatten)]
    pub old_code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageId {
    #[serde(flatten)]
    pub common_message_id: CommonMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightClass {
    #[serde(flatten)]
    pub weight_class: PositiveInteger1digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Finished {
    #[serde(flatten)]
    pub finished: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumidityMeasured {
    #[serde(flatten)]
    pub humidity_measured: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinished {
    #[serde(flatten)]
    pub working_site_finished: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanlinessClass {
    #[serde(flatten)]
    pub cleanliness_class: CleanlinessClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentWorkingSiteVolume {
    #[serde(flatten)]
    pub sent_working_site_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRange {
    #[serde(flatten)]
    pub load_range: LoadRangeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageFinishedDate {
    #[serde(flatten)]
    pub forest_haulage_finished_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasAssortmentChanges {
    #[serde(flatten)]
    pub has_assortment_changes: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessage {
    #[serde(flatten)]
    pub common_message: CommonMessageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Humidity {
    #[serde(flatten)]
    pub humidity: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterVolume {
    #[serde(flatten)]
    pub harvester_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OldDestinationStorage {
    #[serde(flatten)]
    pub old_destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weight {
    #[serde(flatten)]
    pub weight: Integer7digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Density {
    #[serde(flatten)]
    pub density: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeName {
    #[serde(flatten)]
    pub code_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: ScaleFactorTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemiDry {
    #[serde(flatten)]
    pub semi_dry: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: WorkCodeUnitType,
}

