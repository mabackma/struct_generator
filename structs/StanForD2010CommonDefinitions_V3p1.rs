use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassType {
    #[serde(rename = "DiameterClassLowerLimit")]
    pub diameter_class_lower_limit: u32,
    #[serde(rename = "DiameterClassName", skip_serializing_if = "Option::is_none")]
    pub diameter_class_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCertificationType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterDefinitionType {
    #[serde(rename = "DiameterClasses")]
    pub diameter_classes: DiameterClassesType,
    #[serde(rename = "DiameterMINTop", skip_serializing_if = "Option::is_none")]
    pub diameter_m_i_n_top: Option<u32>,
    #[serde(rename = "DiameterMAXButt", skip_serializing_if = "Option::is_none")]
    pub diameter_m_a_x_butt: Option<u32>,
    #[serde(rename = "DiameterTopPosition")]
    pub diameter_top_position: u32,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnDataType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "columnID")]
    pub column_i_d: String,
    #[serde(rename = "columnName")]
    pub column_name: String,
    #[serde(rename = "columnOrder")]
    pub column_order: columnOrder,
    #[serde(rename = "cellID")]
    pub cell_i_d: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    #[serde(rename = "Street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Province", skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonDeliveryDefinitionType {
    #[serde(rename = "DeliveryUserID")]
    pub delivery_user_i_d: UserIDType,
    #[serde(rename = "DeliveryName", skip_serializing_if = "Option::is_none")]
    pub delivery_name: Option<String>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: ModificationDateType,
    #[serde(rename = "DeliveryInfo", skip_serializing_if = "Option::is_none")]
    pub delivery_info: Option<InfoType>,
    #[serde(rename = "DeliveryVersion", skip_serializing_if = "Option::is_none")]
    pub delivery_version: Option<VersionType>,
    #[serde(rename = "DeliveryDestination", skip_serializing_if = "Option::is_none")]
    pub delivery_destination: Option<String>,
    #[serde(rename = "Density", skip_serializing_if = "Option::is_none")]
    pub density: Option<Vec<DensityType>>,
    #[serde(rename = "DeliveryPresentationOrder", skip_serializing_if = "Option::is_none")]
    pub delivery_presentation_order: Option<String>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "DeliveryDescription", skip_serializing_if = "Option::is_none")]
    pub delivery_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthClassType {
    #[serde(rename = "LengthClassLowerLimit")]
    pub length_class_lower_limit: u32,
    #[serde(rename = "LengthClassMargin")]
    pub length_class_margin: u32,
    #[serde(rename = "LongTimberDefinition", skip_serializing_if = "Option::is_none")]
    pub long_timber_definition: Option<LongTimberDefinitionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogVolumeType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "logVolumeCategory")]
    pub log_volume_category: LogVolumeCategoryType,
    #[serde(rename = "logMeasurementCategory")]
    pub log_measurement_category: LogMeasurementCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractNumberType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "ContractCategory")]
    pub contract_category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDefinitionType {
    #[serde(flatten)]
    pub base: CommonObjectDefinitionType,
    #[serde(rename = "TextToMachine", skip_serializing_if = "Option::is_none")]
    pub text_to_machine: Option<String>,
    #[serde(rename = "SubObject", skip_serializing_if = "Option::is_none")]
    pub sub_object: Option<Vec<SubObjectDefinitionType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeDiameterAdjustmentType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductConditionType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableGroupType {
    #[serde(rename = "DataTable")]
    pub data_table: Vec<DataTableType>,
    #[serde(rename = "tableGroupID")]
    pub table_group_i_d: String,
    #[serde(rename = "tableGroupName")]
    pub table_group_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwedishZaccoType {
    #[serde(rename = "ConstantA")]
    pub constant_a: NonNegativeDecimal,
    #[serde(rename = "FactorB")]
    pub factor_b: NonNegativeDecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubObjectDefinitionType {
    #[serde(rename = "SubObjectUserID")]
    pub sub_object_user_i_d: UserIDType,
    #[serde(rename = "SubObjectName", skip_serializing_if = "Option::is_none")]
    pub sub_object_name: Option<String>,
    #[serde(rename = "SubObjectArea", skip_serializing_if = "Option::is_none")]
    pub sub_object_area: Option<PositiveDecimal>,
    #[serde(rename = "LoggingForm", skip_serializing_if = "Option::is_none")]
    pub logging_form: Option<LoggingFormType>,
    #[serde(rename = "RealEstateIDSubObject", skip_serializing_if = "Option::is_none")]
    pub real_estate_i_d_sub_object: Option<RealEstateIDObjectType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopRenderingLineType {
    #[serde(rename = "FormatColor")]
    pub format_color: i32,
    #[serde(rename = "FormatLineStyle")]
    pub format_line_style: FormatLineStyleType,
    #[serde(rename = "FormatLineThickness")]
    pub format_line_thickness: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonSpeciesGroupDefinitionType {
    #[serde(rename = "SpeciesGroupModificationDate")]
    pub species_group_modification_date: ModificationDateType,
    #[serde(rename = "SpeciesGroupUserID")]
    pub species_group_user_i_d: UserIDType,
    #[serde(rename = "SpeciesGroupName")]
    pub species_group_name: String,
    #[serde(rename = "SpeciesGroupInfo", skip_serializing_if = "Option::is_none")]
    pub species_group_info: Option<InfoType>,
    #[serde(rename = "SpeciesGroupVersion", skip_serializing_if = "Option::is_none")]
    pub species_group_version: Option<VersionType>,
    #[serde(rename = "StemTypeDefinition", skip_serializing_if = "Option::is_none")]
    pub stem_type_definition: Option<Vec<StemTypeDefinitionType>>,
    #[serde(rename = "LoggingOrganisation", skip_serializing_if = "Option::is_none")]
    pub logging_organisation: Option<LoggingOrganisationType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "SpeciesGroupPresentationOrder", skip_serializing_if = "Option::is_none")]
    pub species_group_presentation_order: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstonianVolumeParametersType {
    #[serde(rename = "ParameterA1")]
    pub parameter_a1: f64,
    #[serde(rename = "ParameterA2")]
    pub parameter_a2: f64,
    #[serde(rename = "ParameterA3")]
    pub parameter_a3: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradesType {
    #[serde(rename = "StartGrade")]
    pub start_grade: u32,
    #[serde(rename = "MTHStartGrade", skip_serializing_if = "Option::is_none")]
    pub m_t_h_start_grade: Option<u32>,
    #[serde(rename = "Grade")]
    pub grade: Vec<GradeType>,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LongTimberDefinitionType {
    #[serde(rename = "LongLogButtMIN")]
    pub long_log_butt_m_i_n: u32,
    #[serde(rename = "LongLogButtMAX", skip_serializing_if = "Option::is_none")]
    pub long_log_butt_m_a_x: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDefinitionType {
    #[serde(rename = "ObjectUserID")]
    pub object_user_i_d: UserIDType,
    #[serde(rename = "ObjectName", skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[serde(rename = "ObjectModificationDate")]
    pub object_modification_date: ModificationDateType,
    #[serde(rename = "ForestCertification")]
    pub forest_certification: Vec<ForestCertificationType>,
    #[serde(rename = "LoggingForm", skip_serializing_if = "Option::is_none")]
    pub logging_form: Option<LoggingFormType>,
    #[serde(rename = "ObjectArea", skip_serializing_if = "Option::is_none")]
    pub object_area: Option<PositiveDecimal>,
    #[serde(rename = "LoggingOrganisation", skip_serializing_if = "Option::is_none")]
    pub logging_organisation: Option<LoggingOrganisationType>,
    #[serde(rename = "ForestOwner", skip_serializing_if = "Option::is_none")]
    pub forest_owner: Option<ContactInformationType>,
    #[serde(rename = "ContractNumber", skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<ContractNumberType>,
    #[serde(rename = "RealEstateIDObject", skip_serializing_if = "Option::is_none")]
    pub real_estate_i_d_object: Option<RealEstateIDObjectType>,
    #[serde(rename = "AuditingOrganisation", skip_serializing_if = "Option::is_none")]
    pub auditing_organisation: Option<ContactInformationType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHeaderType {
    #[serde(rename = "SenderApplication", skip_serializing_if = "Option::is_none")]
    pub sender_application: Option<String>,
    #[serde(rename = "CreationDate")]
    pub creation_date: StanForD2010DateTimeType,
    #[serde(rename = "ModificationDate")]
    pub modification_date: ModificationDateType,
    #[serde(rename = "ApplicationVersionCreated")]
    pub application_version_created: String,
    #[serde(rename = "ApplicationVersionModified")]
    pub application_version_modified: String,
    #[serde(rename = "CountryCode")]
    pub country_code: String,
    #[serde(rename = "CreationCoordinates", skip_serializing_if = "Option::is_none")]
    pub creation_coordinates: Option<CoordinatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatitudeLongitudeValueType {
    #[serde(flatten)]
    pub base: Xsddecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMeasurementCategory {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDiametersType {
    #[serde(rename = "DiameterMeasuredStartHeight")]
    pub diameter_measured_start_height: u32,
    #[serde(rename = "DiameterMeasuredEndHeight")]
    pub diameter_measured_end_height: u32,
    #[serde(rename = "DiameterValue")]
    pub diameter_value: Vec<DiameterValueType>,
    #[serde(rename = "diameterCategory")]
    pub diameter_category: DiameterCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionProductionType {
    #[serde(rename = "DiameterSectionKey")]
    pub diameter_section_key: u32,
    #[serde(rename = "DiameterSectionVolume")]
    pub diameter_section_volume: NonNegativeDecimal,
    #[serde(rename = "DiameterSectionLimitPosition")]
    pub diameter_section_limit_position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationDateType {
    #[serde(flatten)]
    pub base: StanForD2010DateTimeType,
    #[serde(rename = "modificationAuthor")]
    pub modification_author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDefinitionMachineType {
    #[serde(flatten)]
    pub base: CommonObjectDefinitionType,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
    #[serde(rename = "TextFromMachine", skip_serializing_if = "Option::is_none")]
    pub text_from_machine: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: StanForD2010DateTimeType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<StanForD2010DateTimeType>,
    #[serde(rename = "SubObject", skip_serializing_if = "Option::is_none")]
    pub sub_object: Option<Vec<SubObjectDefinitionMachineType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemGradeType {
    #[serde(rename = "GradeValue")]
    pub grade_value: GradeValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(rename = "StemKey")]
    pub stem_key: u32,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "HarvestDate", skip_serializing_if = "Option::is_none")]
    pub harvest_date: Option<StanForD2010DateTimeType>,
    #[serde(rename = "BioEnergyAdaption", skip_serializing_if = "Option::is_none")]
    pub bio_energy_adaption: Option<BioEnergyAdaptionType>,
    #[serde(rename = "StemNumber")]
    pub stem_number: u32,
    #[serde(rename = "ProcessingCategory")]
    pub processing_category: ProcessingCategoryType,
    #[serde(rename = "StemCoordinates", skip_serializing_if = "Option::is_none")]
    pub stem_coordinates: Option<Vec<CoordinatesType>>,
    #[serde(rename = "StemCode", skip_serializing_if = "Option::is_none")]
    pub stem_code: Option<String>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "StumpTreatment", skip_serializing_if = "Option::is_none")]
    pub stump_treatment: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISCoordinateReferenceSystemType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterValueType {
    #[serde(flatten)]
    pub base: XsdnonNegativeInteger,
    #[serde(rename = "diameterPosition")]
    pub diameter_position: u32,
    #[serde(rename = "diameterMeasurementCategory")]
    pub diameter_measurement_category: DiameterMeasurementCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductMatrixType {
    #[serde(rename = "ProductMatrixItem")]
    pub product_matrix_item: Vec<ProductMatrixItemType>,
    #[serde(rename = "modificationRestrictedPrice")]
    pub modification_restricted_price: bool,
    #[serde(rename = "modificationRestrictedDistribution")]
    pub modification_restricted_distribution: bool,
    #[serde(rename = "modificationRestrictedLimitation")]
    pub modification_restricted_limitation: bool,
    #[serde(rename = "modificationRestrictedBuckingCriteria")]
    pub modification_restricted_bucking_criteria: bool,
    #[serde(rename = "modificationRestrictedLogColorMarking")]
    pub modification_restricted_log_color_marking: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionDefinitionMachineType {
    #[serde(flatten)]
    pub base: DiameterSectionDefinitionType,
    #[serde(rename = "DiameterSectionKey")]
    pub diameter_section_key: u32,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatFontType {
    #[serde(rename = "FormatFontFileName")]
    pub format_font_file_name: String,
    #[serde(rename = "FormatFontName")]
    pub format_font_name: String,
    #[serde(rename = "FormatSymbolNumber")]
    pub format_symbol_number: i32,
    #[serde(rename = "FormatSymbolSize")]
    pub format_symbol_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesDefinitionType {
    #[serde(rename = "SpeciesName")]
    pub species_name: String,
    #[serde(rename = "SpeciesUserID", skip_serializing_if = "Option::is_none")]
    pub species_user_i_d: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemVolumeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDefinitionType {
    #[serde(rename = "VolumeDiameterAdjustment")]
    pub volume_diameter_adjustment: VolumeDiameterAdjustmentType,
    #[serde(rename = "VolumeDiameterCategory")]
    pub volume_diameter_category: VolumeDiameterCategoryBaseType,
    #[serde(rename = "VolumeLengthCategory")]
    pub volume_length_category: VolumeLengthCategoryType,
    #[serde(rename = "VolumeUnderBark")]
    pub volume_under_bark: bool,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarkFunctionCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISFileFormatLineType {
    #[serde(flatten)]
    pub base: GISFileFormatType,
    #[serde(rename = "FormatLineStyle")]
    pub format_line_style: FormatLineStyleType,
    #[serde(rename = "FormatLineThickness")]
    pub format_line_thickness: u32,
    #[serde(rename = "TopRendering", skip_serializing_if = "Option::is_none")]
    pub top_rendering: Option<TopRenderingLineType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassAdjustmentType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionDefinitionType {
    #[serde(rename = "DiameterSectionUserID")]
    pub diameter_section_user_i_d: UserIDType,
    #[serde(rename = "DiameterSectionName")]
    pub diameter_section_name: String,
    #[serde(rename = "MinimumDiameter")]
    pub minimum_diameter: u32,
    #[serde(rename = "MinimumLength", skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<u32>,
    #[serde(rename = "DiameterSectionUsage", skip_serializing_if = "Option::is_none")]
    pub diameter_section_usage: Option<DiameterSectionUsageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanForD2010DateTimeType {
    #[serde(flatten)]
    pub base: XsddateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateIDObjectType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "agency")]
    pub agency: AgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skogforsk2004ScotsPineType {
    #[serde(rename = "Latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<PositiveDecimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplementaryGISFilesType {
    #[serde(rename = "ComplementaryGISFilename")]
    pub complementary_g_i_s_filename: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatitudeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GermanType {
    #[serde(rename = "BarkDeduction")]
    pub bark_deduction: Vec<BarkDeductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBHHeightType {
    #[serde(flatten)]
    pub base: XsdpositiveInteger,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterUnitType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopRenderingPointType {
    #[serde(rename = "FormatColor")]
    pub format_color: i32,
    #[serde(rename = "FormatFont")]
    pub format_font: FormatFontType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemVolumeType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "stemVolumeCategory")]
    pub stem_volume_category: StemVolumeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtEndProfileExtrapolationFunctionType {
    #[serde(rename = "Parameter")]
    pub parameter: Vec<ButtEndProfileExtrapolationFunctionParameterType>,
    #[serde(rename = "ButtEndProfileExtrapolationFunctionCategory")]
    pub butt_end_profile_extrapolation_function_category: ButtEndProfileExtrapolationFunctionCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineWithHeadType {
    #[serde(flatten)]
    pub base: MachineType,
    #[serde(rename = "MachineHeadManufacturer", skip_serializing_if = "Option::is_none")]
    pub machine_head_manufacturer: Option<String>,
    #[serde(rename = "MachineHeadModel", skip_serializing_if = "Option::is_none")]
    pub machine_head_model: Option<MachineHeadModelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorDefinitionType {
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "OperatorUserID")]
    pub operator_user_i_d: UserIDType,
    #[serde(rename = "ContactInformation", skip_serializing_if = "Option::is_none")]
    pub contact_information: Option<ContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatitudeType {
    #[serde(flatten)]
    pub base: LatitudeLongitudeValueType,
    #[serde(rename = "latitudeCategory")]
    pub latitude_category: LatitudeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductMatrixItemType {
    #[serde(rename = "Price")]
    pub price: u32,
    #[serde(rename = "Distribution")]
    pub distribution: u32,
    #[serde(rename = "Limitation")]
    pub limitation: u32,
    #[serde(rename = "BuckingCriteria", skip_serializing_if = "Option::is_none")]
    pub bucking_criteria: Option<BuckingCriteriaType>,
    #[serde(rename = "LogColorMarking", skip_serializing_if = "Option::is_none")]
    pub log_color_marking: Option<LogColorMarkingType>,
    #[serde(rename = "diameterClassLowerLimit")]
    pub diameter_class_lower_limit: u32,
    #[serde(rename = "lengthClassLowerLimit")]
    pub length_class_lower_limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingOrganisationType {
    #[serde(rename = "ContactInformation")]
    pub contact_information: ContactInformationType,
    #[serde(rename = "District", skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(rename = "Team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISFileFormatPolygonType {
    #[serde(flatten)]
    pub base: GISFileFormatType,
    #[serde(rename = "FormatLineStyle")]
    pub format_line_style: FormatLineStyleType,
    #[serde(rename = "FormatLineThickness")]
    pub format_line_thickness: u32,
    #[serde(rename = "FormatFillStyle")]
    pub format_fill_style: FormatFillStyleType,
    #[serde(rename = "TopRendering", skip_serializing_if = "Option::is_none")]
    pub top_rendering: Option<TopRenderingPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarkDeductionType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "lowerDiameterLimit")]
    pub lower_diameter_limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceDiameterType {
    #[serde(flatten)]
    pub base: XsdpositiveInteger,
    #[serde(rename = "referenceDiameterHeight")]
    pub reference_diameter_height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatLineStyleType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowType {
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "ModificationDate")]
    pub modification_date: ModificationDateType,
    #[serde(rename = "Coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<CoordinatesType>,
    #[serde(rename = "ColumnData")]
    pub column_data: Vec<ColumnDataType>,
    #[serde(rename = "rowID")]
    pub row_i_d: String,
    #[serde(rename = "rowOrder")]
    pub row_order: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineHeadModelType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "headModelYear")]
    pub head_model_year: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeValueType {
    #[serde(flatten)]
    pub base: XsdnonNegativeInteger,
    #[serde(rename = "gradeStartPosition")]
    pub grade_start_position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarkDeductionDistanceType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "distanceClassLowerLimit")]
    pub distance_class_lower_limit: u32,
    #[serde(rename = "dBHClassLowerLimit")]
    pub d_b_h_class_lower_limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarkFunctionType {
    #[serde(rename = "SwedishZacco")]
    pub swedish_zacco: SwedishZaccoType,
    #[serde(rename = "German")]
    pub german: GermanType,
    #[serde(rename = "GermanDistanceBased")]
    pub german_distance_based: GermanDistanceType,
    #[serde(rename = "Skogforsk2004ScotsPine")]
    pub skogforsk2004_scots_pine: Skogforsk2004ScotsPineType,
    #[serde(rename = "barkFunctionCategory")]
    pub bark_function_category: BarkFunctionCategoryType,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtEndProfileExtrapolationFunctionParameterType {
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogType {
    #[serde(rename = "LogKey")]
    pub log_key: u32,
    #[serde(rename = "ProductKey")]
    pub product_key: u32,
    #[serde(rename = "LogVolume")]
    pub log_volume: Vec<LogVolumeType>,
    #[serde(rename = "CuttingCategory")]
    pub cutting_category: CuttingCategoryType,
    #[serde(rename = "DiameterSectionProduction", skip_serializing_if = "Option::is_none")]
    pub diameter_section_production: Option<Vec<DiameterSectionProductionType>>,
    #[serde(rename = "TopSawing", skip_serializing_if = "Option::is_none")]
    pub top_sawing: Option<bool>,
    #[serde(rename = "FindButtEndFunction", skip_serializing_if = "Option::is_none")]
    pub find_butt_end_function: Option<bool>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionUsageType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionActionType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimal {
    #[serde(flatten)]
    pub base: Xsddecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeLengthCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinateReferenceSystemType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressType>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "BusinessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(rename = "BusinessID", skip_serializing_if = "Option::is_none")]
    pub business_i_d: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonNegativeDecimal {
    #[serde(flatten)]
    pub base: Xsddecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonLocationDefinitionType {
    #[serde(rename = "LocationUserID")]
    pub location_user_i_d: UserIDType,
    #[serde(rename = "LocationName", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(rename = "LocationInfo", skip_serializing_if = "Option::is_none")]
    pub location_info: Option<InfoType>,
    #[serde(rename = "LocationVersion", skip_serializing_if = "Option::is_none")]
    pub location_version: Option<VersionType>,
    #[serde(rename = "HarvestDate", skip_serializing_if = "Option::is_none")]
    pub harvest_date: Option<StanForD2010DateTimeType>,
    #[serde(rename = "LocationCoordinates", skip_serializing_if = "Option::is_none")]
    pub location_coordinates: Option<CoordinatesType>,
    #[serde(rename = "RoadCondition", skip_serializing_if = "Option::is_none")]
    pub road_condition: Option<String>,
    #[serde(rename = "LocationDescription", skip_serializing_if = "Option::is_none")]
    pub location_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtEndProfileExtrapolationTableType {
    #[serde(rename = "ExtrapolationCoefficient")]
    pub extrapolation_coefficient: Vec<ExtrapolationCoefficientType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISLayerType {
    #[serde(rename = "GISFileName")]
    pub g_i_s_file_name: String,
    #[serde(rename = "GISFileOrder")]
    pub g_i_s_file_order: u32,
    #[serde(rename = "ComplementaryGISFiles", skip_serializing_if = "Option::is_none")]
    pub complementary_g_i_s_files: Option<ComplementaryGISFilesType>,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineType {
    #[serde(rename = "MachineKey")]
    pub machine_key: String,
    #[serde(rename = "MachineUserID")]
    pub machine_user_i_d: UserIDType,
    #[serde(rename = "MachineOwnerID")]
    pub machine_owner_i_d: String,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String,
    #[serde(rename = "MachineBaseManufacturer")]
    pub machine_base_manufacturer: String,
    #[serde(rename = "MachineBaseModel")]
    pub machine_base_model: MachineBaseModelType,
    #[serde(rename = "BaseMachineManufacturerID", skip_serializing_if = "Option::is_none")]
    pub base_machine_manufacturer_i_d: Option<String>,
    #[serde(rename = "RegistrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(rename = "MachineOwner")]
    pub machine_owner: ContactInformationType,
    #[serde(rename = "LoggingContractor")]
    pub logging_contractor: ContactInformationType,
    #[serde(rename = "OperatorDefinition")]
    pub operator_definition: Vec<OperatorDefinitionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "machineCategory")]
    pub machine_category: MachineCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingReasonType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDefinitionWithDetailsType {
    #[serde(flatten)]
    pub base: CommonDeliveryDefinitionType,
    #[serde(rename = "DeliveryKey")]
    pub delivery_key: u32,
    #[serde(rename = "DeliveryEstimation")]
    pub delivery_estimation: DeliveryEstimationType,
    #[serde(rename = "ProductKey", skip_serializing_if = "Option::is_none")]
    pub product_key: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundKnotFunctionType {
    #[serde(rename = "ConstantA")]
    pub constant_a: f64,
    #[serde(rename = "FactorB")]
    pub factor_b: f64,
    #[serde(rename = "FactorC")]
    pub factor_c: f64,
    #[serde(rename = "ToleranceD")]
    pub tolerance_d: f64,
    #[serde(rename = "SoundKnotFunctionGrade")]
    pub sound_knot_function_grade: u32,
    #[serde(rename = "GradeIncluded", skip_serializing_if = "Option::is_none")]
    pub grade_included: Option<Vec<u32>>,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameterCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrapolationCoefficientType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "diameterClass")]
    pub diameter_class: u32,
    #[serde(rename = "distanceClass")]
    pub distance_class: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthClassAdjustmentType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingFormType {
    #[serde(rename = "LoggingFormCode", skip_serializing_if = "Option::is_none")]
    pub logging_form_code: Option<String>,
    #[serde(rename = "LoggingFormDescription", skip_serializing_if = "Option::is_none")]
    pub logging_form_description: Option<String>,
    #[serde(rename = "agency")]
    pub agency: AgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryEstimationType {
    #[serde(rename = "DeliveryEstimationMethod")]
    pub delivery_estimation_method: DeliveryEstimationMethodType,
    #[serde(rename = "DeliveryEstimationUnit")]
    pub delivery_estimation_unit: DeliveryEstimationUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIDType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "agency")]
    pub agency: AgencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeDiameterCategoryBaseType {
    #[serde(flatten)]
    pub base: VolumeDiameterCategoryType,
    #[serde(rename = "volumeDiameterTopPosition")]
    pub volume_diameter_top_position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaUnitType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiverPositionType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioEnergyAdaptionType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DensityType {
    #[serde(flatten)]
    pub base: Xsddecimal,
    #[serde(rename = "densityCategory")]
    pub density_category: DensityCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeDefinitionType {
    #[serde(rename = "StemTypeCode")]
    pub stem_type_code: String,
    #[serde(rename = "StemTypeName")]
    pub stem_type_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryEstimationMethodType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionWithDetailsType {
    #[serde(flatten)]
    pub base: ProductDefinitionWithCommonDetailsType,
    #[serde(rename = "ProductCreationDate", skip_serializing_if = "Option::is_none")]
    pub product_creation_date: Option<StanForD2010DateTimeType>,
    #[serde(rename = "ManualFreeBuck")]
    pub manual_free_buck: bool,
    #[serde(rename = "TopLogFreeBuck")]
    pub top_log_free_buck: bool,
    #[serde(rename = "ProductCondition")]
    pub product_condition: ProductConditionType,
    #[serde(rename = "CuttingWindowDefinition", skip_serializing_if = "Option::is_none")]
    pub cutting_window_definition: Option<CuttingWindowDefinitionType>,
    #[serde(rename = "PermittedGradesDefinition")]
    pub permitted_grades_definition: PermittedGradesDefinitionType,
    #[serde(rename = "LengthDistributionDefinition")]
    pub length_distribution_definition: LengthDistributionDefinitionType,
    #[serde(rename = "LimitationDefinition")]
    pub limitation_definition: LimitationDefinitionType,
    #[serde(rename = "ProductMatrixes")]
    pub product_matrixes: ProductMatrixType,
    #[serde(rename = "LongLogButtHeight", skip_serializing_if = "Option::is_none")]
    pub long_log_butt_height: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCodeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDiameterType {
    #[serde(flatten)]
    pub base: XsdpositiveInteger,
    #[serde(rename = "logDiameterCategory")]
    pub log_diameter_category: LogDiameterCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DensityCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumberCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanForD2010VersionType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryEstimationUnitType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermittedGradesDefinitionType {
    #[serde(rename = "PermittedGradeNumber", skip_serializing_if = "Option::is_none")]
    pub permitted_grade_number: Option<Vec<u32>>,
    #[serde(rename = "GradeToleranceTop", skip_serializing_if = "Option::is_none")]
    pub grade_tolerance_top: Option<u32>,
    #[serde(rename = "GradeToleranceButt", skip_serializing_if = "Option::is_none")]
    pub grade_tolerance_butt: Option<u32>,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopRenderingPolygonType {
    #[serde(rename = "FormatColor")]
    pub format_color: i32,
    #[serde(rename = "FormatLineStyle")]
    pub format_line_style: FormatLineStyleType,
    #[serde(rename = "FormatLineThickness")]
    pub format_line_thickness: u32,
    #[serde(rename = "FormatFillStyle")]
    pub format_fill_style: FormatFillStyleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtEndProfileExtrapolationMethodType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogColorMarkingType {
    #[serde(rename = "Color1")]
    pub color1: bool,
    #[serde(rename = "Color2")]
    pub color2: bool,
    #[serde(rename = "Color3")]
    pub color3: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GermanDistanceType {
    #[serde(rename = "BarkDeduction")]
    pub bark_deduction: Vec<BarkDeductionDistanceType>,
    #[serde(rename = "DistanceClassLowerLimit")]
    pub distance_class_lower_limit: Vec<u32>,
    #[serde(rename = "DBHClassLowerLimit")]
    pub d_b_h_class_lower_limit: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISDatabaseFileSpecificationType {
    #[serde(rename = "ColumnNameObject")]
    pub column_name_object: String,
    #[serde(rename = "ColumnNameSubObject", skip_serializing_if = "Option::is_none")]
    pub column_name_sub_object: Option<String>,
    #[serde(rename = "ColumnNameFeature")]
    pub column_name_feature: String,
    #[serde(rename = "ColumnNameID", skip_serializing_if = "Option::is_none")]
    pub column_name_i_d: Option<String>,
    #[serde(rename = "ColumnNameDescription", skip_serializing_if = "Option::is_none")]
    pub column_name_description: Option<String>,
    #[serde(rename = "ColumnNameTreatment", skip_serializing_if = "Option::is_none")]
    pub column_name_treatment: Option<String>,
    #[serde(rename = "ColumnNameMarked", skip_serializing_if = "Option::is_none")]
    pub column_name_marked: Option<String>,
    #[serde(rename = "ColumnNameWarning", skip_serializing_if = "Option::is_none")]
    pub column_name_warning: Option<String>,
    #[serde(rename = "ColumnNameWarningDistance", skip_serializing_if = "Option::is_none")]
    pub column_name_warning_distance: Option<String>,
    #[serde(rename = "ColumnNameFormatID")]
    pub column_name_format_i_d: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingReasonSpecificationType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeUnitType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthUnitType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionType {
    #[serde(rename = "extensionAction")]
    pub extension_action: ExtensionActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LongitudeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassesType {
    #[serde(rename = "DiameterClass")]
    pub diameter_class: Vec<DiameterClassType>,
    #[serde(rename = "DiameterClassMAX")]
    pub diameter_class_m_a_x: u32,
    #[serde(rename = "DiameterClassAdjustment")]
    pub diameter_class_adjustment: DiameterClassAdjustmentType,
    #[serde(rename = "DiameterUnderBark")]
    pub diameter_under_bark: bool,
    #[serde(rename = "diameterClassCategory")]
    pub diameter_class_category: DiameterClassCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonProductDefinitionType {
    #[serde(rename = "ProductName", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: ModificationDateType,
    #[serde(rename = "ProductInfo", skip_serializing_if = "Option::is_none")]
    pub product_info: Option<InfoType>,
    #[serde(rename = "ProductVersion", skip_serializing_if = "Option::is_none")]
    pub product_version: Option<VersionType>,
    #[serde(rename = "ProductBuyer", skip_serializing_if = "Option::is_none")]
    pub product_buyer: Option<ContactInformationType>,
    #[serde(rename = "ProductClass", skip_serializing_if = "Option::is_none")]
    pub product_class: Option<String>,
    #[serde(rename = "LoggingOrganisation", skip_serializing_if = "Option::is_none")]
    pub logging_organisation: Option<LoggingOrganisationType>,
    #[serde(rename = "ProductDestination", skip_serializing_if = "Option::is_none")]
    pub product_destination: Option<Vec<ContactInformationType>>,
    #[serde(rename = "ProductGroupName", skip_serializing_if = "Option::is_none")]
    pub product_group_name: Option<String>,
    #[serde(rename = "StemTypeCode", skip_serializing_if = "Option::is_none")]
    pub stem_type_code: Option<String>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "ProductPresentationOrder", skip_serializing_if = "Option::is_none")]
    pub product_presentation_order: Option<u32>,
    #[serde(rename = "ProductDescription", skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinatesType {
    #[serde(rename = "Latitude")]
    pub latitude: LatitudeType,
    #[serde(rename = "Longitude")]
    pub longitude: LongitudeType,
    #[serde(rename = "Altitude")]
    pub altitude: f64,
    #[serde(rename = "CoordinateDate", skip_serializing_if = "Option::is_none")]
    pub coordinate_date: Option<StanForD2010DateTimeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingCategoryType {
    #[serde(rename = "CuttingReason")]
    pub cutting_reason: CuttingReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineBaseModelType {
    #[serde(flatten)]
    pub base: Xsdstring,
    #[serde(rename = "baseModelYear")]
    pub base_model_year: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementType {
    #[serde(rename = "LogDiameter")]
    pub log_diameter: Vec<LogDiameterType>,
    #[serde(rename = "LogLength")]
    pub log_length: u32,
    #[serde(rename = "logMeasurementCategory")]
    pub log_measurement_category: LogMeasurementCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubObjectDefinitionMachineType {
    #[serde(flatten)]
    pub base: SubObjectDefinitionType,
    #[serde(rename = "SubObjectKey")]
    pub sub_object_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBunchVolumeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitationDefinitionType {
    #[serde(rename = "LimitationCategory")]
    pub limitation_category: LimitationCategoryType,
    #[serde(rename = "LimitationResult")]
    pub limitation_result: LimitationResultType,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthDefinitionType {
    #[serde(rename = "LengthClassAdjustment")]
    pub length_class_adjustment: LengthClassAdjustmentType,
    #[serde(rename = "LengthClass")]
    pub length_class: Vec<LengthClassType>,
    #[serde(rename = "LengthClassMAX")]
    pub length_class_m_a_x: u32,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthDistributionDefinitionType {
    #[serde(rename = "DistributionAllowed")]
    pub distribution_allowed: bool,
    #[serde(rename = "DistributionCategory")]
    pub distribution_category: DistributionCategoryType,
    #[serde(rename = "MAXDeviation")]
    pub m_a_x_deviation: NonNegativeDecimal,
    #[serde(rename = "DistributionWithinProduct")]
    pub distribution_within_product: bool,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISFileFormatType {
    #[serde(rename = "FormatID")]
    pub format_i_d: String,
    #[serde(rename = "FormatName")]
    pub format_name: String,
    #[serde(rename = "FormatDefaultClass")]
    pub format_default_class: bool,
    #[serde(rename = "FormatColor")]
    pub format_color: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatFillStyleType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumberOfItemsType {
    #[serde(flatten)]
    pub base: XsdpositiveInteger,
    #[serde(rename = "loadNumberOfCategory")]
    pub load_number_of_category: LoadNumberCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingWindowDefinitionType {
    #[serde(rename = "LowerLengthLimit")]
    pub lower_length_limit: i32,
    #[serde(rename = "UpperLengthLimit")]
    pub upper_length_limit: i32,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesGroupDefinitionWithDetailsType {
    #[serde(flatten)]
    pub base: CommonSpeciesGroupDefinitionType,
    #[serde(rename = "DBHHeight")]
    pub d_b_h_height: DBHHeightType,
    #[serde(rename = "Grades")]
    pub grades: GradesType,
    #[serde(rename = "BarkFunction")]
    pub bark_function: BarkFunctionType,
    #[serde(rename = "SoundKnotFunction", skip_serializing_if = "Option::is_none")]
    pub sound_knot_function: Option<SoundKnotFunctionType>,
    #[serde(rename = "ButtEndProfileExtrapolation", skip_serializing_if = "Option::is_none")]
    pub butt_end_profile_extrapolation: Option<ButtEndProfileExtrapolationType>,
    #[serde(rename = "EstonianVolumeParameters", skip_serializing_if = "Option::is_none")]
    pub estonian_volume_parameters: Option<EstonianVolumeParametersType>,
    #[serde(rename = "StemCode", skip_serializing_if = "Option::is_none")]
    pub stem_code: Option<Vec<StemCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeType {
    #[serde(rename = "GradeNumber")]
    pub grade_number: u32,
    #[serde(rename = "GradeName", skip_serializing_if = "Option::is_none")]
    pub grade_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LongitudeType {
    #[serde(flatten)]
    pub base: LatitudeLongitudeValueType,
    #[serde(rename = "longitudeCategory")]
    pub longitude_category: LongitudeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtEndProfileExtrapolationType {
    #[serde(rename = "ReferenceHeight")]
    pub reference_height: u32,
    #[serde(rename = "ButtEndProfileExtrapolationTable", skip_serializing_if = "Option::is_none")]
    pub butt_end_profile_extrapolation_table: Option<ButtEndProfileExtrapolationTableType>,
    #[serde(rename = "ButtEndProfileExtrapolationFunction", skip_serializing_if = "Option::is_none")]
    pub butt_end_profile_extrapolation_function: Option<ButtEndProfileExtrapolationFunctionType>,
    #[serde(rename = "buttEndProfileExtrapolationMethod")]
    pub butt_end_profile_extrapolation_method: ButtEndProfileExtrapolationMethodType,
    #[serde(rename = "modificationRestricted")]
    pub modification_restricted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnclassifiedProductDefinitionMachineType {
    #[serde(rename = "ProductName")]
    pub product_name: String,
    #[serde(rename = "ModificationDate", skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<ModificationDateType>,
    #[serde(rename = "ProductInfo", skip_serializing_if = "Option::is_none")]
    pub product_info: Option<InfoType>,
    #[serde(rename = "ProductVersion", skip_serializing_if = "Option::is_none")]
    pub product_version: Option<VersionType>,
    #[serde(rename = "ProductGroupName", skip_serializing_if = "Option::is_none")]
    pub product_group_name: Option<String>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogVolumeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDefinitionWithCommonDetailsType {
    #[serde(flatten)]
    pub base: CommonProductDefinitionType,
    #[serde(rename = "DiameterDefinition")]
    pub diameter_definition: DiameterDefinitionType,
    #[serde(rename = "LengthDefinition")]
    pub length_definition: LengthDefinitionType,
    #[serde(rename = "PriceDefinition")]
    pub price_definition: PriceDefinitionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuckingCriteriaType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeDiameterCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitationCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightUnitType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesGroupDefinitionWithDetailsAndKeyType {
    #[serde(flatten)]
    pub base: SpeciesGroupDefinitionWithDetailsType,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitationResultType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDefinitionType {
    #[serde(rename = "ScaleKey")]
    pub scale_key: u32,
    #[serde(rename = "ScaleID")]
    pub scale_i_d: String,
    #[serde(rename = "ScaleModel")]
    pub scale_model: String,
    #[serde(rename = "ScaleCategory")]
    pub scale_category: ScaleCategoryType,
    #[serde(rename = "ScaleApplicationVersion")]
    pub scale_application_version: String,
    #[serde(rename = "ScaleCertificate", skip_serializing_if = "Option::is_none")]
    pub scale_certificate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableType {
    #[serde(rename = "Row")]
    pub row: Vec<RowType>,
    #[serde(rename = "tableID")]
    pub table_i_d: String,
    #[serde(rename = "tableName")]
    pub table_name: tableName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISFileFormatPointType {
    #[serde(flatten)]
    pub base: GISFileFormatType,
    #[serde(rename = "FormatFont")]
    pub format_font: FormatFontType,
    #[serde(rename = "TopRendering", skip_serializing_if = "Option::is_none")]
    pub top_rendering: Option<TopRenderingPointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtEndProfileExtrapolationFunctionCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

