#[derive(Debug, Serialize, Deserialize)]
pub struct Employment {
    #[serde(flatten)]
    pub employment: EmploymentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingContract {
    #[serde(flatten)]
    pub working_contract: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trainings {
    #[serde(flatten)]
    pub trainings: TrainingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICEName {
    #[serde(flatten)]
    pub i_c_e_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalName {
    #[serde(flatten)]
    pub additional_name: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoles {
    #[serde(flatten)]
    pub user_roles: UserRolesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Active {
    #[serde(flatten)]
    pub active: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    #[serde(flatten)]
    pub user_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirthDate {
    #[serde(flatten)]
    pub birth_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Startdate {
    #[serde(flatten)]
    pub startdate: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingFreeText {
    #[serde(flatten)]
    pub training_free_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityCode {
    #[serde(flatten)]
    pub nationality_code: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ICETelephone {
    #[serde(flatten)]
    pub i_c_e_telephone: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub user_role: BdtUserRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A1 {
    #[serde(flatten)]
    pub a1: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    #[serde(flatten)]
    pub training: TrainingDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDate {
    #[serde(flatten)]
    pub training_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingId {
    #[serde(flatten)]
    pub training_id: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enddate {
    #[serde(flatten)]
    pub enddate: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machines {
    #[serde(flatten)]
    pub machines: MachinesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Languages {
    #[serde(flatten)]
    pub languages: LanguagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub machine: BdtMachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NationalityFreeText {
    #[serde(flatten)]
    pub nationality_free_text: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct E101 {
    #[serde(flatten)]
    pub e101: BdtYesNoType,
}

