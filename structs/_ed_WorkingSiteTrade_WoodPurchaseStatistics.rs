#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesData {
    #[serde(flatten)]
    pub round_wood_sales_data: RoundWoodSalesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRow {
    #[serde(flatten)]
    pub round_wood_sales_row: RoundWoodSalesRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub area_type: AreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: WtcoPurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRows {
    #[serde(flatten)]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

