#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionData {
    #[serde(flatten)]
    pub inspection_data: InspectionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatus {
    #[serde(flatten)]
    pub project_status: VirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparatedParcelNumber {
    #[serde(flatten)]
    pub unseparated_parcel_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    #[serde(flatten)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwner {
    #[serde(flatten)]
    pub estate_owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Population {
    #[serde(flatten)]
    pub population: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamageOutsideStand {
    #[serde(flatten)]
    pub tree_damage_outside_stand: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Targets {
    #[serde(flatten)]
    pub targets: TargetsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(flatten)]
    pub inspection_type: VirtaInspectionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformation {
    #[serde(flatten)]
    pub data_information: DataInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometries {
    #[serde(flatten)]
    pub help_geometries: HelpGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Law {
    #[serde(flatten)]
    pub law: VirtaLawType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationEvaluationApproval {
    #[serde(flatten)]
    pub association_evaluation_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedLength {
    #[serde(flatten)]
    pub announced_length: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(flatten)]
    pub work_type: CoPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarliestInspectionDate {
    #[serde(flatten)]
    pub earliest_inspection_date: CoDateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KemeraMunicipalityId {
    #[serde(flatten)]
    pub kemera_municipality_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operator {
    #[serde(flatten)]
    pub operator: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MastoInspection {
    #[serde(flatten)]
    pub masto_inspection: VirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status1 {
    #[serde(flatten)]
    pub status1: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SumTableArea {
    #[serde(flatten)]
    pub sum_table_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparatedParcel {
    #[serde(flatten)]
    pub unseparated_parcel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestExtraInfo {
    #[serde(flatten)]
    pub harvest_extra_info: TgtVirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
    #[serde(flatten)]
    pub phase: VirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvertisementDating {
    #[serde(flatten)]
    pub advertisement_dating: VirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalInspectionId {
    #[serde(flatten)]
    pub internal_inspection_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedArea {
    #[serde(flatten)]
    pub announced_area: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitCostArea {
    #[serde(flatten)]
    pub unit_cost_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaId {
    #[serde(flatten)]
    pub virta_id: VirtaIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelCount {
    #[serde(flatten)]
    pub level_count: CoPositiveInteger1digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerSampleAreaApproval {
    #[serde(flatten)]
    pub owner_sample_area_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementId {
    #[serde(flatten)]
    pub announcement_id: AnnouncementIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerActionApproval {
    #[serde(flatten)]
    pub owner_action_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedGroundWood {
    #[serde(flatten)]
    pub detected_ground_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedEnergyWood {
    #[serde(flatten)]
    pub detected_energy_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveIncomplete {
    #[serde(flatten)]
    pub save_incomplete: VirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectorName {
    #[serde(flatten)]
    pub inspector_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Advertiser {
    #[serde(flatten)]
    pub advertiser: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inspection {
    #[serde(flatten)]
    pub inspection: InspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDate {
    #[serde(flatten)]
    pub inspection_date: CoDateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedEnergyWood {
    #[serde(flatten)]
    pub announced_energy_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataName {
    #[serde(flatten)]
    pub data_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainDamageOutsideStand {
    #[serde(flatten)]
    pub terrain_damage_outside_stand: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KemeraId {
    #[serde(flatten)]
    pub kemera_id: VirtaIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubPriceArea {
    #[serde(flatten)]
    pub stub_price_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataId {
    #[serde(flatten)]
    pub data_id: String,
}

