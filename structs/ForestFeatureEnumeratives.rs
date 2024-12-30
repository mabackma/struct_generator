use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    #[serde(rename = "feature_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    #[serde(rename = "original_feature_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    #[serde(rename = "feature_code_extensions_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    #[serde(rename = "feature_additional_code_type.base")]
    pub base: String,
}

