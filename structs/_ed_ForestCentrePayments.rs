#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidy {
    #[serde(flatten)]
    pub subsidy: SubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidies {
    #[serde(flatten)]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidy {
    #[serde(flatten)]
    pub decided_total_subsidy: FccDecidedTotalSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTexts {
    #[serde(flatten)]
    pub payment_texts: PaymentTextsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentText {
    #[serde(flatten)]
    pub payment_text: CoString5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnit {
    #[serde(flatten)]
    pub decided_amount_unit: FccDecidedAmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePayments {
    #[serde(flatten)]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescription {
    #[serde(flatten)]
    pub cost_type_description: FccCostTypeDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentrePaymentsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmount {
    #[serde(flatten)]
    pub decided_amount: FccDecidedAmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payments {
    #[serde(flatten)]
    pub payments: PaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payees {
    #[serde(flatten)]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseNumber {
    #[serde(flatten)]
    pub case_number: FccFinancingActNumberType,
}

