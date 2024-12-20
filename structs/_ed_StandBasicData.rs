#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteState {
    #[serde(flatten)]
    pub complete_state: CoCompleteStateType,
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
pub struct GrowthPlaceDataSource {
    #[serde(flatten)]
    pub growth_place_data_source: CoDataSourceType,
}

