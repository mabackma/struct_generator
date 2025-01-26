use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Percent {
    #[serde(flatten)]
    pub percent: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: DueDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestFundPayment {
    #[serde(flatten)]
    pub forest_fund_payment: ForestFundPaymentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentPermissionDate {
    #[serde(flatten)]
    pub payment_permission_date: PaymentPermissionDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbsoluteQuantity {
    #[serde(flatten)]
    pub absolute_quantity: AbsoluteQuantityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvanceTax {
    #[serde(flatten)]
    pub advance_tax: AdvanceTaxType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VAT {
    #[serde(flatten)]
    pub vat: VATType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payee {
    #[serde(flatten)]
    pub payee: PayeeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentType {
    #[serde(flatten)]
    pub payment_type: PaymentTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceNumber {
    #[serde(flatten)]
    pub sequence_number: SequenceNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    #[serde(flatten)]
    pub payment: PaymentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDataType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "PaymentType")]
    pub payment_type: PaymentTypeType,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "AdvanceTax")]
    pub advance_tax: AdvanceTaxType,
    #[serde(rename = "ForestFundPayment")]
    pub forest_fund_payment: ForestFundPaymentType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "DueDate")]
    pub due_date: DueDateType,
    #[serde(rename = "PaymentPermissionDate", skip_serializing_if = "Option::is_none")]
    pub payment_permission_date: Option<PaymentPermissionDateType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<PaymentDateType>,
    #[serde(rename = "Payee")]
    pub payee: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyAndPercentType {
    #[serde(rename = "AbsoluteQuantity")]
    pub absolute_quantity: AbsoluteQuantityType,
    #[serde(rename = "Percent")]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTaxType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTypeType {
    #[serde(flatten)]
    pub base: CoPaymentTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPaymentType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(flatten)]
    pub base: CoCurrencyType,
}

