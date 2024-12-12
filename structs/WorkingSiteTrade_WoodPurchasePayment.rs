#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(flatten)]
    pub base: CurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPaymentType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTypeType {
    #[serde(flatten)]
    pub base: PaymentTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantityType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
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
pub struct PaymentDataType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
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

