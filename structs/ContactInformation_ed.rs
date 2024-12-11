#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCode {
    #[serde(flatten)]
    pub country_code: ISO3166char2CountryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeName {
    #[serde(flatten)]
    pub whole_name: WholeNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: EmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: AddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationName {
    #[serde(flatten)]
    pub organization_name: OrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: YritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: HenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: PostOfficeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: PostalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: FirstNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: PersonOrganizationNameType,
}

