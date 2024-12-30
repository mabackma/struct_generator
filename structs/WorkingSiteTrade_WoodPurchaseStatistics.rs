use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaType {
    #[serde(flatten)]
    pub area_type: AreaTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundWoodSalesRow {
    #[serde(flatten)]
    pub round_wood_sales_row: RoundWoodSalesRowType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundWoodSalesRows {
    #[serde(flatten)]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundWoodSalesData {
    #[serde(flatten)]
    pub round_wood_sales_data: RoundWoodSalesDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: PurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(flatten)]
    pub base: CoAreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowsType {
    #[serde(rename = "RoundWoodSalesRow")]
    pub round_wood_sales_row: Vec<RoundWoodSalesRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIDType {
    #[serde(flatten)]
    pub base: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: PurchaseModeType,
    #[serde(rename = "AssortmentCompactClasses")]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(rename = "area_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: JhsAlkuPvmTyyppi,
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

