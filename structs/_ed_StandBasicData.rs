#[derive(Debug, Serialize, Deserialize)]
pub struct DitchingYear {
    #[serde(flatten)]
    pub ditching_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteState {
    #[serde(flatten)]
    pub complete_state: CoCompleteStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestriction {
    #[serde(flatten)]
    pub cutting_restriction: CoCuttingRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestriction {
    #[serde(flatten)]
    pub silviculture_restriction: CoSilvicultureRestrictionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetails {
    #[serde(flatten)]
    pub silviculture_restriction_details: SilvicultureRestrictionDetailsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictions {
    #[serde(flatten)]
    pub silviculture_restrictions: SilvicultureRestrictionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningYear {
    #[serde(flatten)]
    pub thinning_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRestrictionEnds {
    #[serde(flatten)]
    pub cutting_restriction_ends: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowthPlaceDataSource {
    #[serde(flatten)]
    pub growth_place_data_source: CoDataSourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: CoStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: CoDrainageStateType,
}

