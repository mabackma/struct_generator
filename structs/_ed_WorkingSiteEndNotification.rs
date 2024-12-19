#[derive(Debug, Serialize, Deserialize)]
pub struct AmountLeft {
    #[serde(flatten)]
    pub amount_left: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactorId {
    #[serde(flatten)]
    pub contactor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interrupted {
    #[serde(flatten)]
    pub interrupted: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeLeft {
    #[serde(flatten)]
    pub volume_left: Decimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotification {
    #[serde(flatten)]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledArea {
    #[serde(flatten)]
    pub fulfilled_area: FulfilledAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfulfilledArea {
    #[serde(flatten)]
    pub unfulfilled_area: PolygonOrMultiPolygon2Type,
}

