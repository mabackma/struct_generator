#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValue {
    #[serde(flatten)]
    pub total_value: TotalValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentType {
    #[serde(flatten)]
    pub payment_type: PaymentTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Percent {
    #[serde(flatten)]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: DueDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VAT {
    #[serde(flatten)]
    pub vat: VATType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantity {
    #[serde(flatten)]
    pub absolute_quantity: AbsoluteQuantityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTax {
    #[serde(flatten)]
    pub advance_tax: AdvanceTaxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDate {
    #[serde(flatten)]
    pub payment_permission_date: PaymentPermissionDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPayment {
    #[serde(flatten)]
    pub forest_fund_payment: ForestFundPaymentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumber {
    #[serde(flatten)]
    pub sequence_number: SequenceNumberType,
}

