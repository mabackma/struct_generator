#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: CoDateYYYY-MMOrYYYY-MM-DDType,
    #[serde(rename = "EndDate")]
    pub end_date: CoDateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<WtcoRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: String,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Xsstring>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<WtcoDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: Xsint,
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: String,
    #[serde(rename = "IncludedInOffer")]
    pub included_in_offer: IncludedInOfferType,
    #[serde(rename = "PurchaserRepresentativePerson")]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "OfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_wood_trade_info: Option<OfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_info: Option<OfferWorkingSiteSilvicultureInfoType>,
    #[serde(rename = "OfferWorkingSitePaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_payment_transactions: Option<WtcoOfferWorkingSitePaymentTransactionsType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<WtcoWorkingSiteNameType>,
    #[serde(rename = "SellerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub seller_representative_person: Option<WtcoSellerRepresentativePersonType>,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<WtcoSellersType>,
    #[serde(rename = "VATInfo", skip_serializing_if = "Option::is_none")]
    pub v_a_t_info: Option<WtcoVATInfoType>,
    #[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
    pub real_estate: Option<WtcoRealEstateType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<WtcoRoadUsingRightType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<WtcoCallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "WsstStands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<String>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WtcoWorkingSiteGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<CoDateType>,
    #[serde(rename = "SilvicultureValidity", skip_serializing_if = "Option::is_none")]
    pub silviculture_validity: Option<SilvicultureValidityType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<PrProductsType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureText", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_text: Option<Xsstring>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDurationType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

