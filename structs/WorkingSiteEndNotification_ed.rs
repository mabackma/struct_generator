#[derive(Debug, Serialize, Deserialize)]
pub struct Interrupted {
    #[serde(flatten)]
    pub interrupted: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountLeft {
    #[serde(flatten)]
    pub amount_left: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactorId {
    #[serde(flatten)]
    pub contactor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledArea {
    #[serde(flatten)]
    pub fulfilled_area: FulfilledAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: AssortmentGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeLeft {
    #[serde(flatten)]
    pub volume_left: Decimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotification {
    #[serde(flatten)]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfulfilledArea {
    #[serde(flatten)]
    pub unfulfilled_area: PolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

