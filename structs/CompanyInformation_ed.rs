#[derive(Debug, Serialize, Deserialize)]
pub struct RepresentativePerson {
    #[serde(flatten)]
    pub representative_person: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractor {
    #[serde(flatten)]
    pub sub_contractor: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registered {
    #[serde(flatten)]
    pub registered: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreements {
    #[serde(flatten)]
    pub collective_agreements: CollectiveAgreementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPersonInFinland {
    #[serde(flatten)]
    pub contact_person_in_finland: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorWrittenAgreement {
    #[serde(flatten)]
    pub sub_contractor_written_agreement: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: CertificationSystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsActive {
    #[serde(flatten)]
    pub is_active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegister {
    #[serde(flatten)]
    pub employer_register: EmployerRegisterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCareInfo {
    #[serde(flatten)]
    pub employee_health_care_info: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxDebt {
    #[serde(flatten)]
    pub tax_debt: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeRegistration {
    #[serde(flatten)]
    pub trade_registration: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreDebtCollectionRegister {
    #[serde(flatten)]
    pub pre_debt_collection_register: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreement {
    #[serde(flatten)]
    pub collective_agreement: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityInsurance {
    #[serde(flatten)]
    pub liability_insurance: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeWrittenAgreement {
    #[serde(flatten)]
    pub employee_written_agreement: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankCode {
    #[serde(flatten)]
    pub bank_code: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePensionCertificate {
    #[serde(flatten)]
    pub employee_pension_certificate: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystems {
    #[serde(flatten)]
    pub quality_systems: QualitySystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeHealthCare {
    #[serde(flatten)]
    pub employee_health_care: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractors {
    #[serde(flatten)]
    pub sub_contractors: SubContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypes {
    #[serde(flatten)]
    pub service_types: ServiceTypesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalAccidentInsurance {
    #[serde(flatten)]
    pub legal_accident_insurance: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyType {
    #[serde(flatten)]
    pub company_type: CompanyTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystem {
    #[serde(flatten)]
    pub quality_system: QualitySystemType,
}

