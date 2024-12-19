#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreementsType {
    #[serde(rename = "CollectiveAgreement")]
    pub collective_agreement: Vec<BdtString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemsType {
    #[serde(rename = "QualitySystem", skip_serializing_if = "Option::is_none")]
    pub quality_system: Option<Vec<BdtQualitySystemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsType {
    #[serde(rename = "SubContractor", skip_serializing_if = "Option::is_none")]
    pub sub_contractor: Option<Vec<BdtString20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OrganizationName")]
    pub organization_name: BdtString100Type,
    #[serde(rename = "RepresentativePerson")]
    pub representative_person: BdtContactInformationType,
    #[serde(rename = "ContactPersonInFinland")]
    pub contact_person_in_finland: BdtContactInformationType,
    #[serde(rename = "CompanyType")]
    pub company_type: BdtCompanyTypeType,
    #[serde(rename = "QualitySystems", skip_serializing_if = "Option::is_none")]
    pub quality_systems: Option<QualitySystemsType>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
    #[serde(rename = "TradeRegistration", skip_serializing_if = "Option::is_none")]
    pub trade_registration: Option<BdtDateType>,
    #[serde(rename = "TaxDebt")]
    pub tax_debt: BdtDateType,
    #[serde(rename = "EmployeePensionCertificate", skip_serializing_if = "Option::is_none")]
    pub employee_pension_certificate: Option<BdtDateType>,
    #[serde(rename = "EmployeeHealthCare")]
    pub employee_health_care: BdtYesNoType,
    #[serde(rename = "EmployeeHealthCareInfo", skip_serializing_if = "Option::is_none")]
    pub employee_health_care_info: Option<BdtString100Type>,
    #[serde(rename = "CollectiveAgreements")]
    pub collective_agreements: CollectiveAgreementsType,
    #[serde(rename = "ServiceTypes")]
    pub service_types: BdtServiceTypesType,
    #[serde(rename = "LiabilityInsurance", skip_serializing_if = "Option::is_none")]
    pub liability_insurance: Option<BdtDateType>,
    #[serde(rename = "LegalAccidentInsurance", skip_serializing_if = "Option::is_none")]
    pub legal_accident_insurance: Option<BdtDateType>,
    #[serde(rename = "SubContractorWrittenAgreement")]
    pub sub_contractor_written_agreement: BdtYesNoType,
    #[serde(rename = "EmployeeWrittenAgreement")]
    pub employee_written_agreement: BdtYesNoType,
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: BdtYesNoType,
    #[serde(rename = "PreDebtCollectionRegister")]
    pub pre_debt_collection_register: BdtYesNoType,
    #[serde(rename = "EmployerRegister")]
    pub employer_register: EmployerRegisterType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BdtString50Type>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<BdtString20Type>,
    #[serde(rename = "IsActive")]
    pub is_active: BdtYesNoType,
    #[serde(rename = "SubContractors", skip_serializing_if = "Option::is_none")]
    pub sub_contractors: Option<SubContractorsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem", skip_serializing_if = "Option::is_none")]
    pub certification_system: Option<Vec<WtcoCertificationSystemType>>,
    #[serde(flatten)]
    pub base: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegisterType {
    #[serde(rename = "Registered")]
    pub registered: BdtYesNoType,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<BdtDateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtDateType>,
}

