#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialVolume {
    #[serde(flatten)]
    pub material_volume: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1 {
    #[serde(flatten)]
    pub work_load1: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadInvoiced {
    #[serde(flatten)]
    pub work_load_invoiced: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier4 {
    #[serde(flatten)]
    pub work_code_qualifier4: WorkCodeQualifierType4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad1Unit {
    #[serde(flatten)]
    pub work_load1_unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMember {
    #[serde(flatten)]
    pub work_grou_member: WorkGrouMemberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier3 {
    #[serde(flatten)]
    pub work_code_qualifier3: WorkCodeQualifierType3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnit {
    #[serde(flatten)]
    pub material_unit: MaterialUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier2 {
    #[serde(flatten)]
    pub work_code_qualifier2: WorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistanceUnit {
    #[serde(flatten)]
    pub transport_distance_unit: DistanceUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadId {
    #[serde(flatten)]
    pub work_load_id: unsignedLong,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2Unit {
    #[serde(flatten)]
    pub work_load2_unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfreezingDate {
    #[serde(flatten)]
    pub unfreezing_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkPointCount {
    #[serde(flatten)]
    pub work_point_count: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembers {
    #[serde(flatten)]
    pub work_grou_members: WorkGrouMembersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoadUnitInvoiced {
    #[serde(flatten)]
    pub work_load_unit_invoiced: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialShipment {
    #[serde(flatten)]
    pub material_shipment: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDistance {
    #[serde(flatten)]
    pub transport_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier1 {
    #[serde(flatten)]
    pub work_code_qualifier1: WorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialSupplier {
    #[serde(flatten)]
    pub material_supplier: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandFinished {
    #[serde(flatten)]
    pub stand_finished: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier5 {
    #[serde(flatten)]
    pub work_code_qualifier5: WorkCodeQualifierType5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLoad2 {
    #[serde(flatten)]
    pub work_load2: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accepted {
    #[serde(flatten)]
    pub accepted: YesNoType,
}

