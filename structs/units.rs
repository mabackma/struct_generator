use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct b {
    #[serde(flatten)]
    pub b: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseUnit {
    #[serde(flatten)]
    pub base_unit: GmlBaseUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct derivationUnitTerm {
    #[serde(flatten)]
    pub derivation_unit_term: GmlDerivationUnitTermType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct d {
    #[serde(flatten)]
    pub d: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct c {
    #[serde(flatten)]
    pub c: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct unitOfMeasure {
    #[serde(flatten)]
    pub unit_of_measure: GmlUnitOfMeasureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct catalogSymbol {
    #[serde(flatten)]
    pub catalog_symbol: GmlCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct roughConversionToPreferredUnit {
    #[serde(flatten)]
    pub rough_conversion_to_preferred_unit: GmlConversionToPreferredUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct factor {
    #[serde(flatten)]
    pub factor: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct quantityType {
    #[serde(flatten)]
    pub quantity_type: GmlStringOrRefType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConventionalUnit {
    #[serde(flatten)]
    pub conventional_unit: GmlConventionalUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DerivedUnit {
    #[serde(flatten)]
    pub derived_unit: GmlDerivedUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct conversionToPreferredUnit {
    #[serde(flatten)]
    pub conversion_to_preferred_unit: GmlConversionToPreferredUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitDefinition {
    #[serde(flatten)]
    pub unit_definition: GmlUnitDefinitionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct a {
    #[serde(flatten)]
    pub a: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct formula {
    #[serde(flatten)]
    pub formula: GmlFormulaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct unitsSystem {
    #[serde(flatten)]
    pub units_system: GmlReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormulaType {
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    #[serde(rename = "b")]
    pub b: f64,
    #[serde(rename = "c")]
    pub c: f64,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DerivationUnitTermType {
    #[serde(flatten)]
    pub base: GmlUnitOfMeasureType,
    #[serde(rename = "exponent")]
    pub exponent: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitOfMeasureType {
    #[serde(rename = "uom")]
    pub uom: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConventionalUnitType {
    #[serde(flatten)]
    pub base: GmlUnitDefinitionType,
    #[serde(rename = "conversionToPreferredUnit")]
    pub gmlconversion_to_preferred_unit: GmlconversionToPreferredUnit,
    #[serde(rename = "roughConversionToPreferredUnit")]
    pub gmlrough_conversion_to_preferred_unit: GmlroughConversionToPreferredUnit,
    #[serde(rename = "derivationUnitTerm", skip_serializing_if = "Option::is_none")]
    pub gmlderivation_unit_term: Option<Vec<GmlderivationUnitTerm>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DerivedUnitType {
    #[serde(flatten)]
    pub base: GmlUnitDefinitionType,
    #[serde(rename = "derivationUnitTerm")]
    pub gmlderivation_unit_term: Vec<GmlderivationUnitTerm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionToPreferredUnitType {
    #[serde(flatten)]
    pub base: GmlUnitOfMeasureType,
    #[serde(rename = "factor")]
    pub factor: f64,
    #[serde(rename = "formula")]
    pub formula: FormulaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitDefinitionType {
    #[serde(flatten)]
    pub base: GmlDefinitionType,
    #[serde(rename = "quantityType")]
    pub gmlquantity_type: GmlquantityType,
    #[serde(rename = "catalogSymbol", skip_serializing_if = "Option::is_none")]
    pub gmlcatalog_symbol: Option<GmlcatalogSymbol>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseUnitType {
    #[serde(flatten)]
    pub base: GmlUnitDefinitionType,
    #[serde(rename = "unitsSystem")]
    pub units_system: ReferenceType,
}

