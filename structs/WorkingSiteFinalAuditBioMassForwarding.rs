use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditBioMassForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestEnergySuitableText {
    #[serde(flatten)]
    pub forest_energy_suitable_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassQuality {
    #[serde(flatten)]
    pub bio_mass_quality: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassQualityText {
    #[serde(flatten)]
    pub bio_mass_quality_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingBiomass {
    #[serde(flatten)]
    pub remaining_biomass: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestEnergySuitable {
    #[serde(flatten)]
    pub forest_energy_suitable: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingBiomassText {
    #[serde(flatten)]
    pub remaining_biomass_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<YesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<String200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "LimitsToWaterSystem", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system: Option<YesNoType>,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub water_system_protection: Option<YesNoType>,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed: Option<YesNoType>,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient: Option<YesNoType>,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
    #[serde(rename = "Storage")]
    pub storage: YesNoType,
    #[serde(rename = "StorageText", skip_serializing_if = "Option::is_none")]
    pub storage_text: Option<String200Type>,
    #[serde(rename = "RoadDamages", skip_serializing_if = "Option::is_none")]
    pub road_damages: Option<YesNoType>,
    #[serde(rename = "RoadDamagesText", skip_serializing_if = "Option::is_none")]
    pub road_damages_text: Option<String200Type>,
    #[serde(rename = "TreeDamages", skip_serializing_if = "Option::is_none")]
    pub tree_damages: Option<YesNoType>,
    #[serde(rename = "TreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_damages_text: Option<String200Type>,
    #[serde(rename = "VehiclePathPressures", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures: Option<YesNoType>,
    #[serde(rename = "VehiclePathPressuresText", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures_text: Option<String200Type>,
    #[serde(rename = "BioMassQuality")]
    pub bio_mass_quality: YesNoType,
    #[serde(rename = "BioMassQualityText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_quality_text: Option<String200Type>,
    #[serde(rename = "ForestEnergySuitable", skip_serializing_if = "Option::is_none")]
    pub forest_energy_suitable: Option<YesNoType>,
    #[serde(rename = "ForestEnergySuitableText", skip_serializing_if = "Option::is_none")]
    pub forest_energy_suitable_text: Option<String200Type>,
    #[serde(rename = "RemainingBiomass", skip_serializing_if = "Option::is_none")]
    pub remaining_biomass: Option<YesNoType>,
    #[serde(rename = "RemainingBiomassText", skip_serializing_if = "Option::is_none")]
    pub remaining_biomass_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

