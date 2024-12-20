#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "SedimentationBasinAndDamDimensioningIsFollowed", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: Option<BdtYesNoNotKnownType>,
    #[serde(rename = "SedimentationBasinAndDamDimensioningIsFollowedText", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: Option<BdtString200Type>,
    #[serde(rename = "SedimentationBasinSizeShapeAndFunctionality")]
    pub sedimentation_basin_size_shape_and_functionality: BdtWorkingQualityType,
    #[serde(rename = "SedimentationBasinSizeShapeAndFunctionalityText", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_size_shape_and_functionality_text: Option<BdtString200Type>,
    #[serde(rename = "SuspensionHoleDensitySizeAndLocation")]
    pub suspension_hole_density_size_and_location: BdtWorkingQualityType,
    #[serde(rename = "SuspensionHoleDensitySizeAndLocationText", skip_serializing_if = "Option::is_none")]
    pub suspension_hole_density_size_and_location_text: Option<BdtString200Type>,
    #[serde(rename = "CleaningBreaksDone")]
    pub cleaning_breaks_done: BdtYesNoType,
    #[serde(rename = "CleaningBreaksDoneText", skip_serializing_if = "Option::is_none")]
    pub cleaning_breaks_done_text: Option<BdtString200Type>,
    #[serde(rename = "DitchCleaningBreakLocation")]
    pub ditch_cleaning_break_location: BdtWorkingQualityType,
    #[serde(rename = "DitchCleaningBreakLocationText", skip_serializing_if = "Option::is_none")]
    pub ditch_cleaning_break_location_text: Option<BdtString200Type>,
    #[serde(rename = "SurfaceRunoffFieldGroundIsUnbroken")]
    pub surface_runoff_field_ground_is_unbroken: BdtYesNoNotKnownType,
    #[serde(rename = "SurfaceRunoffFieldGroundIsUnbrokenText", skip_serializing_if = "Option::is_none")]
    pub surface_runoff_field_ground_is_unbroken_text: Option<BdtString200Type>,
    #[serde(rename = "DitchDepthWidthAndDrainageEffect")]
    pub ditch_depth_width_and_drainage_effect: BdtWorkingQualityType,
    #[serde(rename = "DitchDepthWidthAndDrainageEffectText", skip_serializing_if = "Option::is_none")]
    pub ditch_depth_width_and_drainage_effect_text: Option<BdtString200Type>,
    #[serde(rename = "SomeDitchesNotDiggedAsMentionedInOrder")]
    pub some_ditches_not_digged_as_mentioned_in_order: BdtYesNoType,
    #[serde(rename = "SomeDitchesNotDiggedAsMentionedInOrderText", skip_serializing_if = "Option::is_none")]
    pub some_ditches_not_digged_as_mentioned_in_order_text: Option<BdtString200Type>,
    #[serde(rename = "ExcavationSoilLocation")]
    pub excavation_soil_location: BdtWorkingQualityType,
    #[serde(rename = "ExcavationSoilLocationText", skip_serializing_if = "Option::is_none")]
    pub excavation_soil_location_text: Option<BdtString200Type>,
    #[serde(rename = "ExcavatorMovingAndTreeDamages")]
    pub excavator_moving_and_tree_damages: BdtWorkingQualityType,
    #[serde(rename = "ExcavatorMovingAndTreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub excavator_moving_and_tree_damages_text: Option<BdtString200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: BdtYesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: BdtYesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WaterSystemProtection")]
    pub water_system_protection: BdtYesNoType,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: BdtYesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: BdtYesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: BdtFinalAuditQuestionType,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "AuditQuestion")]
    pub audit_question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<BdtFinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: BdtFinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: BdtString20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: BdtString50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: BdtTimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDrainingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

