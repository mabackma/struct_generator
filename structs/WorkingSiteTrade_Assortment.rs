use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLotInformationTypeDescription {
    #[serde(flatten)]
    pub wood_lot_information_type_description: CoString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuantityUnit {
    #[serde(flatten)]
    pub quantity_unit: WtcoQuantityUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthMin {
    #[serde(flatten)]
    pub length_min: LengthMinType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCompactClasses {
    #[serde(flatten)]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCompactClass {
    #[serde(flatten)]
    pub assortment_compact_class: AssortmentCompactClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLotInformationType {
    #[serde(flatten)]
    pub wood_lot_information_type: WoodLotInformationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalPrice {
    #[serde(flatten)]
    pub total_price: WtcoTotalPriceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quantity {
    #[serde(flatten)]
    pub quantity: WtcoQuantityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentClassCode {
    #[serde(flatten)]
    pub assortment_class_code: AssortmentClassCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Woodlot {
    #[serde(flatten)]
    pub woodlot: WoodLotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentCompact {
    #[serde(flatten)]
    pub assortment_compact: AssortmentCompactType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLots {
    #[serde(flatten)]
    pub wood_lots: WoodLotsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GradeCode {
    #[serde(flatten)]
    pub grade_code: CoGradeCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceMatrix {
    #[serde(flatten)]
    pub price_matrix: PriceMatrixType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodLotInformationValue {
    #[serde(flatten)]
    pub wood_lot_information_value: WoodLotInformationValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentsCompact {
    #[serde(flatten)]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceItem {
    #[serde(flatten)]
    pub price_item: PriceItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentMainGroup {
    #[serde(flatten)]
    pub assortment_main_group: CoAssortmentMainGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthMax {
    #[serde(flatten)]
    pub length_max: LengthMaxType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentClass {
    #[serde(flatten)]
    pub assortment_class: AssortmentClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentClasses {
    #[serde(flatten)]
    pub assortment_classes: AssortmentClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMinType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionRequirementsGroup {
    #[serde(rename = "DiameterMin")]
    pub diameter_min: DiameterMin,
    #[serde(rename = "DiameterMax")]
    pub diameter_max: DiameterMax,
    #[serde(rename = "LengthMax")]
    pub length_max: LengthMax,
    #[serde(rename = "LengthMin")]
    pub length_min: LengthMin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: AssortmentIdentifierGroup,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: QuantityGroup,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<PricesAndCurrencyGroup>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfoType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(rename = "TreeSpeciesType")]
    pub co_tree_species_type: CoTreeSpeciesType,
    #[serde(rename = "EmptyStringType")]
    pub co_empty_string_type: CoEmptyStringType,
    #[serde(rename = "ExtraTreeSpeciesType")]
    pub co_extra_tree_species_type: CoExtraTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeType {
    #[serde(flatten)]
    pub base: CoWoodLotInformationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(flatten)]
    pub base: AssortmentAllElementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricesAndCurrencyGroup {
    #[serde(rename = "Currency")]
    pub currency: Currency,
    #[serde(rename = "TotalPrice")]
    pub total_price: TotalPrice,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIdentifierGroup {
    #[serde(rename = "StemType")]
    pub stem_type: StemType,
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<AssortmentName>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpecies,
    #[serde(rename = "AssortmentMainGroup", skip_serializing_if = "Option::is_none")]
    pub assortment_main_group: Option<AssortmentMainGroup>,
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<AssortmentCode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactType {
    #[serde(flatten)]
    pub base: AssortmentCompactElementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrixType {
    #[serde(rename = "PriceItem")]
    pub price_item: Vec<PriceItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotsType {
    #[serde(rename = "Woodlot")]
    pub woodlot: Vec<WoodLotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassesType {
    #[serde(rename = "AssortmentClass")]
    pub assortment_class: Vec<AssortmentClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItemType {
    #[serde(rename = "AverageStemVolume")]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: WtcoUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentAllElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: AssortmentIdentifierGroup,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<CoUsedPricingMethodType>,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: QuantityGroup,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<PricesAndCurrencyGroup>,
    #[serde(rename = "DimensionRequirementsGroup", skip_serializing_if = "Option::is_none")]
    pub dimension_requirements_group: Option<DimensionRequirementsGroup>,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<CoDecimal3FractionDigitsType>,
    #[serde(rename = "GradeCode", skip_serializing_if = "Option::is_none")]
    pub grade_code: Option<CoGradeCodeType>,
    #[serde(rename = "WoodLots", skip_serializing_if = "Option::is_none")]
    pub wood_lots: Option<WoodLotsType>,
    #[serde(rename = "MeasurementMethod", skip_serializing_if = "Option::is_none")]
    pub measurement_method: Option<CoMeasurementMethodType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
    #[serde(rename = "PriceMatrix", skip_serializing_if = "Option::is_none")]
    pub price_matrix: Option<PriceMatrixType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethodType {
    #[serde(flatten)]
    pub base: CoMeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationGroup {
    #[serde(rename = "WoodLotInformationType")]
    pub wood_lot_information_type: WoodLotInformationType,
    #[serde(rename = "WoodLotInformationValue")]
    pub wood_lot_information_value: WoodLotInformationValue,
    #[serde(rename = "WoodLotInformationTypeDescription", skip_serializing_if = "Option::is_none")]
    pub wood_lot_information_type_description: Option<WoodLotInformationTypeDescription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValueType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityGroup {
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: QuantityUnit,
    #[serde(rename = "Quantity")]
    pub quantity: Quantity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMaxType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotType {
    #[serde(rename = "WoodLotInformationGroup")]
    pub wood_lot_information_group: WoodLotInformationGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCodeType {
    #[serde(flatten)]
    pub base: CoAssortmentClassCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeType {
    #[serde(rename = "StemTypeType")]
    pub co_stem_type_type: CoStemTypeType,
    #[serde(rename = "ExtraStemTypeType")]
    pub co_extra_stem_type_type: CoExtraStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMaxType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMinType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

