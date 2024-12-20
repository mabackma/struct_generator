#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeWrittenAgreement {
    #[serde(flatten)]
    pub employee_written_agreement: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegister {
    #[serde(flatten)]
    pub employer_register: EmployerRegisterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsActive {
    #[serde(flatten)]
    pub is_active: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankCode {
    #[serde(flatten)]
    pub bank_code: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystem {
    #[serde(flatten)]
    pub quality_system: BdtQualitySystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreements {
    #[serde(flatten)]
    pub collective_agreements: CollectiveAgreementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyType {
    #[serde(flatten)]
    pub company_type: BdtCompanyTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystems {
    #[serde(flatten)]
    pub quality_systems: QualitySystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPersonInFinland {
    #[serde(flatten)]
    pub contact_person_in_finland: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepresentativePerson {
    #[serde(flatten)]
    pub representative_person: BdtContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePensionCertificate {
    #[serde(flatten)]
    pub employee_pension_certificate: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registered {
    #[serde(flatten)]
    pub registered: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCareInfo {
    #[serde(flatten)]
    pub employee_health_care_info: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreDebtCollectionRegister {
    #[serde(flatten)]
    pub pre_debt_collection_register: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCare {
    #[serde(flatten)]
    pub employee_health_care: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxDebt {
    #[serde(flatten)]
    pub tax_debt: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractors {
    #[serde(flatten)]
    pub sub_contractors: SubContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalAccidentInsurance {
    #[serde(flatten)]
    pub legal_accident_insurance: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityInsurance {
    #[serde(flatten)]
    pub liability_insurance: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorWrittenAgreement {
    #[serde(flatten)]
    pub sub_contractor_written_agreement: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreement {
    #[serde(flatten)]
    pub collective_agreement: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractor {
    #[serde(flatten)]
    pub sub_contractor: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeRegistration {
    #[serde(flatten)]
    pub trade_registration: BdtDateType,
}

