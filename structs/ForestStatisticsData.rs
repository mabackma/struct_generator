use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestStatisticsData {
    #[serde(flatten)]
    pub forest_statistics_data: ForestStatisticsDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationRow {
    #[serde(flatten)]
    pub operation_row: OperationRowType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestOwnerGroup {
    #[serde(flatten)]
    pub forest_owner_group: ForestOwnerGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsAssortmentCompactClasses {
    #[serde(flatten)]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsQuantity {
    #[serde(flatten)]
    pub statistics_quantity: StatisticsQuantityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsOperation {
    #[serde(flatten)]
    pub statistics_operation: StatisticsOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedStatisticsOperationType {
    #[serde(flatten)]
    pub reported_statistics_operation_type: ReportedStatisticsOperationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingsRows {
    #[serde(flatten)]
    pub loggings_rows: LoggingsRowsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationRows {
    #[serde(flatten)]
    pub operation_rows: OperationRowsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingsRow {
    #[serde(flatten)]
    pub loggings_row: LoggingsRowType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatisticsQuantities {
    #[serde(flatten)]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: StatisticsPurchaseModeCodeType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "StatisticsAssortmentCompactClasses")]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "StatisticsOperation")]
    pub statistics_operation: StatisticsOperationType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "ReportedStatisticsOperationType")]
    pub reported_statistics_operation_type: ReportedStatisticsOperationTypeType,
    #[serde(rename = "StatisticsQuantities")]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperationType {
    #[serde(flatten)]
    pub base: StatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroupType {
    #[serde(flatten)]
    pub base: ForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityType {
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: ExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfoType {
    #[serde(rename = "assortment_info_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantitiesType {
    #[serde(rename = "StatisticsQuantity")]
    pub statistics_quantity: Vec<StatisticsQuantityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: StemTypeType,
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: ExtendedQuantityUnitType,
    #[serde(rename = "AssortmentInfo")]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(rename = "area_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowsType {
    #[serde(rename = "OperationRow")]
    pub operation_row: Vec<OperationRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowsType {
    #[serde(rename = "LoggingsRow")]
    pub loggings_row: Vec<LoggingsRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RwsRoundWoodSalesRows", skip_serializing_if = "Option::is_none")]
    pub rws_round_wood_sales_rows: Option<RoundWoodSalesRows>,
    #[serde(rename = "OperationRows", skip_serializing_if = "Option::is_none")]
    pub operation_rows: Option<OperationRowsType>,
    #[serde(rename = "LoggingsRows", skip_serializing_if = "Option::is_none")]
    pub loggings_rows: Option<LoggingsRowsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: LoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(flatten)]
    pub base: AreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: AlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIDType {
    #[serde(flatten)]
    pub base: YritysTunnusTyyppi,
}

