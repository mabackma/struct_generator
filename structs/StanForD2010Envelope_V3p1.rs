use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddedAttachment {
    #[serde(flatten)]
    pub embedded_attachment: EmbeddedType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compression {
    #[serde(flatten)]
    pub compression: CompressionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentName {
    #[serde(flatten)]
    pub attachment_name: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StanForD2010Envelope {
    #[serde(flatten)]
    pub stan_for_d2010_envelope: StanForD2010EnvelopeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalDocument {
    #[serde(flatten)]
    pub external_document: ExternalDocumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompressionSoftware {
    #[serde(flatten)]
    pub compression_software: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentName {
    #[serde(flatten)]
    pub document_name: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionID {
    #[serde(flatten)]
    pub transmission_i_d: UserIDType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaName {
    #[serde(flatten)]
    pub schema_name: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    #[serde(flatten)]
    pub signature: SignatureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StanForD2010Message {
    #[serde(flatten)]
    pub stan_for_d2010_message: StanForD2010MessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompressionCategory {
    #[serde(flatten)]
    pub compression_category: CompressionCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageAttachment {
    #[serde(flatten)]
    pub message_attachment: AttachmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddedDocument {
    #[serde(flatten)]
    pub embedded_document: EmbeddedType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionInfo {
    #[serde(flatten)]
    pub transmission_info: TransmissionInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLocation {
    #[serde(flatten)]
    pub document_location: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalAttachment {
    #[serde(flatten)]
    pub external_attachment: ExternalDocumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDocument {
    #[serde(flatten)]
    pub message_document: MessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StanForD2010EnvelopeHeader {
    #[serde(flatten)]
    pub stan_for_d2010_envelope_header: StanForD2010EnvelopeHeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanForD2010EnvelopeType {
    #[serde(rename = "StanForD2010EnvelopeHeader")]
    pub stan_for_d2010_envelope_header: StanForD2010EnvelopeHeaderType,
    #[serde(rename = "StanForD2010Message")]
    pub stan_for_d2010_message: Vec<StanForD2010MessageType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
    #[serde(rename = "version")]
    pub version: StanForD2010VersionType,
    #[serde(rename = "versionDate")]
    pub version_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentType {
    #[serde(rename = "EmbeddedAttachment")]
    pub embedded_attachment: EmbeddedType,
    #[serde(rename = "ExternalAttachment")]
    pub external_attachment: ExternalDocumentType,
    #[serde(rename = "AttachmentName", skip_serializing_if = "Option::is_none")]
    pub attachment_name: Option<String>,
    #[serde(rename = "SchemaName", skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "Signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<SignatureType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDocumentType {
    #[serde(rename = "DocumentLocation")]
    pub document_location: String,
    #[serde(rename = "Compression", skip_serializing_if = "Option::is_none")]
    pub compression: Option<CompressionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(rename = "EmbeddedDocument")]
    pub embedded_document: EmbeddedType,
    #[serde(rename = "ExternalDocument")]
    pub external_document: ExternalDocumentType,
    #[serde(rename = "DocumentName", skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "SchemaName", skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "Signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<SignatureType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanForD2010MessageType {
    #[serde(rename = "MessageDocument")]
    pub message_document: MessageType,
    #[serde(rename = "MessageAttachment", skip_serializing_if = "Option::is_none")]
    pub message_attachment: Option<Vec<AttachmentType>>,
    #[serde(rename = "messageCategory")]
    pub message_category: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanForD2010EnvelopeHeaderType {
    #[serde(rename = "TransmissionInfo", skip_serializing_if = "Option::is_none")]
    pub transmission_info: Option<Vec<TransmissionInfoType>>,
    #[serde(rename = "BusinessSender")]
    pub business_sender: ContactInformationType,
    #[serde(rename = "BusinessReceiver", skip_serializing_if = "Option::is_none")]
    pub business_receiver: Option<ContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "encodingMethod")]
    pub encoding_method: encodingMethod,
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmissionInfoType {
    #[serde(rename = "TransmissionID")]
    pub transmission_i_d: UserIDType,
    #[serde(rename = "TransmissionTime")]
    pub transmission_time: StanForD2010DateTimeType,
    #[serde(rename = "TransmissionSender")]
    pub transmission_sender: String,
    #[serde(rename = "TransmissionReceiver", skip_serializing_if = "Option::is_none")]
    pub transmission_receiver: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureType {
    #[serde(rename = "signatureCategory")]
    pub signature_category: SignatureCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionType {
    #[serde(rename = "CompressionCategory")]
    pub compression_category: CompressionCategoryType,
    #[serde(rename = "CompressionSoftware", skip_serializing_if = "Option::is_none")]
    pub compression_software: Option<String>,
}

