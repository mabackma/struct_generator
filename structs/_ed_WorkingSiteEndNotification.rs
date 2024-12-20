#[derive(Debug, Serialize, Deserialize)]
pub struct Interrupted {
    #[serde(flatten)]
    pub interrupted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supported {
    #[serde(flatten)]
    pub supported: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeLeft {
    #[serde(flatten)]
    pub volume_left: BdtDecimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountLeft {
    #[serde(flatten)]
    pub amount_left: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfulfilledArea {
    #[serde(flatten)]
    pub unfulfilled_area: GdtPolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledArea {
    #[serde(flatten)]
    pub fulfilled_area: FulfilledAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactorId {
    #[serde(flatten)]
    pub contactor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotification {
    #[serde(flatten)]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
}

