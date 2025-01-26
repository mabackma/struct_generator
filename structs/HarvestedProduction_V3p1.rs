use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct StemHarvestedProductionType {
    #[serde(flatten)]
    pub base: StemType,
    #[serde(rename = "SingleTreeProcessedStem")]
    pub single_tree_processed_stem: SingleTreeProcessedStemType,
    #[serde(rename = "MultiTreeProcessedStem")]
    pub multi_tree_processed_stem: MultiTreeProcessedStemType,
    #[serde(rename = "SingleTreeFelledStem", skip_serializing_if = "Option::is_none")]
    pub single_tree_felled_stem: Option<SingleTreeFelledStemType>,
    #[serde(rename = "MultiTreeFelledStem")]
    pub multi_tree_felled_stem: MultiTreeFelledStemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDefinedDataHarvestedProductionType {
    #[serde(rename = "DataTableGroup")]
    pub data_table_group: Vec<DataTableGroupHarvestedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleTreeProcessedStemType {
    #[serde(rename = "DBH")]
    pub dbh: u32,
    #[serde(rename = "ReferenceDiameter")]
    pub reference_diameter: ReferenceDiameterType,
    #[serde(rename = "StemGrade")]
    pub stem_grade: Vec<StemGradeType>,
    #[serde(rename = "StemDiameters", skip_serializing_if = "Option::is_none")]
    pub stem_diameters: Option<StemDiametersType>,
    #[serde(rename = "Log")]
    pub log: Vec<LogHarvestedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiTreeProcessedStemType {
    #[serde(rename = "StemBunchKey")]
    pub stem_bunch_key: u32,
    #[serde(rename = "DBH", skip_serializing_if = "Option::is_none")]
    pub dbh: Option<u32>,
    #[serde(rename = "ReferenceDiameter")]
    pub reference_diameter: ReferenceDiameterType,
    #[serde(rename = "StemGrade")]
    pub stem_grade: Vec<StemGradeType>,
    #[serde(rename = "Log")]
    pub log: Vec<LogHarvestedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedProductionType {
    #[serde(rename = "HarvestedProductionHeader")]
    pub harvested_production_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineHarvestedProductionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifiedProductDefinitionMachineHarvestedProductionType {
    #[serde(flatten)]
    pub base: ProductDefinitionWithDetailsType,
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: UserIDType,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineHarvestedProductionType {
    #[serde(flatten)]
    pub base: MachineWithHeadType,
    #[serde(rename = "SpeciesGroupDefinition", skip_serializing_if = "Option::is_none")]
    pub species_group_definition: Option<Vec<SpeciesGroupDefinitionWithDetailsAndKeyType>>,
    #[serde(rename = "ProductDefinition", skip_serializing_if = "Option::is_none")]
    pub product_definition: Option<Vec<ProductDefinitionMachineHarvestedProductionType>>,
    #[serde(rename = "DiameterSectionDefinition", skip_serializing_if = "Option::is_none")]
    pub diameter_section_definition: Option<Vec<DiameterSectionDefinitionMachineType>>,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: Vec<ObjectDefinitionMachineType>,
    #[serde(rename = "Stem", skip_serializing_if = "Option::is_none")]
    pub stem: Option<Vec<StemHarvestedProductionType>>,
    #[serde(rename = "UserDefinedData", skip_serializing_if = "Option::is_none")]
    pub user_defined_data: Option<UserDefinedDataHarvestedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableGroupHarvestedProductionType {
    #[serde(flatten)]
    pub base: DataTableGroupType,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiTreeFelledStemType {
    #[serde(rename = "StemBunchKey")]
    pub stem_bunch_key: u32,
    #[serde(rename = "DBH", skip_serializing_if = "Option::is_none")]
    pub dbh: Option<u32>,
    #[serde(rename = "ReferenceDiameter", skip_serializing_if = "Option::is_none")]
    pub reference_diameter: Option<ReferenceDiameterType>,
    #[serde(rename = "StemVolume", skip_serializing_if = "Option::is_none")]
    pub stem_volume: Option<Vec<StemVolumeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionMachineHarvestedProductionType {
    #[serde(rename = "ProductKey")]
    pub product_key: u32,
    #[serde(rename = "ClassifiedProductDefinition")]
    pub classified_product_definition: ClassifiedProductDefinitionMachineHarvestedProductionType,
    #[serde(rename = "UnclassifiedProductDefinition")]
    pub unclassified_product_definition: UnclassifiedProductDefinitionMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogHarvestedProductionType {
    #[serde(flatten)]
    pub base: LogType,
    #[serde(rename = "LogMeasurement")]
    pub log_measurement: LogMeasurementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleTreeFelledStemType {
    #[serde(rename = "DBH", skip_serializing_if = "Option::is_none")]
    pub dbh: Option<u32>,
    #[serde(rename = "ReferenceDiameter", skip_serializing_if = "Option::is_none")]
    pub reference_diameter: Option<ReferenceDiameterType>,
    #[serde(rename = "StemVolume", skip_serializing_if = "Option::is_none")]
    pub stem_volume: Option<Vec<StemVolumeType>>,
}

