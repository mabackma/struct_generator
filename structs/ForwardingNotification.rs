#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorDataType {
    #[serde(rename = "ScaleAssortmentType")]
    pub scale_assortment_type: BdtScaleAssortmentType,
    #[serde(rename = "LoadCount")]
    pub load_count: BdtPositiveIntegerType,
    #[serde(rename = "Area")]
    pub area: WctAreaCodeType,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "CodeName")]
    pub code_name: BdtString50Type,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<WctScaleFactorTreeSpeciesType>,
    #[serde(rename = "Month", skip_serializing_if = "Option::is_none")]
    pub month: Option<WctMonthType>,
    #[serde(rename = "HarvestingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub harvesting_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "SemiDry", skip_serializing_if = "Option::is_none")]
    pub semi_dry: Option<BdtYesNoType>,
    #[serde(rename = "SnowOrIce", skip_serializing_if = "Option::is_none")]
    pub snow_or_ice: Option<BdtYesNoType>,
    #[serde(rename = "WeightClass", skip_serializing_if = "Option::is_none")]
    pub weight_class: Option<BdtPositiveInteger1digitsType>,
    #[serde(rename = "Humidity", skip_serializing_if = "Option::is_none")]
    pub humidity: Option<BdtDecimal1FractionDigitType>,
    #[serde(rename = "HumidityMeasured", skip_serializing_if = "Option::is_none")]
    pub humidity_measured: Option<BdtYesNoType>,
    #[serde(rename = "ForestHaulageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "CleanlinessClass", skip_serializing_if = "Option::is_none")]
    pub cleanliness_class: Option<BdtCleanlinessClassType>,
    #[serde(rename = "Weight")]
    pub weight: BdtInteger7digitsType,
    #[serde(rename = "Density")]
    pub density: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "MeltedWater", skip_serializing_if = "Option::is_none")]
    pub melted_water: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChangesType {
    #[serde(rename = "AssortmentsChange", skip_serializing_if = "Option::is_none")]
    pub assortments_change: Option<Vec<AssortmentChangeDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessagesType {
    #[serde(rename = "CommonMessage", skip_serializing_if = "Option::is_none")]
    pub common_message: Option<Vec<CommonMessageDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorsType {
    #[serde(rename = "ScaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<Vec<ScaleFactorDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<WctERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<BdtWorkCodeUnitType>,
    #[serde(rename = "SentStorageVolume")]
    pub sent_storage_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "SentWorkingSiteVolume")]
    pub sent_working_site_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "HarvesterVolume", skip_serializing_if = "Option::is_none")]
    pub harvester_volume: Option<BdtDecimal3FractionDigitsType>,
    #[serde(rename = "ForestHaulageDistanceContinued", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance_continued: Option<BdtPositiveIntegerType>,
    #[serde(rename = "Covered")]
    pub covered: BdtYesNoType,
    #[serde(rename = "HasAssortmentChanges")]
    pub has_assortment_changes: BdtYesNoType,
    #[serde(rename = "ScaleTransformation")]
    pub scale_transformation: BdtYesNoType,
    #[serde(rename = "Finished")]
    pub finished: BdtYesNoType,
    #[serde(rename = "DeliveryUserId", skip_serializing_if = "Option::is_none")]
    pub delivery_user_id: Option<BdtString50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: BdtString20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: BdtPositiveIntegerType,
    #[serde(rename = "IsForestHaulageDistanceContinued")]
    pub is_forest_haulage_distance_continued: BdtYesNoType,
    #[serde(rename = "StorageFinished")]
    pub storage_finished: BdtYesNoType,
    #[serde(rename = "StorageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub storage_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "WorkingSiteFinished")]
    pub working_site_finished: BdtYesNoType,
    #[serde(rename = "WorkingSiteFinishedDate", skip_serializing_if = "Option::is_none")]
    pub working_site_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "NotificationDate")]
    pub notification_date: BdtTimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "AssortmentsChanges", skip_serializing_if = "Option::is_none")]
    pub assortments_changes: Option<AssortmentsChangesType>,
    #[serde(rename = "CommonMessages", skip_serializing_if = "Option::is_none")]
    pub common_messages: Option<CommonMessagesType>,
    #[serde(rename = "ScaleFactors", skip_serializing_if = "Option::is_none")]
    pub scale_factors: Option<ScaleFactorsType>,
    #[serde(rename = "ClientApplicationId", skip_serializing_if = "Option::is_none")]
    pub client_application_id: Option<BdtClientApplicationIdType>,
    #[serde(rename = "LoadRange", skip_serializing_if = "Option::is_none")]
    pub load_range: Option<LoadRangeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageDataType {
    #[serde(rename = "CommonMessageId", skip_serializing_if = "Option::is_none")]
    pub common_message_id: Option<WctCommonMessageType>,
    #[serde(rename = "CommonMessageFreeText", skip_serializing_if = "Option::is_none")]
    pub common_message_free_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRangeType {
    #[serde(rename = "StartLoadNumber")]
    pub start_load_number: BdtPositiveInteger4digitsType,
    #[serde(rename = "EndLoadNumber")]
    pub end_load_number: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentChangeDataType {
    #[serde(rename = "OldDestinationStorage")]
    pub old_destination_storage: BdtString20Type,
    #[serde(rename = "OldCode")]
    pub old_code: BdtString50Type,
    #[serde(rename = "NewDestinationStorage")]
    pub new_destination_storage: BdtString20Type,
    #[serde(rename = "NewCode")]
    pub new_code: BdtString50Type,
    #[serde(rename = "ChangeVolume")]
    pub change_volume: BdtDecimal3FractionDigitsType,
}

