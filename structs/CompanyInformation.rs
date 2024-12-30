use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepresentativePerson {
    #[serde(flatten)]
    pub representative_person: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorWrittenAgreement {
    #[serde(flatten)]
    pub sub_contractor_written_agreement: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactPersonInFinland {
    #[serde(flatten)]
    pub contact_person_in_finland: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyType {
    #[serde(flatten)]
    pub company_type: CompanyTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiabilityInsurance {
    #[serde(flatten)]
    pub liability_insurance: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployerRegister {
    #[serde(flatten)]
    pub employer_register: EmployerRegisterType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualitySystems {
    #[serde(flatten)]
    pub quality_systems: QualitySystemsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreDebtCollectionRegister {
    #[serde(flatten)]
    pub pre_debt_collection_register: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeHealthCare {
    #[serde(flatten)]
    pub employee_health_care: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeePensionCertificate {
    #[serde(flatten)]
    pub employee_pension_certificate: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeHealthCareInfo {
    #[serde(flatten)]
    pub employee_health_care_info: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractor {
    #[serde(flatten)]
    pub sub_contractor: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeWrittenAgreement {
    #[serde(flatten)]
    pub employee_written_agreement: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectiveAgreements {
    #[serde(flatten)]
    pub collective_agreements: CollectiveAgreementsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsActive {
    #[serde(flatten)]
    pub is_active: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualitySystem {
    #[serde(flatten)]
    pub quality_system: QualitySystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractors {
    #[serde(flatten)]
    pub sub_contractors: SubContractorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegalAccidentInsurance {
    #[serde(flatten)]
    pub legal_accident_insurance: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankCode {
    #[serde(flatten)]
    pub bank_code: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeRegistration {
    #[serde(flatten)]
    pub trade_registration: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registered {
    #[serde(flatten)]
    pub registered: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectiveAgreement {
    #[serde(flatten)]
    pub collective_agreement: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxDebt {
    #[serde(flatten)]
    pub tax_debt: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem", skip_serializing_if = "Option::is_none")]
    pub certification_system: Option<Vec<CertificationSystemType>>,
    #[serde(flatten)]
    pub base: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegisterType {
    #[serde(rename = "Registered")]
    pub registered: YesNoType,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OrganizationName")]
    pub organization_name: String100Type,
    #[serde(rename = "RepresentativePerson")]
    pub representative_person: ContactInformationType,
    #[serde(rename = "ContactPersonInFinland")]
    pub contact_person_in_finland: ContactInformationType,
    #[serde(rename = "CompanyType")]
    pub company_type: CompanyTypeType,
    #[serde(rename = "QualitySystems", skip_serializing_if = "Option::is_none")]
    pub quality_systems: Option<QualitySystemsType>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
    #[serde(rename = "TradeRegistration", skip_serializing_if = "Option::is_none")]
    pub trade_registration: Option<DateType>,
    #[serde(rename = "TaxDebt")]
    pub tax_debt: DateType,
    #[serde(rename = "EmployeePensionCertificate", skip_serializing_if = "Option::is_none")]
    pub employee_pension_certificate: Option<DateType>,
    #[serde(rename = "EmployeeHealthCare")]
    pub employee_health_care: YesNoType,
    #[serde(rename = "EmployeeHealthCareInfo", skip_serializing_if = "Option::is_none")]
    pub employee_health_care_info: Option<String100Type>,
    #[serde(rename = "CollectiveAgreements")]
    pub collective_agreements: CollectiveAgreementsType,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "LiabilityInsurance", skip_serializing_if = "Option::is_none")]
    pub liability_insurance: Option<DateType>,
    #[serde(rename = "LegalAccidentInsurance", skip_serializing_if = "Option::is_none")]
    pub legal_accident_insurance: Option<DateType>,
    #[serde(rename = "SubContractorWrittenAgreement")]
    pub sub_contractor_written_agreement: YesNoType,
    #[serde(rename = "EmployeeWrittenAgreement")]
    pub employee_written_agreement: YesNoType,
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: YesNoType,
    #[serde(rename = "PreDebtCollectionRegister")]
    pub pre_debt_collection_register: YesNoType,
    #[serde(rename = "EmployerRegister")]
    pub employer_register: EmployerRegisterType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String50Type>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String20Type>,
    #[serde(rename = "IsActive")]
    pub is_active: YesNoType,
    #[serde(rename = "SubContractors", skip_serializing_if = "Option::is_none")]
    pub sub_contractors: Option<SubContractorsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsType {
    #[serde(rename = "SubContractor", skip_serializing_if = "Option::is_none")]
    pub sub_contractor: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemsType {
    #[serde(rename = "QualitySystem", skip_serializing_if = "Option::is_none")]
    pub quality_system: Option<Vec<QualitySystemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreementsType {
    #[serde(rename = "CollectiveAgreement")]
    pub collective_agreement: Vec<String100Type>,
}

