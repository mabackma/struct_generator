use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct FirstName {
    #[serde(flatten)]
    pub first_name: FirstNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonOrganizationName {
    #[serde(flatten)]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryText {
    #[serde(flatten)]
    pub country_text: CountryTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessId {
    #[serde(flatten)]
    pub business_id: JhsYritysTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryCode {
    #[serde(flatten)]
    pub country_code: CoISO3166char2CountryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateText {
    #[serde(flatten)]
    pub state_text: CoString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonId {
    #[serde(flatten)]
    pub person_id: JhsHenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobilePhoneNumber {
    #[serde(flatten)]
    pub mobile_phone_number: MobilePhoneNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateCode {
    #[serde(flatten)]
    pub state_code: StateCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelefaxNumber {
    #[serde(flatten)]
    pub telefax_number: TelefaxNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WholeName {
    #[serde(flatten)]
    pub whole_name: WholeNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastName {
    #[serde(flatten)]
    pub last_name: LastNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstAndLastNameGroup {
    #[serde(rename = "FirstName")]
    pub first_name: FirstName,
    #[serde(rename = "LastName")]
    pub last_name: LastName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@language")]
    pub language: ISO639char2LanguageType,
    #[serde(rename = "IdentifierGroup", skip_serializing_if = "Option::is_none")]
    pub identifier_group: Option<IdentifierGroup>,
    #[serde(rename = "NameAndOrganizationGroup")]
    pub name_and_organization_group: NameAndOrganizationGroup,
    #[serde(rename = "PostAddressGroup", skip_serializing_if = "Option::is_none")]
    pub post_address_group: Option<PostAddressGroup>,
    #[serde(rename = "PhoneAndTelefaxGroup")]
    pub phone_and_telefax_group: PhoneAndTelefaxGroup,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddressType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameBaseType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonNameGroup {
    #[serde(rename = "WholeName")]
    pub whole_name: WholeName,
    #[serde(rename = "PersonOrganizationName", skip_serializing_if = "Option::is_none")]
    pub person_organization_name: Option<PersonOrganizationName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAddressGroup {
    #[serde(rename = "CountryText", skip_serializing_if = "Option::is_none")]
    pub country_text: Option<CountryText>,
    #[serde(rename = "StateCode", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<StateCode>,
    #[serde(rename = "PostalCode")]
    pub postal_code: PostalCode,
    #[serde(rename = "PostOffice")]
    pub post_office: PostOffice,
    #[serde(rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "StateText", skip_serializing_if = "Option::is_none")]
    pub state_text: Option<StateText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCodeType {
    #[serde(flatten)]
    pub base: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: OrganizationName,
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
pub struct PostOfficeType {
    #[serde(flatten)]
    pub base: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberType {
    #[serde(flatten)]
    pub base: JhsPuhelinnumeroTekstiTyyppi,
}

