use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalStemVolume {
    #[serde(flatten)]
    pub total_stem_volume: TotalStemVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalHarvestedProductionHeader {
    #[serde(flatten)]
    pub total_harvested_production_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalProductNumberOfLogBunches {
    #[serde(flatten)]
    pub total_product_number_of_log_bunches: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalLogs {
    #[serde(flatten)]
    pub total_logs: TotalLogsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalProductVolume {
    #[serde(flatten)]
    pub total_product_volume: TotalProductVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalLogLength {
    #[serde(flatten)]
    pub total_log_length: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalProductNumberOfLogs {
    #[serde(flatten)]
    pub total_product_number_of_logs: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnclassifiedProductDefinition {
    #[serde(flatten)]
    pub unclassified_product_definition: UnclassifiedProductDefinitionMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Total {
    #[serde(flatten)]
    pub total: TotalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalHarvestedProduction {
    #[serde(flatten)]
    pub total_harvested_production: TotalHarvestedProductionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalNumberOfStems {
    #[serde(flatten)]
    pub total_number_of_stems: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalNumberOfStemBunches {
    #[serde(flatten)]
    pub total_number_of_stem_bunches: XsdnonNegativeInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalStemVolumeType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "totalStemVolumeCategory")]
    pub total_stem_volume_category: StemVolumeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifiedProductDefinitionMachineTotalHarvestedProductionType {
    #[serde(flatten)]
    pub base: ProductDefinitionWithCommonDetailsType,
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: UserIDType,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalHarvestedProductionType {
    #[serde(rename = "TotalHarvestedProductionHeader")]
    pub total_harvested_production_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineTotalHarvestedProductionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalLogsType {
    #[serde(rename = "ProductKey")]
    pub product_key: u32,
    #[serde(rename = "TotalProductNumberOfLogs")]
    pub total_product_number_of_logs: u32,
    #[serde(rename = "TotalProductNumberOfLogBunches", skip_serializing_if = "Option::is_none")]
    pub total_product_number_of_log_bunches: Option<u32>,
    #[serde(rename = "TotalProductVolume")]
    pub total_product_volume: Vec<TotalProductVolumeType>,
    #[serde(rename = "TotalLogLength", skip_serializing_if = "Option::is_none")]
    pub total_log_length: Option<u32>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalType {
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
    #[serde(rename = "TotalNumberOfStems")]
    pub total_number_of_stems: u32,
    #[serde(rename = "TotalNumberOfStemBunches", skip_serializing_if = "Option::is_none")]
    pub total_number_of_stem_bunches: Option<u32>,
    #[serde(rename = "TotalStemVolume")]
    pub total_stem_volume: Vec<TotalStemVolumeType>,
    #[serde(rename = "ProcessingCategory")]
    pub processing_category: ProcessingCategoryType,
    #[serde(rename = "TotalLogs", skip_serializing_if = "Option::is_none")]
    pub total_logs: Option<Vec<TotalLogsType>>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineTotalHarvestedProductionType {
    #[serde(flatten)]
    pub base: MachineWithHeadType,
    #[serde(rename = "SpeciesGroupDefinition", skip_serializing_if = "Option::is_none")]
    pub species_group_definition: Option<Vec<SpeciesGroupDefinitionWithDetailsAndKeyType>>,
    #[serde(rename = "ProductDefinition", skip_serializing_if = "Option::is_none")]
    pub product_definition: Option<Vec<ProductDefinitionMachineTotalHarvestedProductionType>>,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: Vec<ObjectDefinitionMachineType>,
    #[serde(rename = "Total", skip_serializing_if = "Option::is_none")]
    pub total: Option<Vec<TotalType>>,
    #[serde(rename = "UserDefinedData", skip_serializing_if = "Option::is_none")]
    pub user_defined_data: Option<UserDefinedDataTotalHarvestedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionMachineTotalHarvestedProductionType {
    #[serde(rename = "ProductKey")]
    pub product_key: u32,
    #[serde(rename = "ClassifiedProductDefinition")]
    pub classified_product_definition: ClassifiedProductDefinitionMachineTotalHarvestedProductionType,
    #[serde(rename = "UnclassifiedProductDefinition")]
    pub unclassified_product_definition: UnclassifiedProductDefinitionMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalProductVolumeType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "totalProductVolumeCategory")]
    pub total_product_volume_category: LogVolumeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDefinedDataTotalHarvestedProductionType {
    #[serde(rename = "DataTableGroup")]
    pub data_table_group: Vec<DataTableGroupTotalHarvestedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableGroupTotalHarvestedProductionType {
    #[serde(flatten)]
    pub base: DataTableGroupType,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
}

