#[derive(Debug, Serialize, Deserialize)]
pub struct ProductName {
    #[serde(flatten)]
    pub product_name: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnit {
    #[serde(flatten)]
    pub consumption_unit: ConsumptionUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumption {
    #[serde(flatten)]
    pub consumption: ConsumptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(flatten)]
    pub product: ProductType,
}

