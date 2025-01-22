use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct SurfaceRunoffFieldGroundIsUnbrokenText {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurfaceRunoffFieldGroundIsUnbroken {
    #[serde(flatten)]
    pub surface_runoff_field_ground_is_unbroken: BdtYesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchCleaningBreakLocation {
    #[serde(flatten)]
    pub ditch_cleaning_break_location: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuspensionHoleDensitySizeAndLocation {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavatorMovingAndTreeDamagesText {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavationSoilLocation {
    #[serde(flatten)]
    pub excavation_soil_location: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchCleaningBreakLocationText {
    #[serde(flatten)]
    pub ditch_cleaning_break_location_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuspensionHoleDensitySizeAndLocationText {
    #[serde(flatten)]
    pub suspension_hole_density_size_and_location_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinAndDamDimensioningIsFollowedText {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinSizeShapeAndFunctionalityText {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchDepthWidthAndDrainageEffect {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeDitchesNotDiggedAsMentionedInOrder {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavationSoilLocationText {
    #[serde(flatten)]
    pub excavation_soil_location_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleaningBreaksDoneText {
    #[serde(flatten)]
    pub cleaning_breaks_done_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinAndDamDimensioningIsFollowed {
    #[serde(flatten)]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: BdtYesNoNotKnownType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleaningBreaksDone {
    #[serde(flatten)]
    pub cleaning_breaks_done: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchDepthWidthAndDrainageEffectText {
    #[serde(flatten)]
    pub ditch_depth_width_and_drainage_effect_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeDitchesNotDiggedAsMentionedInOrderText {
    #[serde(flatten)]
    pub some_ditches_not_digged_as_mentioned_in_order_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SedimentationBasinSizeShapeAndFunctionality {
    #[serde(flatten)]
    pub sedimentation_basin_size_shape_and_functionality: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavatorMovingAndTreeDamages {
    #[serde(flatten)]
    pub excavator_moving_and_tree_damages: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "AuditQuestion")]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDrainingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: BdtFinalAuditQuestionType,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: String20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "SedimentationBasinAndDamDimensioningIsFollowed", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: Option<YesNoNotKnownType>,
    #[serde(rename = "SedimentationBasinAndDamDimensioningIsFollowedText", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: Option<String200Type>,
    #[serde(rename = "SedimentationBasinSizeShapeAndFunctionality")]
    pub sedimentation_basin_size_shape_and_functionality: WorkingQualityType,
    #[serde(rename = "SedimentationBasinSizeShapeAndFunctionalityText", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_size_shape_and_functionality_text: Option<String200Type>,
    #[serde(rename = "SuspensionHoleDensitySizeAndLocation")]
    pub suspension_hole_density_size_and_location: WorkingQualityType,
    #[serde(rename = "SuspensionHoleDensitySizeAndLocationText", skip_serializing_if = "Option::is_none")]
    pub suspension_hole_density_size_and_location_text: Option<String200Type>,
    #[serde(rename = "CleaningBreaksDone")]
    pub cleaning_breaks_done: YesNoType,
    #[serde(rename = "CleaningBreaksDoneText", skip_serializing_if = "Option::is_none")]
    pub cleaning_breaks_done_text: Option<String200Type>,
    #[serde(rename = "DitchCleaningBreakLocation")]
    pub ditch_cleaning_break_location: WorkingQualityType,
    #[serde(rename = "DitchCleaningBreakLocationText", skip_serializing_if = "Option::is_none")]
    pub ditch_cleaning_break_location_text: Option<String200Type>,
    #[serde(rename = "SurfaceRunoffFieldGroundIsUnbroken")]
    pub surface_runoff_field_ground_is_unbroken: YesNoNotKnownType,
    #[serde(rename = "SurfaceRunoffFieldGroundIsUnbrokenText", skip_serializing_if = "Option::is_none")]
    pub surface_runoff_field_ground_is_unbroken_text: Option<String200Type>,
    #[serde(rename = "DitchDepthWidthAndDrainageEffect")]
    pub ditch_depth_width_and_drainage_effect: WorkingQualityType,
    #[serde(rename = "DitchDepthWidthAndDrainageEffectText", skip_serializing_if = "Option::is_none")]
    pub ditch_depth_width_and_drainage_effect_text: Option<String200Type>,
    #[serde(rename = "SomeDitchesNotDiggedAsMentionedInOrder")]
    pub some_ditches_not_digged_as_mentioned_in_order: YesNoType,
    #[serde(rename = "SomeDitchesNotDiggedAsMentionedInOrderText", skip_serializing_if = "Option::is_none")]
    pub some_ditches_not_digged_as_mentioned_in_order_text: Option<String200Type>,
    #[serde(rename = "ExcavationSoilLocation")]
    pub excavation_soil_location: WorkingQualityType,
    #[serde(rename = "ExcavationSoilLocationText", skip_serializing_if = "Option::is_none")]
    pub excavation_soil_location_text: Option<String200Type>,
    #[serde(rename = "ExcavatorMovingAndTreeDamages")]
    pub excavator_moving_and_tree_damages: WorkingQualityType,
    #[serde(rename = "ExcavatorMovingAndTreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub excavator_moving_and_tree_damages_text: Option<String200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: YesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection")]
    pub water_system_protection: YesNoType,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: YesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: YesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
}

