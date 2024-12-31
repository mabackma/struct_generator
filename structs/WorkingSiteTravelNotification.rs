use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct KilometersWithBreakHouse {
    #[serde(flatten)]
    pub kilometers_with_break_house: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    #[serde(flatten)]
    pub vehicle: VehicleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TravelStartTime {
    #[serde(flatten)]
    pub travel_start_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KilometersWithTrailer {
    #[serde(flatten)]
    pub kilometers_with_trailer: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson2 {
    #[serde(flatten)]
    pub extra_person2: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kilometers {
    #[serde(flatten)]
    pub kilometers: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPersonText {
    #[serde(flatten)]
    pub extra_person_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson3 {
    #[serde(flatten)]
    pub extra_person3: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    #[serde(flatten)]
    pub route: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson4 {
    #[serde(flatten)]
    pub extra_person4: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KilometersWithCaravan {
    #[serde(flatten)]
    pub kilometers_with_caravan: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SittingMoneyKilometers {
    #[serde(flatten)]
    pub sitting_money_kilometers: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TravelEndTime {
    #[serde(flatten)]
    pub travel_end_time: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraPerson1 {
    #[serde(flatten)]
    pub extra_person1: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "TravelStartTime")]
    pub travel_start_time: TimeStampType,
    #[serde(rename = "TravelEndTime")]
    pub travel_end_time: TimeStampType,
    #[serde(rename = "Vehicle")]
    pub vehicle: VehicleType,
    #[serde(rename = "Kilometers")]
    pub kilometers: PositiveInteger4digitsType,
    #[serde(rename = "Route")]
    pub route: String200Type,
    #[serde(rename = "KilometersWithTrailer", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_trailer: Option<PositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithCaravan", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_caravan: Option<PositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithBreakHouse", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_break_house: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson1", skip_serializing_if = "Option::is_none")]
    pub extra_person1: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson2", skip_serializing_if = "Option::is_none")]
    pub extra_person2: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson3", skip_serializing_if = "Option::is_none")]
    pub extra_person3: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson4", skip_serializing_if = "Option::is_none")]
    pub extra_person4: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPersonText", skip_serializing_if = "Option::is_none")]
    pub extra_person_text: Option<String200Type>,
    #[serde(rename = "SittingMoneyKilometers", skip_serializing_if = "Option::is_none")]
    pub sitting_money_kilometers: Option<PositiveInteger4digitsType>,
}

