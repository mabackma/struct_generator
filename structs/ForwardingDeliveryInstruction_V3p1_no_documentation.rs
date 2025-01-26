use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeciesGroupDefinition {
    #[serde(flatten)]
    pub species_group_definition: CommonSpeciesGroupDefinitionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductDefinition {
    #[serde(flatten)]
    pub product_definition: ProductDefinitionForwardingDeliveryInstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingDeliveryInstructionHeader {
    #[serde(flatten)]
    pub forwarding_delivery_instruction_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
    #[serde(flatten)]
    pub extension: ExtensionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUserID {
    #[serde(flatten)]
    pub product_user_i_d: UserIDType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingDeliveryInstruction {
    #[serde(flatten)]
    pub forwarding_delivery_instruction: ForwardingDeliveryInstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeciesGroupUserID {
    #[serde(flatten)]
    pub species_group_user_i_d: UserIDType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryDefinition {
    #[serde(flatten)]
    pub delivery_definition: DeliveryDefinitionForwardingDeliveryInstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductCreationDate {
    #[serde(flatten)]
    pub product_creation_date: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassifiedProductDefinition {
    #[serde(flatten)]
    pub classified_product_definition: ClassifiedProductDefinitionForwardingDeliveryInstructionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDefinitionForwardingDeliveryInstructionType {
    #[serde(flatten)]
    pub base: CommonDeliveryDefinitionType,
    #[serde(rename = "ProductUserID", skip_serializing_if = "Option::is_none")]
    pub product_user_i_d: Option<Vec<UserIDType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifiedProductDefinitionForwardingDeliveryInstructionType {
    #[serde(flatten)]
    pub base: CommonProductDefinitionType,
    #[serde(rename = "SpeciesGroupUserID")]
    pub species_group_user_i_d: UserIDType,
    #[serde(rename = "ProductCreationDate", skip_serializing_if = "Option::is_none")]
    pub product_creation_date: Option<StanForD2010DateTimeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingDeliveryInstructionType {
    #[serde(rename = "ForwardingDeliveryInstructionHeader")]
    pub forwarding_delivery_instruction_header: MessageHeaderType,
    #[serde(rename = "SpeciesGroupDefinition", skip_serializing_if = "Option::is_none")]
    pub species_group_definition: Option<Vec<CommonSpeciesGroupDefinitionType>>,
    #[serde(rename = "ProductDefinition", skip_serializing_if = "Option::is_none")]
    pub product_definition: Option<Vec<ProductDefinitionForwardingDeliveryInstructionType>>,
    #[serde(rename = "DeliveryDefinition")]
    pub delivery_definition: Vec<DeliveryDefinitionForwardingDeliveryInstructionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionForwardingDeliveryInstructionType {
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: UserIDType,
    #[serde(rename = "ClassifiedProductDefinition")]
    pub classified_product_definition: ClassifiedProductDefinitionForwardingDeliveryInstructionType,
}

