use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    pub base: String,
}

