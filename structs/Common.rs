use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeType {
    #[serde(rename = "OriginalFeatureCodeType")]
    pub original_feature_code_type: OriginalFeatureCodeType,
    #[serde(rename = "FeatureCodeExtensionsType")]
    pub feature_code_extensions_type: FeatureCodeExtensionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCodeType {
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationSpecificationType {
    #[serde(rename = "SilvicultureExtraQualifierType")]
    pub silviculture_extra_qualifier_type: SilvicultureExtraQualifierType,
    #[serde(rename = "CommonOperationExtraQualifierType")]
    pub common_operation_extra_qualifier_type: CommonOperationExtraQualifierType,
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
pub struct String500Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger4digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String3000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileNameType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightType {
    #[serde(rename = "HeightType")]
    pub height_type: HeightType,
    #[serde(rename = "EmptyStringType")]
    pub empty_string_type: EmptyStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1500Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger3digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode1Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceTypeType {
    #[serde(rename = "PersonResourceType")]
    pub person_resource_type: PersonResourceType,
    #[serde(rename = "MachineTypeType")]
    pub machine_type_type: MachineTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsPurchaseModeCodeType {
    #[serde(rename = "PurchaseModeType")]
    pub purchase_mode_type: PurchaseModeType,
    #[serde(rename = "StatisticsPurchaseModeType")]
    pub statistics_purchase_mode_type: StatisticsPurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotKnownType {
    #[serde(rename = "YesNoType")]
    pub yes_no_type: YesNoType,
    #[serde(rename = "NotKnownType")]
    pub not_known_type: NotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedHabitatCodeType {
    #[serde(rename = "HabitatCodeType")]
    pub habitat_code_type: HabitatCodeType,
    #[serde(rename = "VirtaHabitatCodeType")]
    pub virta_habitat_code_type: VirtaHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateType {
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateMmDdYyyyType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer3digitsType {
    pub base: i32,
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
pub struct Decimal4FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5000Type {
    pub base: String,
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
pub struct YearType {
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionQuantityUnitType {
    #[serde(rename = "WideUnitType")]
    pub wide_unit_type: WideUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCaseType {
    #[serde(rename = "ControlUseCaseType")]
    pub control_use_case_type: ControlUseCaseType,
    #[serde(rename = "SelfMonitoringUseCaseType")]
    pub self_monitoring_use_case_type: SelfMonitoringUseCaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTypeType {
    #[serde(rename = "SilvicultureTypeType")]
    pub silviculture_type_type: SilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String50Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger2digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String2000Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax4IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference14Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainWorkCodeType {
    #[serde(rename = "WorkCodeType")]
    pub work_code_type: WorkCodeType,
    #[serde(rename = "WorkCodeGroupType")]
    pub work_code_group_type: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal1FractionDigitType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedQuantityUnitType {
    #[serde(rename = "QuantityUnitType")]
    pub quantity_unit_type: QuantityUnitType,
    #[serde(rename = "StatisticsQuantityUnitType")]
    pub statistics_quantity_unit_type: StatisticsQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierBaseType {
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
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3FractionDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax2IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMainGroupType {
    #[serde(rename = "MainGroupType")]
    pub main_group_type: MainGroupType,
    #[serde(rename = "ExtraMainGroupType")]
    pub extra_main_group_type: ExtraMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String10Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteStateType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStampType {
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger1digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideDevelopmentClassType {
    #[serde(rename = "DevelopmentClassType")]
    pub development_class_type: DevelopmentClassType,
    #[serde(rename = "DevelopmentClassExtensionsType")]
    pub development_class_extensions_type: DevelopmentClassExtensionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCodeType {
    #[serde(rename = "CuttingRestrictionType")]
    pub cutting_restriction_type: CuttingRestrictionType,
    #[serde(rename = "SilvicultureRestrictionType")]
    pub silviculture_restriction_type: SilvicultureRestrictionType,
    #[serde(rename = "OtherRestrictionsType")]
    pub other_restrictions_type: OtherRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax1IntegralPartMax2FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer7digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2PositiveType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String20Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String100Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTimeType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystemType {
    #[serde(rename = "CertificationSystemType")]
    pub certification_system_type: CertificationSystemType,
    #[serde(rename = "PEFCCertificationSystemSubTypeType")]
    pub p_e_f_c_certification_system_sub_type_type: PEFCCertificationSystemSubTypeType,
    #[serde(rename = "FSCCertificationSystemSubTypeType")]
    pub f_s_c_certification_system_sub_type_type: FSCCertificationSystemSubTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3TotalDigitsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateYYYY-MMOrYYYY-MM-DDType {
    #[serde(rename = "gMonthDay")]
    pub xsg_month_day: chrono::NaiveDate,
    #[serde(rename = "DateType")]
    pub date_type: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String200Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger5digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger6digitsType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax1FractionalPartType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String25Type {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
    #[serde(flatten)]
    pub base: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    pub base: String,
}

