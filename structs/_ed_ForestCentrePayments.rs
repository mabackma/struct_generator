#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePayments {
    #[serde(flatten)]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payments {
    #[serde(flatten)]
    pub payments: PaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTexts {
    #[serde(flatten)]
    pub payment_texts: PaymentTextsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payees {
    #[serde(flatten)]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentText {
    #[serde(flatten)]
    pub payment_text: String5000Type,
}

