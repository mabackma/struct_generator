#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeType {
    #[serde(flatten)]
    pub base: CoPurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferTextType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRightType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: CoForestUseDeclarationResponsibleType,
    #[serde(rename = "FudrForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub fudr_forest_use_declaration_reference: Option<ForestUseDeclarationReference>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AsAssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub as_assortment_classes: Option<AssortmentClasses>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerGroupType {
    #[serde(flatten)]
    pub base: CoSellerGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(rename = "PaymentTransactionCategory")]
    pub payment_transaction_category: CoMoneyTransactionCategoryType,
    #[serde(rename = "PaymentTransactionType")]
    pub payment_transaction_type: CoMoneyTransactionTypeType,
    #[serde(rename = "Value")]
    pub value: CoMoneyType,
    #[serde(rename = "PaymentTransactionAsUnitGroup")]
    pub payment_transaction_as_unit_group: PaymentTransactionAsUnitGroup,
    #[serde(rename = "Currency")]
    pub currency: CoCurrencyType,
    #[serde(rename = "PaymentTransactionDescription", skip_serializing_if = "Option::is_none")]
    pub payment_transaction_description: Option<PaymentTransactionDescriptionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CuttingPlannerLiability", skip_serializing_if = "Option::is_none")]
    pub cutting_planner_liability: Option<CuttingPlannerLiabilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeForestFundPaymentType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub base: CoUsedPricingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<SfLocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyPointType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfoType {
    #[serde(rename = "PurchaseMode")]
    pub purchase_mode: PurchaseModeType,
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
    #[serde(rename = "IncludeForestFundPayment")]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
    #[serde(rename = "UsedPricingMethods", skip_serializing_if = "Option::is_none")]
    pub used_pricing_methods: Option<UsedPricingMethodsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityUnitType {
    #[serde(flatten)]
    pub base: CoQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodsType {
    #[serde(rename = "UsedPricingMethod")]
    pub used_pricing_method: Vec<UsedPricingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<CoDocumentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClassType {
    #[serde(flatten)]
    pub base: CoDocumentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderRoleType {
    #[serde(flatten)]
    pub base: CoCallForOfferBusinessSenderRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityType {
    #[serde(flatten)]
    pub base: CoDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemType {
    #[serde(flatten)]
    pub base: CoCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatusType {
    #[serde(flatten)]
    pub base: CoVATStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiabilityType {
    #[serde(flatten)]
    pub base: CoYesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTypeType {
    #[serde(flatten)]
    pub base: CoMessageTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "CoBankAccount", skip_serializing_if = "Option::is_none")]
    pub co_bank_account: Option<BankAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNameType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionAsUnitGroup {
    #[serde(rename = "TotalValue", skip_serializing_if = "Option::is_none")]
    pub total_value: Option<TotalValue>,
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(rename = "QuantityUnit", skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<QuantityUnit>,
    #[serde(rename = "UnitValue", skip_serializing_if = "Option::is_none")]
    pub unit_value: Option<UnitValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludePaymentPlanType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionDescriptionType {
    #[serde(flatten)]
    pub base: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPriceType {
    #[serde(flatten)]
    pub base: CoMoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsType {
    #[serde(rename = "Cutting")]
    pub cutting: Vec<CuttingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem")]
    pub certification_system: Vec<CertificationSystemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescriptionType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersType {
    #[serde(rename = "@subsidyPossibility")]
    pub subsidy_possibility: CoYesNoNotKnownType,
    #[serde(rename = "@sellerGroup")]
    pub seller_group: SellerGroupType,
    #[serde(rename = "Seller")]
    pub seller: Vec<SellerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResourceType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
}

