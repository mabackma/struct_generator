#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: WorkingQualityType,
}

