#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson1 {
    #[serde(flatten)]
    pub extra_person1: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson3 {
    #[serde(flatten)]
    pub extra_person3: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelStartTime {
    #[serde(flatten)]
    pub travel_start_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(flatten)]
    pub route: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson4 {
    #[serde(flatten)]
    pub extra_person4: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelEndTime {
    #[serde(flatten)]
    pub travel_end_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithTrailer {
    #[serde(flatten)]
    pub kilometers_with_trailer: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson2 {
    #[serde(flatten)]
    pub extra_person2: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithBreakHouse {
    #[serde(flatten)]
    pub kilometers_with_break_house: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(flatten)]
    pub vehicle: VehicleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kilometers {
    #[serde(flatten)]
    pub kilometers: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPersonText {
    #[serde(flatten)]
    pub extra_person_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithCaravan {
    #[serde(flatten)]
    pub kilometers_with_caravan: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SittingMoneyKilometers {
    #[serde(flatten)]
    pub sitting_money_kilometers: PositiveInteger4digitsType,
}

