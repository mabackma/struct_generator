#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<FccLocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SpfLocatedSpecialFeature3Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreasType {
    #[serde(rename = "ProcessingArea")]
    pub processing_area: Vec<ProcessingAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: FccPowerOfAttorneyType,
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
    pub id: CoIdStringType,
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
    pub fcc_documents: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaNumberType {
    #[serde(flatten)]
    pub base: Xsinteger,
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
pub struct HabitatOperationsType {
    #[serde(rename = "HabitatOperation")]
    pub habitat_operation: Vec<CoHabitatOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandsType {
    #[serde(rename = "DeclarationStand")]
    pub declaration_stand: Vec<DeclarationStandType>,
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
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationsType {
    #[serde(rename = "DeclarationOtherOperation")]
    pub declaration_other_operation: Vec<CoDeclarationOtherOperationType>,
}

