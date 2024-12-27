#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQuality {
    #[serde(flatten)]
    pub plant_management_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantManagementQualityText {
    #[serde(flatten)]
    pub plant_management_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: BdtYesNoType,
}

