#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfoType {
    #[serde(flatten)]
    pub base: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDateType {
    #[serde(flatten)]
    pub base: DateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationType {
    #[serde(flatten)]
    pub base: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValueType {
    #[serde(flatten)]
    pub base: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDateType {
    #[serde(flatten)]
    pub base: DateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferDistanceType {
    #[serde(flatten)]
    pub base: Decimal4And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionsType {
    #[serde(rename = "UsingRestriction")]
    pub using_restriction: Vec<UsingRestrictionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionType {
    #[serde(rename = "RestrictionType")]
    pub restriction_type: String,
    #[serde(rename = "RestrictionCode")]
    pub restriction_code: String,
    #[serde(rename = "RestrictionStartDate", skip_serializing_if = "Option::is_none")]
    pub restriction_start_date: Option<StartDateType>,
    #[serde(rename = "RestrictionEndDate", skip_serializing_if = "Option::is_none")]
    pub restriction_end_date: Option<EndDateType>,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "RestrictionDescription", skip_serializing_if = "Option::is_none")]
    pub restriction_description: Option<String1500Type>,
    #[serde(rename = "RestrictionOutOfObject", skip_serializing_if = "Option::is_none")]
    pub restriction_out_of_object: Option<YesNoType>,
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
pub struct FeatureInfoType {
    #[serde(flatten)]
    pub base: string,
}

