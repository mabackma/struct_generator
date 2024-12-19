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
    pub status1: Option<ChangeStateType>,
    #[serde(rename = "AnnouncementId", skip_serializing_if = "Option::is_none")]
    pub announcement_id: Option<AnnouncementIdType>,
    #[serde(rename = "KemeraId", skip_serializing_if = "Option::is_none")]
    pub kemera_id: Option<VirtaIdType>,
    #[serde(rename = "EstateOwner", skip_serializing_if = "Option::is_none")]
    pub estate_owner: Option<string>,
    #[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub municipality_number: Option<KuntaKoodiTyyppi>,
    #[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<string>,
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<string>,
    #[serde(rename = "UnseparatedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel_number: Option<string>,
    #[serde(rename = "UnseparatedParcel", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel: Option<string>,
    #[serde(rename = "KemeraMunicipalityId", skip_serializing_if = "Option::is_none")]
    pub kemera_municipality_id: Option<string>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<VirtaTargetSelectionType>,
    #[serde(rename = "Population", skip_serializing_if = "Option::is_none")]
    pub population: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MastoInspection", skip_serializing_if = "Option::is_none")]
    pub masto_inspection: Option<VirtaMastoInspectionType>,
    #[serde(rename = "WorkType", skip_serializing_if = "Option::is_none")]
    pub work_type: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<VirtaPhaseType>,
    #[serde(rename = "ProjectStatus", skip_serializing_if = "Option::is_none")]
    pub project_status: Option<VirtaProjectStatusType>,
    #[serde(rename = "AnnouncedArea", skip_serializing_if = "Option::is_none")]
    pub announced_area: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AnnouncedLength", skip_serializing_if = "Option::is_none")]
    pub announced_length: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Law", skip_serializing_if = "Option::is_none")]
    pub law: Option<VirtaLawType>,
    #[serde(rename = "EarliestInspectionDate", skip_serializing_if = "Option::is_none")]
    pub earliest_inspection_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "Advertiser", skip_serializing_if = "Option::is_none")]
    pub advertiser: Option<VirtaAdvertiserType>,
    #[serde(rename = "Operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<VirtaAdvertiserType>,
    #[serde(rename = "AdvertisementDating", skip_serializing_if = "Option::is_none")]
    pub advertisement_dating: Option<VirtaAdvertisementDatingType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "TreeDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand: Option<VirtaEvaluationType>,
    #[serde(rename = "TerrainDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand: Option<VirtaEvaluationType>,
    #[serde(rename = "InspectionDate", skip_serializing_if = "Option::is_none")]
    pub inspection_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<VirtaExtraInfoType>,
    #[serde(rename = "HarvestExtraInfo", skip_serializing_if = "Option::is_none")]
    pub harvest_extra_info: Option<VirtaExtraInfoType>,
    #[serde(rename = "SumTableArea", skip_serializing_if = "Option::is_none")]
    pub sum_table_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "StubPriceArea", skip_serializing_if = "Option::is_none")]
    pub stub_price_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "UnitCostArea", skip_serializing_if = "Option::is_none")]
    pub unit_cost_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "EvaluationCost", skip_serializing_if = "Option::is_none")]
    pub evaluation_cost: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "InsuranceOrOtherCompensation", skip_serializing_if = "Option::is_none")]
    pub insurance_or_other_compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "OwnerInvolvement", skip_serializing_if = "Option::is_none")]
    pub owner_involvement: Option<VirtaYesNoType>,
    #[serde(rename = "AssociationInvolvement", skip_serializing_if = "Option::is_none")]
    pub association_involvement: Option<VirtaYesNoType>,
    #[serde(rename = "OwnerSampleAreaApproval", skip_serializing_if = "Option::is_none")]
    pub owner_sample_area_approval: Option<VirtaApprovalType>,
    #[serde(rename = "OwnerActionApproval", skip_serializing_if = "Option::is_none")]
    pub owner_action_approval: Option<VirtaApprovalType>,
    #[serde(rename = "MoosePercentage", skip_serializing_if = "Option::is_none")]
    pub moose_percentage: Option<PercentType>,
    #[serde(rename = "AssociationEvaluationApproval", skip_serializing_if = "Option::is_none")]
    pub association_evaluation_approval: Option<VirtaApprovalType>,
    #[serde(rename = "Targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetsType>,
    #[serde(rename = "HelpGeometries", skip_serializing_if = "Option::is_none")]
    pub help_geometries: Option<HelpGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaApprovalType {
    #[serde(flatten)]
    pub base: VirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSumTableAreaType {
    #[serde(flatten)]
    pub base: VirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSaveIncompleteType {
    #[serde(flatten)]
    pub base: VirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetsType {
    #[serde(rename = "Target")]
    pub target: Vec<TargetType>,
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
pub struct VirtaIdType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaMastoInspectionType {
    #[serde(flatten)]
    pub base: VirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertiserType {
    #[serde(flatten)]
    pub base: VirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPhaseType {
    #[serde(flatten)]
    pub base: VirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementIdType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaProjectStatusType {
    #[serde(flatten)]
    pub base: VirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaLawType {
    #[serde(flatten)]
    pub base: VirtaLawType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometriesType {
    #[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
    pub polygon_geometry: Option<Vec<PolygonGeometryType>>,
    #[serde(rename = "LineGeometry", skip_serializing_if = "Option::is_none")]
    pub line_geometry: Option<Vec<LineGeometryType>>,
    #[serde(rename = "PointGeometry", skip_serializing_if = "Option::is_none")]
    pub point_geometry: Option<Vec<PointGeometryType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertisementDatingType {
    #[serde(flatten)]
    pub base: VirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDataType {
    #[serde(rename = "DataInformation")]
    pub data_information: DataInformationType,
    #[serde(rename = "Inspection")]
    pub inspection: Vec<InspectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetSelectionType {
    #[serde(flatten)]
    pub base: VirtaTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionTypeType {
    #[serde(flatten)]
    pub base: VirtaInspectionTypeType,
}

