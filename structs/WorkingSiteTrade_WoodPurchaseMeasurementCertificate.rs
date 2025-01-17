use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
--pub struct MeasurementCertificate {
    #[serde(flatten)]
    pub measurement_certificate: MeasurementCertificateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct PaidValue {
    #[serde(flatten)]
    pub paid_value: PaidValueType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct InsertDate {
    #[serde(flatten)]
    pub insert_date: InsertDateType,
}

#[derive(Serialize, Deserialize, Debug)]
--pub struct MeasurementCertificateType {
    #[serde(flatten)]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@versionNo")]
    pub version_no: VersionNoType,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: MeasurementDateType,
    #[serde(rename = "InsertDate")]
    pub insert_date: InsertDateType,
    #[serde(rename = "MeasurementCertificateType")]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<TextType>,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "PaidValue", skip_serializing_if = "Option::is_none")]
    pub paid_value: Option<PaidValueType>,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "AssortmentClasses")]
    pub as_assortment_classes: AsAssortmentClasses,
    #[serde(rename = "PaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub payment_transactions: Option<WtcoPaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<WtcoDocuments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionNoType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateTypeType {
    #[serde(flatten)]
    pub base: CoMeasurementCertificateTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(flatten)]
    pub base: CoCurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

