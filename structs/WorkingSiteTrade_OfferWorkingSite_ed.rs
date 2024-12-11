#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: VATInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: CallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub offer_working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePerson {
    #[serde(flatten)]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub offer_working_site_wood_trade_info: OfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDuration {
    #[serde(flatten)]
    pub felling_right_duration: FellingRightDurationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: SellerRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: WorkingSiteNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: RealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureText {
    #[serde(flatten)]
    pub offer_working_site_silviculture_text: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidity {
    #[serde(flatten)]
    pub silviculture_validity: SilvicultureValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: DateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: SellersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: DateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    #[serde(flatten)]
    pub products: ProductsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOffer {
    #[serde(flatten)]
    pub included_in_offer: IncludedInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WorkingSiteGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    pub text: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub offer_working_site_silviculture_info: OfferWorkingSiteSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDate {
    #[serde(flatten)]
    pub felling_right_validity_date: FellingRightValidityDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OWorkingSite {
    #[serde(flatten)]
    pub o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: RoadUsingRightType,
}

