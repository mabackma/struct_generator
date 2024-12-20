#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@language")]
    pub language: CoISO639char2LanguageType,
    #[serde(rename = "IdentifierGroup", skip_serializing_if = "Option::is_none")]
    pub identifier_group: Option<String>,
    #[serde(rename = "NameAndOrganizationGroup")]
    pub name_and_organization_group: String,
    #[serde(rename = "PostAddressGroup", skip_serializing_if = "Option::is_none")]
    pub post_address_group: Option<String>,
    #[serde(rename = "PhoneAndTelefaxGroup")]
    pub phone_and_telefax_group: String,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddressType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonNameGroup {
    #[serde(rename = "WholeName")]
    pub whole_name: WholeName,
    #[serde(rename = "PersonOrganizationName", skip_serializing_if = "Option::is_none")]
    pub person_organization_name: Option<PersonOrganizationName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCodeType {
    #[serde(flatten)]
    pub base: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: OrganizationName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstAndLastNameGroup {
    #[serde(rename = "FirstName")]
    pub first_name: FirstName,
    #[serde(rename = "LastName")]
    pub last_name: LastName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberType {
    #[serde(flatten)]
    pub base: JhsPuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOfficeType {
    #[serde(flatten)]
    pub base: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAddressGroup {
    #[serde(rename = "StateText", skip_serializing_if = "Option::is_none")]
    pub state_text: Option<StateText>,
    #[serde(rename = "CountryText", skip_serializing_if = "Option::is_none")]
    pub country_text: Option<CountryText>,
    #[serde(rename = "PostOffice")]
    pub post_office: PostOffice,
    #[serde(rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "StateCode", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<StateCode>,
    #[serde(rename = "PostalCode")]
    pub postal_code: PostalCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneAndTelefaxGroup {
    #[serde(rename = "MobilePhoneNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<MobilePhoneNumber>,
    #[serde(rename = "TelefaxNumber", skip_serializing_if = "Option::is_none")]
    pub telefax_number: Option<TelefaxNumber>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameBaseType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    #[serde(flatten)]
    pub base: Xsstring,
}

