use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifiedProductDefinitionProductInstructionType {
    #[serde(flatten)]
    pub base: ProductDefinitionWithDetailsType,
    #[serde(rename = "SpeciesGroupUserID")]
    pub species_group_user_i_d: UserIDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionProductInstructionType {
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: UserIDType,
    #[serde(rename = "ClassifiedProductDefinition")]
    pub classified_product_definition: ClassifiedProductDefinitionProductInstructionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInstructionType {
    #[serde(rename = "ProductInstructionHeader")]
    pub product_instruction_header: MessageHeaderType,
    #[serde(rename = "ProductDefinition")]
    pub product_definition: Vec<ProductDefinitionProductInstructionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

