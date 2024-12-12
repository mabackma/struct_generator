#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandFinished {
    #[serde(flatten)]
    pub stand_finished: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadId {
    #[serde(flatten)]
    pub work_load_id: unsignedLong,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestSystemPaymentReference {
    #[serde(flatten)]
    pub forest_system_payment_reference: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier5 {
    #[serde(flatten)]
    pub work_code_qualifier5: WorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier3 {
    #[serde(flatten)]
    pub work_code_qualifier3: WorkCodeQualifierType3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCode {
    #[serde(flatten)]
    pub material_code: MaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMember {
    #[serde(flatten)]
    pub work_grou_member: WorkGrouMemberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accepted {
    #[serde(flatten)]
    pub accepted: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: ShortERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1 {
    #[serde(flatten)]
    pub work_load1: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedToCheck {
    #[serde(flatten)]
    pub need_to_check: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialSupplier {
    #[serde(flatten)]
    pub material_supplier: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeListId {
    #[serde(flatten)]
    pub fee_list_id: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadUnitInvoiced {
    #[serde(flatten)]
    pub work_load_unit_invoiced: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembers {
    #[serde(flatten)]
    pub work_grou_members: WorkGrouMembersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBase {
    #[serde(flatten)]
    pub fee_base: FeeBasisDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: ShortERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistance {
    #[serde(flatten)]
    pub transport_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialId {
    #[serde(flatten)]
    pub material_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProducer {
    #[serde(flatten)]
    pub material_producer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialShipment {
    #[serde(flatten)]
    pub material_shipment: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkPointCount {
    #[serde(flatten)]
    pub work_point_count: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreezingDate {
    #[serde(flatten)]
    pub freezing_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2 {
    #[serde(flatten)]
    pub work_load2: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrainSize {
    #[serde(flatten)]
    pub grain_size: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistanceUnit {
    #[serde(flatten)]
    pub transport_distance_unit: DistanceUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeYesNo {
    #[serde(flatten)]
    pub fee_yes_no: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAssortment {
    #[serde(flatten)]
    pub fee_assortment: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    #[serde(flatten)]
    pub material: MaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeId {
    #[serde(flatten)]
    pub fee_id: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infotext {
    #[serde(flatten)]
    pub infotext: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnit {
    #[serde(flatten)]
    pub material_unit: MaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier4 {
    #[serde(flatten)]
    pub work_code_qualifier4: WorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagingDate {
    #[serde(flatten)]
    pub packaging_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfreezingDate {
    #[serde(flatten)]
    pub unfreezing_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialVolume {
    #[serde(flatten)]
    pub material_volume: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeValue {
    #[serde(flatten)]
    pub fee_value: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2Unit {
    #[serde(flatten)]
    pub work_load2_unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Length {
    #[serde(flatten)]
    pub length: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier2 {
    #[serde(flatten)]
    pub work_code_qualifier2: WorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasis {
    #[serde(flatten)]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Materials {
    #[serde(flatten)]
    pub materials: MaterialsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier1 {
    #[serde(flatten)]
    pub work_code_qualifier1: WorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadInvoiced {
    #[serde(flatten)]
    pub work_load_invoiced: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1Unit {
    #[serde(flatten)]
    pub work_load1_unit: WorkCodeUnitType,
}

