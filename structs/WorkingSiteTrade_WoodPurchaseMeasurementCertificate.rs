#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(flatten)]
    pub base: CoCurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateTypeType {
    #[serde(flatten)]
    pub base: CoMeasurementCertificateTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionNoType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@versionNo")]
    pub version_no: VersionNoType,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
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
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: String,
    #[serde(rename = "PaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub payment_transactions: Option<WtcoPaymentTransactionsType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

