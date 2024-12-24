#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithBreakHouse {
    #[serde(flatten)]
    pub kilometers_with_break_house: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(flatten)]
    pub vehicle: BdtVehicleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson3 {
    #[serde(flatten)]
    pub extra_person3: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithTrailer {
    #[serde(flatten)]
    pub kilometers_with_trailer: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SittingMoneyKilometers {
    #[serde(flatten)]
    pub sitting_money_kilometers: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelStartTime {
    #[serde(flatten)]
    pub travel_start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelEndTime {
    #[serde(flatten)]
    pub travel_end_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson2 {
    #[serde(flatten)]
    pub extra_person2: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(flatten)]
    pub route: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KilometersWithCaravan {
    #[serde(flatten)]
    pub kilometers_with_caravan: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPersonText {
    #[serde(flatten)]
    pub extra_person_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kilometers {
    #[serde(flatten)]
    pub kilometers: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson1 {
    #[serde(flatten)]
    pub extra_person1: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraPerson4 {
    #[serde(flatten)]
    pub extra_person4: BdtPositiveInteger4digitsType,
}

