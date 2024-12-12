#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    #[serde(flatten)]
    pub password: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedLength {
    #[serde(flatten)]
    pub announced_length: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(flatten)]
    pub work_type: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerActionApproval {
    #[serde(flatten)]
    pub owner_action_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectorName {
    #[serde(flatten)]
    pub inspector_name: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataId {
    #[serde(flatten)]
    pub data_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerInvolvement {
    #[serde(flatten)]
    pub owner_involvement: VirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestExtraInfo {
    #[serde(flatten)]
    pub harvest_extra_info: VirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KemeraId {
    #[serde(flatten)]
    pub kemera_id: VirtaIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelection {
    #[serde(flatten)]
    pub target_selection: VirtaTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelCount {
    #[serde(flatten)]
    pub level_count: PositiveInteger1digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamageOutsideStand {
    #[serde(flatten)]
    pub tree_damage_outside_stand: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwner {
    #[serde(flatten)]
    pub estate_owner: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operator {
    #[serde(flatten)]
    pub operator: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarliestInspectionDate {
    #[serde(flatten)]
    pub earliest_inspection_date: DateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationEvaluationApproval {
    #[serde(flatten)]
    pub association_evaluation_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoosePercentage {
    #[serde(flatten)]
    pub moose_percentage: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataName {
    #[serde(flatten)]
    pub data_name: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationInvolvement {
    #[serde(flatten)]
    pub association_involvement: VirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveIncomplete {
    #[serde(flatten)]
    pub save_incomplete: VirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
    #[serde(flatten)]
    pub phase: VirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherEvaluation {
    #[serde(flatten)]
    pub other_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceOrOtherCompensation {
    #[serde(flatten)]
    pub insurance_or_other_compensation: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Population {
    #[serde(flatten)]
    pub population: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparatedParcelNumber {
    #[serde(flatten)]
    pub unseparated_parcel_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MastoInspection {
    #[serde(flatten)]
    pub masto_inspection: VirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedArea {
    #[serde(flatten)]
    pub announced_area: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedEnergyWood {
    #[serde(flatten)]
    pub detected_energy_wood: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(flatten)]
    pub inspection_type: VirtaInspectionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedGroundWood {
    #[serde(flatten)]
    pub detected_ground_wood: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvertisementDating {
    #[serde(flatten)]
    pub advertisement_dating: VirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: VirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncedEnergyWood {
    #[serde(flatten)]
    pub announced_energy_wood: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometries {
    #[serde(flatten)]
    pub help_geometries: HelpGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparatedParcel {
    #[serde(flatten)]
    pub unseparated_parcel: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainDamageOutsideStand {
    #[serde(flatten)]
    pub terrain_damage_outside_stand: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaId {
    #[serde(flatten)]
    pub virta_id: VirtaIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatus {
    #[serde(flatten)]
    pub project_status: VirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubPriceArea {
    #[serde(flatten)]
    pub stub_price_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Advertiser {
    #[serde(flatten)]
    pub advertiser: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementId {
    #[serde(flatten)]
    pub announcement_id: AnnouncementIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inspection {
    #[serde(flatten)]
    pub inspection: InspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: DateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: KuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitCostArea {
    #[serde(flatten)]
    pub unit_cost_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaAndMapEvaluation {
    #[serde(flatten)]
    pub area_and_map_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerSampleAreaApproval {
    #[serde(flatten)]
    pub owner_sample_area_approval: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Targets {
    #[serde(flatten)]
    pub targets: TargetsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalInspectionId {
    #[serde(flatten)]
    pub internal_inspection_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KemeraMunicipalityId {
    #[serde(flatten)]
    pub kemera_municipality_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SumTableArea {
    #[serde(flatten)]
    pub sum_table_area: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformation {
    #[serde(flatten)]
    pub data_information: DataInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCost {
    #[serde(flatten)]
    pub evaluation_cost: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDate {
    #[serde(flatten)]
    pub inspection_date: DateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Law {
    #[serde(flatten)]
    pub law: VirtaLawType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionData {
    #[serde(flatten)]
    pub inspection_data: InspectionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status1 {
    #[serde(flatten)]
    pub status1: ChangeStateType,
}

