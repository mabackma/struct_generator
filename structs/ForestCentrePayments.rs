use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subsidy {
    #[serde(flatten)]
    pub subsidy: SubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentrePaymentsDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    #[serde(flatten)]
    pub payment: ForestCentrePaymentDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseNumber {
    #[serde(flatten)]
    pub case_number: FccFinancingActNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedTotalSubsidy {
    #[serde(flatten)]
    pub decided_total_subsidy: FccDecidedTotalSubsidyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeDescription {
    #[serde(flatten)]
    pub cost_type_description: FccCostTypeDescriptionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedAmount {
    #[serde(flatten)]
    pub decided_amount: FccDecidedAmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentrePayments {
    #[serde(flatten)]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTexts {
    #[serde(flatten)]
    pub payment_texts: PaymentTextsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payees {
    #[serde(flatten)]
    pub payees: PayeesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecidedAmountUnit {
    #[serde(flatten)]
    pub decided_amount_unit: FccDecidedAmountUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payments {
    #[serde(flatten)]
    pub payments: PaymentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payee {
    #[serde(flatten)]
    pub payee: PayeeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentText {
    #[serde(flatten)]
    pub payment_text: CoString5000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subsidies {
    #[serde(flatten)]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsType {
    #[serde(rename = "Payment")]
    pub payment: Vec<ForestCentrePaymentDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierBaseContactAndEstateInfoType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<PaymentsRealEstatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<PaymentsRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentDetailsType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CaseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<FccFinancingActNumberType>,
    #[serde(rename = "ForestCentreMessageReferenceType", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference_type: Option<FccForestCentreMessageReferenceType>,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference: Option<FccForestCentreMessageReference>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<FccCompletionDeclarationReference>,
    #[serde(rename = "CompletionDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_number: Option<FccCompletionDeclarationNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<FccCustomerReference>,
    #[serde(rename = "ArrivalDate")]
    pub fcc_arrival_date: FccArrivalDate,
    #[serde(rename = "PaymentReference")]
    pub fcc_payment_reference: FccPaymentReference,
    #[serde(rename = "PaymentDate")]
    pub fcc_payment_date: FccPaymentDate,
    #[serde(rename = "BankAccount")]
    pub fcc_bank_account: FccBankAccount,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<CiContactInformationType>,
    #[serde(rename = "ContactPerson")]
    pub fcc_contact_person: FccContactPerson,
    #[serde(rename = "PaymentTexts", skip_serializing_if = "Option::is_none")]
    pub payment_texts: Option<PaymentTextsType>,
    #[serde(rename = "OverallTotalSubsidy")]
    pub fcc_overall_total_subsidy: FccOverallTotalSubsidy,
    #[serde(rename = "SubsidyAppliers")]
    pub subsidy_appliers: SubsidyAppliersType,
}

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
    pub payment_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType2,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "CostType")]
    pub cost_type: FccCostTypeType2,
    #[serde(rename = "CostTypeDescription")]
    pub cost_type_description: FccCostTypeDescriptionType,
    #[serde(rename = "DecidedAmount", skip_serializing_if = "Option::is_none")]
    pub decided_amount: Option<FccDecidedAmountType>,
    #[serde(rename = "DecidedAmountUnit", skip_serializing_if = "Option::is_none")]
    pub decided_amount_unit: Option<FccDecidedAmountUnitType>,
    #[serde(rename = "DecidedTotalSubsidy")]
    pub decided_total_subsidy: FccDecidedTotalSubsidyType,
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
pub struct PayeesType {
    #[serde(rename = "Payee")]
    pub payee: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentrePayments")]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "Payments")]
    pub payments: PaymentsType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<FccDocuments>,
}

