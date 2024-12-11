#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReason {
    #[serde(flatten)]
    pub dis_qualification_reason: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationSign {
    #[serde(flatten)]
    pub dis_qualification_sign: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentage {
    #[serde(flatten)]
    pub dis_qualification_percentage: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasons {
    #[serde(flatten)]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentageTotal {
    #[serde(flatten)]
    pub dis_qualification_percentage_total: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracySign {
    #[serde(flatten)]
    pub cutting_accuracy_sign: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceIdMJ {
    #[serde(flatten)]
    pub resource_id_m_j: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonText {
    #[serde(flatten)]
    pub dis_qualification_reason_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracy {
    #[serde(flatten)]
    pub cutting_accuracy: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractNumber {
    #[serde(flatten)]
    pub purchase_contract_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: base64Binary,
}

