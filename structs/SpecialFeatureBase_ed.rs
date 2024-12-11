#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValue {
    #[serde(flatten)]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: FeatureAdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Explanation {
    #[serde(flatten)]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: FeatureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionEndDate {
    #[serde(flatten)]
    pub restriction_end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionStartDate {
    #[serde(flatten)]
    pub restriction_start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictions {
    #[serde(flatten)]
    pub using_restrictions: UsingRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: FeatureAdditionalInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestriction {
    #[serde(flatten)]
    pub using_restriction: UsingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDescription {
    #[serde(flatten)]
    pub restriction_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionOutOfObject {
    #[serde(flatten)]
    pub restriction_out_of_object: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: ValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: RestrictionCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: FeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(flatten)]
    pub identifier_type: IdentifierTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: StartDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: EndDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: IdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: RestrictionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistance {
    #[serde(flatten)]
    pub buffer_distance: BufferDistanceType,
}

