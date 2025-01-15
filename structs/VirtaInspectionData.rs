use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpGeometries {
    #[serde(flatten)]
    pub help_geometries: HelpGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstateOwner {
    #[serde(flatten)]
    pub estate_owner: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubPriceArea {
    #[serde(flatten)]
    pub stub_price_area: VirtaSumTableAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamageOutsideStand {
    #[serde(flatten)]
    pub tree_damage_outside_stand: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparatedParcel {
    #[serde(flatten)]
    pub unseparated_parcel: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveIncomplete {
    #[serde(flatten)]
    pub save_incomplete: VirtaSaveIncompleteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataId {
    #[serde(flatten)]
    pub data_id: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operator {
    #[serde(flatten)]
    pub operator: VirtaAdvertiserType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitCostArea {
    #[serde(flatten)]
    pub unit_cost_area: VirtaSumTableAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MastoInspection {
    #[serde(flatten)]
    pub masto_inspection: VirtaMastoInspectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EarliestInspectionDate {
    #[serde(flatten)]
    pub earliest_inspection_date: CoDateMmDdYyyyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataName {
    #[serde(flatten)]
    pub data_name: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkType {
    #[serde(flatten)]
    pub work_type: CoPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncedLength {
    #[serde(flatten)]
    pub announced_length: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectorName {
    #[serde(flatten)]
    pub inspector_name: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncedArea {
    #[serde(flatten)]
    pub announced_area: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerActionApproval {
    #[serde(flatten)]
    pub owner_action_approval: VirtaApprovalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status1 {
    #[serde(flatten)]
    pub status1: CoChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociationEvaluationApproval {
    #[serde(flatten)]
    pub association_evaluation_approval: VirtaApprovalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Population {
    #[serde(flatten)]
    pub population: CoPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KemeraMunicipalityId {
    #[serde(flatten)]
    pub kemera_municipality_id: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestExtraInfo {
    #[serde(flatten)]
    pub harvest_extra_info: TgtVirtaExtraInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SumTableArea {
    #[serde(flatten)]
    pub sum_table_area: VirtaSumTableAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionType {
    #[serde(flatten)]
    pub inspection_type: VirtaInspectionTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncementId {
    #[serde(flatten)]
    pub announcement_id: AnnouncementIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvertisementDating {
    #[serde(flatten)]
    pub advertisement_dating: VirtaAdvertisementDatingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerSampleAreaApproval {
    #[serde(flatten)]
    pub owner_sample_area_approval: VirtaApprovalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparatedParcelNumber {
    #[serde(flatten)]
    pub unseparated_parcel_number: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VirtaId {
    #[serde(flatten)]
    pub virta_id: VirtaIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetectedEnergyWood {
    #[serde(flatten)]
    pub detected_energy_wood: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainDamageOutsideStand {
    #[serde(flatten)]
    pub terrain_damage_outside_stand: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncedEnergyWood {
    #[serde(flatten)]
    pub announced_energy_wood: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KemeraId {
    #[serde(flatten)]
    pub kemera_id: VirtaIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStatus {
    #[serde(flatten)]
    pub project_status: VirtaProjectStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalInspectionId {
    #[serde(flatten)]
    pub internal_inspection_id: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataInformation {
    #[serde(flatten)]
    pub data_information: DataInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Law {
    #[serde(flatten)]
    pub law: VirtaLawType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Advertiser {
    #[serde(flatten)]
    pub advertiser: VirtaAdvertiserType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelCount {
    #[serde(flatten)]
    pub level_count: CoPositiveInteger1digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    #[serde(flatten)]
    pub password: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Targets {
    #[serde(flatten)]
    pub targets: TargetsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inspection {
    #[serde(flatten)]
    pub inspection: InspectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionDate {
    #[serde(flatten)]
    pub inspection_date: CoDateMmDdYyyyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase {
    #[serde(flatten)]
    pub phase: VirtaPhaseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionData {
    #[serde(flatten)]
    pub inspection_data: InspectionDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetectedGroundWood {
    #[serde(flatten)]
    pub detected_ground_wood: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDataType {
    #[serde(rename = "DataInformation")]
    pub data_information: DataInformationType,
    #[serde(rename = "Inspection")]
    pub inspection: Vec<InspectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometriesType {
    #[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_polygon_geometry: Option<Vec<PolygonGeometry>>,
    #[serde(rename = "LineGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_line_geometry: Option<Vec<LineGeometry>>,
    #[serde(rename = "PointGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_point_geometry: Option<Vec<PointGeometry>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetSelectionType {
    #[serde(flatten)]
    pub base: CoVirtaTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(rename = "InternalInspectionId")]
    pub internal_inspection_id: String,
    #[serde(rename = "VirtaId")]
    pub virta_id: VirtaIdType,
    #[serde(rename = "InspectionType", skip_serializing_if = "Option::is_none")]
    pub inspection_type: Option<VirtaInspectionTypeType>,
    #[serde(rename = "SaveIncomplete", skip_serializing_if = "Option::is_none")]
    pub save_incomplete: Option<VirtaSaveIncompleteType>,
    #[serde(rename = "Status1", skip_serializing_if = "Option::is_none")]
    pub status1: Option<CoChangeStateType>,
    #[serde(rename = "AnnouncementId", skip_serializing_if = "Option::is_none")]
    pub announcement_id: Option<AnnouncementIdType>,
    #[serde(rename = "KemeraId", skip_serializing_if = "Option::is_none")]
    pub kemera_id: Option<VirtaIdType>,
    #[serde(rename = "EstateOwner", skip_serializing_if = "Option::is_none")]
    pub estate_owner: Option<String>,
    #[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub municipality_number: Option<JhsKuntaKoodiTyyppi>,
    #[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<String>,
    #[serde(rename = "UnseparatedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel_number: Option<String>,
    #[serde(rename = "UnseparatedParcel", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel: Option<String>,
    #[serde(rename = "KemeraMunicipalityId", skip_serializing_if = "Option::is_none")]
    pub kemera_municipality_id: Option<String>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<VirtaTargetSelectionType>,
    #[serde(rename = "Population", skip_serializing_if = "Option::is_none")]
    pub population: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "MastoInspection", skip_serializing_if = "Option::is_none")]
    pub masto_inspection: Option<VirtaMastoInspectionType>,
    #[serde(rename = "WorkType", skip_serializing_if = "Option::is_none")]
    pub work_type: Option<CoPositiveInteger2digitsType>,
    #[serde(rename = "Phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<VirtaPhaseType>,
    #[serde(rename = "ProjectStatus", skip_serializing_if = "Option::is_none")]
    pub project_status: Option<VirtaProjectStatusType>,
    #[serde(rename = "AnnouncedArea", skip_serializing_if = "Option::is_none")]
    pub announced_area: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AnnouncedLength", skip_serializing_if = "Option::is_none")]
    pub announced_length: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Law", skip_serializing_if = "Option::is_none")]
    pub law: Option<VirtaLawType>,
    #[serde(rename = "EarliestInspectionDate", skip_serializing_if = "Option::is_none")]
    pub earliest_inspection_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "Advertiser", skip_serializing_if = "Option::is_none")]
    pub advertiser: Option<VirtaAdvertiserType>,
    #[serde(rename = "Operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<VirtaAdvertiserType>,
    #[serde(rename = "AdvertisementDating", skip_serializing_if = "Option::is_none")]
    pub advertisement_dating: Option<VirtaAdvertisementDatingType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "TreeDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand: Option<SpVirtaEvaluationType>,
    #[serde(rename = "TerrainDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand: Option<SpVirtaEvaluationType>,
    #[serde(rename = "InspectionDate", skip_serializing_if = "Option::is_none")]
    pub inspection_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<TgtVirtaExtraInfoType>,
    #[serde(rename = "HarvestExtraInfo", skip_serializing_if = "Option::is_none")]
    pub harvest_extra_info: Option<TgtVirtaExtraInfoType>,
    #[serde(rename = "SumTableArea", skip_serializing_if = "Option::is_none")]
    pub sum_table_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "StubPriceArea", skip_serializing_if = "Option::is_none")]
    pub stub_price_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "UnitCostArea", skip_serializing_if = "Option::is_none")]
    pub unit_cost_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "EvaluationCost", skip_serializing_if = "Option::is_none")]
    pub evaluation_cost: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "InsuranceOrOtherCompensation", skip_serializing_if = "Option::is_none")]
    pub insurance_or_other_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "OwnerInvolvement", skip_serializing_if = "Option::is_none")]
    pub owner_involvement: Option<CoVirtaYesNoType>,
    #[serde(rename = "AssociationInvolvement", skip_serializing_if = "Option::is_none")]
    pub association_involvement: Option<CoVirtaYesNoType>,
    #[serde(rename = "OwnerSampleAreaApproval", skip_serializing_if = "Option::is_none")]
    pub owner_sample_area_approval: Option<VirtaApprovalType>,
    #[serde(rename = "OwnerActionApproval", skip_serializing_if = "Option::is_none")]
    pub owner_action_approval: Option<VirtaApprovalType>,
    #[serde(rename = "MoosePercentage", skip_serializing_if = "Option::is_none")]
    pub moose_percentage: Option<CoPercentType>,
    #[serde(rename = "AssociationEvaluationApproval", skip_serializing_if = "Option::is_none")]
    pub association_evaluation_approval: Option<VirtaApprovalType>,
    #[serde(rename = "Targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetsType>,
    #[serde(rename = "HelpGeometries", skip_serializing_if = "Option::is_none")]
    pub help_geometries: Option<HelpGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaMastoInspectionType {
    #[serde(flatten)]
    pub base: CoVirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertiserType {
    #[serde(flatten)]
    pub base: CoVirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaApprovalType {
    #[serde(flatten)]
    pub base: CoVirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformationType {
    #[serde(rename = "DataName")]
    pub data_name: String,
    #[serde(rename = "DataId")]
    pub data_id: String,
    #[serde(rename = "InspectorName")]
    pub inspector_name: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSumTableAreaType {
    #[serde(flatten)]
    pub base: CoVirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertisementDatingType {
    #[serde(flatten)]
    pub base: CoVirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPhaseType {
    #[serde(flatten)]
    pub base: CoVirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetsType {
    #[serde(rename = "Target")]
    pub tgt_target: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaProjectStatusType {
    #[serde(flatten)]
    pub base: CoVirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionTypeType {
    #[serde(flatten)]
    pub base: CoVirtaInspectionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSaveIncompleteType {
    #[serde(flatten)]
    pub base: CoVirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaLawType {
    #[serde(flatten)]
    pub base: CoVirtaLawType,
}

