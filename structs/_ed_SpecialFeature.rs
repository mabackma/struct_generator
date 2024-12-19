#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExaminedDate {
    #[serde(flatten)]
    pub using_right_examined_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightDescription {
    #[serde(flatten)]
    pub using_right_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensation {
    #[serde(flatten)]
    pub using_right_compensation: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryDate {
    #[serde(flatten)]
    pub inventory_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightExists {
    #[serde(flatten)]
    pub using_right_exists: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariables {
    #[serde(flatten)]
    pub feature_specific_additional_variables: FeatureSpecificAdditionalVariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightCompensationResponsible {
    #[serde(flatten)]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
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
pub struct UsingRightCompensationDescription {
    #[serde(flatten)]
    pub using_right_compensation_description: String1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDepotAccessibility {
    #[serde(flatten)]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationDate {
    #[serde(flatten)]
    pub observation_date: DateType,
}

