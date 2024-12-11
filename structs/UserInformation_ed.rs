#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICETelephone {
    #[serde(flatten)]
    pub i_c_e_telephone: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trainings {
    #[serde(flatten)]
    pub trainings: TrainingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub machine: MachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICEName {
    #[serde(flatten)]
    pub i_c_e_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employment {
    #[serde(flatten)]
    pub employment: EmploymentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub user_role: UserRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerContractorId {
    #[serde(flatten)]
    pub owner_contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contractors {
    #[serde(flatten)]
    pub contractors: ContractorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enddate {
    #[serde(flatten)]
    pub enddate: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct E101 {
    #[serde(flatten)]
    pub e101: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingId {
    #[serde(flatten)]
    pub training_id: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Active {
    #[serde(flatten)]
    pub active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Telephone {
    #[serde(flatten)]
    pub telephone: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirthDate {
    #[serde(flatten)]
    pub birth_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalName {
    #[serde(flatten)]
    pub additional_name: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingContract {
    #[serde(flatten)]
    pub working_contract: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDate {
    #[serde(flatten)]
    pub training_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityFreeText {
    #[serde(flatten)]
    pub nationality_free_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machines {
    #[serde(flatten)]
    pub machines: MachinesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingFreeText {
    #[serde(flatten)]
    pub training_free_text: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroups {
    #[serde(flatten)]
    pub work_code_groups: WorkCodeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    #[serde(flatten)]
    pub email: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Startdate {
    #[serde(flatten)]
    pub startdate: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    #[serde(flatten)]
    pub user_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityCode {
    #[serde(flatten)]
    pub nationality_code: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Languages {
    #[serde(flatten)]
    pub languages: LanguagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoles {
    #[serde(flatten)]
    pub user_roles: UserRolesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A1 {
    #[serde(flatten)]
    pub a1: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    #[serde(flatten)]
    pub training: TrainingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(flatten)]
    pub language: LanguageCode1Type,
}

