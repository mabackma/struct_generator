use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTypeType {
    #[serde(rename = "message_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDefinedType {
    #[serde(rename = "not_defined_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatusType {
    #[serde(rename = "working_site_planning_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQualityType {
    #[serde(rename = "stand_quality_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaApprovalType {
    #[serde(rename = "virta_approval_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibilityType {
    #[serde(rename = "forest_depot_accessibility_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraStemTypeType {
    #[serde(rename = "extra_stem_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoninessType {
    #[serde(rename = "restriction_based_on_stoniness_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSuggestionType {
    #[serde(rename = "virta_suggestion_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCodeType {
    #[serde(rename = "habitat_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoType {
    #[serde(rename = "yes_no_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethodType {
    #[serde(rename = "measurement_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRegenerationType {
    #[serde(rename = "virta_regeneration_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationNatureManagementSpecifierType {
    #[serde(rename = "operation_nature_management_specifier_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "sample_plot_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationModeType {
    #[serde(rename = "operation_mode_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTypeType {
    #[serde(rename = "soil_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingOriginType {
    #[serde(rename = "seedling_origin_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PEFCCertificationSystemSubTypeType {
    #[serde(rename = "p_e_f_c_certification_system_sub_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatAdvertisementType {
    #[serde(rename = "virta_habitat_advertisement_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionCategoryType {
    #[serde(rename = "money_transaction_category_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCuttingByMachineType {
    #[serde(rename = "virta_cutting_by_machine_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(rename = "area_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationSubjectType {
    #[serde(rename = "evaluation_subject_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    #[serde(rename = "virta_work_quality_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotKnownType {
    #[serde(rename = "not_known_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatSurvivingType {
    #[serde(rename = "virta_habitat_surviving_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnitType {
    #[serde(rename = "quantity_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationTypeType {
    #[serde(rename = "reported_statistics_operation_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatusType {
    #[serde(rename = "order_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodTypeType {
    #[serde(rename = "used_pricing_method_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionTypeType {
    #[serde(rename = "virta_inspection_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeededType {
    #[serde(rename = "scenery_work_permission_needed_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageSubversionType {
    #[serde(rename = "forest_data_standard_schema_package_subversion_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType3 {
    #[serde(rename = "work_code_qualifier_type3.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectingMethodType {
    #[serde(rename = "collecting_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassType {
    #[serde(rename = "diameter_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceUnitType {
    #[serde(rename = "distance_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingQualityType {
    #[serde(rename = "working_quality_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataMomentType {
    #[serde(rename = "tree_stand_data_moment_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType2 {
    #[serde(rename = "work_code_group_type2.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLocationType {
    #[serde(rename = "point_location_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategoryType {
    #[serde(rename = "spare_tree_category_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationCodeType {
    #[serde(rename = "object_protection_operation_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeUnitType {
    #[serde(rename = "work_code_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurposeType {
    #[serde(rename = "cutting_purpose_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureExtraQualifierType {
    #[serde(rename = "silviculture_extra_qualifier_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroupType {
    #[serde(rename = "financing_act_work_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassType {
    #[serde(rename = "development_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaMastoInspectionType {
    #[serde(rename = "virta_masto_inspection_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestAccessibilityType {
    #[serde(rename = "harvest_accessibility_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClassType {
    #[serde(rename = "difficulty_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineManufacturerType {
    #[serde(rename = "machine_manufacturer_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO639char2LanguageType {
    #[serde(rename = "i_s_o639char2_language_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(rename = "message_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditQuestionType {
    #[serde(rename = "final_audit_question_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroupType {
    #[serde(rename = "main_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType2 {
    #[serde(rename = "work_code_qualifier_type2.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyModeType {
    #[serde(rename = "company_mode_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnitType {
    #[serde(rename = "material_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraTreeSpeciesType {
    #[serde(rename = "extra_tree_species_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemType {
    #[serde(rename = "quality_system_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCodeType {
    #[serde(rename = "machine_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityType {
    #[serde(rename = "priority_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaGroundManipulationMethodType {
    #[serde(rename = "virta_ground_manipulation_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeType {
    #[serde(rename = "forest_centre_work_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageType {
    #[serde(rename = "language_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperationStatusType {
    #[serde(rename = "working_site_planning_operation_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisValueType {
    #[serde(rename = "fee_basis_value_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionType {
    #[serde(rename = "silviculture_restriction_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalTypeType {
    #[serde(rename = "proposal_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeGroupType {
    #[serde(rename = "forest_centre_work_code_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeOfForestObjectType {
    #[serde(rename = "type_of_forest_object_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertiserType {
    #[serde(rename = "virta_advertiser_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPlantEvaluationType {
    #[serde(rename = "virta_plant_evaluation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesConciseType {
    #[serde(rename = "tree_species_concise_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType {
    #[serde(rename = "work_code_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageStateType {
    #[serde(rename = "drainage_state_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlUseCaseType {
    #[serde(rename = "control_use_case_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoProviderRoleType {
    #[serde(rename = "info_provider_role_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClassType {
    #[serde(rename = "drying_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleAssortmentType {
    #[serde(rename = "scale_assortment_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCaseType {
    #[serde(rename = "forest_data_update_use_case_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentGroupType {
    #[serde(rename = "assortment_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroupType {
    #[serde(rename = "forest_owner_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlaceType {
    #[serde(rename = "measurement_place_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeType {
    #[serde(rename = "service_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSCCertificationSystemSubTypeType {
    #[serde(rename = "f_s_c_certification_system_sub_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSumTableAreaType {
    #[serde(rename = "virta_sum_table_area_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderRoleType {
    #[serde(rename = "call_for_offer_business_sender_role_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRestrictionsType {
    #[serde(rename = "other_restrictions_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReviewType {
    #[serde(rename = "virta_review_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetSelectionType {
    #[serde(rename = "virta_target_selection_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPhaseType {
    #[serde(rename = "virta_phase_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionTypeType {
    #[serde(rename = "decision_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeType {
    #[serde(rename = "work_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReasonType {
    #[serde(rename = "virta_reason_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethodType {
    #[serde(rename = "inspection_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPracticeType {
    #[serde(rename = "cutting_realization_practice_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotNeededType {
    #[serde(rename = "yes_no_not_needed_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactSoilTypeType {
    #[serde(rename = "compact_soil_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditAnswerType {
    #[serde(rename = "final_audit_answer_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatePrecipionType {
    #[serde(rename = "date_precipion_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryObjectType {
    #[serde(rename = "decision_geometry_object_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerTypeType {
    #[serde(rename = "final_auditer_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertisementDatingType {
    #[serde(rename = "virta_advertisement_dating_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingTypeType {
    #[serde(rename = "cutting_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionType {
    #[serde(rename = "cutting_restriction_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaEvaluationType {
    #[serde(rename = "virta_evaluation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientApplicationIdType {
    #[serde(rename = "client_application_id_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureIdentifierExtensionType {
    #[serde(rename = "special_feature_identifier_extension_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCodeType {
    #[serde(rename = "financing_act_work_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPerHectareType {
    #[serde(rename = "unit_per_hectare_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResourceType {
    #[serde(rename = "person_resource_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCodeType {
    #[serde(rename = "owner_ship_type_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceTypeType {
    #[serde(rename = "control_reference_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataMessageType {
    #[serde(rename = "forest_centre_data_message_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanlinessClassType {
    #[serde(rename = "cleanliness_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingStemTypeType {
    #[serde(rename = "harvesting_stem_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingExtraQualifierType {
    #[serde(rename = "cutting_extra_qualifier_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClassType {
    #[serde(rename = "fertility_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaLawType {
    #[serde(rename = "virta_law_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodType {
    #[serde(rename = "preferred_contacting_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightResponsibleType {
    #[serde(rename = "using_right_responsible_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeType {
    #[serde(rename = "purchase_mode_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionTypeType {
    #[serde(rename = "restriction_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsibleType {
    #[serde(rename = "forest_use_declaration_responsible_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatusType {
    #[serde(rename = "v_a_t_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCategoryType {
    #[serde(rename = "image_category_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    #[serde(rename = "action_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringUseCaseType {
    #[serde(rename = "self_monitoring_use_case_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(rename = "tree_species_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerType {
    #[serde(rename = "storage_land_owner_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExceptionalPermitForHandlingType {
    #[serde(rename = "virta_exceptional_permit_for_handling_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeType {
    #[serde(rename = "actor_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
    #[serde(rename = "removal_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatTypeType {
    #[serde(rename = "habitat_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStateType {
    #[serde(rename = "operation_state_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestActAreaType {
    #[serde(rename = "forest_act_area_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingClassificationType {
    #[serde(rename = "virta_harvesting_classification_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelectionType {
    #[serde(rename = "target_selection_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClassType {
    #[serde(rename = "terrain_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatusType {
    #[serde(rename = "operation_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    #[serde(rename = "height_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroupType {
    #[serde(rename = "sub_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningDistrictType {
    #[serde(rename = "thinning_district_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCodeType {
    #[serde(rename = "evaluation_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsPurchaseModeType {
    #[serde(rename = "statistics_purchase_mode_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassExtensionsType {
    #[serde(rename = "development_class_extensions_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTotalEstimationType {
    #[serde(rename = "virta_total_estimation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingDirectingType {
    #[serde(rename = "cutting_directing_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityType {
    #[serde(rename = "publicity_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockStatusType {
    #[serde(rename = "previous_block_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceType {
    #[serde(rename = "prevention_substance_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDensityClassType {
    #[serde(rename = "tree_density_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatTypeType {
    #[serde(rename = "virta_habitat_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonOperationExtraQualifierType {
    #[serde(rename = "common_operation_extra_qualifier_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClassType {
    #[serde(rename = "turning_point_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineTypeType {
    #[serde(rename = "machine_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMainGroupType {
    #[serde(rename = "assortment_main_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerTypeType {
    #[serde(rename = "measurer_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraMainGroupType {
    #[serde(rename = "extra_main_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionTypeType {
    #[serde(rename = "money_transaction_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleType {
    #[serde(rename = "vehicle_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeType {
    #[serde(rename = "wood_lot_information_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageVersionType {
    #[serde(rename = "forest_data_standard_schema_package_version_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO3166char2CountryType {
    #[serde(rename = "i_s_o3166char2_country_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSaveIncompleteType {
    #[serde(rename = "virta_save_incomplete_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleType {
    #[serde(rename = "user_role_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocationType {
    #[serde(rename = "habitat_location_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnitType {
    #[serde(rename = "amount_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionMethodType {
    #[serde(rename = "virta_inspection_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateTypeType {
    #[serde(rename = "measurement_certificate_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDryingClassType {
    #[serde(rename = "storage_drying_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentStatusType {
    #[serde(rename = "assortment_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumberType {
    #[serde(rename = "cost_type_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideUnitType {
    #[serde(rename = "wide_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    #[serde(rename = "small_wood_removal_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerGroupType {
    #[serde(rename = "seller_group_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaProjectStatusType {
    #[serde(rename = "virta_project_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRootRotControlEvaluationType {
    #[serde(rename = "virta_root_rot_control_evaluation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClassType {
    #[serde(rename = "document_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatusType {
    #[serde(rename = "working_site_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibilityType {
    #[serde(rename = "harvesting_accessibility_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActFinancingType {
    #[serde(rename = "financing_act_financing_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperationType {
    #[serde(rename = "declaration_soil_preparation_operation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierTypeType {
    #[serde(rename = "identifier_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType1 {
    #[serde(rename = "work_code_qualifier_type1.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(rename = "storey_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyTypeType {
    #[serde(rename = "company_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType4 {
    #[serde(rename = "work_code_qualifier_type4.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityType {
    #[serde(rename = "accessibility_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCodeType {
    #[serde(rename = "peripheral_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSubCategoryType {
    #[serde(rename = "image_sub_category_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCodeType {
    #[serde(rename = "other_habitat_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetPartStatusType {
    #[serde(rename = "virta_target_part_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCodeType {
    #[serde(rename = "language_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperationType {
    #[serde(rename = "declaration_regeneration_operation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationCommitmentType {
    #[serde(rename = "regeneration_commitment_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDaysAreaType {
    #[serde(rename = "degree_days_area_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringTypeType {
    #[serde(rename = "self_monitoring_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCodeType {
    #[serde(rename = "machine_accessory_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifierType {
    #[serde(rename = "forest_damage_qualifier_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeTypeType {
    #[serde(rename = "dead_tree_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(rename = "currency_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
    #[serde(rename = "stem_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperationsType {
    #[serde(rename = "habitat_operations_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingWeightType {
    #[serde(rename = "working_weight_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStringType {
    #[serde(rename = "empty_string_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCodeType {
    #[serde(rename = "material_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreUnitType {
    #[serde(rename = "forest_centre_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryMethodType {
    #[serde(rename = "inventory_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(rename = "main_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluationType {
    #[serde(rename = "preclearing_evaluation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityUnitType {
    #[serde(rename = "statistics_quantity_unit_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeStateType {
    #[serde(rename = "change_state_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(rename = "forest_centre_message_reference_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgencyType {
    #[serde(rename = "operation_urgency_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCodeType {
    #[serde(rename = "assortment_class_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstituteType {
    #[serde(rename = "other_public_substitute_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerResponsible {
    #[serde(rename = "seller_responsible.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceType {
    #[serde(rename = "acceptance_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    #[serde(rename = "virta_damage_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotificationType {
    #[serde(rename = "electronic_notification_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceStatusType {
    #[serde(rename = "business_acceptance_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCultivationMaterialType {
    #[serde(rename = "virta_cultivation_material_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameofAPIType {
    #[serde(rename = "service_nameof_a_p_i_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatCodeType {
    #[serde(rename = "virta_habitat_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationType {
    #[serde(rename = "declaration_other_operation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCodeType {
    #[serde(rename = "reply_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClassType {
    #[serde(rename = "tree_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibilityType {
    #[serde(rename = "transport_accessibility_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditTypeType {
    #[serde(rename = "final_audit_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperationType {
    #[serde(rename = "statistics_operation_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTypeType {
    #[serde(rename = "payment_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateAssortmentRowType {
    #[serde(rename = "measurement_certificate_assortment_row_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataOperationStatusType {
    #[serde(rename = "control_data_operation_status_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureTypeType {
    #[serde(rename = "silviculture_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemType {
    #[serde(rename = "certification_system_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClassType {
    #[serde(rename = "bearing_capacity_class_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaYesNoType {
    #[serde(rename = "virta_yes_no_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    #[serde(rename = "object_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType5 {
    #[serde(rename = "work_code_qualifier_type5.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCodeType {
    #[serde(rename = "grade_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaStandQualityType {
    #[serde(rename = "virta_stand_quality_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementType {
    #[serde(rename = "placement_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(rename = "used_pricing_method_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingSeasonType {
    #[serde(rename = "virta_harvesting_season_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClassType {
    #[serde(rename = "declaration_development_class_type.base")]
    pub base: String,
}

