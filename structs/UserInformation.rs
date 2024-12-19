#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDataType {
    #[serde(rename = "Startdate")]
    pub startdate: BdtDateType,
    #[serde(rename = "Enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<BdtDateType>,
    #[serde(rename = "WorkingContract")]
    pub working_contract: BdtYesNoType,
    #[serde(rename = "Active")]
    pub active: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataType {
    #[serde(rename = "TrainingId")]
    pub training_id: BdtPositiveInteger4digitsType,
    #[serde(rename = "TrainingFreeText", skip_serializing_if = "Option::is_none")]
    pub training_free_text: Option<BdtString50Type>,
    #[serde(rename = "TrainingDate")]
    pub training_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesType {
    #[serde(rename = "UserRole")]
    pub user_role: Vec<BdtUserRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<BdtWorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<Vec<BdtWorkCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<BdtServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformationType {
    #[serde(rename = "UserId")]
    pub user_id: BdtString20Type,
    #[serde(rename = "Contractors")]
    pub contractors: WctContractorsType,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<WctTaxNumberType>,
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Address")]
    pub address: BdtString50Type,
    #[serde(rename = "PostalCode")]
    pub postal_code: BdtString10Type,
    #[serde(rename = "PostOffice")]
    pub post_office: BdtString50Type,
    #[serde(rename = "BirthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<BdtDateType>,
    #[serde(rename = "Telephone")]
    pub telephone: BdtString20Type,
    #[serde(rename = "ICEName", skip_serializing_if = "Option::is_none")]
    pub i_c_e_name: Option<BdtString100Type>,
    #[serde(rename = "ICETelephone", skip_serializing_if = "Option::is_none")]
    pub i_c_e_telephone: Option<BdtString20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<BdtString100Type>,
    #[serde(rename = "UserRoles")]
    pub user_roles: UserRolesType,
    #[serde(rename = "UserName")]
    pub user_name: BdtString100Type,
    #[serde(rename = "AdditionalName", skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<BdtString20Type>,
    #[serde(rename = "Trainings")]
    pub trainings: TrainingsType,
    #[serde(rename = "Employment")]
    pub employment: EmploymentDataType,
    #[serde(rename = "Machines", skip_serializing_if = "Option::is_none")]
    pub machines: Option<MachinesType>,
    #[serde(rename = "Languages")]
    pub languages: LanguagesType,
    #[serde(rename = "NationalityCode")]
    pub nationality_code: BdtString5Type,
    #[serde(rename = "NationalityFreeText", skip_serializing_if = "Option::is_none")]
    pub nationality_free_text: Option<BdtString50Type>,
    #[serde(rename = "WorkCodeGroups", skip_serializing_if = "Option::is_none")]
    pub work_code_groups: Option<WorkCodeGroupsType>,
    #[serde(rename = "Location")]
    pub location: GdtPointGeometryType,
    #[serde(rename = "E101")]
    pub e101: BdtYesNoType,
    #[serde(rename = "A1")]
    pub a1: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingsType {
    #[serde(rename = "Training", skip_serializing_if = "Option::is_none")]
    pub training: Option<Vec<TrainingDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesType {
    #[serde(rename = "Machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<Vec<BdtMachineTypeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesType {
    #[serde(rename = "Language")]
    pub language: Vec<BdtLanguageCode1Type>,
}

