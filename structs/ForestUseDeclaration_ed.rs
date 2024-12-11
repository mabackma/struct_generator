#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDevelopmentClass {
    #[serde(flatten)]
    pub declaration_development_class: DeclarationDevelopmentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherHabitatCode {
    #[serde(flatten)]
    pub other_habitat_code: OtherHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationCommitment {
    #[serde(flatten)]
    pub declaration_regeneration_commitment: RegenerationCommitmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: ExtendedMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperations {
    #[serde(flatten)]
    pub declaration_other_operations: DeclarationOtherOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperations {
    #[serde(flatten)]
    pub habitat_operations: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaReference {
    #[serde(flatten)]
    pub processing_area_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: HabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: SubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreas {
    #[serde(flatten)]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationSoilPreparationOperation {
    #[serde(flatten)]
    pub declaration_soil_preparation_operation: DeclarationSoilPreparationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousDeclaration {
    #[serde(flatten)]
    pub update_previous_declaration: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanAge {
    #[serde(flatten)]
    pub mean_age: AgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsRightsOwnerRepresentative {
    #[serde(flatten)]
    pub cuttings_rights_owner_representative: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPurpose {
    #[serde(flatten)]
    pub cutting_purpose: CuttingPurposeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumber {
    #[serde(flatten)]
    pub processing_area_number: ProcessingAreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPractice {
    #[serde(flatten)]
    pub cutting_realization_practice: CuttingRealizationPracticeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationMainTreeSpecies {
    #[serde(flatten)]
    pub declaration_main_tree_species: TreeSpeciesConciseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperation {
    #[serde(flatten)]
    pub habitat_operation: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStands {
    #[serde(flatten)]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandReference {
    #[serde(flatten)]
    pub declaration_stand_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationTextInformation {
    #[serde(flatten)]
    pub declaration_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandTextInformation {
    #[serde(flatten)]
    pub declaration_stand_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRegenerationOperation {
    #[serde(flatten)]
    pub declaration_regeneration_operation: DeclarationRegenerationOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: LocatedSpecialFeature3Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingArea {
    #[serde(flatten)]
    pub processing_area: ProcessingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRightsOwner {
    #[serde(flatten)]
    pub cutting_rights_owner: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstates {
    #[serde(flatten)]
    pub declaration_real_estates: DeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStand {
    #[serde(flatten)]
    pub declaration_stand: DeclarationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDamageQualifier {
    #[serde(flatten)]
    pub forest_damage_qualifier: ForestDamageQualifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: MeanDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: SenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperation {
    #[serde(flatten)]
    pub declaration_other_operation: DeclarationOtherOperationType,
}

