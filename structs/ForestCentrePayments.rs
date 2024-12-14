#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "Payees")]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTextsType {
    #[serde(rename = "PaymentText")]
    pub payment_text: Vec<String5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsType {
    #[serde(rename = "Payment")]
    pub payment: Vec<ForestCentrePaymentDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType2,
    #[serde(rename = "CostTypeDescription")]
    pub cost_type_description: CostTypeDescriptionType,
    #[serde(rename = "DecidedAmount", skip_serializing_if = "Option::is_none")]
    pub decided_amount: Option<DecidedAmountType>,
    #[serde(rename = "DecidedAmountUnit", skip_serializing_if = "Option::is_none")]
    pub decided_amount_unit: Option<DecidedAmountUnitType>,
    #[serde(rename = "DecidedTotalSubsidy")]
    pub decided_total_subsidy: DecidedTotalSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliersType {
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<SubsidyApplierBaseContactAndEstateInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "Payments")]
    pub payments: PaymentsType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesType {
    #[serde(rename = "Payee")]
    pub payee: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<PaymentsRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ForestCentrePayments")]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentDetailsType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CaseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<FinancingActNumberType>,
    #[serde(rename = "ForestCentreMessageReferenceType", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<ReferenceType>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_reference: Option<ReferenceType>,
    #[serde(rename = "CompletionDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_number: Option<String100Type>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<ReferenceType>,
    #[serde(rename = "ArrivalDate")]
    pub arrival_date: DateMmDdYyyyType,
    #[serde(rename = "PaymentReference")]
    pub payment_reference: PaymentsReferenceType,
    #[serde(rename = "PaymentDate")]
    pub payment_date: DateType,
    #[serde(rename = "BankAccount")]
    pub bank_account: BankAccountType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<ContactInformationType>,
    #[serde(rename = "ContactPerson")]
    pub contact_person: ContactInformationType,
    #[serde(rename = "PaymentTexts", skip_serializing_if = "Option::is_none")]
    pub payment_texts: Option<PaymentTextsType>,
    #[serde(rename = "OverallTotalSubsidy")]
    pub overall_total_subsidy: MoneyType,
    #[serde(rename = "SubsidyAppliers")]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierBaseContactAndEstateInfoType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<PaymentsRealEstatesType>,
}

