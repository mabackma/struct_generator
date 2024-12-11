#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(flatten)]
    pub product: ProductType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationType {
    #[serde(flatten)]
    pub operation_type: OperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResource {
    #[serde(flatten)]
    pub planned_resource: PlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: WideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(flatten)]
    pub currency: CurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationMode {
    #[serde(flatten)]
    pub operation_mode: OperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnit {
    #[serde(flatten)]
    pub consumption_unit: ConsumptionUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPrice {
    #[serde(flatten)]
    pub total_price: TotalPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: string,
}

