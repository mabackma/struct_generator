use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkPointCount {
    #[serde(flatten)]
    pub work_point_count: BdtPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoadInvoiced {
    #[serde(flatten)]
    pub work_load_invoiced: BdtDecimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad1 {
    #[serde(flatten)]
    pub work_load1: BdtDecimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier3 {
    #[serde(flatten)]
    pub work_code_qualifier3: BdtWorkCodeQualifierType3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoadId {
    #[serde(flatten)]
    pub work_load_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad2 {
    #[serde(flatten)]
    pub work_load2: BdtDecimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad1Unit {
    #[serde(flatten)]
    pub work_load1_unit: BdtWorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier5 {
    #[serde(flatten)]
    pub work_code_qualifier5: BdtWorkCodeQualifierType5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfreezingDate {
    #[serde(flatten)]
    pub unfreezing_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialSupplier {
    #[serde(flatten)]
    pub material_supplier: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier1 {
    #[serde(flatten)]
    pub work_code_qualifier1: BdtWorkCodeQualifierType1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoadUnitInvoiced {
    #[serde(flatten)]
    pub work_load_unit_invoiced: BdtWorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier4 {
    #[serde(flatten)]
    pub work_code_qualifier4: BdtWorkCodeQualifierType4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkGrouMember {
    #[serde(flatten)]
    pub work_grou_member: WorkGrouMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accepted {
    #[serde(flatten)]
    pub accepted: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandFinished {
    #[serde(flatten)]
    pub stand_finished: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: BdtString10Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier2 {
    #[serde(flatten)]
    pub work_code_qualifier2: BdtWorkCodeQualifierType2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialUnit {
    #[serde(flatten)]
    pub material_unit: BdtMaterialUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLoad2Unit {
    #[serde(flatten)]
    pub work_load2_unit: BdtWorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkGrouMembers {
    #[serde(flatten)]
    pub work_grou_members: WorkGrouMembersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportDistanceUnit {
    #[serde(flatten)]
    pub transport_distance_unit: BdtDistanceUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialVolume {
    #[serde(flatten)]
    pub material_volume: BdtDecimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialShipment {
    #[serde(flatten)]
    pub material_shipment: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportDistance {
    #[serde(flatten)]
    pub transport_distance: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material")]
    pub material: Vec<MaterialType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembersType {
    #[serde(rename = "WorkGrouMember")]
    pub work_grou_member: Vec<WorkGrouMemberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMemberType {
    #[serde(rename = "ResourceId")]
    pub resource_id: ShortERPIdType,
    #[serde(rename = "UserId")]
    pub user_id: ShortERPIdType,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoadType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<String20Type>,
    #[serde(rename = "WorkLoadId")]
    pub work_load_id: u64,
    #[serde(rename = "Accepted")]
    pub accepted: YesNoType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<String20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<String50Type>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<String50Type>,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier1", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "WorkCodeQualifier2", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "WorkCodeQualifier3", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "WorkCodeQualifier4", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "WorkCodeQualifier5", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier5: Option<Vec<WorkCodeQualifierType5>>,
    #[serde(rename = "WorkLoad1")]
    pub work_load1: Decimal2FractionDigitsType,
    #[serde(rename = "WorkLoad1Unit")]
    pub work_load1_unit: WorkCodeUnitType,
    #[serde(rename = "TransportDistance", skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TransportDistanceUnit", skip_serializing_if = "Option::is_none")]
    pub transport_distance_unit: Option<DistanceUnitType>,
    #[serde(rename = "WorkLoad2", skip_serializing_if = "Option::is_none")]
    pub work_load2: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoad2Unit", skip_serializing_if = "Option::is_none")]
    pub work_load2_unit: Option<WorkCodeUnitType>,
    #[serde(rename = "WorkLoadInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_invoiced: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoadUnitInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_unit_invoiced: Option<WorkCodeUnitType>,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StandFinished", skip_serializing_if = "Option::is_none")]
    pub stand_finished: Option<YesNoType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeStampType>,
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeStampType>,
    #[serde(rename = "SavingTime", skip_serializing_if = "Option::is_none")]
    pub saving_time: Option<TimeStampType>,
    #[serde(rename = "WorkPointCount", skip_serializing_if = "Option::is_none")]
    pub work_point_count: Option<PositiveInteger5digitsType>,
    #[serde(rename = "WorkGrouMembers", skip_serializing_if = "Option::is_none")]
    pub work_grou_members: Option<WorkGrouMembersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: String10Type,
    #[serde(rename = "FeeListId", skip_serializing_if = "Option::is_none")]
    pub fee_list_id: Option<PositiveIntegerType>,
    #[serde(rename = "FeeYesNo", skip_serializing_if = "Option::is_none")]
    pub fee_yes_no: Option<YesNoType>,
    #[serde(rename = "FeeValue", skip_serializing_if = "Option::is_none")]
    pub fee_value: Option<String10Type>,
    #[serde(rename = "FeeAssortment", skip_serializing_if = "Option::is_none")]
    pub fee_assortment: Option<String50Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "NeedToCheck", skip_serializing_if = "Option::is_none")]
    pub need_to_check: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialType {
    #[serde(rename = "MaterialId", skip_serializing_if = "Option::is_none")]
    pub material_id: Option<String20Type>,
    #[serde(rename = "MaterialCode")]
    pub material_code: MaterialCodeType,
    #[serde(rename = "MaterialVolume")]
    pub material_volume: Decimal2FractionDigitsType,
    #[serde(rename = "MaterialUnit")]
    pub material_unit: MaterialUnitType,
    #[serde(rename = "MaterialSupplier", skip_serializing_if = "Option::is_none")]
    pub material_supplier: Option<String50Type>,
    #[serde(rename = "MaterialProducer", skip_serializing_if = "Option::is_none")]
    pub material_producer: Option<String50Type>,
    #[serde(rename = "MaterialShipment", skip_serializing_if = "Option::is_none")]
    pub material_shipment: Option<String20Type>,
    #[serde(rename = "FreezingDate", skip_serializing_if = "Option::is_none")]
    pub freezing_date: Option<DateType>,
    #[serde(rename = "PackagingDate", skip_serializing_if = "Option::is_none")]
    pub packaging_date: Option<DateType>,
    #[serde(rename = "UnfreezingDate", skip_serializing_if = "Option::is_none")]
    pub unfreezing_date: Option<DateType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "GrainSize", skip_serializing_if = "Option::is_none")]
    pub grain_size: Option<PositiveInteger3digitsType>,
}

