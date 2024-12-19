#[derive(Debug, Serialize, Deserialize)]
pub struct TargetAnnouncedAmount {
    #[serde(flatten)]
    pub target_announced_amount: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetNumber {
    #[serde(flatten)]
    pub target_number: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetExtraInfo {
    #[serde(flatten)]
    pub target_extra_info: VirtaExtraInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetId {
    #[serde(flatten)]
    pub target_id: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetParts {
    #[serde(flatten)]
    pub target_parts: TargetPartsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatAdvertisement {
    #[serde(flatten)]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePartNumber {
    #[serde(flatten)]
    pub base_part_number: VirtaPartNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status2 {
    #[serde(flatten)]
    pub status2: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstablishedPartNumber {
    #[serde(flatten)]
    pub established_part_number: VirtaPartNumberType,
}

