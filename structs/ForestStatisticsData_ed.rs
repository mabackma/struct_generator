#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsData {
    #[serde(flatten)]
    pub forest_statistics_data: ForestStatisticsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsAssortmentCompactClasses {
    #[serde(flatten)]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: QuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyID {
    #[serde(flatten)]
    pub company_i_d: CompanyIDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCode {
    #[serde(flatten)]
    pub assortment_class_code: AssortmentClassCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRows {
    #[serde(flatten)]
    pub loggings_rows: LoggingsRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantities {
    #[serde(flatten)]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperation {
    #[serde(flatten)]
    pub statistics_operation: StatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClasses {
    #[serde(flatten)]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRow {
    #[serde(flatten)]
    pub operation_row: OperationRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRows {
    #[serde(flatten)]
    pub operation_rows: OperationRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroup {
    #[serde(flatten)]
    pub forest_owner_group: ForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantity {
    #[serde(flatten)]
    pub statistics_quantity: StatisticsQuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationType {
    #[serde(flatten)]
    pub reported_statistics_operation_type: ReportedStatisticsOperationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: StemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeCode {
    #[serde(flatten)]
    pub purchase_mode_code: StatisticsPurchaseModeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCode {
    #[serde(flatten)]
    pub area_code: AreaCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompact {
    #[serde(flatten)]
    pub assortment_compact: AssortmentCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub area_type: AreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRow {
    #[serde(flatten)]
    pub loggings_row: LoggingsRowType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: ExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: StartDateType,
}

