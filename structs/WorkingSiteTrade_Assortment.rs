#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotsType {
    #[serde(rename = "Woodlot")]
    pub woodlot: Vec<WoodLotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentIdentifierGroup {
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<AssortmentName>,
    #[serde(rename = "AssortmentMainGroup", skip_serializing_if = "Option::is_none")]
    pub assortment_main_group: Option<AssortmentMainGroup>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpecies,
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<AssortmentCode>,
    #[serde(rename = "StemType")]
    pub stem_type: StemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(flatten)]
    pub base: AssortmentAllElementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrixType {
    #[serde(rename = "PriceItem")]
    pub price_item: Vec<PriceItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityGroup {
    #[serde(rename = "Quantity")]
    pub quantity: Quantity,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: QuantityUnit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMinType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItemType {
    #[serde(rename = "AverageStemVolume")]
    pub average_stem_volume: CoDecimal3FractionDigitsType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: WtcoUnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethodType {
    #[serde(flatten)]
    pub base: CoMeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMaxType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotType {
    #[serde(rename = "WoodLotInformationGroup")]
    pub wood_lot_information_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfoType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassCodeType {
    #[serde(flatten)]
    pub base: CoAssortmentClassCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationTypeType {
    #[serde(flatten)]
    pub base: CoWoodLotInformationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionRequirementsGroup {
    #[serde(rename = "LengthMax")]
    pub length_max: LengthMax,
    #[serde(rename = "LengthMin")]
    pub length_min: LengthMin,
    #[serde(rename = "DiameterMin")]
    pub diameter_min: DiameterMin,
    #[serde(rename = "DiameterMax")]
    pub diameter_max: DiameterMax,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassesType {
    #[serde(rename = "AssortmentClass")]
    pub assortment_class: Vec<AssortmentClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationGroup {
    #[serde(rename = "WoodLotInformationTypeDescription", skip_serializing_if = "Option::is_none")]
    pub wood_lot_information_type_description: Option<WoodLotInformationTypeDescription>,
    #[serde(rename = "WoodLotInformationType")]
    pub wood_lot_information_type: WoodLotInformationType,
    #[serde(rename = "WoodLotInformationValue")]
    pub wood_lot_information_value: WoodLotInformationValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMinType {
    #[serde(flatten)]
    pub base: CoDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricesAndCurrencyGroup {
    #[serde(rename = "Currency")]
    pub currency: Currency,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "TotalPrice")]
    pub total_price: TotalPrice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactType {
    #[serde(flatten)]
    pub base: AssortmentCompactElementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: String,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: String,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<String>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotInformationValueType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentAllElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: String,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<CoUsedPricingMethodType>,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: String,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<String>,
    #[serde(rename = "DimensionRequirementsGroup", skip_serializing_if = "Option::is_none")]
    pub dimension_requirements_group: Option<String>,
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
pub struct StemTypeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthMaxType {
    #[serde(flatten)]
    pub base: CoPositiveDecimalMax5IntegralPartMax1FractionalPartType,
}

