#[derive(Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub area_type: AreaTypeType,
}

#[derive(Serialize, Deserialize)]
pub struct RoundWoodSalesRow {
    #[serde(flatten)]
    pub round_wood_sales_row: RoundWoodSalesRowType,
}

#[derive(Serialize, Deserialize)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Serialize, Deserialize)]
pub struct RoundWoodSalesRows {
    #[serde(flatten)]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Serialize, Deserialize)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: WtcoPurchaseModeType,
}

#[derive(Serialize, Deserialize)]
pub struct RoundWoodSalesData {
    #[serde(flatten)]
    pub round_wood_sales_data: RoundWoodSalesDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: JhsAlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: WtcoPurchaseModeType,
    #[serde(rename = "AssortmentCompactClasses")]
    pub assortment_compact_classes: AsAssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowsType {
    #[serde(rename = "RoundWoodSalesRow")]
    pub round_wood_sales_row: Vec<RoundWoodSalesRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(flatten)]
    pub base: CoAreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIDType {
    #[serde(flatten)]
    pub base: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesDataType {
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RoundWoodSalesRows")]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

