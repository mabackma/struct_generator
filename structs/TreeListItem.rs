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
pub struct TreeIdentifiersType {
    #[serde(rename = "TreeIdentifier", skip_serializing_if = "Option::is_none")]
    pub tree_identifier: Option<Vec<TreeIdentifierType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifierType {
    #[serde(rename = "Type")]
    pub r#type: Xsinteger,
    #[serde(rename = "Value")]
    pub value: Xsstring,
}

