use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(flatten)]
    pub product: ProductType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumptionUnit {
    #[serde(flatten)]
    pub consumption_unit: ConsumptionUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: CoString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnitType {
    #[serde(flatten)]
    pub base: CoUnitPerHectareType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationModeType {
    #[serde(flatten)]
    pub base: CoOperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@operationId")]
    pub operation_id: String,
    #[serde(rename = "@standId")]
    pub stand_id: String,
    #[serde(rename = "@productId")]
    pub product_id: String,
    #[serde(rename = "ProductKeyGroup")]
    pub product_key_group: ProductKeyGroup,
    #[serde(rename = "ProductName")]
    pub product_name: CoString500Type,
    #[serde(rename = "Quantity")]
    pub quantity: CoDecimal2FractionDigitsType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoWideUnitType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<WtcTotalPriceType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "Consumption", skip_serializing_if = "Option::is_none")]
    pub consumption: Option<ConsumptionType>,
    #[serde(rename = "ConsumptionUnit", skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<ConsumptionUnitType>,
    #[serde(rename = "PlannedResource", skip_serializing_if = "Option::is_none")]
    pub planned_resource: Option<WtcPlannedResourceType>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTypeType {
    #[serde(rename = "CuttingTypeType")]
    pub op_cutting_type_type: OpCuttingTypeType,
    #[serde(rename = "SilvicultureTypeType")]
    pub op_silviculture_type_type: OpSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsType {
    #[serde(rename = "Product")]
    pub product: Vec<ProductType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: WtcPlannedResourceType,
}

