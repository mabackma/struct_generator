#[derive(Debug, Serialize, Deserialize)]
pub struct MainFeature {
    #[serde(flatten)]
    pub main_feature: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureType {
    #[serde(flatten)]
    pub feature_type: FeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandData {
    #[serde(flatten)]
    pub tree_stand_data: TreeStandDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validity {
    #[serde(flatten)]
    pub validity: ValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCode {
    #[serde(flatten)]
    pub feature_additional_code: FeatureAdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationAmount {
    #[serde(flatten)]
    pub using_right_compensation_amount: Decimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRight {
    #[serde(flatten)]
    pub using_right: UsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: FeatureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalInfo {
    #[serde(flatten)]
    pub feature_additional_info: FeatureAdditionalInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: YesNoNotKnownType,
}

