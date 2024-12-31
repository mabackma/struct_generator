use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(rename = "positive_decimal_max4_integral_part_max2_fractional_part_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileNameType {
    #[serde(rename = "document_file_name_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
    #[serde(rename = "file_binary_restricted_size_type.base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumberType {
    #[serde(rename = "bank_reference_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax1FractionalPartType {
    #[serde(rename = "positive_decimal_max5_integral_part_max1_fractional_part_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCodeType {
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentDate", skip_serializing_if = "Option::is_none")]
    pub document_date: Option<DateType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2Type {
    #[serde(rename = "decimal3_and2_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5TotalDigitsType {
    #[serde(rename = "decimal5_total_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateMmDdYyyyType {
    #[serde(rename = "date_mm_dd_yyyy_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String2000Type {
    #[serde(rename = "string2000_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5000Type {
    #[serde(rename = "string5000_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierBaseType {
    #[serde(rename = "identifier_base_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4TotalDigitsType {
    #[serde(rename = "decimal4_total_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteStateType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    #[serde(rename = "percent_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2PositiveType {
    #[serde(rename = "decimal4_and2_positive_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(rename = "reference_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode1Type {
    #[serde(rename = "language_code1_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer7digitsType {
    #[serde(rename = "integer7digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6TotalDigitsType {
    #[serde(rename = "decimal6_total_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2PositiveType {
    #[serde(rename = "decimal3_and2_positive_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String50Type {
    #[serde(rename = "string50_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String100Type {
    #[serde(rename = "string100_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String200Type {
    #[serde(rename = "string200_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2Type {
    #[serde(rename = "decimal7_and2_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTimeType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(rename = "positive_decimal_max2_integral_part_max1_fractional_part_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String25Type {
    #[serde(rename = "string25_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    #[serde(rename = "document_description_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String20Type {
    #[serde(rename = "string20_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4FractionDigitsType {
    #[serde(rename = "decimal4_fraction_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateType {
    #[serde(rename = "date_type.base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1000Type {
    #[serde(rename = "string1000_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1500Type {
    #[serde(rename = "string1500_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document4MBType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference14Type {
    #[serde(rename = "reference14_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger6digitsType {
    #[serde(rename = "positive_integer6digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal1FractionDigitType {
    #[serde(rename = "decimal1_fraction_digit_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2FractionDigitsType {
    #[serde(rename = "decimal2_fraction_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2PositiveType {
    #[serde(rename = "decimal7_and2_positive_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANType {
    #[serde(rename = "i_b_a_n_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String500Type {
    #[serde(rename = "string500_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumberType {
    #[serde(flatten)]
    pub base: KuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger5digitsType {
    #[serde(rename = "positive_integer5digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumberType {
    #[serde(rename = "stratum_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    #[serde(rename = "file_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3FractionDigitsType {
    #[serde(rename = "decimal3_fraction_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger2digitsType {
    #[serde(rename = "positive_integer2digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String10Type {
    #[serde(rename = "string10_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNameType {
    #[serde(flatten)]
    pub base: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3TotalDigitsType {
    #[serde(rename = "decimal3_total_digits_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String3000Type {
    #[serde(rename = "string3000_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger3digitsType {
    #[serde(rename = "positive_integer3digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(rename = "stem_count_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICType {
    #[serde(rename = "b_i_c_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeIntegerType {
    #[serde(rename = "negative_integer_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax1IntegralPartMax2FractionalPartType {
    #[serde(rename = "positive_decimal_max1_integral_part_max2_fractional_part_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(rename = "age_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2Type {
    #[serde(rename = "decimal2_and2_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceType {
    #[serde(rename = "data_source_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStampType {
    #[serde(rename = "time_stamp_type.base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2Type {
    #[serde(rename = "decimal4_and2_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax2FractionalPartType {
    #[serde(rename = "positive_decimal_max5_integral_part_max2_fractional_part_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2PositiveType {
    #[serde(rename = "decimal2_and2_positive_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer3digitsType {
    #[serde(rename = "integer3digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    #[serde(rename = "identifier_value_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YearType {
    #[serde(rename = "year_type.base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction2Type {
    #[serde(rename = "percent_with_fraction2_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringType {
    #[serde(rename = "id_string_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5Type {
    #[serde(rename = "string5_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger4digitsType {
    #[serde(rename = "positive_integer4digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger1digitsType {
    #[serde(rename = "positive_integer1digits_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveIntegerType {
    #[serde(rename = "positive_integer_type.base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction1Type {
    #[serde(rename = "percent_with_fraction1_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

