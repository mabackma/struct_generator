#[derive(Debug, Serialize, Deserialize)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier5 {
    #[serde(flatten)]
    pub work_code_qualifier5: BdtWorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1Unit {
    #[serde(flatten)]
    pub work_load1_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandFinished {
    #[serde(flatten)]
    pub stand_finished: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialVolume {
    #[serde(flatten)]
    pub material_volume: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2Unit {
    #[serde(flatten)]
    pub work_load2_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMember {
    #[serde(flatten)]
    pub work_grou_member: WorkGrouMemberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfreezingDate {
    #[serde(flatten)]
    pub unfreezing_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1 {
    #[serde(flatten)]
    pub work_load1: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembers {
    #[serde(flatten)]
    pub work_grou_members: WorkGrouMembersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnit {
    #[serde(flatten)]
    pub material_unit: BdtMaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadId {
    #[serde(flatten)]
    pub work_load_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistanceUnit {
    #[serde(flatten)]
    pub transport_distance_unit: BdtDistanceUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialSupplier {
    #[serde(flatten)]
    pub material_supplier: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier2 {
    #[serde(flatten)]
    pub work_code_qualifier2: BdtWorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier1 {
    #[serde(flatten)]
    pub work_code_qualifier1: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadInvoiced {
    #[serde(flatten)]
    pub work_load_invoiced: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkPointCount {
    #[serde(flatten)]
    pub work_point_count: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistance {
    #[serde(flatten)]
    pub transport_distance: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadUnitInvoiced {
    #[serde(flatten)]
    pub work_load_unit_invoiced: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialShipment {
    #[serde(flatten)]
    pub material_shipment: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier4 {
    #[serde(flatten)]
    pub work_code_qualifier4: BdtWorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accepted {
    #[serde(flatten)]
    pub accepted: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier3 {
    #[serde(flatten)]
    pub work_code_qualifier3: BdtWorkCodeQualifierType3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2 {
    #[serde(flatten)]
    pub work_load2: BdtDecimal2FractionDigitsType,
}

