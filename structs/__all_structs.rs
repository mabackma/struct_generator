#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKolmasRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaGroundManipulationMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtLanguageCode1Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString20Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlUseCaseType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlOverallEvaluationDataType {
    #[serde(rename = "RestrictionBasedOnStoniness", skip_serializing_if = "Option::is_none")]
    pub restriction_based_on_stoniness: Option<CoRestrictionBasedOnStoninessType>,
    #[serde(rename = "PreclearingEvaluation", skip_serializing_if = "Option::is_none")]
    pub preclearing_evaluation: Option<CoPreclearingEvaluationType>,
    #[serde(rename = "DeclarationDeliveringEvaluation", skip_serializing_if = "Option::is_none")]
    pub declaration_delivering_evaluation: Option<CoEvaluationCodeType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<CoEvaluationCodeType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<CoEvaluationCodeType>,
    #[serde(rename = "TreeDamageOutsideStandEvaluation", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand_evaluation: Option<CoEvaluationCodeType>,
    #[serde(rename = "TerrainDamageOutsideStandEvaluation", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand_evaluation: Option<CoEvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinklocatorModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkarcType {
    #[serde(rename = "XlinkarcModel")]
    pub xlinkarc_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary2Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<MeanAgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<CoDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoMeanHeightType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6TotalDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal4And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDatePrecipionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesType {
    #[serde(rename = "OtherTreeSpecies")]
    pub other_tree_species: Vec<OtherTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInformationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoSellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPercentWithFraction1Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDigitPositiveIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistanceType {
    #[serde(flatten)]
    pub base: CoDecimal4And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReasonType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesType {
    #[serde(rename = "Payee")]
    pub payee: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: StemTypeType,
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoExtendedQuantityUnitType,
    #[serde(rename = "AssortmentInfo")]
    pub assortment_info: AssortmentInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementsType {
    #[serde(rename = "Measurement")]
    pub measurement: Vec<MeasurementDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasisType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "FeeBasis")]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkextendedModel {
    #[serde(rename = "xlink_locator")]
    pub xlink_locator: xlink_locator,
    #[serde(rename = "xlink_title")]
    pub xlink_title: xlink_title,
    #[serde(rename = "xlink_arc")]
    pub xlink_arc: xlink_arc,
    #[serde(rename = "xlink_resource")]
    pub xlink_resource: xlink_resource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKatuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfoType {
    #[serde(rename = "CuttingAreaPreclearingNeed", skip_serializing_if = "Option::is_none")]
    pub cutting_area_preclearing_need: Option<CuttingAreaPreclearingNeedType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "StoutTimberClassifier", skip_serializing_if = "Option::is_none")]
    pub stout_timber_classifier: Option<StoutTimberClassifierType>,
    #[serde(rename = "LoggingAccessibility", skip_serializing_if = "Option::is_none")]
    pub logging_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<CoStemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger1digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: ParcelNumberType,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDataType {
    #[serde(rename = "EvaluationCost")]
    pub evaluation_cost: CoDecimal7And2PositiveType,
    #[serde(rename = "PartEastingCoordinate")]
    pub part_easting_coordinate: Xsstring,
    #[serde(rename = "PartNorthingCoordinate")]
    pub part_northing_coordinate: Xsstring,
    #[serde(rename = "InsuranceOrOtherCompensation")]
    pub insurance_or_other_compensation: CoDecimal7And2PositiveType,
    #[serde(rename = "TotalCompensation")]
    pub total_compensation: CoDecimal7And2PositiveType,
    #[serde(rename = "Compensation")]
    pub compensation: CoDecimal7And2PositiveType,
    #[serde(rename = "AlreadyPaidCompensation")]
    pub already_paid_compensation: CoDecimal7And2PositiveType,
    #[serde(rename = "TotalCompensations")]
    pub total_compensations: CoDecimal7And2PositiveType,
    #[serde(rename = "OwnerInvolvement")]
    pub owner_involvement: CoVirtaYesNoType,
    #[serde(rename = "AssociationInvolvement")]
    pub association_involvement: CoVirtaYesNoType,
    #[serde(rename = "MoosePercentage")]
    pub moose_percentage: CoPercentType,
    #[serde(rename = "Class1DamageCount")]
    pub class1_damage_count: CoStemCountType,
    #[serde(rename = "Class2DamageCount")]
    pub class2_damage_count: CoStemCountType,
    #[serde(rename = "Class3DamageCount")]
    pub class3_damage_count: CoStemCountType,
    #[serde(rename = "Class4DamageCount")]
    pub class4_damage_count: CoStemCountType,
    #[serde(rename = "DamagedSeedlingCount")]
    pub damaged_seedling_count: CoStemCountType,
    #[serde(rename = "NotDamagedSeedlingCount")]
    pub not_damaged_seedling_count: CoStemCountType,
    #[serde(rename = "LargePeelDamage")]
    pub large_peel_damage: CoStemCountType,
    #[serde(rename = "SmallPeelDamage")]
    pub small_peel_damage: CoStemCountType,
    #[serde(rename = "NeedForTreatment")]
    pub need_for_treatment: CoVirtaYesNoType,
    #[serde(rename = "RepairPlantingCosts")]
    pub repair_planting_costs: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSubCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionRequirementsGroup {
    #[serde(rename = "LengthMax")]
    pub length_max: LengthMax,
    #[serde(rename = "LengthMin")]
    pub length_min: LengthMin,
    #[serde(rename = "DiameterMin")]
    pub diameter_min: DiameterMin,
    #[serde(rename = "DiameterMax")]
    pub diameter_max: DiameterMax,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum2Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoMeanHeightType>,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<CoSeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnersType {
    #[serde(rename = "ForestOwner")]
    pub forest_owner: Vec<ForestOwnerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "TsTreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "Payments")]
    pub payments: PaymentsType,
    #[serde(rename = "FccDocuments", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeliveryTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwnersType {
    #[serde(rename = "RealEstateOwner")]
    pub real_estate_owner: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurementMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtImageCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
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
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: BdtPositiveInteger3digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "FertilizerVolumeOrdered")]
    pub fertilizer_volume_ordered: BdtPositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasured")]
    pub fertilizer_volume_measured: BdtPositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasuredText", skip_serializing_if = "Option::is_none")]
    pub fertilizer_volume_measured_text: Option<BdtString200Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestUseDeclarationResponsibleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtInteger7digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDataGroup {
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjects>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<Actors>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedData>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StandBasicData>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicData>,
    #[serde(rename = "ControlDataWaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub control_data_water_system_protection: Option<ControlDataWaterSystemProtection>,
    #[serde(rename = "MapSymbol_MapSymbol", skip_serializing_if = "Option::is_none")]
    pub map_symbol__map_symbol: Option<MapSymbol_MapSymbol>,
    #[serde(rename = "SelfMonitoringObjectProtectionOperationsData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_object_protection_operations_data: Option<SelfMonitoringObjectProtectionOperationsData>,
    #[serde(rename = "ControlDataMooseDamageData", skip_serializing_if = "Option::is_none")]
    pub control_data_moose_damage_data: Option<ControlDataMooseDamageData>,
    #[serde(rename = "op_Operations", skip_serializing_if = "Option::is_none")]
    pub op__operations: Option<op_Operations>,
    #[serde(rename = "st_SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st__special_features: Option<st_SpecialFeatures>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicData>,
    #[serde(rename = "workingSiteFinalAuditRoadMaking_WorkingSiteFinalAuditRoadMaking", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_road_making__working_site_final_audit_road_making: Option<workingSiteFinalAuditRoadMaking_WorkingSiteFinalAuditRoadMaking>,
    #[serde(rename = "ts_TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts__tree_stand_data: Option<ts_TreeStandData>,
    #[serde(rename = "workingSiteFinalAuditDraining_WorkingSiteFinalAuditDraining", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_draining__working_site_final_audit_draining: Option<workingSiteFinalAuditDraining_WorkingSiteFinalAuditDraining>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<SoilData>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicData>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignData>,
    #[serde(rename = "ControlOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub control_overall_evaluation_data: Option<ControlOverallEvaluationData>,
    #[serde(rename = "ControlDataRegeneration", skip_serializing_if = "Option::is_none")]
    pub control_data_regeneration: Option<ControlDataRegeneration>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Documents>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicData>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<RestrictionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStemTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtResourceTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoNotNeededType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaWorkQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString10Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolDataType {
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<WctERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<BdtString20Type>,
    #[serde(rename = "MapSymbolName", skip_serializing_if = "Option::is_none")]
    pub map_symbol_name: Option<BdtString20Type>,
    #[serde(rename = "FeatureType", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<BdtFeatureTypeType>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: BdtFeatureCodeType,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<BdtFeatureAdditionalCodeType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<BdtString1000Type>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<BdtString1000Type>,
    #[serde(rename = "Geometry")]
    pub geometry: GdtAlternativeGeometries2Type,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<SfBufferDistanceType>,
    #[serde(rename = "CanModify")]
    pub can_modify: BdtYesNoType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "DitchType", skip_serializing_if = "Option::is_none")]
    pub ditch_type: Option<WctDitchTypeType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<BdtPositiveInteger5digitsType>,
    #[serde(rename = "Depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MaterialCode", skip_serializing_if = "Option::is_none")]
    pub material_code: Option<BdtMaterialCodeType>,
    #[serde(rename = "MaterialInfoText", skip_serializing_if = "Option::is_none")]
    pub material_info_text: Option<BdtString1000Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<BdtString100Type>,
    #[serde(rename = "SpareGroupOfTrees", skip_serializing_if = "Option::is_none")]
    pub spare_group_of_trees: Option<WctSpareTreesByCategoryType>,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAreaPreclearingNeedType {
    #[serde(flatten)]
    pub base: YesNoSellerResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMinType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "CallForOfferWorkingSiteDetails")]
    pub call_for_offer_working_site_details: Vec<CallForOfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeType {
    #[serde(rename = "Header")]
    pub header: HeaderHeaderType,
    #[serde(rename = "Message")]
    pub message: PayloadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScheduleType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger2digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPaymentType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratumType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: CoStratumNumberType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "Age")]
    pub age: AgeType,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: CoMeanHeightType,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<CoSeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingConditionAndQualityType {
    #[serde(flatten)]
    pub base: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandsType {
    #[serde(rename = "FinancingActCompletionStand")]
    pub financing_act_completion_stand: Vec<FinancingActCompletionStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: BdtString20Type,
    #[serde(rename = "NotificationType")]
    pub notification_type: NotificationTypeType,
    #[serde(rename = "RecipientType")]
    pub recipient_type: RecipientTypeType,
    #[serde(rename = "SenderUserId")]
    pub sender_user_id: BdtString20Type,
    #[serde(rename = "SendTimestamp")]
    pub send_timestamp: BdtTimeStampType,
    #[serde(rename = "StatusTimestamp")]
    pub status_timestamp: BdtTimeStampType,
    #[serde(rename = "StatusCode")]
    pub status_code: StatusCodeType,
    #[serde(rename = "OriginalMessage")]
    pub original_message: BdtString1000Type,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<BdtString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkresourceType {
    #[serde(rename = "XlinkresourceModel")]
    pub xlinkresource_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSaveIncompleteType {
    #[serde(flatten)]
    pub base: CoVirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMachineTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrataType {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumeroTyyppi {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadType {
    #[serde(rename = "Acknowledge")]
    pub acknowledge: AcknowledgeAcknowledgeType,
    #[serde(rename = "CompanyInformation")]
    pub company_information: CompanyInformationCompanyInformationType,
    #[serde(rename = "Contract")]
    pub contract: ContractContractType,
    #[serde(rename = "ExternalFile")]
    pub external_file: ExternalFileExternalFileType,
    #[serde(rename = "ForwardingEstimate")]
    pub forwarding_estimate: ForwardingEstimateForwardingEstimateType,
    #[serde(rename = "ForwardingNotification")]
    pub forwarding_notification: ForwardingNotificationForwardingNotificationType,
    #[serde(rename = "HarvestingOrder")]
    pub harvesting_order: HarvestingOrderHarvestingOrderType,
    #[serde(rename = "Image")]
    pub image: ImageImageType,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: MapSymbolMapSymbolType,
    #[serde(rename = "OrderConfirmation")]
    pub order_confirmation: OrderConfirmationOrderConfirmationType,
    #[serde(rename = "ProductInstructionProductInstruction")]
    pub product_instruction_product_instruction: String,
    #[serde(rename = "QualityAttachment")]
    pub quality_attachment: QualityAttachmentQualityAttachmentType,
    #[serde(rename = "Resource")]
    pub resource: ResourceResourceType,
    #[serde(rename = "ResourceSchedule")]
    pub resource_schedule: ResourceScheduleResourceScheduleType,
    #[serde(rename = "SilvicultureOrder")]
    pub silviculture_order: SilvicultureOrderSilvicultureOrderType,
    #[serde(rename = "ServiceBuyerResourceLocations")]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsServiceBuyerResourceLocationsType,
    #[serde(rename = "SmsOperatorStatus")]
    pub sms_operator_status: SmsOperatorStatusSmsOperatorStatusType,
    #[serde(rename = "StanfordFile")]
    pub stanford_file: StanfordFileStanfordFileType,
    #[serde(rename = "UserInformation")]
    pub user_information: UserInformationUserInformationType,
    #[serde(rename = "WeekCalendar")]
    pub week_calendar: WeekCalendarWeekCalendarType,
    #[serde(rename = "WorkingSiteAccounting")]
    pub working_site_accounting: WorkingSiteAccountingWorkingSiteAccountingType,
    #[serde(rename = "WorkingSiteEndNotification")]
    pub working_site_end_notification: WorkingSiteEndNotificationWorkingSiteEndNotificationType,
    #[serde(rename = "WorkingSiteFeeBasis")]
    pub working_site_fee_basis: WorkingSiteFeeBasisWorkingSiteFeeBasisType,
    #[serde(rename = "WorkingSiteFinalAuditBioMassForwarding")]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingWorkingSiteFinalAuditBioMassForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditDraining")]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingWorkingSiteFinalAuditDrainingType,
    #[serde(rename = "WorkingSiteFinalAuditFertilization")]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationWorkingSiteFinalAuditFertilizationType,
    #[serde(rename = "WorkingSiteFinalAuditHarvesting")]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingWorkingSiteFinalAuditHarvestingType,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement")]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementWorkingSiteFinalAuditPlantManagementType,
    #[serde(rename = "WorkingSiteFinalAuditRoadMaking")]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingWorkingSiteFinalAuditRoadMakingType,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture")]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureWorkingSiteFinalAuditSilvicultureType,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning")]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningWorkingSiteFinalAuditSoilConditioningType,
    #[serde(rename = "WorkingSiteFinalAuditStumpForwarding")]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingWorkingSiteFinalAuditStumpForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditStumpLifting")]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingWorkingSiteFinalAuditStumpLiftingType,
    #[serde(rename = "WorkingSiteFinalAuditDynamic")]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicWorkingSiteFinalAuditDynamicType,
    #[serde(rename = "WorkingSiteForwardedProduction")]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionWorkingSiteForwardedProductionType,
    #[serde(rename = "WorkingSiteForwardingQualityControl")]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlWorkingSiteForwardingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestedProduction")]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionWorkingSiteHarvestedProductionType,
    #[serde(rename = "WorkingSiteHarvestingQualityControl")]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlWorkingSiteHarvestingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestingQualityControlManual")]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualWorkingSiteHarvestingQualityControlManualType,
    #[serde(rename = "WorkingSiteOperational")]
    pub working_site_operational: WorkingSiteOperationalWorkingSiteOperationalType,
    #[serde(rename = "WorkingSiteOperationalUpdate")]
    pub working_site_operational_update: WorkingSiteOperationalUpdateWorkingSiteOperationalUpdateType,
    #[serde(rename = "WorkingSiteQualityControlCutting")]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingWorkingSiteQualityControlCuttingType,
    #[serde(rename = "WorkingSiteQualityControlFertilization")]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationWorkingSiteQualityControlFertilizationType,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement")]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementWorkingSiteQualityControlPlantManagementType,
    #[serde(rename = "WorkingSiteQualityControlSilviculture")]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureWorkingSiteQualityControlSilvicultureType,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning")]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningWorkingSiteQualityControlSoilConditioningType,
    #[serde(rename = "WorkingSiteQualityNotification")]
    pub working_site_quality_notification: WorkingSiteQualityNotificationWorkingSiteQualityNotificationType,
    #[serde(rename = "WorkingSiteStatus")]
    pub working_site_status: WorkingSiteStatusWorkingSiteStatusType,
    #[serde(rename = "WorkingSiteTravelNotification")]
    pub working_site_travel_notification: WorkingSiteTravelNotificationWorkingSiteTravelNotificationType,
    #[serde(rename = "WorkingSiteWorkLoad")]
    pub working_site_work_load: WorkingSiteWorkLoadWorkingSiteWorkLoadType,
    #[serde(rename = "WorkingSiteWorkTime")]
    pub working_site_work_time: WorkingSiteWorkTimeWorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType2 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal3And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct showType {
    #[serde(flatten)]
    pub base: Xstoken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "ObjectType")]
    pub object_type: CoDecisionGeometryObjectType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<CoReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "GdtPointLineAndPolygonGeometriesGroup")]
    pub gdt_point_line_and_polygon_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "PaymentReference", skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<PaymentsReferenceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CoBankAccount", skip_serializing_if = "Option::is_none")]
    pub co_bank_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActorsType {
    #[serde(rename = "CompletionDeclarationActor")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionType {
    #[serde(rename = "Tree")]
    pub tree: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralsType {
    #[serde(rename = "PeripheralCode", skip_serializing_if = "Option::is_none")]
    pub peripheral_code: Option<Vec<BdtPeripheralCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLoppuHetkiTyyppi {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String200Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertisementDatingType {
    #[serde(flatten)]
    pub base: CoVirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetType {
    #[serde(rename = "Status2")]
    pub status2: CoChangeStateType,
    #[serde(rename = "TargetId")]
    pub target_id: Xsstring,
    #[serde(rename = "TargetNumber")]
    pub target_number: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
    #[serde(rename = "BasePartNumber")]
    pub base_part_number: VirtaPartNumberType,
    #[serde(rename = "EstablishedPartNumber")]
    pub established_part_number: VirtaPartNumberType,
    #[serde(rename = "TargetAnnouncedAmount")]
    pub target_announced_amount: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "HabitatAdvertisement")]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
    #[serde(rename = "TargetExtraInfo")]
    pub target_extra_info: VirtaExtraInfoType,
    #[serde(rename = "GmlPolygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<String>,
    #[serde(rename = "TargetParts", skip_serializing_if = "Option::is_none")]
    pub target_parts: Option<TargetPartsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKieliKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliersType {
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<SubsidyApplierBaseContactAndEstateInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandKeyGroup2 {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSitePlanningStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulksType {
    #[serde(rename = "StemTypeBulk", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulk: Option<Vec<StemTypeBulkType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLoppuPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummaryType {
    #[serde(rename = "TreeSpeciesData")]
    pub tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoRemovalClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrataType {
    #[serde(rename = "StemDistributionStratum")]
    pub stem_distribution_stratum: Vec<StemDistributionStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBusinessAcceptanceStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccPowerOfAttorneyType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString200Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBasicDataType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<CoUseCaseType>,
    #[serde(rename = "ControlNo", skip_serializing_if = "Option::is_none")]
    pub control_no: Option<CoString100Type>,
    #[serde(rename = "ForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_number: Option<FccForestUseDeclarationNumberType>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<FccFinancingActNumberType>,
    #[serde(rename = "ControlReferenceType", skip_serializing_if = "Option::is_none")]
    pub control_reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "ControlReference", skip_serializing_if = "Option::is_none")]
    pub control_reference: Option<CoReferenceType>,
    #[serde(rename = "ControlDate", skip_serializing_if = "Option::is_none")]
    pub control_date: Option<CoDateType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<CoTargetSelectionType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<CoString500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctHopperTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoTotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExceptionalPermitForHandlingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedStatisticsOperationTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoInspectionMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestDataUpdateType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendarType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer3digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkinTyyppi {
    #[serde(flatten)]
    pub base: Xsboolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasicDataType {
    #[serde(rename = "@timeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<TwoDigitPositiveIntegerType>,
    #[serde(rename = "SamplePlotSize", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size: Option<SamplePlotSizeType>,
    #[serde(rename = "SamplePlotSizeTreeReduction", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size_tree_reduction: Option<SamplePlotSizeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHarvestingClassificationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingPurposeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiNameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: OrganizationName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationsType {
    #[serde(rename = "Organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<OrganizationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsType {
    #[serde(rename = "Attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreementsType {
    #[serde(rename = "CollectiveAgreement")]
    pub collective_agreement: Vec<BdtString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferWoodTradeInfoType {
    #[serde(rename = "PurchaseMode")]
    pub purchase_mode: PurchaseModeType,
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
    #[serde(rename = "IncludeForestFundPayment")]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
    #[serde(rename = "UsedPricingMethods", skip_serializing_if = "Option::is_none")]
    pub used_pricing_methods: Option<UsedPricingMethodsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSaveIncompleteType {
    #[serde(flatten)]
    pub base: CoVirtaSaveIncompleteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideDevelopmentClassType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CuttingPlannerLiability", skip_serializing_if = "Option::is_none")]
    pub cutting_planner_liability: Option<CuttingPlannerLiabilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionsType {
    #[serde(rename = "CaseAction")]
    pub case_action: Vec<CaseActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<Vec<FeeBasisDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBearingCapacityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionAsUnitGroup {
    #[serde(rename = "TotalValue", skip_serializing_if = "Option::is_none")]
    pub total_value: Option<TotalValue>,
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(rename = "QuantityUnit", skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<QuantityUnit>,
    #[serde(rename = "UnitValue", skip_serializing_if = "Option::is_none")]
    pub unit_value: Option<UnitValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNameType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedWideUnitType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTargetPartStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSilvicultureBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributesType {
    #[serde(rename = "TreeSpeciesAttribute", skip_serializing_if = "Option::is_none")]
    pub tree_species_attribute: Option<Vec<TreeSpeciesAttributeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RwsRoundWoodSalesRows", skip_serializing_if = "Option::is_none")]
    pub rws_round_wood_sales_rows: Option<String>,
    #[serde(rename = "OperationRows", skip_serializing_if = "Option::is_none")]
    pub operation_rows: Option<OperationRowsType>,
    #[serde(rename = "LoggingsRows", skip_serializing_if = "Option::is_none")]
    pub loggings_rows: Option<LoggingsRowsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeSpecifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAndOriginalYearGroup {
    #[serde(rename = "ProposalYear")]
    pub proposal_year: ProposalYear,
    #[serde(rename = "OriginalProposalYear", skip_serializing_if = "Option::is_none")]
    pub original_proposal_year: Option<OriginalProposalYear>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeType {
    #[serde(rename = "WorkingCode")]
    pub working_code: BdtWorkCodeType,
    #[serde(rename = "Amount")]
    pub amount: BdtDecimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<BdtDifficultyClassType>,
    #[serde(rename = "Attribute1", skip_serializing_if = "Option::is_none")]
    pub attribute1: Option<Vec<BdtWorkCodeQualifierType1>>,
    #[serde(rename = "Attribute2", skip_serializing_if = "Option::is_none")]
    pub attribute2: Option<Vec<BdtWorkCodeQualifierType2>>,
    #[serde(rename = "Attribute3", skip_serializing_if = "Option::is_none")]
    pub attribute3: Option<Vec<BdtWorkCodeQualifierType3>>,
    #[serde(rename = "Attribute4", skip_serializing_if = "Option::is_none")]
    pub attribute4: Option<Vec<BdtWorkCodeQualifierType4>>,
    #[serde(rename = "Attribute5", skip_serializing_if = "Option::is_none")]
    pub attribute5: Option<Vec<BdtWorkCodeQualifierType5>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString100Type,
    #[serde(rename = "StartDate")]
    pub start_date: BdtTimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtTimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: BdtTimeStampType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<BdtString100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Xsbase64Binary>,
    #[serde(rename = "StemTypeVolumes")]
    pub stem_type_volumes: StemTypeVolumesType,
    #[serde(rename = "AssortmentVolumes")]
    pub assortment_volumes: AssortmentVolumesType,
    #[serde(rename = "AssortmentVolumesUnclassified", skip_serializing_if = "Option::is_none")]
    pub assortment_volumes_unclassified: Option<AssortmentVolumesUnclassifiedType>,
    #[serde(rename = "AssortmentMatrixVolumes")]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
    #[serde(rename = "ProductUserIds")]
    pub product_user_ids: Vec<ProductUserIdsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaReasonType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDiameterClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@operationId")]
    pub operation_id: Xsstring,
    #[serde(rename = "@standId")]
    pub stand_id: Xsstring,
    #[serde(rename = "@productId")]
    pub product_id: Xsstring,
    #[serde(rename = "ProductKeyGroup")]
    pub product_key_group: String,
    #[serde(rename = "ProductName")]
    pub product_name: CoString500Type,
    #[serde(rename = "Quantity")]
    pub quantity: CoDecimal2FractionDigitsType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoWideUnitType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<WtcTotalPriceType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "Consumption", skip_serializing_if = "Option::is_none")]
    pub consumption: Option<ConsumptionType>,
    #[serde(rename = "ConsumptionUnit", skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<ConsumptionUnitType>,
    #[serde(rename = "PlannedResource", skip_serializing_if = "Option::is_none")]
    pub planned_resource: Option<WtcPlannedResourceType>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: BdtString20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ContractCode", skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<BdtString50Type>,
    #[serde(rename = "ValidityDateBegin")]
    pub validity_date_begin: BdtDateType,
    #[serde(rename = "ValidityDateEnd", skip_serializing_if = "Option::is_none")]
    pub validity_date_end: Option<BdtDateType>,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "ContractInfo", skip_serializing_if = "Option::is_none")]
    pub contract_info: Option<BdtString1000Type>,
    #[serde(rename = "MeasureDeviceCheckRequired")]
    pub measure_device_check_required: BdtYesNoType,
    #[serde(rename = "CompanyMode", skip_serializing_if = "Option::is_none")]
    pub company_mode: Option<BdtCompanyModeType>,
    #[serde(rename = "SubContractorsAllowed")]
    pub sub_contractors_allowed: BdtYesNoType,
    #[serde(rename = "WorkingAreas", skip_serializing_if = "Option::is_none")]
    pub working_areas: Option<WorkingAreasType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHarvestingAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonNameGroup {
    #[serde(rename = "WholeName")]
    pub whole_name: WholeName,
    #[serde(rename = "PersonOrganizationName", skip_serializing_if = "Option::is_none")]
    pub person_organization_name: Option<PersonOrganizationName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType2 {
    #[serde(flatten)]
    pub base: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3_1Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "LocationTimestamp")]
    pub location_timestamp: BdtTimeStampType,
    #[serde(rename = "Location")]
    pub location: GdtPointGeometryType,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WctWorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<BdtString50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<BdtPositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationUrgencyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaMastoInspectionType {
    #[serde(flatten)]
    pub base: CoVirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYearType {
    #[serde(flatten)]
    pub base: XsgYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPointType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6_2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType3 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDataType {
    #[serde(rename = "StemId")]
    pub stem_id: BdtPositiveIntegerType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<BdtHarvestingStemTypeType>,
    #[serde(rename = "SelectionType")]
    pub selection_type: ControlStemSelectionType,
    #[serde(rename = "RandomControlStemRejectedReason", skip_serializing_if = "Option::is_none")]
    pub random_control_stem_rejected_reason: Option<BdtString100Type>,
    #[serde(rename = "StemCoordinates", skip_serializing_if = "Option::is_none")]
    pub stem_coordinates: Option<GdtPointGeometryType>,
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString100Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusDataType {
    #[serde(rename = "PreventionCompleted", skip_serializing_if = "Option::is_none")]
    pub prevention_completed: Option<CoYesNoType>,
    #[serde(rename = "PreventionSubstance", skip_serializing_if = "Option::is_none")]
    pub prevention_substance: Option<CoPreventionSubstanceType>,
    #[serde(rename = "PreventionSubstanceProductName", skip_serializing_if = "Option::is_none")]
    pub prevention_substance_product_name: Option<CoString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSumTableAreaType {
    #[serde(flatten)]
    pub base: CoVirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TstTreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeatureCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "OfferWorkingSiteDetails")]
    pub offer_working_site_details: Vec<OfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType2 {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber")]
    pub location_municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<CoMunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberType {
    #[serde(flatten)]
    pub base: JhsPuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactSoilTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandId")]
    pub stand_id: BdtString20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKansalaisuusKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEdellinenSukuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctWorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "@DeliveryUserId")]
    pub delivery_user_id: BdtString50Type,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: Vec<BdtString50Type>,
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaPhaseType {
    #[serde(flatten)]
    pub base: CoVirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctAreaCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityType {
    #[serde(flatten)]
    pub base: Xsstring,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFinancingActWorkGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
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
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: WctERPIdType,
    #[serde(rename = "QuestionAsText", skip_serializing_if = "Option::is_none")]
    pub question_as_text: Option<BdtString200Type>,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: BdtString50Type,
    #[serde(rename = "QuestionAnswerAsText", skip_serializing_if = "Option::is_none")]
    pub question_answer_as_text: Option<BdtString50Type>,
    #[serde(rename = "QuestionAnswerAdditionalText", skip_serializing_if = "Option::is_none")]
    pub question_answer_additional_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<FccLocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<BdtFinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: BdtFinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: BdtString50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: BdtTimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicDataType>,
    #[serde(rename = "ControlForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub control_forest_use_declaration: Option<ControlForestUseDeclarationType>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicDataType>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicDataType>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<DamageDataType>,
    #[serde(rename = "ControlDataSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub control_data_special_features: Option<ControlDataSpecialFeaturesType>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignDataType>,
    #[serde(rename = "ObjectOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub object_overall_evaluation_data: Option<ControlOverallEvaluationDataType>,
    #[serde(rename = "ControlEvaluations", skip_serializing_if = "Option::is_none")]
    pub control_evaluations: Option<ControlEvaluationsType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType1 {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@realEstateId")]
    pub real_estate_id: Xsstring,
    #[serde(rename = "@parcelId")]
    pub parcel_id: Xsstring,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataWithGeometryType,
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<String>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "OpPlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub op_planned_operation_chains: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorsType {
    #[serde(rename = "Actor")]
    pub actor: Vec<ActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "MapSymbols")]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "CostType")]
    pub cost_type: FccCostTypeType2,
    #[serde(rename = "CostTypeDescription")]
    pub cost_type_description: FccCostTypeDescriptionType,
    #[serde(rename = "DecidedAmount", skip_serializing_if = "Option::is_none")]
    pub decided_amount: Option<FccDecidedAmountType>,
    #[serde(rename = "DecidedAmountUnit", skip_serializing_if = "Option::is_none")]
    pub decided_amount_unit: Option<FccDecidedAmountUnitType>,
    #[serde(rename = "DecidedTotalSubsidy")]
    pub decided_total_subsidy: FccDecidedTotalSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteStateType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtThinningDistrictType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditerTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingWeightType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerType {
    #[serde(rename = "PersonId", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<BdtString20Type>,
    #[serde(rename = "OrganisationId", skip_serializing_if = "Option::is_none")]
    pub organisation_id: Option<BdtString20Type>,
    #[serde(rename = "PersonRole", skip_serializing_if = "Option::is_none")]
    pub person_role: Option<BdtString50Type>,
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: BdtString20Type,
    #[serde(rename = "EmailAddress")]
    pub email_address: BdtString50Type,
    #[serde(rename = "LanguageCode")]
    pub language_code: BdtLanguageCodeType,
    #[serde(rename = "SendWorkingAloneNotification", skip_serializing_if = "Option::is_none")]
    pub send_working_alone_notification: Option<BdtYesNoType>,
    #[serde(rename = "SendNotifications", skip_serializing_if = "Option::is_none")]
    pub send_notifications: Option<BdtYesNoType>,
    #[serde(rename = "NotificationContactPerson", skip_serializing_if = "Option::is_none")]
    pub notification_contact_person: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSoilTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType4 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherPublicSubstituteType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccForestUseDeclarationNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CddDistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMessageTypeType {
    #[serde(flatten)]
    pub base: WtcoMessageTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDistanceUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct typeType {
    #[serde(flatten)]
    pub base: Xstoken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwsWorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WctWorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<BdtString50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<BdtPositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourcesType {
    #[serde(rename = "AuditionResource", skip_serializing_if = "Option::is_none")]
    pub audition_resource: Option<AuditionResourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OrganizationName")]
    pub organization_name: BdtString100Type,
    #[serde(rename = "RepresentativePerson")]
    pub representative_person: BdtContactInformationType,
    #[serde(rename = "ContactPersonInFinland")]
    pub contact_person_in_finland: BdtContactInformationType,
    #[serde(rename = "CompanyType")]
    pub company_type: BdtCompanyTypeType,
    #[serde(rename = "QualitySystems", skip_serializing_if = "Option::is_none")]
    pub quality_systems: Option<QualitySystemsType>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
    #[serde(rename = "TradeRegistration", skip_serializing_if = "Option::is_none")]
    pub trade_registration: Option<BdtDateType>,
    #[serde(rename = "TaxDebt")]
    pub tax_debt: BdtDateType,
    #[serde(rename = "EmployeePensionCertificate", skip_serializing_if = "Option::is_none")]
    pub employee_pension_certificate: Option<BdtDateType>,
    #[serde(rename = "EmployeeHealthCare")]
    pub employee_health_care: BdtYesNoType,
    #[serde(rename = "EmployeeHealthCareInfo", skip_serializing_if = "Option::is_none")]
    pub employee_health_care_info: Option<BdtString100Type>,
    #[serde(rename = "CollectiveAgreements")]
    pub collective_agreements: CollectiveAgreementsType,
    #[serde(rename = "ServiceTypes")]
    pub service_types: BdtServiceTypesType,
    #[serde(rename = "LiabilityInsurance", skip_serializing_if = "Option::is_none")]
    pub liability_insurance: Option<BdtDateType>,
    #[serde(rename = "LegalAccidentInsurance", skip_serializing_if = "Option::is_none")]
    pub legal_accident_insurance: Option<BdtDateType>,
    #[serde(rename = "SubContractorWrittenAgreement")]
    pub sub_contractor_written_agreement: BdtYesNoType,
    #[serde(rename = "EmployeeWrittenAgreement")]
    pub employee_written_agreement: BdtYesNoType,
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: BdtYesNoType,
    #[serde(rename = "PreDebtCollectionRegister")]
    pub pre_debt_collection_register: BdtYesNoType,
    #[serde(rename = "EmployerRegister")]
    pub employer_register: EmployerRegisterType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BdtString50Type>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<BdtString20Type>,
    #[serde(rename = "IsActive")]
    pub is_active: BdtYesNoType,
    #[serde(rename = "SubContractors", skip_serializing_if = "Option::is_none")]
    pub sub_contractors: Option<SubContractorsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaReviewType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMaxType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDamageQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecisionTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax5IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString2000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger2digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: String,
    #[serde(rename = "GmlMultiSurface")]
    pub gml_multi_surface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger2digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTypeType {
    #[serde(flatten)]
    pub base: WtcoMessageTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<BdtString20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AsAssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType4 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureType {
    #[serde(rename = "SfFeatureDataGroup")]
    pub sf_feature_data_group: String,
    #[serde(rename = "SfFeatureInfo", skip_serializing_if = "Option::is_none")]
    pub sf_feature_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeDaysAreaType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleEltType {
    #[serde(rename = "XlinktitleModel")]
    pub xlinktitle_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaExceptionalPermitForHandlingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extendedModel {
    #[serde(rename = "xlink_locator")]
    pub xlink_locator: xlink_locator,
    #[serde(rename = "xlink_title")]
    pub xlink_title: xlink_title,
    #[serde(rename = "xlink_arc")]
    pub xlink_arc: xlink_arc,
    #[serde(rename = "xlink_resource")]
    pub xlink_resource: xlink_resource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtHarvestingStemTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegisterType {
    #[serde(rename = "Registered")]
    pub registered: BdtYesNoType,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<BdtDateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: StbStandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "FacAreaNo")]
    pub fac_area_no: String,
    #[serde(rename = "FacFinancingActWorkCode")]
    pub fac_financing_act_work_code: String,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeType {
    #[serde(rename = "OwnerShipTypeCode")]
    pub owner_ship_type_code: CoOwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<CoString500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoppersType {
    #[serde(rename = "Hopper", skip_serializing_if = "Option::is_none")]
    pub hopper: Option<Vec<HopperType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPerHectareType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "SubsidyApplierReference")]
    pub subsidy_applier_reference: Vec<SubsidyApplierReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtSimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatePrecipionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointType {
    #[serde(rename = "Diameter")]
    pub diameter: DiameterType,
    #[serde(rename = "CumulativeMass")]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "SelfMonitoringBasicData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_basic_data: Option<SelfMonitoringBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "Works", skip_serializing_if = "Option::is_none")]
    pub works: Option<WorksType>,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_plant_management: Option<WorkingSiteFinalAuditPlantManagementSelfMonitoringWorkingSiteFinalAuditPlantManagementType>,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_plant_management: Option<WorkingSiteQualityControlPlantManagementSelfMonitoringWorkingSiteQualityControlPlantManagementType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageSelfMonitoringImageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoltosuhdeTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceType {
    #[serde(rename = "SubsidyApplierId")]
    pub subsidy_applier_id: CoIdStringNotEmptyType,
    #[serde(rename = "MainApplier")]
    pub main_applier: CoYesNoType,
    #[serde(rename = "ShareOfOwnerShip", skip_serializing_if = "Option::is_none")]
    pub share_of_owner_ship: Option<CoPercentType>,
    #[serde(rename = "OwnerShipType")]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActGeometryType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "FeatureCode")]
    pub feature_code: CoFeatureCodeType,
    #[serde(rename = "FeatureInfo")]
    pub feature_info: CoString500Type,
    #[serde(rename = "GdtPointAndLineGeometriesGroup")]
    pub gdt_point_and_line_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCaseType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeibullType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferType {
    #[serde(rename = "RelatedCallForOfferId")]
    pub related_call_for_offer_id: Xsstring,
    #[serde(rename = "RelatedCallForOfferDescription", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offer_description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationType {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: BdtTimeStampType,
    #[serde(rename = "CalibrationAdjustment")]
    pub calibration_adjustment: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonsType {
    #[serde(rename = "DeclarationPolygon")]
    pub declaration_polygon: Vec<DeclarationPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsStatusryhmaTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaGroup {
    #[serde(rename = "ProposalAreaPercent", skip_serializing_if = "Option::is_none")]
    pub proposal_area_percent: Option<ProposalAreaPercent>,
    #[serde(rename = "ProposalArea", skip_serializing_if = "Option::is_none")]
    pub proposal_area: Option<ProposalArea>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoadType {
    #[serde(rename = "PartitialLoadId")]
    pub partitial_load_id: XsnonNegativeInteger,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "LoadVolume", skip_serializing_if = "Option::is_none")]
    pub load_volume: Option<BdtDecimal3FractionDigitsType>,
    #[serde(rename = "LoadGreenMass")]
    pub load_green_mass: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSiteNameType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TliTreeListItemType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "TreeNumber", skip_serializing_if = "Option::is_none")]
    pub tree_number: Option<Xsinteger>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<CoStoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoAgeType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<CoDiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<CoMeanHeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<CoVolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TreeIdentifiers", skip_serializing_if = "Option::is_none")]
    pub tree_identifiers: Option<TreeIdentifiersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtImageSubCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetaType {
    #[serde(rename = "ShapeAlfa")]
    pub shape_alfa: ShapeAlfaType,
    #[serde(rename = "ShapeBeta")]
    pub shape_beta: ShapeBetaType,
    #[serde(rename = "Minimum")]
    pub minimum: MinimumType,
    #[serde(rename = "Maximum")]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDataType {
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<BdtString100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "ResourceName")]
    pub resource_name: BdtString50Type,
    #[serde(rename = "WorkingHoursBusinessDay")]
    pub working_hours_business_day: BdtPositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSaturday")]
    pub working_hours_saturday: BdtPositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSunday")]
    pub working_hours_sunday: BdtPositiveInteger2digitsType,
    #[serde(rename = "Days")]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCultivationMaterialType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaYesNoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTimeStampType {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "GdtAlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerArea")]
    pub service_buyer_area: BdtString20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<BdtString50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: BdtString20Type,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointLineAndPolygonGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringDataType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: CoYesNoType,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDryingClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataGroup {
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClass>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<StandQuality>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<MainTreeSpecies>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTagType {
    #[serde(flatten)]
    pub base: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassesType {
    #[serde(rename = "AssortmentClass")]
    pub assortment_class: Vec<AssortmentClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsYritysTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionSubstanceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandBasicDataType {
    #[serde(rename = "ControlStandArea", skip_serializing_if = "Option::is_none")]
    pub control_stand_area: Option<CoAreaType>,
    #[serde(rename = "InspectionMethod", skip_serializing_if = "Option::is_none")]
    pub inspection_method: Option<CoInspectionMethodType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleOfPreClearingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNoType {
    #[serde(flatten)]
    pub base: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawinghoursDataType {
    #[serde(rename = "Hours")]
    pub hours: BdtPositiveInteger2digitsType,
    #[serde(rename = "Minutes")]
    pub minutes: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteNumeroTyyppi {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSukuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratumType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "SeedlingBeginningCode", skip_serializing_if = "Option::is_none")]
    pub seedling_beginning_code: Option<CoSeedlingOriginType>,
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountInSpot", skip_serializing_if = "Option::is_none")]
    pub amount_in_spot: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountOutsideSpot", skip_serializing_if = "Option::is_none")]
    pub amount_outside_spot: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "AmountUnit", skip_serializing_if = "Option::is_none")]
    pub amount_unit: Option<CoAmountUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctResponsibleOfPreClearingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaDamageClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "Explanation")]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal7And2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageSubversionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpCuttingTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "StandAvgDiameterSummary")]
    pub stand_avg_diameter_summary: BdtPositiveInteger3digitsType,
    #[serde(rename = "StandAvgHeightSummary")]
    pub stand_avg_height_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "StandAvgDominantHeight", skip_serializing_if = "Option::is_none")]
    pub stand_avg_dominant_height: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StandAvgAgeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_age_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StandAvgStemCountSummary")]
    pub stand_avg_stem_count_summary: BdtPositiveInteger5digitsType,
    #[serde(rename = "StandBasalAreaSummary", skip_serializing_if = "Option::is_none")]
    pub stand_basal_area_summary: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StandVolumeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_volume_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StandStemDamagesPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_stem_damages_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandRootDamagesPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_root_damages_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandCorrectHeightStumpsPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_correct_height_stumps_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandTooHeightStumpsPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_too_height_stumps_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandVehiclePathTooDeepPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_vehicle_path_too_deep_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandAvgVehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub stand_avg_vehicle_path_distance: Option<BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "StandVehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub stand_vehicle_path_width: Option<BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TreeSummaries")]
    pub tree_summaries: TreeSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKutsumaNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    #[serde(flatten)]
    pub base: StbStandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesConciseType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferType {
    #[serde(rename = "@callForOfferId")]
    pub call_for_offer_id: Xsstring,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@versionNo")]
    pub version_no: Xsint,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "OfferBusinessSender")]
    pub offer_business_sender: OfferBusinessSenderType,
    #[serde(rename = "CallForOfferBusinessSender", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_business_sender: Option<WtcoCallForOfferBusinessSenderType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "OfferDate")]
    pub offer_date: OfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "OfferText", skip_serializing_if = "Option::is_none")]
    pub offer_text: Option<OfferTextType>,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<WtcoCallForOfferTextType>,
    #[serde(rename = "WsOfferWorkingSites")]
    pub ws_offer_working_sites: String,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: ReRealEstatesWithOwnersInformationType2,
    #[serde(rename = "ProcessingAreas")]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct roleType {
    #[serde(flatten)]
    pub base: XsanyURI,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameofAPIType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TssTreeStandSummary2Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<MeanAgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<CoDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoMeanHeightType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMainGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectsType {
    #[serde(rename = "GeometryObject")]
    pub geometry_object: Vec<GeometryObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger5digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctFinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<BdtPositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstAndLastNameGroup {
    #[serde(rename = "FirstName")]
    pub first_name: FirstName,
    #[serde(rename = "LastName")]
    pub last_name: LastName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<WctERPIdType>,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "Name")]
    pub name: BdtString50Type,
    #[serde(rename = "CodeGroup")]
    pub code_group: BdtAssortmentGroupType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<BdtTreeSpeciesType>,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<BdtStemTypeType>,
    #[serde(rename = "Quality")]
    pub quality: BdtString5Type,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<BdtWorkCodeUnitType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "MeasurementMethod")]
    pub measurement_method: BdtMeasurementMethodType,
    #[serde(rename = "MeasurementPlace")]
    pub measurement_place: BdtMeasurementPlaceType,
    #[serde(rename = "DiameterMin", skip_serializing_if = "Option::is_none")]
    pub diameter_min: Option<BdtPositiveIntegerType>,
    #[serde(rename = "DiameterMax", skip_serializing_if = "Option::is_none")]
    pub diameter_max: Option<BdtPositiveIntegerType>,
    #[serde(rename = "HeightMin", skip_serializing_if = "Option::is_none")]
    pub height_min: Option<BdtDecimal1FractionDigitType>,
    #[serde(rename = "HeightMax", skip_serializing_if = "Option::is_none")]
    pub height_max: Option<BdtDecimal1FractionDigitType>,
    #[serde(rename = "CanModify", skip_serializing_if = "Option::is_none")]
    pub can_modify: Option<BdtYesNoType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<BdtString200Type>,
    #[serde(rename = "DeliveryUserId", skip_serializing_if = "Option::is_none")]
    pub delivery_user_id: Option<BdtString50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstatesType {
    #[serde(rename = "PayeeAndRealEstate")]
    pub payee_and_real_estate: Vec<PayeeAndRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourceType {
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfowsWorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WctWorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<BdtString50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<BdtPositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembersType {
    #[serde(rename = "WorkGrouMember")]
    pub work_grou_member: Vec<WorkGrouMemberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectsType {
    #[serde(rename = "ChildObject")]
    pub child_object: Vec<ChildObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolsType {
    #[serde(rename = "Symbol")]
    pub symbol: Vec<MapSymbolDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaPlantEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(rename = "ChildObjectType")]
    pub child_object_type: CoObjectTypeType,
    #[serde(rename = "ChildObjectTypeSpecifier", skip_serializing_if = "Option::is_none")]
    pub child_object_type_specifier: Option<ObjectTypeSpecifierType>,
    #[serde(rename = "ChildObjectId")]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsType {
    #[serde(rename = "SubContractor", skip_serializing_if = "Option::is_none")]
    pub sub_contractor: Option<Vec<BdtString20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreForestDataUpdateObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1500Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString200Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageDataType {
    #[serde(rename = "ReferenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<CoReferenceType>,
    #[serde(rename = "KeyElementNS", skip_serializing_if = "Option::is_none")]
    pub key_element_n_s: Option<CoString500Type>,
    #[serde(rename = "KeyElementName", skip_serializing_if = "Option::is_none")]
    pub key_element_name: Option<CoString200Type>,
    #[serde(rename = "KeyElementId", skip_serializing_if = "Option::is_none")]
    pub key_element_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "KeyInfoAsText", skip_serializing_if = "Option::is_none")]
    pub key_info_as_text: Option<CoString2000Type>,
    #[serde(rename = "ErrorCode")]
    pub error_code: CoString25Type,
    #[serde(rename = "ErrorMessage")]
    pub error_message: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItemType {
    #[serde(rename = "ListId")]
    pub list_id: BdtPositiveIntegerType,
    #[serde(rename = "ListItem")]
    pub list_item: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeSpecifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoSellerResponsibleType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperationsType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiversType {
    #[serde(rename = "DecisionReceiver")]
    pub decision_receiver: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTotalEstimationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctCommonMessageType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationUrgencyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaCultivationMaterialType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostilokeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTerrainClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiFirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctStanfordTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationDataType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: Xsstring,
    #[serde(rename = "GeometryObjects")]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativesType {
    #[serde(rename = "WorkingRepresentative")]
    pub working_representative: Vec<WorkingRepresentativeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaMastoInspectionType {
    #[serde(flatten)]
    pub base: CoVirtaMastoInspectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandsType {
    #[serde(rename = "FinancingActApplicationStand")]
    pub financing_act_application_stand: Vec<FinancingActApplicationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignDataType {
    #[serde(rename = "RootDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub root_damage_percentage: Option<CoPositiveIntegerType>,
    #[serde(rename = "StemDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub stem_damage_percentage: Option<CoPositiveIntegerType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathMeanDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_mean_distance: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathMeanWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_mean_width: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathSubsidenceLength", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_length: Option<Decimal3_1Type>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<Decimal3_1Type>,
    #[serde(rename = "CuttingBy", skip_serializing_if = "Option::is_none")]
    pub cutting_by: Option<CoVirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<CoVirtaHarvestingSeasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaProjectStatusType {
    #[serde(flatten)]
    pub base: CoVirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CddDistributionModelGroup {
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSB,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistribution,
    #[serde(rename = "Weibull")]
    pub weibull: Weibull,
    #[serde(rename = "Normal")]
    pub normal: Normal,
    #[serde(rename = "Gamma")]
    pub gamma: Gamma,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExtraInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStemTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "Explanation")]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessagesType {
    #[serde(rename = "CommonMessage", skip_serializing_if = "Option::is_none")]
    pub common_message: Option<Vec<CommonMessageDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlCuttingType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlBaseCuttingType,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<BdtDateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: NotEmptyTreeSpeciesType,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "SawlogPercent", skip_serializing_if = "Option::is_none")]
    pub sawlog_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "TotalSawlogVolume", skip_serializing_if = "Option::is_none")]
    pub total_sawlog_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "TotalPulpwoodVolume", skip_serializing_if = "Option::is_none")]
    pub total_pulpwood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TotalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<CoVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectKeys", skip_serializing_if = "Option::is_none")]
    pub object_keys: Option<ObjectKeysType>,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingOriginType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeStateType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebaseListItemType {
    #[serde(rename = "Id")]
    pub id: BdtPositiveIntegerType,
    #[serde(rename = "FeeValue")]
    pub fee_value: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO3166char2CountryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<BdtString20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<BdtString50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: BdtString20Type,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "BeginDate")]
    pub begin_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandNumberType {
    #[serde(flatten)]
    pub base: StbStandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingDataType {
    #[serde(rename = "SeedlingStratum")]
    pub seedling_stratum: Vec<SeedlingStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal2FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSiviilisaatyTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "BusinessId")]
    pub business_id: Xsstring,
    #[serde(rename = "OrganizationName")]
    pub organization_name: Xsstring,
    #[serde(rename = "Roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RolesType>>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServicesType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfoType {
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<CoHarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<CoStemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationsType {
    #[serde(rename = "ResourceLocation")]
    pub resource_location: Vec<ResourceLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometryType {
    #[serde(rename = "Id")]
    pub id: Xsstring,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<Xsstring>,
    #[serde(rename = "GmlLineString")]
    pub gml_line_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal7And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<Vec<MaterialType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMaxType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantitiesType {
    #[serde(rename = "StatisticsQuantity")]
    pub statistics_quantity: Vec<StatisticsQuantityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtQualitySystemType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCertificationSystemType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureExtraQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatAdvertisementType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyTransactionTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockInfoType {
    #[serde(rename = "PreviousBlock", skip_serializing_if = "Option::is_none")]
    pub previous_block: Option<Vec<BdtString20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffersType {
    #[serde(rename = "RelatedCallForOffer")]
    pub related_call_for_offer: Vec<RelatedCallForOfferType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationsType {
    #[serde(rename = "ObjectProtectionOperation")]
    pub object_protection_operation: Vec<ObjectProtectionOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaYesNoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DifficultyClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoOfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeCuttingType {
    #[serde(rename = "CodeGroup")]
    pub code_group: BdtAssortmentGroupType,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<BdtPositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifierType {
    #[serde(rename = "Type")]
    pub r#type: Xsinteger,
    #[serde(rename = "Value")]
    pub value: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditQuestionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlkuPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainWorkCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPeripheralCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtIdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String100Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateType {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xmimebase64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaApprovalType {
    #[serde(flatten)]
    pub base: CoVirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataMomentType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformationType {
    #[serde(rename = "FSFNumber", skip_serializing_if = "Option::is_none")]
    pub f_s_f_number: Option<BdtString50Type>,
    #[serde(rename = "FSFValidity", skip_serializing_if = "Option::is_none")]
    pub f_s_f_validity: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString500Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsCareOfTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingWeightType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType2 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDryingClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionMainGroup {
    #[serde(rename = "SilvicultureRestrictions", skip_serializing_if = "Option::is_none")]
    pub silviculture_restrictions: Option<SilvicultureRestrictions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: StbAreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesConciseType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<CoHeightClassType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoHeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<CoDiameterClassType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoDiameterType>,
    #[serde(rename = "RemovalClass", skip_serializing_if = "Option::is_none")]
    pub removal_class: Option<CoRemovalClassType>,
    #[serde(rename = "CuttingStemCount", skip_serializing_if = "Option::is_none")]
    pub cutting_stem_count: Option<CuttingStemCountType>,
    #[serde(rename = "SmallWoodRemovalClass", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_class: Option<CoSmallWoodRemovalClassType>,
    #[serde(rename = "SmallWoodRemovalVolume", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_volume: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletion")]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<CoString2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricesAndCurrencyGroup {
    #[serde(rename = "Currency")]
    pub currency: Currency,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "TotalPrice")]
    pub total_price: TotalPrice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType5 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateMmDdYyyyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotKnownType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoReportedStatisticsOperationTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: FccPowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<FdForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<BdtDateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtDateType>,
    #[serde(rename = "ForestUseDeclarationNotNeeded", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_not_needed: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMemberType {
    #[serde(rename = "ResourceId")]
    pub resource_id: WctShortERPIdType,
    #[serde(rename = "UserId")]
    pub user_id: WctShortERPIdType,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<WctTaxNumberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePriorityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSubGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString50Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeType {
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Value")]
    pub value: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStorageLandOwnerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal1FractionDigitType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleAssortmentType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMassType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorType {
    #[serde(rename = "@actorType")]
    pub actor_type: CoActorTypeType,
    #[serde(rename = "@actorTypeSpecifier")]
    pub actor_type_specifier: ActorTypeSpecifierType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "ControlAdditionalInformation", skip_serializing_if = "Option::is_none")]
    pub control_additional_information: Option<ControlAdditionalInformationType>,
    #[serde(rename = "PowerOfAttorney", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney: Option<FccPowerOfAttorneyType>,
    #[serde(rename = "PowerOfAttorneyDate", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_date: Option<FccPowerOfAttorneyDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCertificationSystemType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFertilityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal7And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStatisticsOperationType {
    #[serde(flatten)]
    pub base: CoStatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationDataType {
    #[serde(rename = "CultivatedCropStemCount")]
    pub cultivated_crop_stem_count: CoStemCountType,
    #[serde(rename = "NaturalCropStemCount")]
    pub natural_crop_stem_count: CoStemCountType,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<CoStemCountType>,
    #[serde(rename = "CultivatedDeadStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_dead_stem_count: Option<CoStemCountType>,
    #[serde(rename = "StockingWIthSeedlings")]
    pub stocking_w_ith_seedlings: Xsinteger,
    #[serde(rename = "GroundManipulationMethod")]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<ThreeDigitPositiveIntegerType>,
    #[serde(rename = "CultivationMaterial")]
    pub cultivation_material: TwoDigitPositiveIntegerType,
    #[serde(rename = "PlantingWorkQuality")]
    pub planting_work_quality: Xsinteger,
    #[serde(rename = "SoilModificationEstimate")]
    pub soil_modification_estimate: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestCentreSelfMonitoringDataType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: CoYesNoType,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSellerResponsible {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: BdtLanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: BdtString1000Type,
    #[serde(flatten)]
    pub base: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String50Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestObjectDataObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectKeys", skip_serializing_if = "Option::is_none")]
    pub object_keys: Option<ObjectKeysType>,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLocationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct actuateType {
    #[serde(flatten)]
    pub base: Xstoken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureType {
    #[serde(rename = "CuttingRelated", skip_serializing_if = "Option::is_none")]
    pub cutting_related: Option<CuttingRelatedType>,
    #[serde(rename = "Cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<CostType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentWithFraction1Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<BdtStoreyType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: BdtDecimal1FractionDigitType,
    #[serde(rename = "StemCount")]
    pub stem_count: BdtPositiveIntegerType,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<BdtPositiveIntegerType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<BdtPositiveIntegerType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionNoType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrataType {
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Vec<DeadTreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoadType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<BdtString20Type>,
    #[serde(rename = "WorkLoadId")]
    pub work_load_id: XsunsignedLong,
    #[serde(rename = "Accepted")]
    pub accepted: BdtYesNoType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "UserId")]
    pub user_id: BdtString20Type,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<BdtString20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<BdtString50Type>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<BdtString50Type>,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier1", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier1: Option<Vec<BdtWorkCodeQualifierType1>>,
    #[serde(rename = "WorkCodeQualifier2", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier2: Option<Vec<BdtWorkCodeQualifierType2>>,
    #[serde(rename = "WorkCodeQualifier3", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier3: Option<Vec<BdtWorkCodeQualifierType3>>,
    #[serde(rename = "WorkCodeQualifier4", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier4: Option<Vec<BdtWorkCodeQualifierType4>>,
    #[serde(rename = "WorkCodeQualifier5", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier5: Option<Vec<BdtWorkCodeQualifierType5>>,
    #[serde(rename = "WorkLoad1")]
    pub work_load1: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkLoad1Unit")]
    pub work_load1_unit: BdtWorkCodeUnitType,
    #[serde(rename = "TransportDistance", skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TransportDistanceUnit", skip_serializing_if = "Option::is_none")]
    pub transport_distance_unit: Option<BdtDistanceUnitType>,
    #[serde(rename = "WorkLoad2", skip_serializing_if = "Option::is_none")]
    pub work_load2: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkLoad2Unit", skip_serializing_if = "Option::is_none")]
    pub work_load2_unit: Option<BdtWorkCodeUnitType>,
    #[serde(rename = "WorkLoadInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_invoiced: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkLoadUnitInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_unit_invoiced: Option<BdtWorkCodeUnitType>,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StandFinished", skip_serializing_if = "Option::is_none")]
    pub stand_finished: Option<BdtYesNoType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<BdtTimeStampType>,
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<BdtTimeStampType>,
    #[serde(rename = "SavingTime", skip_serializing_if = "Option::is_none")]
    pub saving_time: Option<BdtTimeStampType>,
    #[serde(rename = "WorkPointCount", skip_serializing_if = "Option::is_none")]
    pub work_point_count: Option<BdtPositiveInteger5digitsType>,
    #[serde(rename = "WorkGrouMembers", skip_serializing_if = "Option::is_none")]
    pub work_grou_members: Option<WorkGrouMembersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSiteStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDataStandardSchemaPackageVersionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtVehicleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChangesType {
    #[serde(rename = "AssortmentsChange", skip_serializing_if = "Option::is_none")]
    pub assortments_change: Option<Vec<AssortmentChangeDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "FinancingActApplicationGeometry")]
    pub financing_act_application_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: CoDateYYYY-MMOrYYYY-MM-DDType,
    #[serde(rename = "EndDate")]
    pub end_date: CoDateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoWideUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalAreaType {
    #[serde(flatten)]
    pub base: CoDecimal4And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<BdtPositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestOwnerGroupType {
    #[serde(flatten)]
    pub base: CoForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeatureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "SilviculturalOperations", skip_serializing_if = "Option::is_none")]
    pub silvicultural_operations: Option<SilviculturalOperationsType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<PrProductsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccountingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContarctorId")]
    pub contarctor_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "AccountingDate")]
    pub accounting_date: BdtTimeStampType,
    #[serde(rename = "FinalAccounting")]
    pub final_accounting: BdtYesNoType,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsedPricingMethodTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSamplePlotType {
    #[serde(rename = "TestAreaNumber")]
    pub test_area_number: BdtPositiveIntegerType,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoninessType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocumentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoPaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRangeType {
    #[serde(rename = "StartLoadNumber")]
    pub start_load_number: BdtPositiveInteger4digitsType,
    #[serde(rename = "EndLoadNumber")]
    pub end_load_number: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateDataType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType2,
    #[serde(rename = "SubsidyApplierReferenceList")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRatingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraMainGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtOrderStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedUnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccCostTypeType2 {
    #[serde(flatten)]
    pub base: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumesType {
    #[serde(rename = "StemTypeVolume")]
    pub stem_type_volume: Vec<StemTypeVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLajiKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "FudrForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<String>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AsAssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<String>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<WctFinalAuditSpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessagesType {
    #[serde(rename = "ErrorMessageData")]
    pub error_message_data: Vec<ErrorMessageDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeededType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMakingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkarcModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactStandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StbStandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformationType {
    #[serde(rename = "DataName")]
    pub data_name: Xsstring,
    #[serde(rename = "DataId")]
    pub data_id: Xsstring,
    #[serde(rename = "InspectorName")]
    pub inspector_name: Xsstring,
    #[serde(rename = "Password")]
    pub password: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: BdtString20Type,
    #[serde(rename = "PurchaseContractExtraInfo", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_extra_info: Option<BdtString3000Type>,
    #[serde(rename = "WorkingSiteNumber")]
    pub working_site_number: WctWorkingSiteNumberType,
    #[serde(rename = "WorkingSiteName")]
    pub working_site_name: BdtString100Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: BdtString20Type,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "CustomerType", skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<BdtString50Type>,
    #[serde(rename = "SilvicultureContractNumber", skip_serializing_if = "Option::is_none")]
    pub silviculture_contract_number: Option<BdtString20Type>,
    #[serde(rename = "Organisation1", skip_serializing_if = "Option::is_none")]
    pub organisation1: Option<BdtString20Type>,
    #[serde(rename = "Organisation2", skip_serializing_if = "Option::is_none")]
    pub organisation2: Option<BdtString20Type>,
    #[serde(rename = "Organisation3", skip_serializing_if = "Option::is_none")]
    pub organisation3: Option<BdtString20Type>,
    #[serde(rename = "Organisation4", skip_serializing_if = "Option::is_none")]
    pub organisation4: Option<BdtString20Type>,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<BdtString50Type>,
    #[serde(rename = "ServiceBuyerContactInformation")]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
    #[serde(rename = "CustomerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub customer_representative_person: Option<BdtContactInformationType>,
    #[serde(rename = "ForestOwners")]
    pub forest_owners: ForestOwnersType,
    #[serde(rename = "AuthorizationsToSendWsoInformation", skip_serializing_if = "Option::is_none")]
    pub authorizations_to_send_wso_information: Option<AuthorizationsToSendWsoInformationType>,
    #[serde(rename = "TransportCompany", skip_serializing_if = "Option::is_none")]
    pub transport_company: Option<BdtContactInformationType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BdtBearingCapacityClassType>,
    #[serde(rename = "Accessibility")]
    pub accessibility: BdtAccessibilityType,
    #[serde(rename = "TerrainClass", skip_serializing_if = "Option::is_none")]
    pub terrain_class: Option<BdtTerrainClassType>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "ResultsOfAccessibilityAnalysis", skip_serializing_if = "Option::is_none")]
    pub results_of_accessibility_analysis: Option<ResultsOfAccessibilityAnalysisType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<BdtPositiveInteger5digitsType>,
    #[serde(rename = "TerrainPlanningDone", skip_serializing_if = "Option::is_none")]
    pub terrain_planning_done: Option<BdtYesNoType>,
    #[serde(rename = "PreClearingInformation", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_information: Option<PreClearingInformationType>,
    #[serde(rename = "WorkingSitePlanningStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_status: Option<BdtWorkingSitePlanningStatusType>,
    #[serde(rename = "WorkingSitePlanningOperation", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_operation: Option<BdtWorkingSitePlanningOperationStatusType>,
    #[serde(rename = "WorkingSitePlanningInfo", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_info: Option<BdtString3000Type>,
    #[serde(rename = "WorkingSitePlannedForHarvestingDate", skip_serializing_if = "Option::is_none")]
    pub working_site_planned_for_harvesting_date: Option<BdtDateType>,
    #[serde(rename = "WorkingSiteStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_status: Option<BdtWorkingSiteStatusType>,
    #[serde(rename = "ReadyToDo")]
    pub ready_to_do: BdtYesNoType,
    #[serde(rename = "SellersLogs", skip_serializing_if = "Option::is_none")]
    pub sellers_logs: Option<BdtString200Type>,
    #[serde(rename = "SellersLogsInfo", skip_serializing_if = "Option::is_none")]
    pub sellers_logs_info: Option<BdtString1000Type>,
    #[serde(rename = "WorkingsiteInfo", skip_serializing_if = "Option::is_none")]
    pub workingsite_info: Option<BdtString3000Type>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<WorkingSitePriorityType>,
    #[serde(rename = "OperationTimeStart", skip_serializing_if = "Option::is_none")]
    pub operation_time_start: Option<BdtDateType>,
    #[serde(rename = "OperationTimeEnd", skip_serializing_if = "Option::is_none")]
    pub operation_time_end: Option<BdtDateType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "ForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration: Option<ForestUseDeclarationType>,
    #[serde(rename = "OperationRestriction", skip_serializing_if = "Option::is_none")]
    pub operation_restriction: Option<OperationRestrictionType>,
    #[serde(rename = "ContractValidDate", skip_serializing_if = "Option::is_none")]
    pub contract_valid_date: Option<BdtDateType>,
    #[serde(rename = "MaterialAreaId", skip_serializing_if = "Option::is_none")]
    pub material_area_id: Option<BdtString20Type>,
    #[serde(rename = "QualityAttachments", skip_serializing_if = "Option::is_none")]
    pub quality_attachments: Option<BdtString100Type>,
    #[serde(rename = "TransportArea", skip_serializing_if = "Option::is_none")]
    pub transport_area: Option<BdtString10Type>,
    #[serde(rename = "HasSupport", skip_serializing_if = "Option::is_none")]
    pub has_support: Option<BdtYesNoType>,
    #[serde(rename = "FinancingSustainableForestry", skip_serializing_if = "Option::is_none")]
    pub financing_sustainable_forestry: Option<FinancingSustainableForestryType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<BdtDateType>,
    #[serde(rename = "PreNotificationAllowed", skip_serializing_if = "Option::is_none")]
    pub pre_notification_allowed: Option<BdtYesNoType>,
    #[serde(rename = "BeginNotificationAllowed", skip_serializing_if = "Option::is_none")]
    pub begin_notification_allowed: Option<BdtYesNoType>,
    #[serde(rename = "SendNotificationsAlways", skip_serializing_if = "Option::is_none")]
    pub send_notifications_always: Option<BdtYesNoType>,
    #[serde(rename = "LargeSummaryReportRequired", skip_serializing_if = "Option::is_none")]
    pub large_summary_report_required: Option<BdtYesNoType>,
    #[serde(rename = "TestAreaMethod", skip_serializing_if = "Option::is_none")]
    pub test_area_method: Option<BdtSamplePlotType>,
    #[serde(rename = "TestAreaRequired", skip_serializing_if = "Option::is_none")]
    pub test_area_required: Option<BdtYesNoType>,
    #[serde(rename = "IslandWorkingSite", skip_serializing_if = "Option::is_none")]
    pub island_working_site: Option<BdtYesNoType>,
    #[serde(rename = "StormWorkingSite", skip_serializing_if = "Option::is_none")]
    pub storm_working_site: Option<BdtYesNoType>,
    #[serde(rename = "SodWorkingSite", skip_serializing_if = "Option::is_none")]
    pub sod_working_site: Option<BdtYesNoType>,
    #[serde(rename = "CanCultivateInAutumn", skip_serializing_if = "Option::is_none")]
    pub can_cultivate_in_autumn: Option<BdtYesNoType>,
    #[serde(rename = "MixedForestRegenarationMethods", skip_serializing_if = "Option::is_none")]
    pub mixed_forest_regenaration_methods: Option<BdtYesNoType>,
    #[serde(rename = "IsValueForceWorkingSite", skip_serializing_if = "Option::is_none")]
    pub is_value_force_working_site: Option<BdtYesNoType>,
    #[serde(rename = "ForestCertification")]
    pub forest_certification: Vec<BdtCertificationSystemType>,
    #[serde(rename = "CertificationHandlingInstructions", skip_serializing_if = "Option::is_none")]
    pub certification_handling_instructions: Option<BdtString3000Type>,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<WctSpareTreesByCategoryType>,
    #[serde(rename = "DryingClass", skip_serializing_if = "Option::is_none")]
    pub drying_class: Option<BdtDryingClassType>,
    #[serde(rename = "HumidityPercentage", skip_serializing_if = "Option::is_none")]
    pub humidity_percentage: Option<BdtDecimal1FractionDigitType>,
    #[serde(rename = "CuttingControlRequired", skip_serializing_if = "Option::is_none")]
    pub cutting_control_required: Option<BdtYesNoType>,
    #[serde(rename = "CuttingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub cutting_finished_date: Option<BdtDateType>,
    #[serde(rename = "StumpLiftingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_finished_date: Option<BdtDateType>,
    #[serde(rename = "BiomassFinishedDate", skip_serializing_if = "Option::is_none")]
    pub biomass_finished_date: Option<BdtDateType>,
    #[serde(rename = "AssortmentIncrementAllowed", skip_serializing_if = "Option::is_none")]
    pub assortment_increment_allowed: Option<BdtYesNoType>,
    #[serde(rename = "EnvironmentalObjectInfo", skip_serializing_if = "Option::is_none")]
    pub environmental_object_info: Option<BdtString3000Type>,
    #[serde(rename = "WorkingSafetyInfo", skip_serializing_if = "Option::is_none")]
    pub working_safety_info: Option<BdtString3000Type>,
    #[serde(rename = "AccessRightsInfo", skip_serializing_if = "Option::is_none")]
    pub access_rights_info: Option<BdtString3000Type>,
    #[serde(rename = "MainWorkCode")]
    pub main_work_code: BdtMainWorkCodeType,
    #[serde(rename = "ProductionFileSendFrequency", skip_serializing_if = "Option::is_none")]
    pub production_file_send_frequency: Option<BdtPositiveIntegerType>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<BdtString50Type>,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<WctPricingMethodType>,
    #[serde(rename = "ContinuousCoverForestry", skip_serializing_if = "Option::is_none")]
    pub continuous_cover_forestry: Option<BdtYesNoType>,
    #[serde(rename = "PreviousBlockState", skip_serializing_if = "Option::is_none")]
    pub previous_block_state: Option<BdtPreviousBlockStatusType>,
    #[serde(rename = "PreviousBlocks", skip_serializing_if = "Option::is_none")]
    pub previous_blocks: Option<PreviousBlockInfoType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "Stands")]
    pub stands: StandsType,
    #[serde(rename = "Storages", skip_serializing_if = "Option::is_none")]
    pub storages: Option<StoragesType>,
    #[serde(rename = "StoragesForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StemTypeBulks", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulks: Option<StemTypeBulksType>,
    #[serde(rename = "TreeSpeciesAttributes", skip_serializing_if = "Option::is_none")]
    pub tree_species_attributes: Option<TreeSpeciesAttributesType>,
    #[serde(rename = "ProductUserIds", skip_serializing_if = "Option::is_none")]
    pub product_user_ids: Option<ProductUserIdsType>,
    #[serde(rename = "DiameterSections", skip_serializing_if = "Option::is_none")]
    pub diameter_sections: Option<DiameterSectionsType>,
    #[serde(rename = "OtherRemarks", skip_serializing_if = "Option::is_none")]
    pub other_remarks: Option<BdtString3000Type>,
    #[serde(rename = "Deliveries", skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<DeliveriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtLanguageCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: CoStatisticsPurchaseModeCodeType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "StatisticsAssortmentCompactClasses")]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStringType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<String>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<BdtString20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<BdtString50Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "OrderId")]
    pub order_id: BdtString20Type,
    #[serde(rename = "OrderStatus")]
    pub order_status: BdtOrderStatusType,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "Id")]
    pub id: Xsstring,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<Xsstring>,
    #[serde(rename = "GmlPoint")]
    pub gml_point: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRootRotControlEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementsType {
    #[serde(rename = "LogDiameter")]
    pub log_diameter: BdtPositiveInteger3digitsType,
    #[serde(rename = "ControlLogDiameter")]
    pub control_log_diameter: BdtPositiveInteger3digitsType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BearingCapacityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowsType {
    #[serde(rename = "OperationRow")]
    pub operation_row: Vec<OperationRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlerType {
    #[serde(rename = "@role")]
    pub role: CoString100Type,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationServiceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesType {
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: Vec<AssortmentVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContactorId")]
    pub contactor_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "Interrupted")]
    pub interrupted: BdtYesNoType,
    #[serde(rename = "FulfilledArea", skip_serializing_if = "Option::is_none")]
    pub fulfilled_area: Option<FulfilledAreasType>,
    #[serde(rename = "UnfulfilledArea", skip_serializing_if = "Option::is_none")]
    pub unfulfilled_area: Option<GdtPolygonOrMultiPolygon2Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString1000Type>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoWoodLotInformationTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationNameBaseType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "EnvlEnvelope")]
    pub envl_envelope: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType3 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumberType {
    #[serde(flatten)]
    pub base: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomassType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DeclarationStandReference")]
    pub declaration_stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: StbAreaType,
    #[serde(rename = "CuttingPurpose")]
    pub cutting_purpose: CoCuttingPurposeType,
    #[serde(rename = "CuttingRealizationPractice")]
    pub cutting_realization_practice: CoCuttingRealizationPracticeType,
    #[serde(rename = "ForestDamageQualifier", skip_serializing_if = "Option::is_none")]
    pub forest_damage_qualifier: Option<CoForestDamageQualifierType>,
    #[serde(rename = "HabitatCode")]
    pub habitat_code: CoHabitatCodeType,
    #[serde(rename = "HabitatOperations", skip_serializing_if = "Option::is_none")]
    pub habitat_operations: Option<HabitatOperationsType>,
    #[serde(rename = "OtherHabitatCode", skip_serializing_if = "Option::is_none")]
    pub other_habitat_code: Option<CoOtherHabitatCodeType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "DeclarationMainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub declaration_main_tree_species: Option<CoTreeSpeciesConciseType>,
    #[serde(rename = "DeclarationDevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub declaration_development_class: Option<CoDeclarationDevelopmentClassType>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<CoAgeType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "DeclarationRegenerationCommitment", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_commitment: Option<CoRegenerationCommitmentType>,
    #[serde(rename = "DeclarationRegenerationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_operation: Option<CoDeclarationRegenerationOperationType>,
    #[serde(rename = "DeclarationSoilPreparationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_soil_preparation_operation: Option<CoDeclarationSoilPreparationOperationType>,
    #[serde(rename = "DeclarationOtherOperations", skip_serializing_if = "Option::is_none")]
    pub declaration_other_operations: Option<DeclarationOtherOperationsType>,
    #[serde(rename = "DeclarationStandTextInformation", skip_serializing_if = "Option::is_none")]
    pub declaration_stand_text_information: Option<CoString2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotsType {
    #[serde(rename = "Woodlot")]
    pub woodlot: Vec<WoodLotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlersType {
    #[serde(rename = "DecisionHandler")]
    pub decision_handler: Vec<DecisionHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String2000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystemType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationResponsibleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSuggestionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "Payees")]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLiftingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: BdtString100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "Quality")]
    pub quality: BdtString5Type,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCuttingType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlBaseCuttingType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "QualityControlDate")]
    pub quality_control_date: BdtDateType,
    #[serde(rename = "SamplePlotsSummaries")]
    pub sample_plots_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount")]
    pub planned_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal1FractionDigitType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoElectronicNotificationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPreviousBlockStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsUlkomaaPostitoimipaikkaNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentWithFraction2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataStandardSchemaPackageVersionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitesType {
    #[serde(rename = "WorkingSite", skip_serializing_if = "Option::is_none")]
    pub working_site: Option<Vec<WorkingSiteType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAmountUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString5Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfaType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurementPlaceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaLawType {
    #[serde(flatten)]
    pub base: CoVirtaLawType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsTurvakieltoKytkinTyyppi {
    #[serde(flatten)]
    pub base: Xsboolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlkuHetkiTyyppi {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileNameType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstNameType {
    #[serde(flatten)]
    pub base: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotKnownType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPointLocationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurposeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<CoPreferredContactingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTypeType {
    #[serde(flatten)]
    pub base: CoPaymentTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandInfoType {
    #[serde(flatten)]
    pub base: StbStandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtInteger3digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreasType {
    #[serde(rename = "FulfilledArea")]
    pub fulfilled_area: Vec<FulfilledAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStampType {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferTextType {
    #[serde(flatten)]
    pub base: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveriesType {
    #[serde(rename = "Delivery")]
    pub delivery: Vec<DeliveryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowsType {
    #[serde(rename = "LoggingsRow")]
    pub loggings_row: Vec<LoggingsRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String3000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String20Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumberType {
    #[serde(flatten)]
    pub base: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal6TotalDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString25Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedQuantityUnitType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineAccessoryCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoRoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctShortERPIdType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiEmailAddressType {
    #[serde(flatten)]
    pub base: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger1digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAgeType {
    #[serde(flatten)]
    pub base: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonType {
    #[serde(rename = "ReasonCode")]
    pub reason_code: CoString10Type,
    #[serde(rename = "ReasonDescription")]
    pub reason_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteKirjainTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaGroundManipulationMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAndOrganizationGroup {
    #[serde(rename = "OrganizationName")]
    pub organization_name: OrganizationName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PEFCCertificationSystemSubTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoCallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: Xsdecimal,
    #[serde(rename = "@location")]
    pub location: CoPointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: String,
    #[serde(rename = "GmllineStringProperty")]
    pub gmlline_string_property: String,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal3FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationType {
    #[serde(rename = "OperationCode", skip_serializing_if = "Option::is_none")]
    pub operation_code: Option<CoObjectProtectionOperationCodeType>,
    #[serde(rename = "OperationDescription", skip_serializing_if = "Option::is_none")]
    pub operation_description: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoDigitPositiveIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActFinancingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: CoDateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: CoInventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TstTreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<String>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<String>,
    #[serde(rename = "TssTreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<String>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<String>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMinType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDataType {
    #[serde(rename = "Startdate")]
    pub startdate: BdtDateType,
    #[serde(rename = "Enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<BdtDateType>,
    #[serde(rename = "WorkingContract")]
    pub working_contract: BdtYesNoType,
    #[serde(rename = "Active")]
    pub active: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAddressGroup {
    #[serde(rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
    #[serde(rename = "PostalCode")]
    pub postal_code: PostalCode,
    #[serde(rename = "StateText", skip_serializing_if = "Option::is_none")]
    pub state_text: Option<StateText>,
    #[serde(rename = "StateCode", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<StateCode>,
    #[serde(rename = "PostOffice")]
    pub post_office: PostOffice,
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "CountryText", skip_serializing_if = "Option::is_none")]
    pub country_text: Option<CountryText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "CodAdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub cod_additional_details: Option<String>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataType {
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: CoForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger4digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientApplicationIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPaymentType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationsType {
    #[serde(rename = "SelfMonitoringEvaluation")]
    pub self_monitoring_evaluation: Vec<SelfMonitoringEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(rename = "ServiceNameOfAPI")]
    pub service_name_of_a_p_i: BdtServiceNameofAPIType,
    #[serde(rename = "AuthorizedToSend")]
    pub authorized_to_send: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainsType {
    #[serde(rename = "PlannedOperationChain")]
    pub planned_operation_chain: Vec<PlannedOperationChainType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: BdtString20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: BdtPositiveIntegerType,
    #[serde(rename = "IsForestHaulageDistanceContinued")]
    pub is_forest_haulage_distance_continued: BdtYesNoType,
    #[serde(rename = "StorageFinished")]
    pub storage_finished: BdtYesNoType,
    #[serde(rename = "StorageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub storage_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "WorkingSiteFinished")]
    pub working_site_finished: BdtYesNoType,
    #[serde(rename = "WorkingSiteFinishedDate", skip_serializing_if = "Option::is_none")]
    pub working_site_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "NotificationDate")]
    pub notification_date: BdtTimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "AssortmentsChanges", skip_serializing_if = "Option::is_none")]
    pub assortments_changes: Option<AssortmentsChangesType>,
    #[serde(rename = "CommonMessages", skip_serializing_if = "Option::is_none")]
    pub common_messages: Option<CommonMessagesType>,
    #[serde(rename = "ScaleFactors", skip_serializing_if = "Option::is_none")]
    pub scale_factors: Option<ScaleFactorsType>,
    #[serde(rename = "ClientApplicationId", skip_serializing_if = "Option::is_none")]
    pub client_application_id: Option<BdtClientApplicationIdType>,
    #[serde(rename = "LoadRange", skip_serializing_if = "Option::is_none")]
    pub load_range: Option<LoadRangeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetailsType {
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: Xsint,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<CoString1500Type>,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoActorTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformationType {
    #[serde(rename = "PreinformDate", skip_serializing_if = "Option::is_none")]
    pub preinform_date: Option<CoDateType>,
    #[serde(rename = "PreinformDetails", skip_serializing_if = "Option::is_none")]
    pub preinform_details: Option<CoString1000Type>,
    #[serde(rename = "InTerrain", skip_serializing_if = "Option::is_none")]
    pub in_terrain: Option<CoYesNoType>,
    #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineManufacturerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctScaleFactorTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "DiameterCalibrationDate")]
    pub diameter_calibration_date: BdtTimeStampType,
    #[serde(rename = "DiameterCalibrationReason")]
    pub diameter_calibration_reason: BdtString200Type,
    #[serde(rename = "DiameterCalibrationDescription")]
    pub diameter_calibration_description: BdtString200Type,
    #[serde(rename = "DiameterCalibrationAdjustment")]
    pub diameter_calibration_adjustment: BdtInteger3digitsType,
    #[serde(rename = "DiameterCalibrationAdjustmentButtLog", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration_adjustment_butt_log: Option<BdtInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoRealEstateType {
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreDataMessageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMaterialUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelefaxNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<CoReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: String,
    #[serde(rename = "CfowsCFOWorkingSite")]
    pub cfows_c_f_o_working_site: String,
    #[serde(rename = "BaBusinessAcceptance")]
    pub ba_business_acceptance: String,
    #[serde(rename = "WppPayment")]
    pub wpp_payment: String,
    #[serde(rename = "WpmcMeasurementCertificate")]
    pub wpmc_measurement_certificate: String,
    #[serde(rename = "FudrForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: String,
    #[serde(rename = "WstcrContract")]
    pub wstcr_contract: String,
    #[serde(rename = "WstoOffer")]
    pub wsto_offer: String,
    #[serde(rename = "OwsOWorkingSite")]
    pub ows_o_working_site: String,
    #[serde(rename = "OsuOperations")]
    pub osu_operations: String,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: String,
    #[serde(rename = "MsMapSymbol")]
    pub ms_map_symbol: String,
    #[serde(rename = "AckAcknowledge")]
    pub ack_acknowledge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: BdtDecimal1FractionDigitType,
    #[serde(rename = "Orientation")]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<BdtWorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkinTyyppi {
    #[serde(flatten)]
    pub base: Xsboolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "RelatedCallForOffers", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offers: Option<RelatedCallForOffersType>,
    #[serde(rename = "AdditionalCode", skip_serializing_if = "Option::is_none")]
    pub additional_code: Option<AdditionalCodeType>,
    #[serde(rename = "CallForOfferBusinessSender")]
    pub call_for_offer_business_sender: WtcoCallForOfferBusinessSenderType,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<WtcoSellersType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "CallForOfferDate")]
    pub call_for_offer_date: CallForOfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<WtcoCallForOfferTextType>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
    #[serde(rename = "CallForOfferSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_silviculture_info: Option<CallForOfferSilvicultureInfoType>,
    #[serde(rename = "WsCallForOfferWorkingSites")]
    pub ws_call_for_offer_working_sites: String,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoMessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<CiEmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: FudForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislationType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "CompensationDescription", skip_serializing_if = "Option::is_none")]
    pub compensation_description: Option<Xsstring>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionQuantityUnitType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostitoimipaikkaNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger3digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfBasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<CoDateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUnitPerHectareType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "TestAreaNumber")]
    pub test_area_number: BdtPositiveIntegerType,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCasesType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<Vec<CoForestDataUpdateUseCaseType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "LengthCalibrationDate")]
    pub length_calibration_date: BdtTimeStampType,
    #[serde(rename = "LengthCalibrationReason")]
    pub length_calibration_reason: BdtString200Type,
    #[serde(rename = "LengthCalibrationDescription")]
    pub length_calibration_description: BdtString200Type,
    #[serde(rename = "LengthCalibrationAdjustment")]
    pub length_calibration_adjustment: BdtInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcroleType {
    #[serde(flatten)]
    pub base: XsanyURI,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationOtherOperationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOtherPublicSubstituteType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsedPricingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctQualityOfTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionUnitType {
    #[serde(flatten)]
    pub base: CoUnitPerHectareType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String10Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionsType {
    #[serde(rename = "DiameterSection")]
    pub diameter_section: Vec<SectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItemType {
    #[serde(rename = "AverageStemVolume")]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: WtcoUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnitType {
    #[serde(flatten)]
    pub base: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyTransactionCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumeType {
    #[serde(rename = "ProductUserId")]
    pub product_user_id: BdtString100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: BdtString50Type,
    #[serde(rename = "Quality")]
    pub quality: BdtString5Type,
    #[serde(rename = "LengthClass")]
    pub length_class: BdtPositiveInteger4digitsType,
    #[serde(rename = "DiameterClass")]
    pub diameter_class: BdtPositiveInteger4digitsType,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodsType {
    #[serde(rename = "UsedPricingMethod")]
    pub used_pricing_method: Vec<UsedPricingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationsType {
    #[serde(rename = "Specification")]
    pub specification: Vec<SpecificationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString1000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeBetweenProposalYearsGroup {
    #[serde(rename = "MinProposalYear")]
    pub min_proposal_year: MinProposalYear,
    #[serde(rename = "MaxProposalYear")]
    pub max_proposal_year: MaxProposalYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateAssortmentRowType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAssortmentNameType {
    #[serde(flatten)]
    pub base: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEtunimetNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatAdvertisementType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesSummaryType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSumTableAreaType {
    #[serde(flatten)]
    pub base: CoVirtaSumTableAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamicType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseDynamicType,
    #[serde(rename = "FinalAuditSpareTrees", skip_serializing_if = "Option::is_none")]
    pub final_audit_spare_trees: Option<FinalAuditSpareTreesByCategoryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger6digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraStemTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsoluteQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBranchBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialType {
    #[serde(rename = "MaterialId")]
    pub material_id: BdtString20Type,
    #[serde(rename = "MaterialCode")]
    pub material_code: BdtMaterialCodeType,
    #[serde(rename = "CommercialName", skip_serializing_if = "Option::is_none")]
    pub commercial_name: Option<BdtString100Type>,
    #[serde(rename = "Amount")]
    pub amount: BdtDecimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtMaterialUnitType,
    #[serde(rename = "Supplier", skip_serializing_if = "Option::is_none")]
    pub supplier: Option<BdtString50Type>,
    #[serde(rename = "MaterialProducer", skip_serializing_if = "Option::is_none")]
    pub material_producer: Option<BdtString50Type>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "GrainSize", skip_serializing_if = "Option::is_none")]
    pub grain_size: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "DeliveryNumber", skip_serializing_if = "Option::is_none")]
    pub delivery_number: Option<BdtString20Type>,
    #[serde(rename = "Delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<BdtYesNoType>,
    #[serde(rename = "DeliveryDate", skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<BdtDateType>,
    #[serde(rename = "PackagingDate", skip_serializing_if = "Option::is_none")]
    pub packaging_date: Option<BdtDateType>,
    #[serde(rename = "FreezingDate", skip_serializing_if = "Option::is_none")]
    pub freezing_date: Option<BdtDateType>,
    #[serde(rename = "MeltingDate", skip_serializing_if = "Option::is_none")]
    pub melting_date: Option<BdtDateType>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString100Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "MaterialDeliveryType", skip_serializing_if = "Option::is_none")]
    pub material_delivery_type: Option<MaterialDeliveryTypeType>,
    #[serde(rename = "RegionOfOrigin", skip_serializing_if = "Option::is_none")]
    pub region_of_origin: Option<BdtString10Type>,
    #[serde(rename = "DegreeDays", skip_serializing_if = "Option::is_none")]
    pub degree_days: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "Altitude", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<BdtServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<BdtImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<BdtImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<BdtFeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<WctERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<BdtString20Type>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<GdtPointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<BdtString50Type>,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "ImageDate", skip_serializing_if = "Option::is_none")]
    pub image_date: Option<BdtTimeStampType>,
    #[serde(rename = "Filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<BdtString100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Xsbase64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate")]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
    #[serde(rename = "FinancingActApplicationGeometries")]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType1 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostilokerolyhenneTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcPlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: Xsstring,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: WtcPlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAlayksikkoNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwnerType {
    #[serde(rename = "CiNameAndOrganizationGroup")]
    pub ci_name_and_organization_group: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleAttrType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeNameType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTreeDecimalType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedHabitatCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityDataType {
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "TransportAccessibility", skip_serializing_if = "Option::is_none")]
    pub transport_accessibility: Option<CoTransportAccessibilityType>,
    #[serde(rename = "HarvestingAccessibility", skip_serializing_if = "Option::is_none")]
    pub harvesting_accessibility: Option<CoHarvestingAccessibilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: WtcoPurchaseModeType,
    #[serde(rename = "AssortmentCompactClasses")]
    pub assortment_compact_classes: AsAssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummaryType {
    #[serde(rename = "OperativeTreeSpeciesData")]
    pub operative_tree_species_data: Vec<TsTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreasType {
    #[serde(rename = "ProcessingArea")]
    pub processing_area: Vec<ProcessingAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@source")]
    pub source: Xsstring,
    #[serde(rename = "ContactInformation")]
    pub contact_information: CiContactInformationType,
    #[serde(rename = "ContactMunicipality")]
    pub contact_municipality: CoMunicipalityNumberType,
    #[serde(rename = "ContactLocationInformation")]
    pub contact_location_information: GdtAlternativeGeometriesType,
    #[serde(rename = "RequestCode")]
    pub request_code: RequestCodeType,
    #[serde(rename = "RequestInfo", skip_serializing_if = "Option::is_none")]
    pub request_info: Option<CoString2000Type>,
    #[serde(rename = "PreferredContactingMethods", skip_serializing_if = "Option::is_none")]
    pub preferred_contacting_methods: Option<PreferredContactingMethodsType>,
    #[serde(rename = "CreateDate")]
    pub create_date: CoDateType,
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<CoDateType>,
    #[serde(rename = "ForestPropertyDataSet", skip_serializing_if = "Option::is_none")]
    pub forest_property_data_set: Option<ForestPropertyDataSetType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationsType {
    #[serde(rename = "SilviculturalOperation")]
    pub silvicultural_operation: Vec<SilviculturalOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarianceType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference14Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometriesType {
    #[serde(rename = "HgPolygonGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_polygon_geometry: Option<Vec<String>>,
    #[serde(rename = "HgLineGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_line_geometry: Option<Vec<String>>,
    #[serde(rename = "HgPointGeometry", skip_serializing_if = "Option::is_none")]
    pub hg_point_geometry: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecimal4And2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDataType {
    #[serde(rename = "LogKey")]
    pub log_key: BdtString10Type,
    #[serde(rename = "ProductKey")]
    pub product_key: WctERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: BdtPositiveInteger4digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: BdtPositiveInteger4digitsType,
    #[serde(rename = "CalibrationUseLog", skip_serializing_if = "Option::is_none")]
    pub calibration_use_log: Option<BdtYesNoType>,
    #[serde(rename = "LogDiameterClass", skip_serializing_if = "Option::is_none")]
    pub log_diameter_class: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "LogLengthClass", skip_serializing_if = "Option::is_none")]
    pub log_length_class: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "LogMeasurements")]
    pub log_measurements: Vec<LogMeasurementsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetkiTyyppi {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct labelType {
    #[serde(flatten)]
    pub base: XsNCName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeCuttingType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<CodForestObjectDataObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSilvicultureBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "ReRealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<String>,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax5IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "FudrForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<String>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AsAssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<String>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditAnswerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesGroup {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNimilajiKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeasurementMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: CoDateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: CoInventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TstTreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<String>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<String>,
    #[serde(rename = "TssTreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<String>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<String>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuntaNumeroTyyppi {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditPlantManagementBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: BdtPositiveInteger2digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString50Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoQuantityUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectDataType {
    #[serde(rename = "SelfMonitoringStandArea", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_stand_area: Option<CoAreaType>,
    #[serde(rename = "GoalTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub goal_tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "GoalStemCount", skip_serializing_if = "Option::is_none")]
    pub goal_stem_count: Option<CoStemCountType>,
    #[serde(rename = "GoalAmountOfSoilPreparationSpot", skip_serializing_if = "Option::is_none")]
    pub goal_amount_of_soil_preparation_spot: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RealAmountOfSoilPreparationSpot", skip_serializing_if = "Option::is_none")]
    pub real_amount_of_soil_preparation_spot: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "EstimatedWorkingTimeConsumption", skip_serializing_if = "Option::is_none")]
    pub estimated_working_time_consumption: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "TimeIntervalForMeasuringSamplePlot", skip_serializing_if = "Option::is_none")]
    pub time_interval_for_measuring_sample_plot: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "Notices", skip_serializing_if = "Option::is_none")]
    pub notices: Option<CoString1000Type>,
    #[serde(rename = "WorkSafetyRisks", skip_serializing_if = "Option::is_none")]
    pub work_safety_risks: Option<WorkSafetyRisksType>,
    #[serde(rename = "SoilPreparationSpotsAreEnough", skip_serializing_if = "Option::is_none")]
    pub soil_preparation_spots_are_enough: Option<CoYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSBType {
    #[serde(rename = "ShapeGamma")]
    pub shape_gamma: ShapeGammaType,
    #[serde(rename = "ShapeDelta")]
    pub shape_delta: ShapeDeltaType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location")]
    pub location: LocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformationType {
    #[serde(rename = "AuthorizationToSendWsoInformation")]
    pub authorization_to_send_wso_information: Vec<AuthorizationToSendWsoInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoIncludePaymentPlanType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionType {
    #[serde(rename = "ActionType")]
    pub action_type: Xsstring,
    #[serde(rename = "ActionDescription", skip_serializing_if = "Option::is_none")]
    pub action_description: Option<CoString1000Type>,
    #[serde(rename = "ActionDate")]
    pub action_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: Xsstring,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: WtcPlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReviewType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate2Type {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: CoDateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: CoInventoryMethodType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<TstTreeStrata2Type>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<String>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TssTreeStandSummary2Type>,
    #[serde(rename = "SdTrees", skip_serializing_if = "Option::is_none")]
    pub sd_trees: Option<String>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<String>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<String>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUsingRightResponsibleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCuttingPurposeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperationStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: String,
    #[serde(rename = "FacCompletionDeclarationReference")]
    pub fac_completion_declaration_reference: String,
    #[serde(rename = "FacFinancingActNumber")]
    pub fac_financing_act_number: String,
    #[serde(rename = "FacFinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_reference: Option<String>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<String>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<CoBankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: CoYesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: CoYesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_declaration_text_information: Option<String>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: String,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: String,
    #[serde(rename = "FacSender")]
    pub fac_sender: String,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<String>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: String,
    #[serde(rename = "FacStartDate")]
    pub fac_start_date: String,
    #[serde(rename = "FacEndDate")]
    pub fac_end_date: String,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: String,
    #[serde(rename = "FacCompletionDeclarationActors")]
    pub fac_completion_declaration_actors: String,
    #[serde(rename = "FacWorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub fac_working_representatives: Option<String>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreaType {
    #[serde(rename = "OperationalRegion")]
    pub operational_region: BdtString50Type,
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeOfForestObjectType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesType {
    #[serde(rename = "Role")]
    pub role: Vec<OrganizationRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandKeyGroup1 {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStateType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal3FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumesType {
    #[serde(rename = "AssortmentMatrixVolume")]
    pub assortment_matrix_volume: Vec<AssortmentMatrixVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<CoDateType>,
    #[serde(rename = "SilvicultureValidity", skip_serializing_if = "Option::is_none")]
    pub silviculture_validity: Option<SilvicultureValidityType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<PrProductsType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureText", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_text: Option<Xsstring>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeType {
    #[serde(rename = "ReplyTo")]
    pub reply_to: BdtString50Type,
    #[serde(rename = "StatusCode")]
    pub status_code: BdtPositiveInteger3digitsType,
    #[serde(rename = "StatusMessage")]
    pub status_message: BdtString1000Type,
    #[serde(rename = "OriginalMessageType")]
    pub original_message_type: BdtString50Type,
    #[serde(rename = "StatusMessages", skip_serializing_if = "Option::is_none")]
    pub status_messages: Option<StatusMessageLanguageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationInfoGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3And2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMainGroupType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<CodForestCentreControlObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "CaseNumber")]
    pub case_number: FccFinancingActNumberType,
    #[serde(rename = "CaseDate")]
    pub case_date: CoDateType,
    #[serde(rename = "DecisionNumber")]
    pub decision_number: CoString100Type,
    #[serde(rename = "DecisionType")]
    pub decision_type: CoDecisionTypeType,
    #[serde(rename = "DecisionDate")]
    pub decision_date: CoDateType,
    #[serde(rename = "FccCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<String>,
    #[serde(rename = "FccFinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_application_reference: Option<String>,
    #[serde(rename = "FccCompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<String>,
    #[serde(rename = "FccForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_use_declaration_reference: Option<String>,
    #[serde(rename = "Acceptance")]
    pub acceptance: CoAcceptanceType,
    #[serde(rename = "CaseActions")]
    pub case_actions: CaseActionsType,
    #[serde(rename = "OriginalSender")]
    pub original_sender: CiContactInformationType,
    #[serde(rename = "DecisionReceivers")]
    pub decision_receivers: DecisionReceiversType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<CiContactInformationType>,
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstatesType,
    #[serde(rename = "DecisionGeometries", skip_serializing_if = "Option::is_none")]
    pub decision_geometries: Option<DecisionGeometriesType>,
    #[serde(rename = "Works")]
    pub works: WorksType,
    #[serde(rename = "SubsidyArgument", skip_serializing_if = "Option::is_none")]
    pub subsidy_argument: Option<SubsidyArgumentType>,
    #[serde(rename = "FinancingActData", skip_serializing_if = "Option::is_none")]
    pub financing_act_data: Option<FinancingActDataType>,
    #[serde(rename = "Justifications")]
    pub justifications: JustificationsType,
    #[serde(rename = "RectificationDemand")]
    pub rectification_demand: CoString5000Type,
    #[serde(rename = "DecisionHandlers")]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EbEnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGammaType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlaceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTimeType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonsType {
    #[serde(rename = "Reason")]
    pub reason: Vec<ReasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManualType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "InfoText")]
    pub info_text: BdtString200Type,
    #[serde(rename = "Measurements")]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercentType {
    #[serde(flatten)]
    pub base: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorsType {
    #[serde(rename = "CompletionActor")]
    pub completion_actor: Vec<CompletionActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesType {
    #[serde(rename = "UserRole")]
    pub user_role: Vec<BdtUserRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyAndPercentType {
    #[serde(rename = "AbsoluteQuantity")]
    pub absolute_quantity: AbsoluteQuantityType,
    #[serde(rename = "Percent")]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstructionType {
    #[serde(rename = "AppliedLength")]
    pub applied_length: Vec<Decimal6_2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectDataGroup", skip_serializing_if = "Option::is_none")]
    pub object_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4TotalDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountType {
    #[serde(flatten)]
    pub base: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupsType {
    #[serde(rename = "SpareTreeGroup")]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdStringType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHeightClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<BdtServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceType {
    #[serde(rename = "@message")]
    pub message: MessageTypeType,
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: Xsstring,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "BusinessMessageTimeStamp")]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
    #[serde(rename = "BusinessAcceptanceActor")]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
    #[serde(rename = "BusinessAcceptanceStatus")]
    pub business_acceptance_status: CoBusinessAcceptanceStatusType,
    #[serde(rename = "BusinessAcceptanceId", skip_serializing_if = "Option::is_none")]
    pub business_acceptance_id: Option<BusinessAcceptanceIdType>,
    #[serde(rename = "AdditionalInformation")]
    pub additional_information: AdditionalInformationType,
    #[serde(rename = "BusinessAcceptanceDate")]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeStandDataDate2Type {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: CoDateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: CoInventoryMethodType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<TstTreeStrata2Type>,
    #[serde(rename = "DtsDeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<String>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TssTreeStandSummary2Type>,
    #[serde(rename = "SdTrees", skip_serializing_if = "Option::is_none")]
    pub sd_trees: Option<String>,
    #[serde(rename = "SdStemDistribution", skip_serializing_if = "Option::is_none")]
    pub sd_stem_distribution: Option<String>,
    #[serde(rename = "SdsStemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub sds_stem_distribution_strata: Option<String>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStandQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoQuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPhaseType {
    #[serde(flatten)]
    pub base: CoVirtaPhaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointAndLineGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentrePayments")]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeadTreeTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIdentifierGroup {
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<AssortmentName>,
    #[serde(rename = "AssortmentMainGroup", skip_serializing_if = "Option::is_none")]
    pub assortment_main_group: Option<AssortmentMainGroup>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpecies,
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<AssortmentCode>,
    #[serde(rename = "StemType")]
    pub stem_type: StemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesType {
    #[serde(rename = "Trees")]
    pub trees: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoMessageTypeType {
    #[serde(flatten)]
    pub base: WtcoMessageTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDecisionGeometryObjectType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAcceptanceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<PaymentsRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotificationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeStandDataMomentType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListType {
    #[serde(rename = "FeeBaseListItem")]
    pub fee_base_list_item: Vec<FeebaseListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationGroup {
    #[serde(rename = "WoodLotInformationTypeDescription", skip_serializing_if = "Option::is_none")]
    pub wood_lot_information_type_description: Option<WoodLotInformationTypeDescription>,
    #[serde(rename = "WoodLotInformationType")]
    pub wood_lot_information_type: WoodLotInformationType,
    #[serde(rename = "WoodLotInformationValue")]
    pub wood_lot_information_value: WoodLotInformationValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlanType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummaryType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "MeanAge")]
    pub mean_age: MeanAgeType,
    #[serde(rename = "BasalArea")]
    pub basal_area: CoBasalAreaType,
    #[serde(rename = "StemCount")]
    pub stem_count: StemCountType,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<CoDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: CoMeanHeightType,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume")]
    pub volume: VolumeType,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth")]
    pub volume_growth: VolumeGrowthType,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CoCurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonsType {
    #[serde(rename = "DisQualificationReason", skip_serializing_if = "Option::is_none")]
    pub dis_qualification_reason: Option<Vec<DisQualificationReasonDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotType {
    #[serde(rename = "WoodLotInformationGroup")]
    pub wood_lot_information_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtUserRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSpareTreeCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extended {
    #[serde(rename = "XlinkextendedModel", skip_serializing_if = "Option::is_none")]
    pub xlinkextended_model: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fromType {
    #[serde(flatten)]
    pub base: XsNCName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateYYYY-MMOrYYYY-MM-DDType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(flatten)]
    pub base: StbStandInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentAllElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: String,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<CoUsedPricingMethodType>,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: String,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<String>,
    #[serde(rename = "DimensionRequirementsGroup", skip_serializing_if = "Option::is_none")]
    pub dimension_requirements_group: Option<String>,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "GradeCode", skip_serializing_if = "Option::is_none")]
    pub grade_code: Option<CoGradeCodeType>,
    #[serde(rename = "WoodLots", skip_serializing_if = "Option::is_none")]
    pub wood_lots: Option<WoodLotsType>,
    #[serde(rename = "MeasurementMethod", skip_serializing_if = "Option::is_none")]
    pub measurement_method: Option<CoMeasurementMethodType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
    #[serde(rename = "PriceMatrix", skip_serializing_if = "Option::is_none")]
    pub price_matrix: Option<PriceMatrixType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKeyType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelectionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiPersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNimiTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger3digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResourceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageStateType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDefinedType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerGroupType {
    #[serde(flatten)]
    pub base: CoForestOwnerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoChangeStateType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinkresourceModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateType {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistancesType {
    #[serde(rename = "StorageForestHaulageDistance")]
    pub storage_forest_haulage_distance: Vec<StorageForestHaulageDistanceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "AlternativeIdentifier")]
    pub alternative_identifier: AlternativeIdentifierType,
    #[serde(rename = "AlternativeName", skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<AlternativeNameType>,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingStemTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<WtcoRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ImageCount")]
    pub image_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<BdtString20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<BdtString50Type>,
    #[serde(rename = "WorkingSitePlanningStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_status: Option<BdtWorkingSitePlanningStatusType>,
    #[serde(rename = "WorkingSitePlanningOperation", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_operation: Option<BdtWorkingSitePlanningOperationStatusType>,
    #[serde(rename = "WorkingSitePlanningInfo", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_info: Option<BdtString3000Type>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<BdtAccessibilityType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "Storages", skip_serializing_if = "Option::is_none")]
    pub storages: Option<StoragesType>,
    #[serde(rename = "StoragesForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "StorageProposals", skip_serializing_if = "Option::is_none")]
    pub storage_proposals: Option<StoragesType>,
    #[serde(rename = "StoragesProposalForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_proposal_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "ProductUserIds", skip_serializing_if = "Option::is_none")]
    pub product_user_ids: Option<ProductUserIdsType>,
    #[serde(rename = "CanCultivateInAutumn", skip_serializing_if = "Option::is_none")]
    pub can_cultivate_in_autumn: Option<BdtYesNoType>,
    #[serde(rename = "DelinationObjectOrderId", skip_serializing_if = "Option::is_none")]
    pub delination_object_order_id: Option<BdtString200Type>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "WorkingsiteInfo", skip_serializing_if = "Option::is_none")]
    pub workingsite_info: Option<BdtString3000Type>,
    #[serde(rename = "PurchaseContractExtraInfo", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_extra_info: Option<BdtString3000Type>,
    #[serde(rename = "EnvironmentalObjectInfo", skip_serializing_if = "Option::is_none")]
    pub environmental_object_info: Option<BdtString3000Type>,
    #[serde(rename = "WorkingSafetyInfo", skip_serializing_if = "Option::is_none")]
    pub working_safety_info: Option<BdtString3000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5TotalDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocationsType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceLocations", skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<ResourceLocationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaysType {
    #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
    pub day: Option<Vec<DayType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "DBH")]
    pub dbh: BdtPositiveInteger3digitsType,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulkType {
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "Bulk")]
    pub bulk: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadType {
    #[serde(rename = "LoadNumber")]
    pub load_number: XsnonNegativeInteger,
    #[serde(rename = "ForwardingDistance")]
    pub forwarding_distance: XsnonNegativeInteger,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "PartitialLoad")]
    pub partitial_load: Vec<PartitialLoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "StemCountOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub stem_count_other_tree_species: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "MeanHeightOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_height_other_tree_species: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub basal_area_other_tree_species: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanDiameterOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_other_tree_species: Option<BdtPositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseDynamicType {
    #[serde(rename = "Attributes")]
    pub attributes: AttributesType,
    #[serde(rename = "Audition")]
    pub audition: AuditionType,
    #[serde(rename = "AuditionResources", skip_serializing_if = "Option::is_none")]
    pub audition_resources: Option<AuditionResourcesType>,
    #[serde(rename = "Questions")]
    pub questions: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalType {
    #[serde(rename = "Mean")]
    pub mean: MeanType,
    #[serde(rename = "Variance")]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetSelectionType {
    #[serde(flatten)]
    pub base: CoVirtaTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermissionType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItemType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "TreeNumber", skip_serializing_if = "Option::is_none")]
    pub tree_number: Option<Xsinteger>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<CoStoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoAgeType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<CoDiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<CoMeanHeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<CoVolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TreeIdentifiers", skip_serializing_if = "Option::is_none")]
    pub tree_identifiers: Option<TreeIdentifiersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIDType {
    #[serde(flatten)]
    pub base: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceType {
    #[serde(rename = "StorageForestHaulageDistanceGroup")]
    pub storage_forest_haulage_distance_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysisType {
    #[serde(rename = "Accessibility")]
    pub accessibility: BdtAccessibilityType,
    #[serde(rename = "Percentage")]
    pub percentage: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsType {
    #[serde(rename = "Cutting")]
    pub cutting: Vec<CuttingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingExtraQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfFeatureAdditionalInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClass {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPankkitiliTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStorageDryingClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: NotEmptyTreeSpeciesType,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "SawlogPercent", skip_serializing_if = "Option::is_none")]
    pub sawlog_percent: Option<CoSawLogPercentType>,
    #[serde(rename = "TotalSawlogVolume", skip_serializing_if = "Option::is_none")]
    pub total_sawlog_volume: Option<CoSawLogVolumeType>,
    #[serde(rename = "TotalPulpwoodVolume", skip_serializing_if = "Option::is_none")]
    pub total_pulpwood_volume: Option<CoPulpWoodVolumeType>,
    #[serde(rename = "TotalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<CoVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsPurchaseModeCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclarationType {
    #[serde(rename = "CuttingRealizationPractice", skip_serializing_if = "Option::is_none")]
    pub cutting_realization_practice: Option<CoCuttingTypeType>,
    #[serde(rename = "HarvestingSignControlClassifier", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_control_classifier: Option<CoEvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString10Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometryType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<CoAreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<CoAreaType>,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDateType {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuolemaPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeanHeightType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateRegisterUnitGroup {
    #[serde(rename = "GroupNumber")]
    pub group_number: GroupNumber,
    #[serde(rename = "UnitNumber")]
    pub unit_number: UnitNumber,
    #[serde(rename = "AreaNumber")]
    pub area_number: AreaNumber,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumber,
    #[serde(rename = "MunicipalityName", skip_serializing_if = "Option::is_none")]
    pub municipality_name: Option<MunicipalityName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcModel {
    #[serde(rename = "xlink_title", skip_serializing_if = "Option::is_none")]
    pub xlink_title: Option<Vec<xlink_title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMachineManufacturerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDrainageStateType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesGroup {
    #[serde(rename = "RealEstate")]
    pub real_estate: RealEstate,
    #[serde(rename = "RealEstateName", skip_serializing_if = "Option::is_none")]
    pub real_estate_name: Option<RealEstateName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDevelopmentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodForestCentreControlObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicDataType>,
    #[serde(rename = "ControlForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub control_forest_use_declaration: Option<ControlForestUseDeclarationType>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicDataType>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicDataType>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<DamageDataType>,
    #[serde(rename = "ControlDataSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub control_data_special_features: Option<ControlDataSpecialFeaturesType>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignDataType>,
    #[serde(rename = "ObjectOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub object_overall_evaluation_data: Option<ControlOverallEvaluationDataType>,
    #[serde(rename = "ControlEvaluations", skip_serializing_if = "Option::is_none")]
    pub control_evaluations: Option<ControlEvaluationsType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceGroup {
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: ForestHaulageDistance,
    #[serde(rename = "StorageId")]
    pub storage_id: StorageId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaStandQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType2 {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsPurchaseModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaLawType {
    #[serde(flatten)]
    pub base: CoVirtaLawType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3TotalDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtSoilTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsIkaluokkaTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningDistrictType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSelfMonitoringTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummariesType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPostinumeroKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumeroTyyppi {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfLocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSahkoinenAsiointiTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierBaseContactAndEstateInfoType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<PaymentsRealEstatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceType {
    #[serde(rename = "XlinkresourceModel")]
    pub xlinkresource_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsType {
    #[serde(rename = "Payment")]
    pub payment: Vec<ForestCentrePaymentDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActDataType {
    #[serde(rename = "WorksDueDate")]
    pub works_due_date: CoDateType,
    #[serde(rename = "CompletionDeclarationDeliveryDueDate")]
    pub completion_declaration_delivery_due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetailsType {
    #[serde(rename = "SilvicultureRestrictionGroup")]
    pub silviculture_restriction_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<CoReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "ProcessingAreaNumber")]
    pub processing_area_number: ProcessingAreaNumberType,
    #[serde(rename = "ProcessingAreaReference")]
    pub processing_area_reference: CoReferenceType,
    #[serde(rename = "DeclarationStands")]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct toType {
    #[serde(flatten)]
    pub base: XsNCName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "OOffer", skip_serializing_if = "Option::is_none")]
    pub o_offer: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: PowerOfAttorneyType,
    #[serde(rename = "RightToSpecifyBankAccountsOfPaymentTransactions")]
    pub right_to_specify_bank_accounts_of_payment_transactions: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRestrictionsType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YesNoNotNeededType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionType {
    #[serde(rename = "Organisation1", skip_serializing_if = "Option::is_none")]
    pub organisation1: Option<BdtString20Type>,
    #[serde(rename = "Organisation2", skip_serializing_if = "Option::is_none")]
    pub organisation2: Option<BdtString20Type>,
    #[serde(rename = "Organisation3", skip_serializing_if = "Option::is_none")]
    pub organisation3: Option<BdtString20Type>,
    #[serde(rename = "Organisation4", skip_serializing_if = "Option::is_none")]
    pub organisation4: Option<BdtString20Type>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "Description")]
    pub description: BdtString100Type,
    #[serde(rename = "Code")]
    pub code: BdtString20Type,
    #[serde(rename = "MinDiameter")]
    pub min_diameter: BdtPositiveIntegerType,
    #[serde(rename = "MinLength")]
    pub min_length: BdtPositiveIntegerType,
    #[serde(rename = "UserCode")]
    pub user_code: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString50Type,
    #[serde(rename = "StartDate")]
    pub start_date: BdtTimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtTimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: BdtTimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
    #[serde(rename = "LoadCount")]
    pub load_count: XsnonNegativeInteger,
    #[serde(rename = "Load")]
    pub load: Vec<LoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StBaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYesNoNotKnownType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuntaKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDateType {
    #[serde(rename = "@type")]
    pub r#type: CoDatePrecipionType,
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentType {
    #[serde(flatten)]
    pub base: CoPercentWithFraction2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringImageType {
    #[serde(flatten)]
    pub base: ImageBaseType,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<BdtServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<BdtImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<BdtImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<BdtFeatureCodeType>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<GdtPointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<BdtString50Type>,
    #[serde(rename = "ImageDate")]
    pub image_date: BdtTimeStampType,
    #[serde(rename = "Filename")]
    pub filename: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "StartTime")]
    pub start_time: BdtTimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYearType {
    #[serde(flatten)]
    pub base: XsgYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtYesNoNotKnownType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaStandQualityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType {
    #[serde(rename = "CostTypeNumber")]
    pub cost_type_number: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataGroup {
    #[serde(rename = "MainGroup")]
    pub main_group: MainGroup,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageState>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonOrganizationNameType {
    #[serde(flatten)]
    pub base: OrganizationNameBaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTotalEstimationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfFeatureDataGroup {
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPlantEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "Question")]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRestrictionType {
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<BdtString1000Type>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<BdtDateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightType {
    #[serde(rename = "UsingRightExists")]
    pub using_right_exists: CoYesNoNotKnownType,
    #[serde(rename = "UsingRightExaminedDate")]
    pub using_right_examined_date: CoDateType,
    #[serde(rename = "UsingRightCompensation")]
    pub using_right_compensation: CoYesNoNotKnownType,
    #[serde(rename = "UsingRightCompensationAmount", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_amount: Option<CoDecimal7And2Type>,
    #[serde(rename = "UsingRightCompensationDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_description: Option<CoString1500Type>,
    #[serde(rename = "UsingRightCompensationResponsible")]
    pub using_right_compensation_responsible: CoUsingRightResponsibleType,
    #[serde(rename = "UsingRightDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_description: Option<CoString1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDataStandardSchemaPackageSubversionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityOfTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationType {
    #[serde(rename = "@assessorActorId")]
    pub assessor_actor_id: CoIdStringType,
    #[serde(rename = "@assessObjectActorId")]
    pub assess_object_actor_id: CoIdStringType,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<CoEvaluationCodeType>,
    #[serde(rename = "EvaluationText", skip_serializing_if = "Option::is_none")]
    pub evaluation_text: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTypeType {
    #[serde(flatten)]
    pub base: CoAreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiLastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtAssortmentGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHabitatSurvivingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger6digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCleanlinessClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptancesType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "BaBusinessAcceptance", skip_serializing_if = "Option::is_none")]
    pub ba_business_acceptance: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicDataType {
    #[serde(rename = "ProjectNo", skip_serializing_if = "Option::is_none")]
    pub project_no: Option<ProjectNoType>,
    #[serde(rename = "SelfMonitoringType", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_type: Option<CoSelfMonitoringTypeType>,
    #[serde(rename = "SelfMonitoringDate", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_date: Option<CoDateType>,
    #[serde(rename = "FccForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_use_declaration_number: Option<String>,
    #[serde(rename = "FccFinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_number: Option<String>,
    #[serde(rename = "FccCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YearType {
    #[serde(flatten)]
    pub base: XsgYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfBufferDistanceType {
    #[serde(flatten)]
    pub base: CoDecimal4And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simple {
    #[serde(rename = "XlinksimpleModel")]
    pub xlinksimple_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeIdentifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAgeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaTargetSelectionType {
    #[serde(flatten)]
    pub base: CoVirtaTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaAdvertiserType {
    #[serde(flatten)]
    pub base: CoVirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPreventionSubstanceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNameType {
    #[serde(flatten)]
    pub base: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(rename = "Contractors")]
    pub contractors: WctContractorsType,
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<BdtString100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "ResourceName")]
    pub resource_name: BdtString50Type,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<BdtMachineManufacturerType>,
    #[serde(rename = "Model", skip_serializing_if = "Option::is_none")]
    pub model: Option<BdtString50Type>,
    #[serde(rename = "ModelYear", skip_serializing_if = "Option::is_none")]
    pub model_year: Option<BdtYearType>,
    #[serde(rename = "HarvesterModel", skip_serializing_if = "Option::is_none")]
    pub harvester_model: Option<BdtString50Type>,
    #[serde(rename = "HavesterModelYear", skip_serializing_if = "Option::is_none")]
    pub havester_model_year: Option<BdtYearType>,
    #[serde(rename = "DeploymentYear", skip_serializing_if = "Option::is_none")]
    pub deployment_year: Option<BdtYearType>,
    #[serde(rename = "DeploymentMonth", skip_serializing_if = "Option::is_none")]
    pub deployment_month: Option<BdtString5Type>,
    #[serde(rename = "OwnWeight", skip_serializing_if = "Option::is_none")]
    pub own_weight: Option<BdtPositiveInteger6digitsType>,
    #[serde(rename = "WorkingWeight", skip_serializing_if = "Option::is_none")]
    pub working_weight: Option<BdtWorkingWeightType>,
    #[serde(rename = "CommunicationType", skip_serializing_if = "Option::is_none")]
    pub communication_type: Option<BdtString100Type>,
    #[serde(rename = "MeasuringDeviceVersion", skip_serializing_if = "Option::is_none")]
    pub measuring_device_version: Option<BdtString50Type>,
    #[serde(rename = "MeasuringDeviceLastControl", skip_serializing_if = "Option::is_none")]
    pub measuring_device_last_control: Option<BdtDateType>,
    #[serde(rename = "LoaderScaleModel", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model: Option<BdtString50Type>,
    #[serde(rename = "LoaderScaleModelYear", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model_year: Option<BdtYearType>,
    #[serde(rename = "ServiceStartDate")]
    pub service_start_date: BdtDateType,
    #[serde(rename = "OutOfServiceStartDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_start_date: Option<BdtDateType>,
    #[serde(rename = "OutOfServiceEndDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_end_date: Option<BdtDateType>,
    #[serde(rename = "LoadRating", skip_serializing_if = "Option::is_none")]
    pub load_rating: Option<WctLoadRatingType>,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<GdtPointGeometryType>,
    #[serde(rename = "ElevatorCertificate", skip_serializing_if = "Option::is_none")]
    pub elevator_certificate: Option<BdtYesNoType>,
    #[serde(rename = "ExtinguisherVerificationDate", skip_serializing_if = "Option::is_none")]
    pub extinguisher_verification_date: Option<BdtDateType>,
    #[serde(rename = "Telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<BdtString20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<BdtString50Type>,
    #[serde(rename = "SubContractorResource", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_resource: Option<BdtYesNoType>,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: BdtString20Type,
    #[serde(rename = "SubContractorVerified", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_verified: Option<BdtYesNoType>,
    #[serde(rename = "Removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<BdtYesNoType>,
    #[serde(rename = "RemoveDate", skip_serializing_if = "Option::is_none")]
    pub remove_date: Option<BdtDateType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "OrdererResponsibilityDocumentsChecked", skip_serializing_if = "Option::is_none")]
    pub orderer_responsibility_documents_checked: Option<BdtYesNoType>,
    #[serde(rename = "Peripherals", skip_serializing_if = "Option::is_none")]
    pub peripherals: Option<PeripheralsType>,
    #[serde(rename = "ExternalSystemInUse", skip_serializing_if = "Option::is_none")]
    pub external_system_in_use: Option<BdtYesNoType>,
    #[serde(rename = "ExternalSystemName", skip_serializing_if = "Option::is_none")]
    pub external_system_name: Option<BdtString50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionTypeType {
    #[serde(flatten)]
    pub base: CoVirtaInspectionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDataType {
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Data")]
    pub data: XshexBinary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataType {
    #[serde(rename = "SoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityType {
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: CoExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaryType {
    #[serde(rename = "Storey")]
    pub storey: BdtStoreyType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "AgeSummary", skip_serializing_if = "Option::is_none")]
    pub age_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StemCountSummary")]
    pub stem_count_summary: BdtPositiveInteger4digitsType,
    #[serde(rename = "BasalAreaSummary", skip_serializing_if = "Option::is_none")]
    pub basal_area_summary: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSummary")]
    pub mean_height_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "MeanDiameterSummary")]
    pub mean_diameter_summary: BdtPositiveInteger3digitsType,
    #[serde(rename = "VolumeSummary", skip_serializing_if = "Option::is_none")]
    pub volume_summary: Option<BdtPositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<String>,
    #[serde(rename = "GdtPolygonGeometry")]
    pub gdt_polygon_geometry: String,
    #[serde(rename = "GdtMultiPolygonGeometry")]
    pub gdt_multi_polygon_geometry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsMaatunnusKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationRegenerationOperationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandNumberExtensionType {
    #[serde(flatten)]
    pub base: StbStandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSahkopostiosoiteTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachmentType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<BdtString50Type>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: BdtDateType,
    #[serde(rename = "Version")]
    pub version: BdtString10Type,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString1000Type>,
    #[serde(rename = "Filename")]
    pub filename: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEtuNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaInspectionTypeType {
    #[serde(flatten)]
    pub base: CoVirtaInspectionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax1IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpVirtaEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataType {
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformationType {
    #[serde(rename = "ResponsibleOfPreClearing", skip_serializing_if = "Option::is_none")]
    pub responsible_of_pre_clearing: Option<WctResponsibleOfPreClearingType>,
    #[serde(rename = "PreClearingExecutionTime", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_execution_time: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document4MBType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesType {
    #[serde(rename = "Language")]
    pub language: Vec<BdtLanguageCode1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDataType {
    #[serde(rename = "ProposalType")]
    pub proposal_type: ProposalTypeType,
    #[serde(rename = "PlanningYearAndOperationUrgencyGroup")]
    pub planning_year_and_operation_urgency_group: String,
    #[serde(rename = "TimeBetweenProposalYearsGroup")]
    pub time_between_proposal_years_group: String,
    #[serde(rename = "ProposalAndOriginalYearGroup")]
    pub proposal_and_original_year_group: String,
    #[serde(rename = "ProposalDate")]
    pub proposal_date: ProposalDateType,
    #[serde(rename = "ProposalAreaGroup", skip_serializing_if = "Option::is_none")]
    pub proposal_area_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringUseCaseType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCaseType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetsType {
    #[serde(rename = "TgtTarget")]
    pub tgt_target: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValueType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedAmountType {
    #[serde(flatten)]
    pub base: CoDecimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsVoimassaoloKytkinTyyppi {
    #[serde(flatten)]
    pub base: Xsboolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDepotAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDateYYYY-MMOrYYYY-MM-DDType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkingSitePlanningOperationStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdentifierValueType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoOperationNatureManagementSpecifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVolumeGrowthType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctDitchTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xsbase64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsBICKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDataSourceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount")]
    pub completed_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformationType {
    #[serde(rename = "ServiceBuyer")]
    pub service_buyer: Vec<ServiceBuyerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryType {
    #[serde(rename = "DeliveryUserId")]
    pub delivery_user_id: BdtString50Type,
    #[serde(rename = "DeliveryInfo")]
    pub delivery_info: BdtString50Type,
    #[serde(rename = "DeliveryName")]
    pub delivery_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyType {
    #[serde(flatten)]
    pub base: CoCurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDataType {
    #[serde(rename = "DataInformation")]
    pub data_information: DataInformationType,
    #[serde(rename = "Inspection")]
    pub inspection: Vec<InspectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingRestrictionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaCuttingByMachineType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandsType {
    #[serde(rename = "DeclarationStand")]
    pub declaration_stand: Vec<DeclarationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPuhelinnumeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationBankAccountType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CoBankAccount")]
    pub co_bank_account: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString500Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "StandId")]
    pub stand_id: BdtString20Type,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPolygonOrMultiPolygon2Type,
    #[serde(rename = "RegisterUnitId", skip_serializing_if = "Option::is_none")]
    pub register_unit_id: Option<BdtString20Type>,
    #[serde(rename = "ParcelNo", skip_serializing_if = "Option::is_none")]
    pub parcel_no: Option<BdtPositiveInteger6digitsType>,
    #[serde(rename = "ParcelLabel", skip_serializing_if = "Option::is_none")]
    pub parcel_label: Option<BdtString100Type>,
    #[serde(rename = "ForestPlanStandId", skip_serializing_if = "Option::is_none")]
    pub forest_plan_stand_id: Option<BdtString10Type>,
    #[serde(rename = "DitchOrRoadPlanId", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_id: Option<BdtString10Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<BdtString100Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<BdtFertilityClassType>,
    #[serde(rename = "StumpTreatment", skip_serializing_if = "Option::is_none")]
    pub stump_treatment: Option<BdtYesNoType>,
    #[serde(rename = "BiomassCollection", skip_serializing_if = "Option::is_none")]
    pub biomass_collection: Option<BdtYesNoType>,
    #[serde(rename = "StumpRaising", skip_serializing_if = "Option::is_none")]
    pub stump_raising: Option<BdtYesNoType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<BdtWideDevelopmentClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<BdtSoilTypeType>,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<BdtYesNoType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "LabelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<GdtPointGeometryType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<BdtString1000Type>,
    #[serde(rename = "ForestUseDeclarationStandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_stand_extra_info: Option<BdtString2000Type>,
    #[serde(rename = "ForestUseDeclarationStandFellingPurpose", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_stand_felling_purpose: Option<BdtCuttingPurposeType>,
    #[serde(rename = "StandTreesCuttingVolume", skip_serializing_if = "Option::is_none")]
    pub stand_trees_cutting_volume: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "GrowingTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub growing_tree_species: Option<BdtTreeSpeciesType>,
    #[serde(rename = "TargetDensityGroup", skip_serializing_if = "Option::is_none")]
    pub target_density_group: Option<String>,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "StandTreesCutting", skip_serializing_if = "Option::is_none")]
    pub stand_trees_cutting: Option<StandTreesCuttingType>,
    #[serde(rename = "StandTreesCurrent", skip_serializing_if = "Option::is_none")]
    pub stand_trees_current: Option<StandTreesType>,
    #[serde(rename = "StandTreesStratumLeaving", skip_serializing_if = "Option::is_none")]
    pub stand_trees_stratum_leaving: Option<StandTreesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsType {
    #[serde(rename = "Product")]
    pub product: Vec<ProductType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctPricingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFinancingActFinancingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocument4MBType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTreeSpeciesType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTreeSpeciesType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeatureType {
    #[serde(rename = "HabitatCode", skip_serializing_if = "Option::is_none")]
    pub habitat_code: Option<CoExtendedHabitatCodeType>,
    #[serde(rename = "HabitatType", skip_serializing_if = "Option::is_none")]
    pub habitat_type: Option<CoHabitatTypeType>,
    #[serde(rename = "HabitatSurviving", skip_serializing_if = "Option::is_none")]
    pub habitat_surviving: Option<CoYesNoType>,
    #[serde(rename = "ExceptionalPermitForHandling", skip_serializing_if = "Option::is_none")]
    pub exceptional_permit_for_handling: Option<CoYesNoType>,
    #[serde(rename = "HabitatLocation", skip_serializing_if = "Option::is_none")]
    pub habitat_location: Option<CoHabitatLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayType {
    #[serde(rename = "CalendarDay")]
    pub calendar_day: BdtDateType,
    #[serde(rename = "Hours")]
    pub hours: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String500Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsUlkomaaHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimumType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMainWorkCodeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoQuantityUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPartNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStratumNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger1digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachineType {
    #[serde(rename = "MachineCode", skip_serializing_if = "Option::is_none")]
    pub machine_code: Option<CoMachineCodeType>,
    #[serde(rename = "MachineDescription", skip_serializing_if = "Option::is_none")]
    pub machine_description: Option<CoString500Type>,
    #[serde(rename = "MachineAccessoryCode", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_code: Option<CoMachineAccessoryCodeType>,
    #[serde(rename = "MachineAccessoryDescription", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_description: Option<CoString500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamagesType {
    #[serde(rename = "PreviousMooseDamage")]
    pub previous_moose_damage: Vec<PreviousMooseDamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearAndOperationUrgencyGroup {
    #[serde(rename = "PlanningYear")]
    pub planning_year: PlanningYear,
    #[serde(rename = "OperationUrgency")]
    pub operation_urgency: OperationUrgency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativeType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartsType {
    #[serde(rename = "TpTargetPart")]
    pub tp_target_part: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeaturesType {
    #[serde(rename = "ControlDataSpecialFeature")]
    pub control_data_special_feature: Vec<ControlDataSpecialFeatureType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRelatedType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstatesType {
    #[serde(rename = "FinancingActRealEstate")]
    pub financing_act_real_estate: Vec<FinancingActRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBasalAreaType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeCharType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "Status5")]
    pub status5: CoChangeStateType,
    #[serde(rename = "TreeNumber")]
    pub tree_number: Xsstring,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeDiameter", skip_serializing_if = "Option::is_none")]
    pub tree_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeHeight", skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "TreeCount", skip_serializing_if = "Option::is_none")]
    pub tree_count: Option<CoStemCountType>,
    #[serde(rename = "WorkQuality", skip_serializing_if = "Option::is_none")]
    pub work_quality: Option<VirtaWorkQualityType>,
    #[serde(rename = "DamageClass", skip_serializing_if = "Option::is_none")]
    pub damage_class: Option<VirtaDamageClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageType {
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "StorageDisplayId", skip_serializing_if = "Option::is_none")]
    pub storage_display_id: Option<BdtString20Type>,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPointAndLineOrPolygonType,
    #[serde(rename = "GeometryModificationAllowed")]
    pub geometry_modification_allowed: BdtYesNoType,
    #[serde(rename = "PlowingName", skip_serializing_if = "Option::is_none")]
    pub plowing_name: Option<BdtString50Type>,
    #[serde(rename = "PlowingTelephone", skip_serializing_if = "Option::is_none")]
    pub plowing_telephone: Option<BdtString20Type>,
    #[serde(rename = "PlowingEmail", skip_serializing_if = "Option::is_none")]
    pub plowing_email: Option<BdtString50Type>,
    #[serde(rename = "PlowingArranged", skip_serializing_if = "Option::is_none")]
    pub plowing_arranged: Option<BdtYesNoType>,
    #[serde(rename = "PlowingDate", skip_serializing_if = "Option::is_none")]
    pub plowing_date: Option<BdtDateType>,
    #[serde(rename = "Accessibility")]
    pub accessibility: BdtTransportAccessibilityType,
    #[serde(rename = "TurningPointClass", skip_serializing_if = "Option::is_none")]
    pub turning_point_class: Option<BdtTurningPointClassType>,
    #[serde(rename = "StorageInfo", skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<BdtString500Type>,
    #[serde(rename = "DeliveryRestriction", skip_serializing_if = "Option::is_none")]
    pub delivery_restriction: Option<BdtYesNoType>,
    #[serde(rename = "StorageName", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<BdtString50Type>,
    #[serde(rename = "StorageAddress", skip_serializing_if = "Option::is_none")]
    pub storage_address: Option<BdtString500Type>,
    #[serde(rename = "StorageDryingClass", skip_serializing_if = "Option::is_none")]
    pub storage_drying_class: Option<BdtStorageDryingClassType>,
    #[serde(rename = "StorageLandOwner", skip_serializing_if = "Option::is_none")]
    pub storage_land_owner: Option<BdtStorageLandOwnerType>,
    #[serde(rename = "StorageLandOwnerInformation", skip_serializing_if = "Option::is_none")]
    pub storage_land_owner_information: Option<BdtContactInformationType>,
    #[serde(rename = "StorageAdditionalRemarks", skip_serializing_if = "Option::is_none")]
    pub storage_additional_remarks: Option<BdtString3000Type>,
    #[serde(rename = "StorageLinkedToWorkingSite", skip_serializing_if = "Option::is_none")]
    pub storage_linked_to_working_site: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaRegenerationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightResponsibleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integer7digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDocumentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "SpecialFeatures")]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorsType {
    #[serde(rename = "ApplicationActor")]
    pub application_actor: Vec<ApplicationActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPurchaseModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbIdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString100Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: BdtString20Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "ControlReferenceMass")]
    pub control_reference_mass: BdtDecimal1FractionDigitType,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
    #[serde(rename = "ScaleData")]
    pub scale_data: Vec<ScaleDataType>,
    #[serde(rename = "Calibration", skip_serializing_if = "Option::is_none")]
    pub calibration: Option<Vec<CalibrationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctERPIdType {
    #[serde(flatten)]
    pub base: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataOperationStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerType {
    #[serde(rename = "Owner")]
    pub owner: BdtContactInformationType,
    #[serde(rename = "OwnerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub owner_representative_person: Option<BdtContactInformationType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "Bulk", skip_serializing_if = "Option::is_none")]
    pub bulk: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<WctQualityOfTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TgtVirtaExtraInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: BdtString10Type,
    #[serde(rename = "FeeType")]
    pub fee_type: BdtFeeBasisValueType,
    #[serde(rename = "FeeText")]
    pub fee_text: BdtString50Type,
    #[serde(rename = "FeeUnit", skip_serializing_if = "Option::is_none")]
    pub fee_unit: Option<BdtString10Type>,
    #[serde(rename = "InfoTextMandatory")]
    pub info_text_mandatory: BdtYesNoType,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<BdtString50Type>,
    #[serde(rename = "ValueList", skip_serializing_if = "Option::is_none")]
    pub value_list: Option<ValueListType>,
    #[serde(rename = "FeeBasisList", skip_serializing_if = "Option::is_none")]
    pub fee_basis_list: Option<FeeBasisListType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsPaayksikkoNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetPartStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseHarvestingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<BdtString20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingsType {
    #[serde(rename = "Training", skip_serializing_if = "Option::is_none")]
    pub training: Option<Vec<TrainingDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCallForOfferBusinessSenderRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FdForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "ReRealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<String>,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    #[serde(flatten)]
    pub base: XspositiveInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger3digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuvausTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoEvaluationSubjectType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStoreyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeHeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTimeType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "StartTime")]
    pub start_time: BdtTimeStampType,
    #[serde(rename = "EndTime")]
    pub end_time: BdtTimeStampType,
    #[serde(rename = "SavingTime")]
    pub saving_time: BdtTimeStampType,
    #[serde(rename = "Sawinghours", skip_serializing_if = "Option::is_none")]
    pub sawinghours: Option<SawinghoursDataType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "TrTree")]
    pub tr_tree: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorType {
    #[serde(rename = "ActorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<CoIdStringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPriorityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger5digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaSuggestionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaApprovalType {
    #[serde(flatten)]
    pub base: CoVirtaApprovalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesType {
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<StorageType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsOsoiteNumeroTyyppi {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctTaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListType {
    #[serde(rename = "FeeBasisListItem")]
    pub fee_basis_list_item: Vec<FeebasisListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumberType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeQualifierType1 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumeroTyyppi {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentPermissionDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String5000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemSelectionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(rename = "HopperNumber")]
    pub hopper_number: BdtString20Type,
    #[serde(rename = "HopperType")]
    pub hopper_type: WctHopperTypeType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPointGeometryType,
    #[serde(rename = "HopperLocationFromGPS")]
    pub hopper_location_from_g_p_s: BdtYesNoType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformationType {
    #[serde(rename = "UserId")]
    pub user_id: BdtString20Type,
    #[serde(rename = "Contractors")]
    pub contractors: WctContractorsType,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<WctTaxNumberType>,
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Address")]
    pub address: BdtString50Type,
    #[serde(rename = "PostalCode")]
    pub postal_code: BdtString10Type,
    #[serde(rename = "PostOffice")]
    pub post_office: BdtString50Type,
    #[serde(rename = "BirthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<BdtDateType>,
    #[serde(rename = "Telephone")]
    pub telephone: BdtString20Type,
    #[serde(rename = "ICEName", skip_serializing_if = "Option::is_none")]
    pub i_c_e_name: Option<BdtString100Type>,
    #[serde(rename = "ICETelephone", skip_serializing_if = "Option::is_none")]
    pub i_c_e_telephone: Option<BdtString20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<BdtString100Type>,
    #[serde(rename = "UserRoles")]
    pub user_roles: UserRolesType,
    #[serde(rename = "UserName")]
    pub user_name: BdtString100Type,
    #[serde(rename = "AdditionalName", skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<BdtString20Type>,
    #[serde(rename = "Trainings")]
    pub trainings: TrainingsType,
    #[serde(rename = "Employment")]
    pub employment: EmploymentDataType,
    #[serde(rename = "Machines", skip_serializing_if = "Option::is_none")]
    pub machines: Option<MachinesType>,
    #[serde(rename = "Languages")]
    pub languages: LanguagesType,
    #[serde(rename = "NationalityCode")]
    pub nationality_code: BdtString5Type,
    #[serde(rename = "NationalityFreeText", skip_serializing_if = "Option::is_none")]
    pub nationality_free_text: Option<BdtString50Type>,
    #[serde(rename = "WorkCodeGroups", skip_serializing_if = "Option::is_none")]
    pub work_code_groups: Option<WorkCodeGroupsType>,
    #[serde(rename = "Location")]
    pub location: GdtPointGeometryType,
    #[serde(rename = "E101")]
    pub e101: BdtYesNoType,
    #[serde(rename = "A1")]
    pub a1: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierBaseType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<String>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaximumType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtStoreyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "TravelStartTime")]
    pub travel_start_time: BdtTimeStampType,
    #[serde(rename = "TravelEndTime")]
    pub travel_end_time: BdtTimeStampType,
    #[serde(rename = "Vehicle")]
    pub vehicle: BdtVehicleType,
    #[serde(rename = "Kilometers")]
    pub kilometers: BdtPositiveInteger4digitsType,
    #[serde(rename = "Route")]
    pub route: BdtString200Type,
    #[serde(rename = "KilometersWithTrailer", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_trailer: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithCaravan", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_caravan: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithBreakHouse", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_break_house: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson1", skip_serializing_if = "Option::is_none")]
    pub extra_person1: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson2", skip_serializing_if = "Option::is_none")]
    pub extra_person2: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson3", skip_serializing_if = "Option::is_none")]
    pub extra_person3: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson4", skip_serializing_if = "Option::is_none")]
    pub extra_person4: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "ExtraPersonText", skip_serializing_if = "Option::is_none")]
    pub extra_person_text: Option<BdtString200Type>,
    #[serde(rename = "SittingMoneyKilometers", skip_serializing_if = "Option::is_none")]
    pub sitting_money_kilometers: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FacFinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessMessageTimeStampType {
    #[serde(flatten)]
    pub base: CoTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal5_1Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesType {
    #[serde(rename = "SpareTreeCategory")]
    pub spare_tree_category: BdtSpareTreeCategoryType,
    #[serde(rename = "AmountOfSpareTrees")]
    pub amount_of_spare_trees: BdtPositiveInteger5digitsType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<BdtTreeSpeciesType>,
    #[serde(rename = "MeanDiameterOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_of_spare_trees: Option<BdtDiameterType>,
    #[serde(rename = "MeanHeightOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_height_of_spare_trees: Option<BdtHeightType>,
    #[serde(rename = "VolumeOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub volume_of_spare_trees: Option<BdtVolumeType>,
    #[serde(rename = "DiameterClassOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub diameter_class_of_spare_trees: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaInspectionMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfFeatureInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccPowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString1500Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreSelfMonitoringObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatSurvivingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorsType {
    #[serde(rename = "ScaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<Vec<ScaleFactorDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkinTyyppi {
    #[serde(flatten)]
    pub base: Xsboolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: XshexBinary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDefType {
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@mainType")]
    pub main_type: MainTypeType,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "OperationStatus")]
    pub operation_status: CoOperationStatusType,
    #[serde(rename = "ActingDate")]
    pub acting_date: ActingDateType,
    #[serde(rename = "ResponsibleActor", skip_serializing_if = "Option::is_none")]
    pub responsible_actor: Option<ResponsibleActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategoryType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMeasurerTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationsType {
    #[serde(rename = "DeclarationOtherOperation")]
    pub declaration_other_operation: Vec<CoDeclarationOtherOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoWorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionGroup {
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<SilvicultureRestrictionEnds>,
    #[serde(rename = "SilvicultureRestriction")]
    pub silviculture_restriction: SilvicultureRestriction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax5IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctMonthType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPreclearingEvaluationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTargetSelectionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtServiceTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoIdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsValiaikainenHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonDataType {
    #[serde(rename = "DisQualificationReason")]
    pub dis_qualification_reason: BdtString10Type,
    #[serde(rename = "DisQualificationReasonText")]
    pub dis_qualification_reason_text: BdtString200Type,
    #[serde(rename = "DisQualificationPercentage")]
    pub dis_qualification_percentage: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString100Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMessageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<BdtString20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatOperationsType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "FacUpdatePreviousMessage")]
    pub fac_update_previous_message: String,
    #[serde(rename = "FacFinancingActApplicationReference")]
    pub fac_financing_act_application_reference: String,
    #[serde(rename = "FacFinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_number: Option<String>,
    #[serde(rename = "FacCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fac_customer_reference: Option<String>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: CoOtherPublicSubstituteType,
    #[serde(rename = "FacFinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_application_text_information: Option<String>,
    #[serde(rename = "FacLanguage")]
    pub fac_language: String,
    #[serde(rename = "FacSender")]
    pub fac_sender: String,
    #[serde(rename = "FacAttorney", skip_serializing_if = "Option::is_none")]
    pub fac_attorney: Option<String>,
    #[serde(rename = "FacSentDate")]
    pub fac_sent_date: String,
    #[serde(rename = "FacElectronicNotification")]
    pub fac_electronic_notification: String,
    #[serde(rename = "FinancingType")]
    pub financing_type: CoFinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "FacEstimatedStartDate")]
    pub fac_estimated_start_date: String,
    #[serde(rename = "FacEstimatedEndDate")]
    pub fac_estimated_end_date: String,
    #[serde(rename = "FacSubsidyAmount")]
    pub fac_subsidy_amount: String,
    #[serde(rename = "FacFinancingActWorkGroup")]
    pub fac_financing_act_work_group: String,
    #[serde(rename = "FacCopOperationProject")]
    pub fac_cop_operation_project: String,
    #[serde(rename = "FacFinancingActRealEstates")]
    pub fac_financing_act_real_estates: String,
    #[serde(rename = "FacApplicationActors")]
    pub fac_application_actors: String,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "FacDocuments", skip_serializing_if = "Option::is_none")]
    pub fac_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionsMainGroup {
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction: Option<CuttingRestriction>,
    #[serde(rename = "CuttingRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction_ends: Option<CuttingRestrictionEnds>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String1000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: CoReferenceType,
    #[serde(rename = "Area")]
    pub area: StbAreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<CoSubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<CoHeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<CoDiameterClassType>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication")]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<CoString2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperationType {
    #[serde(flatten)]
    pub base: CoStatisticsOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionModelGroup {
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSB,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistribution,
    #[serde(rename = "Weibull")]
    pub weibull: Weibull,
    #[serde(rename = "Normal")]
    pub normal: Normal,
    #[serde(rename = "Gamma")]
    pub gamma: Gamma,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate")]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
    #[serde(rename = "FinancingActCompletionGeometries")]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode1Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageDataType {
    #[serde(rename = "CommonMessageId", skip_serializing_if = "Option::is_none")]
    pub common_message_id: Option<WctCommonMessageType>,
    #[serde(rename = "CommonMessageFreeText", skip_serializing_if = "Option::is_none")]
    pub common_message_free_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWorkCodeType {
    #[serde(rename = "WorkingCode")]
    pub working_code: BdtWorkCodeType,
    #[serde(rename = "Amount")]
    pub amount: BdtDecimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<BdtDifficultyClassType>,
    #[serde(rename = "Attribute1", skip_serializing_if = "Option::is_none")]
    pub attribute1: Option<Vec<BdtWorkCodeQualifierType1>>,
    #[serde(rename = "Attribute2", skip_serializing_if = "Option::is_none")]
    pub attribute2: Option<Vec<BdtWorkCodeQualifierType2>>,
    #[serde(rename = "Attribute3", skip_serializing_if = "Option::is_none")]
    pub attribute3: Option<Vec<BdtWorkCodeQualifierType3>>,
    #[serde(rename = "Attribute4", skip_serializing_if = "Option::is_none")]
    pub attribute4: Option<Vec<BdtWorkCodeQualifierType4>>,
    #[serde(rename = "Attribute5", skip_serializing_if = "Option::is_none")]
    pub attribute5: Option<Vec<BdtWorkCodeQualifierType5>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveDecimalMax2IntegralPartMax1FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreaType {
    #[serde(rename = "Geometry")]
    pub geometry: GdtPolygonOrMultiPolygon2Type,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMaterialUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperType {
    #[serde(rename = "CaliperId", skip_serializing_if = "Option::is_none")]
    pub caliper_id: Option<BdtString200Type>,
    #[serde(rename = "CaliperApplication", skip_serializing_if = "Option::is_none")]
    pub caliper_application: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    #[serde(flatten)]
    pub base: StbStandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsKuudesRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtAssortmentStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionContactInformationType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "FirstName")]
    pub first_name: CiFirstNameType,
    #[serde(rename = "LastName")]
    pub last_name: CiLastNameType,
    #[serde(rename = "PersonOrganizationName")]
    pub person_organization_name: CiPersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingIncomeType {
    #[serde(flatten)]
    pub base: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtWideDevelopmentClassType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialUnitType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreWorkCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratumType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "CddDistributionModelGroup")]
    pub cdd_distribution_model_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveDecimalMax1IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSCCertificationSystemSubTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: StbAreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfIdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpSilvicultureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStandBasicDataDateType {
    #[serde(flatten)]
    pub base: StbStandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: CfowsWorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger4digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureDataGroup {
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCode>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoVATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ForestCentreDecision")]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageType {
    #[serde(flatten)]
    pub base: ImageBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "Category")]
    pub category: BdtImageCategoryType,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<BdtImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<BdtFeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<WctERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<BdtString20Type>,
    #[serde(rename = "Position")]
    pub position: GdtPointGeometryType,
    #[serde(rename = "InfoText")]
    pub info_text: BdtString200Type,
    #[serde(rename = "Photographer")]
    pub photographer: BdtString50Type,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "ImageDate")]
    pub image_date: BdtTimeStampType,
    #[serde(rename = "Filename")]
    pub filename: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: BdtDecimal3FractionDigitsType,
    #[serde(rename = "AmountNotified")]
    pub amount_notified: BdtDecimal3FractionDigitsType,
    #[serde(rename = "AmountAccounted")]
    pub amount_accounted: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectBasicDataType {
    #[serde(rename = "ObjectReferenceType", skip_serializing_if = "Option::is_none")]
    pub object_reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<CoReferenceType>,
    #[serde(rename = "ControlledOperationType", skip_serializing_if = "Option::is_none")]
    pub controlled_operation_type: Option<CoCostTypeNumberType>,
    #[serde(rename = "ControlledOperationDescription", skip_serializing_if = "Option::is_none")]
    pub controlled_operation_description: Option<CoString100Type>,
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFertilityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtHeightType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometriesGroup {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSoilConditioningBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAreaTypeType {
    #[serde(flatten)]
    pub base: CoAreaTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinktitleEltType {
    #[serde(rename = "XlinktitleModel")]
    pub xlinktitle_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentDetailsType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CaseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<FccFinancingActNumberType>,
    #[serde(rename = "FccForestCentreMessageReferenceType", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference_type: Option<String>,
    #[serde(rename = "FccForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub fcc_forest_centre_message_reference: Option<String>,
    #[serde(rename = "FccCompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_reference: Option<String>,
    #[serde(rename = "FccCompletionDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_completion_declaration_number: Option<String>,
    #[serde(rename = "FccCustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<String>,
    #[serde(rename = "FccArrivalDate")]
    pub fcc_arrival_date: String,
    #[serde(rename = "FccPaymentReference")]
    pub fcc_payment_reference: String,
    #[serde(rename = "FccPaymentDate")]
    pub fcc_payment_date: String,
    #[serde(rename = "FccBankAccount")]
    pub fcc_bank_account: String,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<CiContactInformationType>,
    #[serde(rename = "FccContactPerson")]
    pub fcc_contact_person: String,
    #[serde(rename = "PaymentTexts", skip_serializing_if = "Option::is_none")]
    pub payment_texts: Option<PaymentTextsType>,
    #[serde(rename = "FccOverallTotalSubsidy")]
    pub fcc_overall_total_subsidy: String,
    #[serde(rename = "SubsidyAppliers")]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSceneryWorkPermissionNeededType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StbStandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "SoilDataGroup")]
    pub soil_data_group: String,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: String,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StbStandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingDirectingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString2000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersType {
    #[serde(rename = "@subsidyPossibility")]
    pub subsidy_possibility: CoYesNoNotKnownType,
    #[serde(rename = "@sellerGroup")]
    pub seller_group: SellerGroupType,
    #[serde(rename = "Seller")]
    pub seller: Vec<SellerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<BdtString20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem", skip_serializing_if = "Option::is_none")]
    pub certification_system: Option<Vec<WtcoCertificationSystemType>>,
    #[serde(flatten)]
    pub base: BdtCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestActAreaType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoExtendedQuantityUnitType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyType {
    #[serde(flatten)]
    pub base: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompanyModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBetaType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompanyTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "EstateRegisterUnitGroup")]
    pub estate_register_unit_group: String,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub location_municipality_number: Option<CoMunicipalityNumberType>,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<CoMunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal1FractionDigitType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicity {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: Vec<String>,
    #[serde(rename = "Publicity")]
    pub publicity: CoPublicityType,
    #[serde(rename = "PublicityOrganizations", skip_serializing_if = "Option::is_none")]
    pub publicity_organizations: Option<OOrganizationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CiContactInformationType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hrefType {
    #[serde(flatten)]
    pub base: XsanyURI,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDocumentType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentDate", skip_serializing_if = "Option::is_none")]
    pub document_date: Option<DateType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDifficultyClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinklocatorType {
    #[serde(rename = "XlinklocatorModel")]
    pub xlinklocator_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointGeometryType {
    #[serde(rename = "Id")]
    pub id: Xsstring,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<Xsstring>,
    #[serde(rename = "GmlPoint")]
    pub gml_point: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPositiveInteger4digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiabilityType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetkiTyyppi {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanlinessClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeatureAdditionalCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSeedlingOriginType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCertificationSystemType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoForestDataUpdateUseCaseType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstateType {
    #[serde(flatten)]
    pub base: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditQuestionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSmallWoodRemovalClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfLocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "GdtAlternativeGeometries2Group")]
    pub gdt_alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtServiceNameofAPIType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListType {
    #[serde(rename = "ValueListItem")]
    pub value_list_item: Vec<ValueListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassifiedType {
    #[serde(rename = "AssortmentVolumeUnclassified")]
    pub assortment_volume_unclassified: Vec<AssortmentVolumeUnclassifiedType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "GdtSimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simpleModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTaxType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSilvicultureRestrictionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDiameterType {
    #[serde(flatten)]
    pub base: Decimal2And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FourDigitPositiveIntegerType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionType {
    #[serde(rename = "CoRestrictionType")]
    pub co_restriction_type: String,
    #[serde(rename = "CoRestrictionCode")]
    pub co_restriction_code: String,
    #[serde(rename = "RestrictionStartDate", skip_serializing_if = "Option::is_none")]
    pub restriction_start_date: Option<StartDateType>,
    #[serde(rename = "RestrictionEndDate", skip_serializing_if = "Option::is_none")]
    pub restriction_end_date: Option<EndDateType>,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "RestrictionDescription", skip_serializing_if = "Option::is_none")]
    pub restriction_description: Option<CoString1500Type>,
    #[serde(rename = "RestrictionOutOfObject", skip_serializing_if = "Option::is_none")]
    pub restriction_out_of_object: Option<CoYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTransportAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeysType {
    #[serde(rename = "StandKeyGroup1")]
    pub stand_key_group1: String,
    #[serde(rename = "StandKeyGroup2")]
    pub stand_key_group2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SfBasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentClassExtensionsType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoExtendedMainGroupType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsHuoneistotunnisteJakokirjainTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WctLoadRatingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagementType {
    #[serde(rename = "DitchMeanDepth")]
    pub ditch_mean_depth: Xsstring,
    #[serde(rename = "DitchMeanWidth")]
    pub ditch_mean_width: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSitesType {
    #[serde(rename = "ContractWorkingSiteDetails")]
    pub contract_working_site_details: Vec<ContractWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneAndTelefaxGroup {
    #[serde(rename = "TelefaxNumber", skip_serializing_if = "Option::is_none")]
    pub telefax_number: Option<TelefaxNumber>,
    #[serde(rename = "MobilePhoneNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<MobilePhoneNumber>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReplyType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Acceptance")]
    pub acceptance: CoAcceptanceType,
    #[serde(rename = "ReplyCode")]
    pub reply_code: CoReplyCodeType,
    #[serde(rename = "MessageType")]
    pub message_type: CoMessageType,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<CoReferenceType>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<CoDateType>,
    #[serde(rename = "RegistrationId", skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<CoReferenceType>,
    #[serde(rename = "ErrorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<ErrorMessagesType>,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtCompactSoilTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpfLocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "GdtSimpleAlternativeGeometriesGroup")]
    pub gdt_simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: Xsdecimal,
    #[serde(rename = "@location")]
    pub location: CoPointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: String,
    #[serde(rename = "GmllineStringProperty")]
    pub gmlline_string_property: String,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratumType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "DeadTreeType")]
    pub dead_tree_type: DeadTreeTypeType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtTurningPointClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCodeType {
    #[serde(flatten)]
    pub base: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoStumpBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsFaksinumeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYearType {
    #[serde(flatten)]
    pub base: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
    #[serde(flatten)]
    pub base: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsTreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TsTreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFinalAuditAnswerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsVakinainenKytkinTyyppi {
    #[serde(flatten)]
    pub base: Xsboolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationNatureManagementSpecifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureIdentifierExtensionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSukupuoliKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoLeafBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActorsType {
    #[serde(rename = "InformedActor")]
    pub informed_actor: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMainGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifierType5 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@type")]
    pub r#type: ObjectTypeType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ParentObjects", skip_serializing_if = "Option::is_none")]
    pub parent_objects: Option<ParentObjectsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StStandBasicDataType>,
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<String>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionsType {
    #[serde(rename = "UsingRestriction")]
    pub using_restriction: Vec<UsingRestrictionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPublicityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsToinenRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPracticeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtAlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransactionTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaProjectStatusType {
    #[serde(flatten)]
    pub base: CoVirtaProjectStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString5000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtScaleAssortmentType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<BdtServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMeasurementCertificateTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassifiedType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: BdtString50Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<BdtString50Type>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<BdtString5Type>,
    #[serde(rename = "ProductGroupName", skip_serializing_if = "Option::is_none")]
    pub product_group_name: Option<BdtString50Type>,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDeltaType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesType {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal4And2Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOfficeType {
    #[serde(flatten)]
    pub base: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationSpecificationType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentDate", skip_serializing_if = "Option::is_none")]
    pub document_date: Option<DateType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoRestrictionBasedOnStoninessType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomassType {
    #[serde(flatten)]
    pub base: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustificationsType {
    #[serde(rename = "Justification")]
    pub justification: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestActAreaType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccDecidedAmountUnitType {
    #[serde(flatten)]
    pub base: CoForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDataType {
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "PaymentType")]
    pub payment_type: PaymentTypeType,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "AdvanceTax")]
    pub advance_tax: AdvanceTaxType,
    #[serde(rename = "ForestFundPayment")]
    pub forest_fund_payment: ForestFundPaymentType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "DueDate")]
    pub due_date: DueDateType,
    #[serde(rename = "PaymentPermissionDate", skip_serializing_if = "Option::is_none")]
    pub payment_permission_date: Option<PaymentPermissionDateType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<PaymentDateType>,
    #[serde(rename = "Payee")]
    pub payee: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<BdtString100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccLocationEstateType {
    #[serde(flatten)]
    pub base: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoString1000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequests {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct String25Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaAdvertiserType {
    #[serde(flatten)]
    pub base: CoVirtaAdvertiserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaAdvertisementDatingType {
    #[serde(flatten)]
    pub base: CoVirtaAdvertisementDatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicDataType {
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<CoReferenceType>,
    #[serde(rename = "NonPersonificationId", skip_serializing_if = "Option::is_none")]
    pub non_personification_id: Option<CoString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "MooseDamageDeclarationReference")]
    pub moose_damage_declaration_reference: CoReferenceType,
    #[serde(rename = "MooseDamageDate")]
    pub moose_damage_date: CoDateType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<CoString2000Type>,
    #[serde(rename = "CompensationApplicant")]
    pub compensation_applicant: ContactInformationBankAccountType,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "FccAttorney", skip_serializing_if = "Option::is_none")]
    pub fcc_attorney: Option<String>,
    #[serde(rename = "MooseDamageDeclarationRealEstates")]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
    #[serde(rename = "CompensationByInsurance")]
    pub compensation_by_insurance: CompensationByInsuranceType,
    #[serde(rename = "CompensationByLegislation")]
    pub compensation_by_legislation: CompensationByLegislationType,
    #[serde(rename = "PreviousMooseDamages", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damages: Option<PreviousMooseDamagesType>,
    #[serde(rename = "FccDocuments", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoInventoryMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoAssortmentMainGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCodeType {
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinksimpleModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectType {
    #[serde(rename = "@id")]
    pub id: CoIdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: CoObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<String>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedCompletionDataType {
    #[serde(rename = "OperationStatus", skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<CoControlDataOperationStatusType>,
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString100Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
    #[serde(rename = "Stem")]
    pub stem: Vec<StemDataType>,
    #[serde(rename = "LengthCalibration", skip_serializing_if = "Option::is_none")]
    pub length_calibration: Option<Vec<LengthCalibrationType>>,
    #[serde(rename = "DiameterCalibration", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration: Option<Vec<DiameterCalibrationType>>,
    #[serde(rename = "Caliper", skip_serializing_if = "Option::is_none")]
    pub caliper: Option<CaliperType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationsType {
    #[serde(rename = "ControlEvaluation")]
    pub control_evaluation: Vec<ControlEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoYesNoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRegenerationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingRealizationPracticeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowsType {
    #[serde(rename = "RoundWoodSalesRow")]
    pub round_wood_sales_row: Vec<RoundWoodSalesRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DryingClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryObjectType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysisType {
    #[serde(rename = "ResultOfAccessibilityAnalysis")]
    pub result_of_accessibility_analysis: Vec<ResultOfAccessibilityAnalysisType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsAmmattiKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorDataType {
    #[serde(rename = "ScaleAssortmentType")]
    pub scale_assortment_type: BdtScaleAssortmentType,
    #[serde(rename = "LoadCount")]
    pub load_count: BdtPositiveIntegerType,
    #[serde(rename = "Area")]
    pub area: WctAreaCodeType,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "CodeName")]
    pub code_name: BdtString50Type,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<WctScaleFactorTreeSpeciesType>,
    #[serde(rename = "Month", skip_serializing_if = "Option::is_none")]
    pub month: Option<WctMonthType>,
    #[serde(rename = "HarvestingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub harvesting_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "SemiDry", skip_serializing_if = "Option::is_none")]
    pub semi_dry: Option<BdtYesNoType>,
    #[serde(rename = "SnowOrIce", skip_serializing_if = "Option::is_none")]
    pub snow_or_ice: Option<BdtYesNoType>,
    #[serde(rename = "WeightClass", skip_serializing_if = "Option::is_none")]
    pub weight_class: Option<BdtPositiveInteger1digitsType>,
    #[serde(rename = "Humidity", skip_serializing_if = "Option::is_none")]
    pub humidity: Option<BdtDecimal1FractionDigitType>,
    #[serde(rename = "HumidityMeasured", skip_serializing_if = "Option::is_none")]
    pub humidity_measured: Option<BdtYesNoType>,
    #[serde(rename = "ForestHaulageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_finished_date: Option<BdtTimeStampType>,
    #[serde(rename = "CleanlinessClass", skip_serializing_if = "Option::is_none")]
    pub cleanliness_class: Option<BdtCleanlinessClassType>,
    #[serde(rename = "Weight")]
    pub weight: BdtInteger7digitsType,
    #[serde(rename = "Density")]
    pub density: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "MeltedWater", skip_serializing_if = "Option::is_none")]
    pub melted_water: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal2And2PositiveType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataMessageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<CoYesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "SfUsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub sf_using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<CoDateType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TsTreeStandDataType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoActionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometriesType {
    #[serde(rename = "DecisionGeometry")]
    pub decision_geometry: Vec<DecisionGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReductionType {
    #[serde(rename = "StumpStemCount", skip_serializing_if = "Option::is_none")]
    pub stump_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StumpMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stump_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsNeljasRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCurrencyType {
    #[serde(flatten)]
    pub base: CoCurrencyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: JhsAlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductKeyGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSoilConditioningBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePhoneNumberType {
    #[serde(flatten)]
    pub base: PhoneNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureExtraQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVATStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataGroup {
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageState>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<MainGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "FinancingActCompletionGeometry")]
    pub financing_act_completion_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoSellerGroupType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoHabitatLocationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoObjectTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDensityGroup {
    #[serde(rename = "TargetDensity", skip_serializing_if = "Option::is_none")]
    pub target_density: Option<TargetDensity>,
    #[serde(rename = "DeciduousTreeTargetDensityPercent", skip_serializing_if = "Option::is_none")]
    pub deciduous_tree_target_density_percent: Option<DeciduousTreeTargetDensityPercent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiersType {
    #[serde(rename = "TreeIdentifier", skip_serializing_if = "Option::is_none")]
    pub tree_identifier: Option<Vec<TreeIdentifierType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationType {
    #[serde(rename = "EvaluationCategory", skip_serializing_if = "Option::is_none")]
    pub evaluation_category: Option<CoEvaluationSubjectType>,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<Xsinteger>,
    #[serde(rename = "EvaluationDescription", skip_serializing_if = "Option::is_none")]
    pub evaluation_description: Option<CoString1000Type>,
    #[serde(rename = "MainReason", skip_serializing_if = "Option::is_none")]
    pub main_reason: Option<CoYesNoType>,
    #[serde(rename = "ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<Xsinteger>,
    #[serde(rename = "ReasonDescription", skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataTreeStandDataDateType {
    #[serde(flatten)]
    pub base: TsTreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentWithFraction1Type {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesType {
    #[serde(rename = "Service")]
    pub service: Vec<OrganizationServiceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(rename = "InternalInspectionId")]
    pub internal_inspection_id: Xsstring,
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
    pub estate_owner: Option<Xsstring>,
    #[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub municipality_number: Option<JhsKuntaKoodiTyyppi>,
    #[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<Xsstring>,
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<Xsstring>,
    #[serde(rename = "UnseparatedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel_number: Option<Xsstring>,
    #[serde(rename = "UnseparatedParcel", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel: Option<Xsstring>,
    #[serde(rename = "KemeraMunicipalityId", skip_serializing_if = "Option::is_none")]
    pub kemera_municipality_id: Option<Xsstring>,
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
pub struct SilvicultureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCuttingByMachineType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestryType {
    #[serde(rename = "BlockIsFSFBlock", skip_serializing_if = "Option::is_none")]
    pub block_is_f_s_f_block: Option<BdtYesNoType>,
    #[serde(rename = "FSFInformation", skip_serializing_if = "Option::is_none")]
    pub f_s_f_information: Option<Vec<FSFInformationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoProviderRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbAreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StRestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtFeeBasisValueType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtString3000Type {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FudForestUseDeclarationType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<BdtDateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtDateType>,
    #[serde(rename = "ForestUseDeclarationNotNeeded", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_not_needed: Option<BdtYesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDataType {
    #[serde(rename = "MeasurementId")]
    pub measurement_id: BdtPositiveIntegerType,
    #[serde(rename = "Measurer")]
    pub measurer: BdtString50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "SelectionType")]
    pub selection_type: BdtString10Type,
    #[serde(rename = "Temperature")]
    pub temperature: Xsinteger,
    #[serde(rename = "ProductKey")]
    pub product_key: WctERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: BdtPositiveInteger5digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: BdtPositiveInteger5digitsType,
    #[serde(rename = "LogCount")]
    pub log_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "ControlLogCount")]
    pub control_log_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: CoMessageType,
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoDocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageType {
    #[serde(rename = "PreviousMooseDamageEvaluationDate", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_date: Option<CoDateType>,
    #[serde(rename = "PreviousMooseDamageEvaluationMunicipality", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_municipality: Option<CoMunicipalityNameType>,
    #[serde(rename = "PreviousSameAreaMooseDamageCompensationYear", skip_serializing_if = "Option::is_none")]
    pub previous_same_area_moose_damage_compensation_year: Option<CoYearType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccCostTypeDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesDataType {
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RoundWoodSalesRows")]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "DeclarationPolygonReference")]
    pub declaration_polygon_reference: CoReferenceType,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionsType {
    #[serde(rename = "SilvicultureRestrictionDetails")]
    pub silviculture_restriction_details: Vec<SilvicultureRestrictionDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "Description")]
    pub description: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "Objects")]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfoType {
    #[serde(rename = "PurchaseMode")]
    pub purchase_mode: PurchaseModeType,
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
    #[serde(rename = "IncludeForestFundPayment")]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
    #[serde(rename = "UsedPricingMethods", skip_serializing_if = "Option::is_none")]
    pub used_pricing_methods: Option<UsedPricingMethodsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: String,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: String,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<String>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "Name")]
    pub name: BdtString50Type,
    #[serde(rename = "PlannedVolume")]
    pub planned_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolume")]
    pub harvested_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolumeAccounted")]
    pub harvested_volume_accounted: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolume")]
    pub forwarded_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolumeAccounted")]
    pub forwarded_volume_accounted: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: BdtPositiveInteger4digitsType,
    #[serde(rename = "ForestHaulageDistanceContinued", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance_continued: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesType {
    #[serde(rename = "Machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<Vec<BdtMachineTypeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceIdMJ", skip_serializing_if = "Option::is_none")]
    pub resource_id_m_j: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: BdtString20Type,
    #[serde(rename = "ForestOwner", skip_serializing_if = "Option::is_none")]
    pub forest_owner: Option<BdtString100Type>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "DisQualificationPercentageTotal")]
    pub dis_qualification_percentage_total: BdtDecimal2FractionDigitsType,
    #[serde(rename = "CuttingAccuracy")]
    pub cutting_accuracy: BdtDecimal2FractionDigitsType,
    #[serde(rename = "DisQualificationSign")]
    pub dis_qualification_sign: BdtString5Type,
    #[serde(rename = "CuttingAccuracySign")]
    pub cutting_accuracy_sign: BdtString5Type,
    #[serde(rename = "Document", skip_serializing_if = "Option::is_none")]
    pub document: Option<Xsbase64Binary>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Xsbase64Binary>,
    #[serde(rename = "DisQualificationReasons")]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerResponsible {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDurationType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoUseCaseType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoProposalTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingDirectingType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SfLocatedSpecialFeature1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveInteger5digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FacFinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub fac_financing_act_completion_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistributionType {
    #[serde(rename = "CumulativePoint")]
    pub cumulative_point: Vec<CumulativePointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataType {
    #[serde(rename = "TrainingId")]
    pub training_id: BdtPositiveInteger4digitsType,
    #[serde(rename = "TrainingFreeText", skip_serializing_if = "Option::is_none")]
    pub training_free_text: Option<BdtString50Type>,
    #[serde(rename = "TrainingDate")]
    pub training_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationOfAttorneyType {
    #[serde(rename = "Authorization")]
    pub authorization: CoYesNoType,
    #[serde(rename = "Date")]
    pub date: CoDateType,
    #[serde(rename = "AttorneyReceivesPayment")]
    pub attorney_receives_payment: CoYesNoType,
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: CoString2000Type,
    #[serde(rename = "PowerOfAttorneyDocument", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_document: Option<Xmimebase64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsLajiTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoBankReferenceNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebasisListItemType {
    #[serde(rename = "Id")]
    pub id: BdtPositiveIntegerType,
    #[serde(rename = "FeeText")]
    pub fee_text: BdtString50Type,
    #[serde(rename = "FeeUnit")]
    pub fee_unit: BdtString10Type,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<BdtString50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoStemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTimeStampType {
    #[serde(flatten)]
    pub base: XsdateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageType {
    #[serde(rename = "MainDamage", skip_serializing_if = "Option::is_none")]
    pub main_damage: Option<CoYesNoType>,
    #[serde(rename = "DamageSourceCode", skip_serializing_if = "Option::is_none")]
    pub damage_source_code: Option<Xsstring>,
    #[serde(rename = "DamageSourceDescription", skip_serializing_if = "Option::is_none")]
    pub damage_source_description: Option<CoString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationStatusType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EbEnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeHeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectProtectionOperationsDataType {
    #[serde(rename = "BorderingWithWaterAreaOrStream", skip_serializing_if = "Option::is_none")]
    pub bordering_with_water_area_or_stream: Option<CoYesNoType>,
    #[serde(rename = "LengthOfDitchDiggedDuringSoilPreparation", skip_serializing_if = "Option::is_none")]
    pub length_of_ditch_digged_during_soil_preparation: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "ObjectProtectionOperations", skip_serializing_if = "Option::is_none")]
    pub object_protection_operations: Option<ObjectProtectionOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageDataType {
    #[serde(rename = "Damage")]
    pub damage: Vec<DamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsSyntymaPvmTyyppi {
    #[serde(flatten)]
    pub base: Xsdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsEnsimmainenRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsViidesRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: CoFeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<SfValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<SfFeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<SfFeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<CoDateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: CoForestCentreWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: CoForestCentreWorkCodeType,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReferenceType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: Decimal4FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidityType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<BdtDateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@versionNo")]
    pub version_no: VersionNoType,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: MeasurementDateType,
    #[serde(rename = "InsertDate")]
    pub insert_date: InsertDateType,
    #[serde(rename = "MeasurementCertificateType")]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<TextType>,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "PaidValue", skip_serializing_if = "Option::is_none")]
    pub paid_value: Option<PaidValueType>,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: String,
    #[serde(rename = "PaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub payment_transactions: Option<WtcoPaymentTransactionsType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageAccessibilityType {
    #[serde(flatten)]
    pub base: CoAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(rename = "PaymentTransactionCategory")]
    pub payment_transaction_category: CoMoneyTransactionCategoryType,
    #[serde(rename = "PaymentTransactionType")]
    pub payment_transaction_type: CoMoneyTransactionTypeType,
    #[serde(rename = "Value")]
    pub value: CoMoneyType,
    #[serde(rename = "PaymentTransactionAsUnitGroup")]
    pub payment_transaction_as_unit_group: String,
    #[serde(rename = "Currency")]
    pub currency: CoCurrencyType,
    #[serde(rename = "PaymentTransactionDescription", skip_serializing_if = "Option::is_none")]
    pub payment_transaction_description: Option<PaymentTransactionDescriptionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotEmptyTreeSpeciesType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemsType {
    #[serde(rename = "QualitySystem", skip_serializing_if = "Option::is_none")]
    pub quality_system: Option<Vec<BdtQualitySystemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot", skip_serializing_if = "Option::is_none")]
    pub sample_plot: Option<Vec<SamplePlotType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoTransportAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingSeasonType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoPurchaseModeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoMainTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: BdtYesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<BdtYesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "LimitsToWaterSystem", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system: Option<BdtYesNoType>,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<BdtString200Type>,
    #[serde(rename = "WaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub water_system_protection: Option<BdtYesNoType>,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingSafetyNoticed", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed: Option<BdtYesNoType>,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingInstructionsSufficient", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient: Option<BdtYesNoType>,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<BdtString200Type>,
    #[serde(rename = "StumpLiftingSuitable")]
    pub stump_lifting_suitable: BdtYesNoType,
    #[serde(rename = "StumpLiftingSuitableText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_suitable_text: Option<BdtString200Type>,
    #[serde(rename = "StumpCuttingAsInstructed")]
    pub stump_cutting_as_instructed: BdtYesNoType,
    #[serde(rename = "StumpCuttingAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub stump_cutting_as_instructed_text: Option<BdtString200Type>,
    #[serde(rename = "StumpTidiness", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness: Option<BdtYesNoType>,
    #[serde(rename = "StumpTidinessText", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness_text: Option<BdtString200Type>,
    #[serde(rename = "RemainingStumpCount", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count: Option<BdtYesNoType>,
    #[serde(rename = "RemainingStumpCountText", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count_text: Option<BdtString200Type>,
    #[serde(rename = "StumpLiftingQuality", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality: Option<BdtWorkingQualityType>,
    #[serde(rename = "StumpLiftingQualityText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectingMethodType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISO639char2LanguageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtDecimal2FractionDigitsType {
    #[serde(flatten)]
    pub base: Xsdecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XlinktitleModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDensityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionType {
    #[serde(rename = "SceneryWorkPermissionNeeded")]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
    #[serde(rename = "SceneryWorkPermissionAcceptance", skip_serializing_if = "Option::is_none")]
    pub scenery_work_permission_acceptance: Option<CoDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoutTimberClassifierType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeDataType {
    #[serde(rename = "OperationTreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub operation_tree_species_summary: Option<OperationTreeSpeciesSummaryType>,
    #[serde(rename = "GrowingTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub growing_tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "MaterialAmount", skip_serializing_if = "Option::is_none")]
    pub material_amount: Option<CoDecimal2FractionDigitsType>,
    #[serde(rename = "MaterialAmountUnit", skip_serializing_if = "Option::is_none")]
    pub material_amount_unit: Option<CoMaterialUnitType>,
    #[serde(rename = "TargetStemCount", skip_serializing_if = "Option::is_none")]
    pub target_stem_count: Option<CoStemCountType>,
    #[serde(rename = "TargetBasalArea", skip_serializing_if = "Option::is_none")]
    pub target_basal_area: Option<CoBasalAreaType>,
    #[serde(rename = "TargetAmount", skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<AmountType>,
    #[serde(rename = "TargetAmountUnit", skip_serializing_if = "Option::is_none")]
    pub target_amount_unit: Option<ExtendedWideUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordTreeSpeciesType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: OwsWorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartType {
    #[serde(rename = "Status3", skip_serializing_if = "Option::is_none")]
    pub status3: Option<CoChangeStateType>,
    #[serde(rename = "PartNumber")]
    pub part_number: Xsstring,
    #[serde(rename = "PartsDetectedArea", skip_serializing_if = "Option::is_none")]
    pub parts_detected_area: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "CuttingMethod", skip_serializing_if = "Option::is_none")]
    pub cutting_method: Option<OpCuttingTypeType>,
    #[serde(rename = "RegenerationType", skip_serializing_if = "Option::is_none")]
    pub regeneration_type: Option<VirtaRegenerationType>,
    #[serde(rename = "TargetPartStatus", skip_serializing_if = "Option::is_none")]
    pub target_part_status: Option<VirtaTargetPartStatusType>,
    #[serde(rename = "OperationDate", skip_serializing_if = "Option::is_none")]
    pub operation_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "OperationYear", skip_serializing_if = "Option::is_none")]
    pub operation_year: Option<CoYearType>,
    #[serde(rename = "Classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<CoPositiveIntegerType>,
    #[serde(rename = "Review", skip_serializing_if = "Option::is_none")]
    pub review: Option<VirtaReviewType>,
    #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<VirtaReasonType>,
    #[serde(rename = "Review2", skip_serializing_if = "Option::is_none")]
    pub review2: Option<VirtaReviewType>,
    #[serde(rename = "Reason2", skip_serializing_if = "Option::is_none")]
    pub reason2: Option<VirtaReasonType>,
    #[serde(rename = "InspectionMethod", skip_serializing_if = "Option::is_none")]
    pub inspection_method: Option<VirtaInspectionMethodType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "SeedStemCount", skip_serializing_if = "Option::is_none")]
    pub seed_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StubStemCount", skip_serializing_if = "Option::is_none")]
    pub stub_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "CultivatedCropStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "NaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub natural_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "DeadStemCount", skip_serializing_if = "Option::is_none")]
    pub dead_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<Xsdecimal>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<VirtaStandQualityType>,
    #[serde(rename = "HabitatCode", skip_serializing_if = "Option::is_none")]
    pub habitat_code: Option<VirtaHabitatCodeType>,
    #[serde(rename = "HabitatType", skip_serializing_if = "Option::is_none")]
    pub habitat_type: Option<VirtaHabitatTypeType>,
    #[serde(rename = "HabitatSurviving", skip_serializing_if = "Option::is_none")]
    pub habitat_surviving: Option<VirtaHabitatSurvivingType>,
    #[serde(rename = "ExceptionalPermitForHandling", skip_serializing_if = "Option::is_none")]
    pub exceptional_permit_for_handling: Option<VirtaExceptionalPermitForHandlingType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<CoMainGroupType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<CoDrainageStateType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "PineDecimal", skip_serializing_if = "Option::is_none")]
    pub pine_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SpruceDecimal", skip_serializing_if = "Option::is_none")]
    pub spruce_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "BirchDecimal", skip_serializing_if = "Option::is_none")]
    pub birch_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "StockingWithSeedlings", skip_serializing_if = "Option::is_none")]
    pub stocking_with_seedlings: Option<CoVirtaYesNoType>,
    #[serde(rename = "GroundManipulationMethod", skip_serializing_if = "Option::is_none")]
    pub ground_manipulation_method: Option<VirtaGroundManipulationMethodType>,
    #[serde(rename = "SoilImprovementEvaluation", skip_serializing_if = "Option::is_none")]
    pub soil_improvement_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "ClearingEstimation", skip_serializing_if = "Option::is_none")]
    pub clearing_estimation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "DamageSource", skip_serializing_if = "Option::is_none")]
    pub damage_source: Option<CoFeatureTypeType>,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<OpSilvicultureTypeType>,
    #[serde(rename = "CultivationMaterial", skip_serializing_if = "Option::is_none")]
    pub cultivation_material: Option<VirtaCultivationMaterialType>,
    #[serde(rename = "PlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub plant_evaluation: Option<VirtaPlantEvaluationType>,
    #[serde(rename = "GrassControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub grass_control_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "SproutForestControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub sprout_forest_control_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "SeedPlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub seed_plant_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "RootRotControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub root_rot_control_evaluation: Option<VirtaRootRotControlEvaluationType>,
    #[serde(rename = "HarvestingClassifiation", skip_serializing_if = "Option::is_none")]
    pub harvesting_classifiation: Option<VirtaHarvestingClassificationType>,
    #[serde(rename = "RootDamageCount", skip_serializing_if = "Option::is_none")]
    pub root_damage_count: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "StemDamageCount", skip_serializing_if = "Option::is_none")]
    pub stem_damage_count: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "TotalEstimation", skip_serializing_if = "Option::is_none")]
    pub total_estimation: Option<VirtaTotalEstimationType>,
    #[serde(rename = "CuttingByMachine", skip_serializing_if = "Option::is_none")]
    pub cutting_by_machine: Option<VirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<VirtaHarvestingSeasonType>,
    #[serde(rename = "PartEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_easting_coordinate: Option<Xsstring>,
    #[serde(rename = "PartNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_northing_coordinate: Option<Xsstring>,
    #[serde(rename = "NotDamagedCount", skip_serializing_if = "Option::is_none")]
    pub not_damaged_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class1damageCount", skip_serializing_if = "Option::is_none")]
    pub class1damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class2damageCount", skip_serializing_if = "Option::is_none")]
    pub class2damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class3damageCount", skip_serializing_if = "Option::is_none")]
    pub class3damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class4damageCount", skip_serializing_if = "Option::is_none")]
    pub class4damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "SmallPeelDamage", skip_serializing_if = "Option::is_none")]
    pub small_peel_damage: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "LargePeelDamage", skip_serializing_if = "Option::is_none")]
    pub large_peel_damage: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "DamagedSeedlingCount", skip_serializing_if = "Option::is_none")]
    pub damaged_seedling_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "PineLog", skip_serializing_if = "Option::is_none")]
    pub pine_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "SpruceLog", skip_serializing_if = "Option::is_none")]
    pub spruce_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "BirchLog", skip_serializing_if = "Option::is_none")]
    pub birch_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "PinePulp", skip_serializing_if = "Option::is_none")]
    pub pine_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "LogM3sum", skip_serializing_if = "Option::is_none")]
    pub log_m3sum: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "PulpM3sum", skip_serializing_if = "Option::is_none")]
    pub pulp_m3sum: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "SprucePulp", skip_serializing_if = "Option::is_none")]
    pub spruce_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "BirchPulp", skip_serializing_if = "Option::is_none")]
    pub birch_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "OtherTimberValue", skip_serializing_if = "Option::is_none")]
    pub other_timber_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "EnergyTimberValue", skip_serializing_if = "Option::is_none")]
    pub energy_timber_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "CuttingValue", skip_serializing_if = "Option::is_none")]
    pub cutting_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "DestroyedCuttingValue", skip_serializing_if = "Option::is_none")]
    pub destroyed_cutting_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "ExpectedValueCoefficient", skip_serializing_if = "Option::is_none")]
    pub expected_value_coefficient: Option<CoPositiveDecimalMax1IntegralPartMax2FractionalPartType>,
    #[serde(rename = "ExpectedValueSurplus", skip_serializing_if = "Option::is_none")]
    pub expected_value_surplus: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Compensation", skip_serializing_if = "Option::is_none")]
    pub compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AlreadyPaidCompensation", skip_serializing_if = "Option::is_none")]
    pub already_paid_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "NeedForTreatment", skip_serializing_if = "Option::is_none")]
    pub need_for_treatment: Option<CoVirtaYesNoType>,
    #[serde(rename = "Suggestion", skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<VirtaSuggestionType>,
    #[serde(rename = "Phase2youngCropCount", skip_serializing_if = "Option::is_none")]
    pub phase2young_crop_count: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RecommendedDensity", skip_serializing_if = "Option::is_none")]
    pub recommended_density: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RepairPlantingCosts", skip_serializing_if = "Option::is_none")]
    pub repair_planting_costs: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
    #[serde(rename = "GeometryStatus", skip_serializing_if = "Option::is_none")]
    pub geometry_status: Option<Xsstring>,
    #[serde(rename = "GeometryId", skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<Xsstring>,
    #[serde(rename = "GmlPolygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortERPIdType {
    #[serde(flatten)]
    pub base: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeInfoType {
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Productivity")]
    pub productivity: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "ProductivityUnit")]
    pub productivity_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPulpWoodVolumeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsIBANTunnusTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSoilConditioningBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoCuttingExtraQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTextsType {
    #[serde(rename = "PaymentText")]
    pub payment_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoVirtaHarvestingSeasonType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType2,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcoSellersType {
    #[serde(rename = "@subsidyPossibility")]
    pub subsidy_possibility: CoYesNoNotKnownType,
    #[serde(rename = "@sellerGroup")]
    pub seller_group: SellerGroupType,
    #[serde(rename = "Seller")]
    pub seller: Vec<SellerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrixType {
    #[serde(rename = "PriceItem")]
    pub price_item: Vec<PriceItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateMmDdYyyyType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcType {
    #[serde(rename = "XlinkarcModel")]
    pub xlinkarc_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationCommitmentType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityGroup {
    #[serde(rename = "Quantity")]
    pub quantity: Quantity,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: QuantityUnit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisksType {
    #[serde(rename = "WorkSafetyRiskDescription")]
    pub work_safety_risk_description: Vec<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonOperationExtraQualifierType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingClassificationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumberType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlBaseCuttingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<BdtDateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentChangeDataType {
    #[serde(rename = "OldDestinationStorage")]
    pub old_destination_storage: BdtString20Type,
    #[serde(rename = "OldCode")]
    pub old_code: BdtString50Type,
    #[serde(rename = "NewDestinationStorage")]
    pub new_destination_storage: BdtString20Type,
    #[serde(rename = "NewCode")]
    pub new_code: BdtString50Type,
    #[serde(rename = "ChangeVolume")]
    pub change_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseCompactStandBasicDataType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CoCompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<StbStandNumberType>,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StbStandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<CoYearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<CoYearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<CoAccessibilityType>,
    #[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data_date: Option<StbStandBasicDataDateType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StbStandInfoType>,
    #[serde(rename = "CoDataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<CoDataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: SpecificationCodeType,
    #[serde(rename = "SpecificationText", skip_serializing_if = "Option::is_none")]
    pub specification_text: Option<CoString2000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: ReRealEstatesWithOwnersInformationType2,
    #[serde(rename = "DeclarationPolygons")]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfoType {
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<WtcoIncludePaymentPlanType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingVolumeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibilityType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsuranceType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "InsuranceCompany", skip_serializing_if = "Option::is_none")]
    pub insurance_company: Option<CoString500Type>,
    #[serde(rename = "InsuranceNumber", skip_serializing_if = "Option::is_none")]
    pub insurance_number: Option<CoString100Type>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: CoIdStringNotEmptyType,
    #[serde(rename = "PayeeId")]
    pub payee_id: CoIdStringNotEmptyType,
    #[serde(rename = "ParticipationPercentage", skip_serializing_if = "Option::is_none")]
    pub participation_percentage: Option<CoPercentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GdtPolygonOrMultiPolygon2Type {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<String>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositiveInteger6digitsType {
    #[serde(flatten)]
    pub base: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorType {
    #[serde(rename = "XlinklocatorModel")]
    pub xlinklocator_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "StatisticsOperation")]
    pub statistics_operation: StatisticsOperationType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "ReportedStatisticsOperationType")]
    pub reported_statistics_operation_type: CoReportedStatisticsOperationTypeType,
    #[serde(rename = "StatisticsQuantities")]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtBearingCapacityClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStringNotEmptyType {
    #[serde(flatten)]
    pub base: IdStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisValueType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoFeatureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XshexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: String,
    #[serde(flatten)]
    pub base: XshexBinary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<Vec<AssortmentType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtMaterialCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbAreaDecreaseType {
    #[serde(flatten)]
    pub base: StbAreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FccFinancingActNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDifficultyClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JhsValtiotunnusKoodiTyyppi {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TsTreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoInfoProviderRoleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanType {
    #[serde(flatten)]
    pub base: Xsfloat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "ProductUserId", skip_serializing_if = "Option::is_none")]
    pub product_user_id: Option<BdtString50Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "FileFormat")]
    pub file_format: BdtString5Type,
    #[serde(rename = "Label")]
    pub label: BdtString100Type,
    #[serde(rename = "DocumentClass", skip_serializing_if = "Option::is_none")]
    pub document_class: Option<BdtDocumentClassType>,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupType2 {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreasType {
    #[serde(rename = "WorkingArea", skip_serializing_if = "Option::is_none")]
    pub working_area: Option<Vec<WorkingAreaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLineAndPolygonGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentType {
    #[serde(rename = "SubsidyArgumentText")]
    pub subsidy_argument_text: Vec<CoString5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationSubjectType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BdtClientApplicationIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationDevelopmentClassType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoPaymentTypeType {
    #[serde(flatten)]
    pub base: CoPaymentTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "Id")]
    pub id: Xsstring,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<Xsstring>,
    #[serde(rename = "GmlPolygon")]
    pub gml_polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoISO639char2LanguageType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoDeclarationSoilPreparationOperationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: String,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Xsstring>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<WtcoDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WtcTotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

