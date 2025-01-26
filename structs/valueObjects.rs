use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct valueComponents {
    #[serde(flatten)]
    pub value_components: GmlValueArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boolean {
    #[serde(flatten)]
    pub boolean: boolean,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryList {
    #[serde(flatten)]
    pub category_list: GmlCodeOrNullListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositeValue {
    #[serde(flatten)]
    pub composite_value: GmlCompositeValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountExtent {
    #[serde(flatten)]
    pub count_extent: GmlCountExtentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuantityList {
    #[serde(flatten)]
    pub quantity_list: GmlMeasureOrNullListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryExtent {
    #[serde(flatten)]
    pub category_extent: GmlCategoryExtentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct valueComponent {
    #[serde(flatten)]
    pub value_component: GmlValuePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueArray {
    #[serde(flatten)]
    pub value_array: GmlValueArrayType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct valueProperty {
    #[serde(flatten)]
    pub value_property: GmlValuePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuantityExtent {
    #[serde(flatten)]
    pub quantity_extent: GmlQuantityExtentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BooleanList {
    #[serde(flatten)]
    pub boolean_list: GmlbooleanOrNullList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountList {
    #[serde(flatten)]
    pub count_list: GmlintegerOrNullList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPropertyType {
    #[serde(flatten)]
    pub base: GmlValuePropertyType,
    #[serde(rename = "Category")]
    pub gml_category: GmlCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueArrayType {
    #[serde(flatten)]
    pub base: GmlCompositeValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountExtentType {
    #[serde(flatten)]
    pub base: GmlintegerOrNullList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountPropertyType {
    #[serde(flatten)]
    pub base: GmlValuePropertyType,
    #[serde(rename = "Count")]
    pub gml_count: GmlCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueArrayPropertyType {
    #[serde(rename = "Value")]
    pub gml_value: Vec<GmlValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityExtentType {
    #[serde(flatten)]
    pub base: GmlMeasureOrNullListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanPropertyType {
    #[serde(flatten)]
    pub base: GmlValuePropertyType,
    #[serde(rename = "Boolean")]
    pub gml_boolean: GmlBoolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryExtentType {
    #[serde(flatten)]
    pub base: GmlCodeOrNullListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeValueType {
    #[serde(flatten)]
    pub base: GmlAbstractGMLType,
    #[serde(rename = "valueComponent", skip_serializing_if = "Option::is_none")]
    pub gmlvalue_component: Option<Vec<GmlvalueComponent>>,
    #[serde(rename = "valueComponents", skip_serializing_if = "Option::is_none")]
    pub gmlvalue_components: Option<GmlvalueComponents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityPropertyType {
    #[serde(flatten)]
    pub base: GmlValuePropertyType,
    #[serde(rename = "Quantity")]
    pub gml_quantity: GmlQuantity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValuePropertyType {
    #[serde(rename = "Value")]
    pub gml_value: GmlValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalarValuePropertyType {
    #[serde(flatten)]
    pub base: GmlValuePropertyType,
    #[serde(rename = "ScalarValue")]
    pub gml_scalar_value: GmlScalarValue,
}

