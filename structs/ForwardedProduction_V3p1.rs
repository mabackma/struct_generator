use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardEndDate {
    #[serde(flatten)]
    pub forward_end_date: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadingCoordinates {
    #[serde(flatten)]
    pub loading_coordinates: CoordinatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationKey {
    #[serde(flatten)]
    pub location_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingStatus {
    #[serde(flatten)]
    pub forwarding_status: ForwardingStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnloadingTime {
    #[serde(flatten)]
    pub unloading_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DistanceFromLastUnloading {
    #[serde(flatten)]
    pub distance_from_last_unloading: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardedProductionHeader {
    #[serde(flatten)]
    pub forwarded_production_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadKey {
    #[serde(flatten)]
    pub load_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardStartDate {
    #[serde(flatten)]
    pub forward_start_date: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialLoad {
    #[serde(flatten)]
    pub partial_load: PartialLoadType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialLoadKey {
    #[serde(flatten)]
    pub partial_load_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardedProduction {
    #[serde(flatten)]
    pub forwarded_production: ForwardedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadType {
    #[serde(rename = "LoadKey")]
    pub load_key: u32,
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "LoadNumber")]
    pub load_number: u32,
    #[serde(rename = "DistanceFromLastUnloading", skip_serializing_if = "Option::is_none")]
    pub distance_from_last_unloading: Option<u32>,
    #[serde(rename = "UnloadingTime")]
    pub unloading_time: StanForD2010DateTimeType,
    #[serde(rename = "PartialLoad")]
    pub partial_load: Vec<PartialLoadType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDefinedDataForwardedProductionType {
    #[serde(rename = "DataTableGroup")]
    pub data_table_group: Vec<DataTableGroupForwardedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedProductionType {
    #[serde(rename = "ForwardedProductionHeader")]
    pub forwarded_production_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineForwardedProductionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineForwardedProductionType {
    #[serde(flatten)]
    pub base: MachineType,
    #[serde(rename = "SpeciesGroupDefinition", skip_serializing_if = "Option::is_none")]
    pub species_group_definition: Option<Vec<SpeciesGroupDefinitionForwardedProductionType>>,
    #[serde(rename = "ProductDefinition", skip_serializing_if = "Option::is_none")]
    pub product_definition: Option<Vec<ProductDefinitionMachineForwardedProductionType>>,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: Vec<ObjectDefinitionMachineType>,
    #[serde(rename = "LocationDefinition", skip_serializing_if = "Option::is_none")]
    pub location_definition: Option<Vec<LocationDefinitionMachineType>>,
    #[serde(rename = "DeliveryDefinition", skip_serializing_if = "Option::is_none")]
    pub delivery_definition: Option<Vec<DeliveryDefinitionWithDetailsType>>,
    #[serde(rename = "Load", skip_serializing_if = "Option::is_none")]
    pub load: Option<Vec<LoadType>>,
    #[serde(rename = "ForwardingStatus", skip_serializing_if = "Option::is_none")]
    pub forwarding_status: Option<Vec<ForwardingStatusType>>,
    #[serde(rename = "ScaleDefinition", skip_serializing_if = "Option::is_none")]
    pub scale_definition: Option<Vec<ScaleDefinitionType>>,
    #[serde(rename = "UserDefinedData", skip_serializing_if = "Option::is_none")]
    pub user_defined_data: Option<UserDefinedDataForwardedProductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadVolumeType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "loadVolumeCategory")]
    pub load_volume_category: LoadVolumeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesGroupDefinitionForwardedProductionType {
    #[serde(flatten)]
    pub base: CommonSpeciesGroupDefinitionType,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationDefinitionMachineType {
    #[serde(flatten)]
    pub base: CommonLocationDefinitionType,
    #[serde(rename = "LocationKey")]
    pub location_key: u32,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadGreenMassType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "loadGreenMassMethod")]
    pub load_green_mass_method: LoadGreenMassMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifiedProductDefinitionMachineForwardedProductionType {
    #[serde(flatten)]
    pub base: CommonProductDefinitionType,
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: UserIDType,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialLoadType {
    #[serde(rename = "PartialLoadKey")]
    pub partial_load_key: u32,
    #[serde(rename = "DeliveryKey")]
    pub delivery_key: u32,
    #[serde(rename = "LocationKey")]
    pub location_key: u32,
    #[serde(rename = "LoadNumberOfItems", skip_serializing_if = "Option::is_none")]
    pub load_number_of_items: Option<LoadNumberOfItemsType>,
    #[serde(rename = "LoadVolume", skip_serializing_if = "Option::is_none")]
    pub load_volume: Option<Vec<LoadVolumeType>>,
    #[serde(rename = "LoadGreenMass", skip_serializing_if = "Option::is_none")]
    pub load_green_mass: Option<LoadGreenMassType>,
    #[serde(rename = "LoadingCoordinates", skip_serializing_if = "Option::is_none")]
    pub loading_coordinates: Option<CoordinatesType>,
    #[serde(rename = "ScaleKey", skip_serializing_if = "Option::is_none")]
    pub scale_key: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableGroupForwardedProductionType {
    #[serde(flatten)]
    pub base: DataTableGroupType,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
    #[serde(rename = "LocationKey")]
    pub location_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionMachineForwardedProductionType {
    #[serde(rename = "ProductKey")]
    pub product_key: u32,
    #[serde(rename = "ClassifiedProductDefinition")]
    pub classified_product_definition: ClassifiedProductDefinitionMachineForwardedProductionType,
    #[serde(rename = "UnclassifiedProductDefinition")]
    pub unclassified_product_definition: UnclassifiedProductDefinitionMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadVolumeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingStatusType {
    #[serde(rename = "LocationKey")]
    pub location_key: u32,
    #[serde(rename = "DeliveryKey")]
    pub delivery_key: u32,
    #[serde(rename = "ForwardStartDate")]
    pub forward_start_date: StanForD2010DateTimeType,
    #[serde(rename = "ForwardEndDate", skip_serializing_if = "Option::is_none")]
    pub forward_end_date: Option<StanForD2010DateTimeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadGreenMassMethodType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

