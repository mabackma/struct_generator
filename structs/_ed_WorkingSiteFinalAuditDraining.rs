#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamagesText {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrder {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbroken {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowed {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinAndDamDimensioningIsFollowedText {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomeDitchesNotDiggedAsMentionedInOrderText {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffectText {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocation {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspensionHoleDensitySizeAndLocationText {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavatorMovingAndTreeDamages {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocationText {
    #[serde(flatten)]
    pub excavation_soil_location_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcavationSoilLocation {
    #[serde(flatten)]
    pub excavation_soil_location: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceRunoffFieldGroundIsUnbrokenText {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocation {
    #[serde(flatten)]
    pub ditch_cleaning_break_location: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDone {
    #[serde(flatten)]
    pub cleaning_breaks_done: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchDepthWidthAndDrainageEffect {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchCleaningBreakLocationText {
    #[serde(flatten)]
    pub ditch_cleaning_break_location_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionality {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SedimentationBasinSizeShapeAndFunctionalityText {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningBreaksDoneText {
    #[serde(flatten)]
    pub cleaning_breaks_done_text: String200Type,
}

