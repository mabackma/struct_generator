#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax1IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierBaseType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateMmDdYyyyType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStampType {
    #[serde(flatten)]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6TotalDigitsType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer3digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction2Type {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2PositiveType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2Type {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeIntegerType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String500Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String25Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTimeType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4FractionDigitsType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String20Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteStateType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCodeType {
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2Type {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger6digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer7digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
    #[serde(flatten)]
    pub base: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference14Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveIntegerType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5000Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2PositiveType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5TotalDigitsType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1500Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateType {
    #[serde(flatten)]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2Type {
    #[serde(flatten)]
    pub base: f64,
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
pub struct PositiveInteger5digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2PositiveType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String50Type {
    #[serde(flatten)]
    pub base: String,
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
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction1Type {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3FractionDigitsType {
    #[serde(flatten)]
    pub base: f64,
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
    pub file_binary: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4TotalDigitsType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String10Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger1digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3TotalDigitsType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger4digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2PositiveType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2FractionDigitsType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumberType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String3000Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileNameType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumberType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String2000Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode1Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2Type {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String100Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1000Type {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YearType {
    #[serde(flatten)]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger3digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger2digitsType {
    #[serde(flatten)]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal1FractionDigitType {
    #[serde(flatten)]
    pub base: f64,
}

