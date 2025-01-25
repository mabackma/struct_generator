use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_status: BdtOrderStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderConfirmation {
    #[serde(flatten)]
    pub order_confirmation: OrderConfirmationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: BdtString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerArea {
    #[serde(flatten)]
    pub service_buyer_area: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<String20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "OrderStatus")]
    pub order_status: OrderStatusType,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
}

