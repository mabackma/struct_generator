use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherRepairReason {
    #[serde(flatten)]
    pub other_repair_reason: ColOtherRepairReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringStartTime {
    #[serde(flatten)]
    pub monitoring_start_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringTimeLength {
    #[serde(flatten)]
    pub monitoring_time_length: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumberOfHarvestedStems {
    #[serde(flatten)]
    pub number_of_harvested_stems: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineDrivenDistance {
    #[serde(flatten)]
    pub machine_driven_distance: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationalMonitoring {
    #[serde(flatten)]
    pub operational_monitoring: OperationalMonitoringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombinedEndTime {
    #[serde(flatten)]
    pub combined_end_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumberOfLoads {
    #[serde(flatten)]
    pub number_of_loads: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherMachineDownTimeStandardCode {
    #[serde(flatten)]
    pub other_machine_down_time_standard_code: ColOtherMachineDownTimeCodeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualMachineWorkTime {
    #[serde(flatten)]
    pub individual_machine_work_time: IndividualMachineWorkTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBHHeight {
    #[serde(flatten)]
    pub d_b_h_height: DBHHeightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombinedMachineWorkTime {
    #[serde(flatten)]
    pub combined_machine_work_time: CombinedMachineWorkTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringSettingsKey {
    #[serde(flatten)]
    pub monitoring_settings_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwarderData {
    #[serde(flatten)]
    pub forwarder_data: ForwarderDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorKey {
    #[serde(flatten)]
    pub operator_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherMachineData {
    #[serde(flatten)]
    pub other_machine_data: OtherMachineDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherMachineDownTimeCategory {
    #[serde(flatten)]
    pub other_machine_down_time_category: OtherMachineDownTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeLength {
    #[serde(flatten)]
    pub time_length: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringFilterTimeRun {
    #[serde(flatten)]
    pub monitoring_filter_time_run: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShiftCategory {
    #[serde(flatten)]
    pub shift_category: ShiftCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportInterval {
    #[serde(flatten)]
    pub report_interval: ReportIntervalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisturbanceManufacturerCode {
    #[serde(flatten)]
    pub disturbance_manufacturer_code: ManufacturerCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeDescription {
    #[serde(flatten)]
    pub code_description: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LowerTimeLimit {
    #[serde(flatten)]
    pub lower_time_limit: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BarkFunction {
    #[serde(flatten)]
    pub bark_function: BarkFunctionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaintenanceStandardCode {
    #[serde(flatten)]
    pub maintenance_standard_code: ColMaintenanceCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineEngineTime {
    #[serde(flatten)]
    pub machine_engine_time: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SparePartDescription {
    #[serde(flatten)]
    pub spare_part_description: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepairManufacturerCode {
    #[serde(flatten)]
    pub repair_manufacturer_code: ManufacturerCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombinedMachineRunTime {
    #[serde(flatten)]
    pub combined_machine_run_time: CombinedMachineRunTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualMachineRunTimeCategory {
    #[serde(flatten)]
    pub individual_machine_run_time_category: MachineRunTimeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalForwardedVolume {
    #[serde(flatten)]
    pub total_forwarded_volume: TotalForwardedVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardedGreenMass {
    #[serde(flatten)]
    pub forwarded_green_mass: NonNegativeDecimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mechanical {
    #[serde(flatten)]
    pub mechanical: LoaderLinkageMechanicalRepairReasonCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineTime {
    #[serde(flatten)]
    pub engine_time: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryDefinitionEstimation {
    #[serde(flatten)]
    pub delivery_definition_estimation: DeliveryEstimationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Electrical {
    #[serde(flatten)]
    pub electrical: LoaderLinkageElectricalRepairReasonCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtEndProfileExtrapolation {
    #[serde(flatten)]
    pub butt_end_profile_extrapolation: ButtEndProfileExtrapolationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorWorkTime {
    #[serde(flatten)]
    pub operator_work_time: OperatorWorkTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvesterData {
    #[serde(flatten)]
    pub harvester_data: HarvesterDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportStartTime {
    #[serde(flatten)]
    pub report_start_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportEndTime {
    #[serde(flatten)]
    pub report_end_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoOfShortDownTimes {
    #[serde(flatten)]
    pub no_of_short_down_times: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoundKnotFunction {
    #[serde(flatten)]
    pub sound_knot_function: SoundKnotFunctionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringFilterTimeMinimum {
    #[serde(flatten)]
    pub monitoring_filter_time_minimum: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectKey {
    #[serde(flatten)]
    pub object_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorLogInCoordinates {
    #[serde(flatten)]
    pub operator_log_in_coordinates: CoordinatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShiftStartTime {
    #[serde(flatten)]
    pub shift_start_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingHeadRepairReason {
    #[serde(flatten)]
    pub harvesting_head_repair_reason: HarvestingHeadRepairReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hydraulics {
    #[serde(flatten)]
    pub hydraulics: LoaderLinkageHydraulicsRepairReasonCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportFilterCategory {
    #[serde(flatten)]
    pub report_filter_category: ReportFilterCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingCategory {
    #[serde(flatten)]
    pub processing_category: ProcessingCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataTableGroup {
    #[serde(flatten)]
    pub data_table_group: DataTableGroupOperationalMonitoringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringFilterTimeDown {
    #[serde(flatten)]
    pub monitoring_filter_time_down: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorLogOutCoordinates {
    #[serde(flatten)]
    pub operator_log_out_coordinates: CoordinatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualMachineDownTime {
    #[serde(flatten)]
    pub individual_machine_down_time: MachineDownTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombinedMachineDownTime {
    #[serde(flatten)]
    pub combined_machine_down_time: CombinedMachineDownTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShiftOrder {
    #[serde(flatten)]
    pub shift_order: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuelConsumption {
    #[serde(flatten)]
    pub fuel_consumption: NonNegativeDecimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualUnutilizedTimeCategory {
    #[serde(flatten)]
    pub individual_unutilized_time_category: ColUnutilizedTimeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repair {
    #[serde(flatten)]
    pub repair: RepairType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrequencyShortDownTimes {
    #[serde(flatten)]
    pub frequency_short_down_times: FrequencyShortDownTimesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maintenance {
    #[serde(flatten)]
    pub maintenance: MaintenanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaintenanceManufacturerCode {
    #[serde(flatten)]
    pub maintenance_manufacturer_code: ManufacturerCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDefinedData {
    #[serde(flatten)]
    pub user_defined_data: UserDefinedDataOperationalMonitoringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpperTimeLimit {
    #[serde(flatten)]
    pub upper_time_limit: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrivenDistance {
    #[serde(flatten)]
    pub driven_distance: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SparePartsNoOfItems {
    #[serde(flatten)]
    pub spare_parts_no_of_items: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherMachineDownTimeManufacturerCode {
    #[serde(flatten)]
    pub other_machine_down_time_manufacturer_code: ManufacturerCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Disturbance {
    #[serde(flatten)]
    pub disturbance: DisturbanceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringSettings {
    #[serde(flatten)]
    pub monitoring_settings: MonitoringSettingsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorLoginTime {
    #[serde(flatten)]
    pub operator_login_time: OperatorLoginTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShiftDescription {
    #[serde(flatten)]
    pub shift_description: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombinedUnutilizedTime {
    #[serde(flatten)]
    pub combined_unutilized_time: CombinedUnutilizedTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpareParts {
    #[serde(flatten)]
    pub spare_parts: SparePartsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadNumberOfItems {
    #[serde(flatten)]
    pub load_number_of_items: LoadNumberOfItemsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShiftEndTime {
    #[serde(flatten)]
    pub shift_end_time: StanForD2010DateTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCode {
    #[serde(flatten)]
    pub stem_code: StemCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineRunTimeCategory {
    #[serde(flatten)]
    pub machine_run_time_category: MachineRunTimeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorShiftDefinition {
    #[serde(flatten)]
    pub operator_shift_definition: OperatorShiftDefinitionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Air {
    #[serde(flatten)]
    pub air: ColCarrierAirRepairReasonCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarrierRepairReason {
    #[serde(flatten)]
    pub carrier_repair_reason: CarrierRepairReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubObjectKey {
    #[serde(flatten)]
    pub sub_object_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationalMonitoringHeader {
    #[serde(flatten)]
    pub operational_monitoring_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalVolumeOfHarvestedLogs {
    #[serde(flatten)]
    pub total_volume_of_harvested_logs: TotalVolumeOfHarvestedLogsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grades {
    #[serde(flatten)]
    pub grades: GradesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnutilizedTimeCategory {
    #[serde(flatten)]
    pub unutilized_time_category: ColUnutilizedTimeCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisturbanceStandardCode {
    #[serde(flatten)]
    pub disturbance_standard_code: ColDisturbanceCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeciesGroupKey {
    #[serde(flatten)]
    pub species_group_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShifKey {
    #[serde(flatten)]
    pub shif_key: XsdpositiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderLinkageRepairReason {
    #[serde(flatten)]
    pub loader_linkage_repair_reason: LoaderLinkageRepairReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstonianVolumeParameters {
    #[serde(flatten)]
    pub estonian_volume_parameters: EstonianVolumeParametersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SparePartIdentity {
    #[serde(flatten)]
    pub spare_part_identity: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineFuelConsumption {
    #[serde(flatten)]
    pub machine_fuel_consumption: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualShortDownTime {
    #[serde(flatten)]
    pub individual_short_down_time: IndividualShortDownTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairType {
    #[serde(rename = "CarrierRepairReason")]
    pub carrier_repair_reason: CarrierRepairReasonType,
    #[serde(rename = "LoaderLinkageRepairReason")]
    pub loader_linkage_repair_reason: LoaderLinkageRepairReasonType,
    #[serde(rename = "HarvestingHeadRepairReason")]
    pub harvesting_head_repair_reason: HarvestingHeadRepairReasonType,
    #[serde(rename = "OtherRepairReason")]
    pub other_repair_reason: OtherRepairReasonType,
    #[serde(rename = "RepairManufacturerCode", skip_serializing_if = "Option::is_none")]
    pub repair_manufacturer_code: Option<ManufacturerCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportFilterCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTableGroupOperationalMonitoringType {
    #[serde(flatten)]
    pub base: DataTableGroupType,
    #[serde(rename = "ObjectKey")]
    pub object_key: u32,
    #[serde(rename = "ShifKey")]
    pub shif_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceType {
    #[serde(rename = "MaintenanceStandardCode")]
    pub maintenance_standard_code: MaintenanceCodeType,
    #[serde(rename = "MaintenanceManufacturerCode", skip_serializing_if = "Option::is_none")]
    pub maintenance_manufacturer_code: Option<ManufacturerCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportIntervalType {
    #[serde(rename = "ReportStartTime")]
    pub report_start_time: StanForD2010DateTimeType,
    #[serde(rename = "ReportEndTime")]
    pub report_end_time: StanForD2010DateTimeType,
    #[serde(rename = "ReportFilterCategory")]
    pub report_filter_category: ReportFilterCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationalMonitoringType {
    #[serde(rename = "OperationalMonitoringHeader")]
    pub operational_monitoring_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineOperationalMonitoringType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualMachineWorkTimeType {
    #[serde(flatten)]
    pub base: MonitoringTimeType,
    #[serde(rename = "MonitoringSettingsKey")]
    pub monitoring_settings_key: u32,
    #[serde(rename = "OtherMachineData", skip_serializing_if = "Option::is_none")]
    pub other_machine_data: Option<OtherMachineDataType>,
    #[serde(rename = "IndividualMachineRunTimeCategory")]
    pub individual_machine_run_time_category: MachineRunTimeCategoryType,
    #[serde(rename = "IndividualMachineDownTime")]
    pub individual_machine_down_time: MachineDownTimeType,
    #[serde(rename = "IndividualUnutilizedTimeCategory")]
    pub individual_unutilized_time_category: UnutilizedTimeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalForwardedVolumeType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "forwardedVolumeCategory")]
    pub forwarded_volume_category: ForwardedVolumeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesGroupDefinitionOperationalMonitoringType {
    #[serde(flatten)]
    pub base: CommonSpeciesGroupDefinitionType,
    #[serde(rename = "DBHHeight", skip_serializing_if = "Option::is_none")]
    pub d_b_h_height: Option<DBHHeightType>,
    #[serde(rename = "Grades", skip_serializing_if = "Option::is_none")]
    pub grades: Option<GradesType>,
    #[serde(rename = "BarkFunction", skip_serializing_if = "Option::is_none")]
    pub bark_function: Option<BarkFunctionType>,
    #[serde(rename = "SoundKnotFunction", skip_serializing_if = "Option::is_none")]
    pub sound_knot_function: Option<SoundKnotFunctionType>,
    #[serde(rename = "ButtEndProfileExtrapolation", skip_serializing_if = "Option::is_none")]
    pub butt_end_profile_extrapolation: Option<ButtEndProfileExtrapolationType>,
    #[serde(rename = "EstonianVolumeParameters", skip_serializing_if = "Option::is_none")]
    pub estonian_volume_parameters: Option<EstonianVolumeParametersType>,
    #[serde(rename = "StemCode", skip_serializing_if = "Option::is_none")]
    pub stem_code: Option<Vec<StemCodeType>>,
    #[serde(rename = "SpeciesGroupKey")]
    pub species_group_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinedMachineRunTimeType {
    #[serde(rename = "MachineRunTimeCategory")]
    pub machine_run_time_category: MachineRunTimeCategoryType,
    #[serde(rename = "TimeLength")]
    pub time_length: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorShiftDefinitionType {
    #[serde(rename = "ShifKey")]
    pub shif_key: u32,
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "ObjectKey", skip_serializing_if = "Option::is_none")]
    pub object_key: Option<u32>,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "ShiftOrder", skip_serializing_if = "Option::is_none")]
    pub shift_order: Option<u32>,
    #[serde(rename = "ShiftCategory", skip_serializing_if = "Option::is_none")]
    pub shift_category: Option<ShiftCategoryType>,
    #[serde(rename = "ShiftDescription", skip_serializing_if = "Option::is_none")]
    pub shift_description: Option<String>,
    #[serde(rename = "ShiftStartTime")]
    pub shift_start_time: StanForD2010DateTimeType,
    #[serde(rename = "ShiftEndTime", skip_serializing_if = "Option::is_none")]
    pub shift_end_time: Option<StanForD2010DateTimeType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderLinkageRepairReasonType {
    #[serde(rename = "Electrical")]
    pub electrical: LoaderLinkageElectricalRepairReasonCodeType,
    #[serde(rename = "Hydraulics")]
    pub hydraulics: LoaderLinkageHydraulicsRepairReasonCodeType,
    #[serde(rename = "Mechanical")]
    pub mechanical: LoaderLinkageMechanicalRepairReasonCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarrierRepairReasonType {
    #[serde(rename = "Electrical")]
    pub electrical: CarrierElectricalRepairReasonCodeType,
    #[serde(rename = "Hydraulics")]
    pub hydraulics: CarrierHydraulicsRepairReasonCodeType,
    #[serde(rename = "Mechanical")]
    pub mechanical: CarrierMechanicalRepairReasonCodeType,
    #[serde(rename = "Air")]
    pub air: CarrierAirRepairReasonCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingHeadRepairReasonType {
    #[serde(rename = "Electrical")]
    pub electrical: HarvestingHeadElectricalRepairReasonCodeType,
    #[serde(rename = "Hydraulics")]
    pub hydraulics: HarvestingHeadHydraulicsRepairReasonCodeType,
    #[serde(rename = "Mechanical")]
    pub mechanical: HarvestingHeadMechanicalRepairReasonCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorLoginTimeType {
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "MonitoringStartTime")]
    pub monitoring_start_time: StanForD2010DateTimeType,
    #[serde(rename = "MonitoringTimeLength")]
    pub monitoring_time_length: u32,
    #[serde(rename = "OperatorLogInCoordinates", skip_serializing_if = "Option::is_none")]
    pub operator_log_in_coordinates: Option<CoordinatesType>,
    #[serde(rename = "OperatorLogOutCoordinates", skip_serializing_if = "Option::is_none")]
    pub operator_log_out_coordinates: Option<CoordinatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorWorkTimeType {
    #[serde(flatten)]
    pub base: MonitoringTimeType,
    #[serde(rename = "ShifKey", skip_serializing_if = "Option::is_none")]
    pub shif_key: Option<u32>,
    #[serde(rename = "OperatorWorkTimeCategory")]
    pub operator_work_time_category: OperatorWorkTimeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinedUnutilizedTimeType {
    #[serde(rename = "UnutilizedTimeCategory")]
    pub unutilized_time_category: UnutilizedTimeCategoryType,
    #[serde(rename = "TimeLength")]
    pub time_length: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVolumeOfHarvestedLogsType {
    #[serde(flatten)]
    pub base: NonNegativeDecimal,
    #[serde(rename = "harvestedLogsVolumeCategory")]
    pub harvested_logs_volume_category: HarvestedLogsVolumeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedLogsVolumeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringTimeType {
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "ObjectKey", skip_serializing_if = "Option::is_none")]
    pub object_key: Option<u32>,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "MonitoringStartTime")]
    pub monitoring_start_time: StanForD2010DateTimeType,
    #[serde(rename = "MonitoringTimeLength")]
    pub monitoring_time_length: u32,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherMachineDownTimeType {
    #[serde(rename = "OtherMachineDownTimeStandardCode")]
    pub other_machine_down_time_standard_code: OtherMachineDownTimeCodeCategoryType,
    #[serde(rename = "OtherMachineDownTimeManufacturerCode", skip_serializing_if = "Option::is_none")]
    pub other_machine_down_time_manufacturer_code: Option<ManufacturerCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualShortDownTimeType {
    #[serde(flatten)]
    pub base: ShortDownTimeType,
    #[serde(rename = "MonitoringStartTime")]
    pub monitoring_start_time: StanForD2010DateTimeType,
    #[serde(rename = "MonitoringTimeLength")]
    pub monitoring_time_length: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherMachineDataType {
    #[serde(rename = "EngineTime")]
    pub engine_time: u32,
    #[serde(rename = "DrivenDistance")]
    pub driven_distance: u32,
    #[serde(rename = "FuelConsumption")]
    pub fuel_consumption: NonNegativeDecimal,
    #[serde(rename = "HarvesterData", skip_serializing_if = "Option::is_none")]
    pub harvester_data: Option<Vec<HarvesterDataType>>,
    #[serde(rename = "ForwarderData", skip_serializing_if = "Option::is_none")]
    pub forwarder_data: Option<ForwarderDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolumeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManufacturerCodeType {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "CodeDescription", skip_serializing_if = "Option::is_none")]
    pub code_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDefinedDataOperationalMonitoringType {
    #[serde(rename = "DataTableGroup")]
    pub data_table_group: Vec<DataTableGroupOperationalMonitoringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinedMachineDownTimeType {
    #[serde(flatten)]
    pub base: MachineDownTimeType,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<StanForD2010DateTimeType>,
    #[serde(rename = "TimeLength")]
    pub time_length: u32,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterDataType {
    #[serde(rename = "TotalVolumeOfHarvestedLogs", skip_serializing_if = "Option::is_none")]
    pub total_volume_of_harvested_logs: Option<Vec<TotalVolumeOfHarvestedLogsType>>,
    #[serde(rename = "NumberOfHarvestedStems")]
    pub number_of_harvested_stems: u32,
    #[serde(rename = "ProcessingCategory")]
    pub processing_category: ProcessingCategoryType,
    #[serde(rename = "SpeciesGroupKey", skip_serializing_if = "Option::is_none")]
    pub species_group_key: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisturbanceType {
    #[serde(rename = "DisturbanceStandardCode")]
    pub disturbance_standard_code: DisturbanceCodeType,
    #[serde(rename = "DisturbanceManufacturerCode", skip_serializing_if = "Option::is_none")]
    pub disturbance_manufacturer_code: Option<ManufacturerCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineOperationalMonitoringType {
    #[serde(flatten)]
    pub base: MachineWithHeadType,
    #[serde(rename = "SpeciesGroupDefinition", skip_serializing_if = "Option::is_none")]
    pub species_group_definition: Option<Vec<SpeciesGroupDefinitionOperationalMonitoringType>>,
    #[serde(rename = "ObjectDefinition", skip_serializing_if = "Option::is_none")]
    pub object_definition: Option<Vec<ObjectDefinitionMachineType>>,
    #[serde(rename = "IndividualMachineWorkTime", skip_serializing_if = "Option::is_none")]
    pub individual_machine_work_time: Option<Vec<IndividualMachineWorkTimeType>>,
    #[serde(rename = "CombinedMachineWorkTime", skip_serializing_if = "Option::is_none")]
    pub combined_machine_work_time: Option<Vec<CombinedMachineWorkTimeType>>,
    #[serde(rename = "OperatorLoginTime", skip_serializing_if = "Option::is_none")]
    pub operator_login_time: Option<Vec<OperatorLoginTimeType>>,
    #[serde(rename = "OperatorShiftDefinition", skip_serializing_if = "Option::is_none")]
    pub operator_shift_definition: Option<Vec<OperatorShiftDefinitionType>>,
    #[serde(rename = "OperatorWorkTime", skip_serializing_if = "Option::is_none")]
    pub operator_work_time: Option<Vec<OperatorWorkTimeType>>,
    #[serde(rename = "ReportInterval")]
    pub report_interval: ReportIntervalType,
    #[serde(rename = "MonitoringSettings")]
    pub monitoring_settings: Vec<MonitoringSettingsType>,
    #[serde(rename = "MachineEngineTime")]
    pub machine_engine_time: u32,
    #[serde(rename = "MachineDrivenDistance")]
    pub machine_driven_distance: u32,
    #[serde(rename = "MachineFuelConsumption")]
    pub machine_fuel_consumption: u32,
    #[serde(rename = "IndividualShortDownTime", skip_serializing_if = "Option::is_none")]
    pub individual_short_down_time: Option<Vec<IndividualShortDownTimeType>>,
    #[serde(rename = "FrequencyShortDownTimes", skip_serializing_if = "Option::is_none")]
    pub frequency_short_down_times: Option<Vec<FrequencyShortDownTimesType>>,
    #[serde(rename = "UserDefinedData", skip_serializing_if = "Option::is_none")]
    pub user_defined_data: Option<UserDefinedDataOperationalMonitoringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineRunTimeCategoryType {
    #[serde(flatten)]
    pub base: ColMachineRunTimeCategoryListType,
    #[serde(rename = "otherWorkCategory")]
    pub other_work_category: OtherWorkCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparePartsType {
    #[serde(rename = "SparePartIdentity")]
    pub spare_part_identity: String,
    #[serde(rename = "SparePartDescription", skip_serializing_if = "Option::is_none")]
    pub spare_part_description: Option<String>,
    #[serde(rename = "SparePartsNoOfItems")]
    pub spare_parts_no_of_items: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrequencyShortDownTimesType {
    #[serde(flatten)]
    pub base: ShortDownTimeType,
    #[serde(rename = "LowerTimeLimit")]
    pub lower_time_limit: u32,
    #[serde(rename = "UpperTimeLimit")]
    pub upper_time_limit: u32,
    #[serde(rename = "NoOfShortDownTimes")]
    pub no_of_short_down_times: u32,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<StanForD2010DateTimeType>,
    #[serde(rename = "CombinedEndTime", skip_serializing_if = "Option::is_none")]
    pub combined_end_time: Option<StanForD2010DateTimeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderDataType {
    #[serde(rename = "ForwardedGreenMass", skip_serializing_if = "Option::is_none")]
    pub forwarded_green_mass: Option<NonNegativeDecimal>,
    #[serde(rename = "TotalForwardedVolume", skip_serializing_if = "Option::is_none")]
    pub total_forwarded_volume: Option<Vec<TotalForwardedVolumeType>>,
    #[serde(rename = "NumberOfLoads", skip_serializing_if = "Option::is_none")]
    pub number_of_loads: Option<u32>,
    #[serde(rename = "LoadNumberOfItems", skip_serializing_if = "Option::is_none")]
    pub load_number_of_items: Option<LoadNumberOfItemsType>,
    #[serde(rename = "DeliveryDefinitionEstimation")]
    pub delivery_definition_estimation: DeliveryEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombinedMachineWorkTimeType {
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "StartTime")]
    pub start_time: StanForD2010DateTimeType,
    #[serde(rename = "ObjectKey", skip_serializing_if = "Option::is_none")]
    pub object_key: Option<u32>,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "MonitoringSettingsKey")]
    pub monitoring_settings_key: u32,
    #[serde(rename = "OtherMachineData", skip_serializing_if = "Option::is_none")]
    pub other_machine_data: Option<OtherMachineDataType>,
    #[serde(rename = "CombinedMachineRunTime", skip_serializing_if = "Option::is_none")]
    pub combined_machine_run_time: Option<Vec<CombinedMachineRunTimeType>>,
    #[serde(rename = "CombinedMachineDownTime", skip_serializing_if = "Option::is_none")]
    pub combined_machine_down_time: Option<Vec<CombinedMachineDownTimeType>>,
    #[serde(rename = "CombinedUnutilizedTime", skip_serializing_if = "Option::is_none")]
    pub combined_unutilized_time: Option<Vec<CombinedUnutilizedTimeType>>,
    #[serde(rename = "CombinedEndTime")]
    pub combined_end_time: StanForD2010DateTimeType,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineDownTimeType {
    #[serde(rename = "Repair")]
    pub repair: RepairType,
    #[serde(rename = "Maintenance")]
    pub maintenance: MaintenanceType,
    #[serde(rename = "Disturbance")]
    pub disturbance: DisturbanceType,
    #[serde(rename = "OtherMachineDownTimeCategory")]
    pub other_machine_down_time_category: OtherMachineDownTimeType,
    #[serde(rename = "SpareParts", skip_serializing_if = "Option::is_none")]
    pub spare_parts: Option<Vec<SparePartsType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringSettingsType {
    #[serde(rename = "MonitoringFilterTimeDown")]
    pub monitoring_filter_time_down: u32,
    #[serde(rename = "MonitoringFilterTimeRun")]
    pub monitoring_filter_time_run: u32,
    #[serde(rename = "MonitoringFilterTimeMinimum")]
    pub monitoring_filter_time_minimum: u32,
    #[serde(rename = "MonitoringSettingsKey")]
    pub monitoring_settings_key: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortDownTimeType {
    #[serde(rename = "OperatorKey")]
    pub operator_key: u32,
    #[serde(rename = "ObjectKey", skip_serializing_if = "Option::is_none")]
    pub object_key: Option<u32>,
    #[serde(rename = "SubObjectKey", skip_serializing_if = "Option::is_none")]
    pub sub_object_key: Option<u32>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
}

