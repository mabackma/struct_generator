use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingAreaNumber {
    #[serde(flatten)]
    pub processing_area_number: ProcessingAreaNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherHabitatCode {
    #[serde(flatten)]
    pub other_habitat_code: CoOtherHabitatCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStands {
    #[serde(flatten)]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRegenerationCommitment {
    #[serde(flatten)]
    pub declaration_regeneration_commitment: CoRegenerationCommitmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRealEstates {
    #[serde(flatten)]
    pub declaration_real_estates: DeclarationRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingsRightsOwnerRepresentative {
    #[serde(flatten)]
    pub cuttings_rights_owner_representative: CiContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationOtherOperations {
    #[serde(flatten)]
    pub declaration_other_operations: DeclarationOtherOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationMainTreeSpecies {
    #[serde(flatten)]
    pub declaration_main_tree_species: CoTreeSpeciesConciseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStandReference {
    #[serde(flatten)]
    pub declaration_stand_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingRightsOwner {
    #[serde(flatten)]
    pub cutting_rights_owner: CiContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingPurpose {
    #[serde(flatten)]
    pub cutting_purpose: CoCuttingPurposeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatOperation {
    #[serde(flatten)]
    pub habitat_operation: CoHabitatOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStandTextInformation {
    #[serde(flatten)]
    pub declaration_stand_text_information: CoString2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDamageQualifier {
    #[serde(flatten)]
    pub forest_damage_qualifier: CoForestDamageQualifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationStand {
    #[serde(flatten)]
    pub declaration_stand: DeclarationStandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRegenerationOperation {
    #[serde(flatten)]
    pub declaration_regeneration_operation: CoDeclarationRegenerationOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingAreas {
    #[serde(flatten)]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationOtherOperation {
    #[serde(flatten)]
    pub declaration_other_operation: CoDeclarationOtherOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingAreaReference {
    #[serde(flatten)]
    pub processing_area_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationTextInformation {
    #[serde(flatten)]
    pub declaration_text_information: CoString2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePreviousDeclaration {
    #[serde(flatten)]
    pub update_previous_declaration: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationSoilPreparationOperation {
    #[serde(flatten)]
    pub declaration_soil_preparation_operation: CoDeclarationSoilPreparationOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingArea {
    #[serde(flatten)]
    pub processing_area: ProcessingAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationDevelopmentClass {
    #[serde(flatten)]
    pub declaration_development_class: CoDeclarationDevelopmentClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatOperations {
    #[serde(flatten)]
    pub habitat_operations: HabitatOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandsType {
    #[serde(rename = "DeclarationStand")]
    pub declaration_stand: Vec<DeclarationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationsType {
    #[serde(rename = "DeclarationOtherOperation")]
    pub declaration_other_operation: Vec<CoDeclarationOtherOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: ReRealEstatesWithOwnersInformationType2,
    #[serde(rename = "ProcessingAreas")]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "UpdatePreviousDeclaration")]
    pub update_previous_declaration: CoYesNoType,
    #[serde(rename = "DeclarationReference")]
    pub declaration_reference: CoReferenceType,
    #[serde(rename = "DeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub declaration_text_information: Option<CoString2000Type>,
    #[serde(rename = "SpecialPermission")]
    pub special_permission: CoYesNoType,
    #[serde(rename = "CuttingRightsOwner", skip_serializing_if = "Option::is_none")]
    pub cutting_rights_owner: Option<CiContactInformationType>,
    #[serde(rename = "CuttingsRightsOwnerRepresentative", skip_serializing_if = "Option::is_none")]
    pub cuttings_rights_owner_representative: Option<CiContactInformationType>,
    #[serde(rename = "Sender")]
    pub sender: SenderType,
    #[serde(rename = "DeclarationRealEstates")]
    pub declaration_real_estates: DeclarationRealEstatesType,
    #[serde(rename = "FccDocuments", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<Documents>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "ProcessingAreaNumber")]
    pub processing_area_number: ProcessingAreaNumberType,
    #[serde(rename = "ProcessingAreaReference")]
    pub processing_area_reference: CoReferenceType,
    #[serde(rename = "DeclarationStands")]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
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
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<FccLocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: FccPowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumberType {
    #[serde(rename = "base")]
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SpfLocatedSpecialFeature3Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperationsType {
    #[serde(rename = "HabitatOperation")]
    pub habitat_operation: Vec<CoHabitatOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreasType {
    #[serde(rename = "ProcessingArea")]
    pub processing_area: Vec<ProcessingAreaType>,
}

