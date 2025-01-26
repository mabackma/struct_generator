use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ICEName {
    #[serde(flatten)]
    pub i_c_e_name: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirthDate {
    #[serde(flatten)]
    pub birth_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Active {
    #[serde(flatten)]
    pub active: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct E101 {
    #[serde(flatten)]
    pub e101: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingId {
    #[serde(flatten)]
    pub training_id: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingContract {
    #[serde(flatten)]
    pub working_contract: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRoles {
    #[serde(flatten)]
    pub user_roles: UserRolesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machines {
    #[serde(flatten)]
    pub machines: MachinesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ICETelephone {
    #[serde(flatten)]
    pub i_c_e_telephone: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserName {
    #[serde(flatten)]
    pub user_name: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Languages {
    #[serde(flatten)]
    pub languages: LanguagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingFreeText {
    #[serde(flatten)]
    pub training_free_text: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalName {
    #[serde(flatten)]
    pub additional_name: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingDate {
    #[serde(flatten)]
    pub training_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Startdate {
    #[serde(flatten)]
    pub startdate: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A1 {
    #[serde(flatten)]
    pub a1: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRole {
    #[serde(flatten)]
    pub user_role: BdtUserRoleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Training {
    #[serde(flatten)]
    pub training: TrainingDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Enddate {
    #[serde(flatten)]
    pub enddate: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NationalityCode {
    #[serde(flatten)]
    pub nationality_code: BdtString5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Employment {
    #[serde(flatten)]
    pub employment: EmploymentDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    #[serde(flatten)]
    pub machine: BdtMachineTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NationalityFreeText {
    #[serde(flatten)]
    pub nationality_free_text: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trainings {
    #[serde(flatten)]
    pub trainings: TrainingsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesType {
    #[serde(rename = "UserRole")]
    pub user_role: Vec<UserRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingsType {
    #[serde(rename = "Training", skip_serializing_if = "Option::is_none")]
    pub training: Option<Vec<TrainingDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformationType {
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "Contractors")]
    pub contractors: ContractorsType,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: String20Type,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address")]
    pub address: String50Type,
    #[serde(rename = "PostalCode")]
    pub postal_code: String10Type,
    #[serde(rename = "PostOffice")]
    pub post_office: String50Type,
    #[serde(rename = "BirthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<DateType>,
    #[serde(rename = "Telephone")]
    pub telephone: String20Type,
    #[serde(rename = "ICEName", skip_serializing_if = "Option::is_none")]
    pub i_c_e_name: Option<String100Type>,
    #[serde(rename = "ICETelephone", skip_serializing_if = "Option::is_none")]
    pub i_c_e_telephone: Option<String20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String100Type>,
    #[serde(rename = "UserRoles")]
    pub user_roles: UserRolesType,
    #[serde(rename = "UserName")]
    pub user_name: String100Type,
    #[serde(rename = "AdditionalName", skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String20Type>,
    #[serde(rename = "Trainings")]
    pub trainings: TrainingsType,
    #[serde(rename = "Employment")]
    pub employment: EmploymentDataType,
    #[serde(rename = "Machines", skip_serializing_if = "Option::is_none")]
    pub machines: Option<MachinesType>,
    #[serde(rename = "Languages")]
    pub languages: LanguagesType,
    #[serde(rename = "NationalityCode")]
    pub nationality_code: String5Type,
    #[serde(rename = "NationalityFreeText", skip_serializing_if = "Option::is_none")]
    pub nationality_free_text: Option<String50Type>,
    #[serde(rename = "WorkCodeGroups", skip_serializing_if = "Option::is_none")]
    pub work_code_groups: Option<WorkCodeGroupsType>,
    #[serde(rename = "Location")]
    pub location: PointGeometryType,
    #[serde(rename = "E101")]
    pub e101: YesNoType,
    #[serde(rename = "A1")]
    pub a1: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesType {
    #[serde(rename = "Language")]
    pub language: Vec<LanguageCode1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<WorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesType {
    #[serde(rename = "Machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<Vec<MachineTypeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<Vec<WorkCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDataType {
    #[serde(rename = "Startdate")]
    pub startdate: DateType,
    #[serde(rename = "Enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<DateType>,
    #[serde(rename = "WorkingContract")]
    pub working_contract: YesNoType,
    #[serde(rename = "Active")]
    pub active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataType {
    #[serde(rename = "TrainingId")]
    pub training_id: PositiveInteger4digitsType,
    #[serde(rename = "TrainingFreeText", skip_serializing_if = "Option::is_none")]
    pub training_free_text: Option<String50Type>,
    #[serde(rename = "TrainingDate")]
    pub training_date: DateType,
}

