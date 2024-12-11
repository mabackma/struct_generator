#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDate {
    #[serde(flatten)]
    pub stand_basic_data_date: StandBasicDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionEnds {
    #[serde(flatten)]
    pub silviculture_restriction_ends: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: IdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: AreaDecreaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfo {
    #[serde(flatten)]
    pub stand_info: StandInfoType,
}

