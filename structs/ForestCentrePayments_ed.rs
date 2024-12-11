#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentrePaymentsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payee {
    #[serde(flatten)]
    pub payee: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payees {
    #[serde(flatten)]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidies {
    #[serde(flatten)]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payments {
    #[serde(flatten)]
    pub payments: PaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(flatten)]
    pub payment: ForestCentrePaymentDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescription {
    #[serde(flatten)]
    pub cost_type_description: CostTypeDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePayments {
    #[serde(flatten)]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnit {
    #[serde(flatten)]
    pub decided_amount_unit: DecidedAmountUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentText {
    #[serde(flatten)]
    pub payment_text: String5000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTexts {
    #[serde(flatten)]
    pub payment_texts: PaymentTextsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: CostTypeType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: SubsidyApplierBaseContactAndEstateInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: PaymentsRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: PaymentsRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmount {
    #[serde(flatten)]
    pub decided_amount: DecidedAmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsidy {
    #[serde(flatten)]
    pub subsidy: SubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseNumber {
    #[serde(flatten)]
    pub case_number: FinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidy {
    #[serde(flatten)]
    pub decided_total_subsidy: DecidedTotalSubsidyType,
}

