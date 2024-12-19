#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnitType {
    #[serde(flatten)]
    pub base: UnitPerHectareType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTypeType {
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
    pub product_key_group: String,
    #[serde(rename = "ProductName")]
    pub product_name: String500Type,
    #[serde(rename = "Quantity")]
    pub quantity: Decimal2FractionDigitsType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: WideUnitType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<TotalPriceType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "Consumption", skip_serializing_if = "Option::is_none")]
    pub consumption: Option<ConsumptionType>,
    #[serde(rename = "ConsumptionUnit", skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<ConsumptionUnitType>,
    #[serde(rename = "PlannedResource", skip_serializing_if = "Option::is_none")]
    pub planned_resource: Option<PlannedResourceType>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String1500Type>,
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
    pub planned_resource: PlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationModeType {
    #[serde(flatten)]
    pub base: OperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductKeyGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfoGroup {
}

