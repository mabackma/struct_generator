use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataEnumDefinitionType {
    #[serde(rename = "EnumValue")]
    pub enum_value: Vec<String>,
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataColumnDefinitionType {
    #[serde(rename = "columnID")]
    pub column_i_d: String,
    #[serde(rename = "columnName")]
    pub column_name: String,
    #[serde(rename = "dataCategory")]
    pub data_category: DataCategoryType,
    #[serde(rename = "optional")]
    pub optional: bool,
    #[serde(rename = "columnOrder")]
    pub column_order: columnOrder,
    #[serde(rename = "enumListID")]
    pub enum_list_i_d: String,
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataCellDefinitionType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "cellID")]
    pub cell_i_d: String,
    #[serde(rename = "cellText")]
    pub cell_text: String,
    #[serde(rename = "dataCategory")]
    pub data_category: DataCategoryType,
    #[serde(rename = "optional")]
    pub optional: bool,
    #[serde(rename = "columnOrder")]
    pub column_order: u32,
    #[serde(rename = "enumListID")]
    pub enum_list_i_d: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRowDefinitionType {
    #[serde(rename = "DataCellDefinition")]
    pub data_cell_definition: Vec<DataCellDefinitionType>,
    #[serde(rename = "rowID")]
    pub row_i_d: String,
    #[serde(rename = "rowOrder")]
    pub row_order: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDefinedDataInstructionType {
    #[serde(rename = "UserDefinedDataInstructionHeader")]
    pub user_defined_data_instruction_header: MessageHeaderType,
    #[serde(rename = "UserDefinedDataDefinition")]
    pub user_defined_data_definition: UserDefinedDataDefinitionType,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputDataLocationType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableDefinitionType {
    #[serde(rename = "DataColumnDefinition")]
    pub data_column_definition: Vec<DataColumnDefinitionType>,
    #[serde(rename = "DataRowDefinition", skip_serializing_if = "Option::is_none")]
    pub data_row_definition: Option<Vec<DataRowDefinitionType>>,
    #[serde(rename = "tableID")]
    pub table_i_d: String,
    #[serde(rename = "tableName")]
    pub table_name: tableName,
    #[serde(rename = "minRowCount")]
    pub min_row_count: u32,
    #[serde(rename = "maxRowCount")]
    pub max_row_count: u32,
    #[serde(rename = "tableTriggerPoint")]
    pub table_trigger_point: TableTriggerPointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableGroupDefinitionType {
    #[serde(rename = "DataTableDefinition")]
    pub data_table_definition: Vec<DataTableDefinitionType>,
    #[serde(rename = "DataEnumDefinition", skip_serializing_if = "Option::is_none")]
    pub data_enum_definition: Option<Vec<DataEnumDefinitionType>>,
    #[serde(rename = "tableGroupID")]
    pub table_group_i_d: String,
    #[serde(rename = "tableGroupName")]
    pub table_group_name: String,
    #[serde(rename = "outputDataLocation")]
    pub output_data_location: OutputDataLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableTriggerPointType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDefinedDataDefinitionType {
    #[serde(rename = "DataTableGroupDefinition")]
    pub data_table_group_definition: Vec<DataTableGroupDefinitionType>,
}

