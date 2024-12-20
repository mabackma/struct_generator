#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOffer {
    #[serde(flatten)]
    pub included_in_offer: IncludedInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidity {
    #[serde(flatten)]
    pub silviculture_validity: SilvicultureValidityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureText {
    #[serde(flatten)]
    pub offer_working_site_silviculture_text: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OWorkingSite {
    #[serde(flatten)]
    pub o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub offer_working_site_payment_transactions: WtcoOfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePerson {
    #[serde(flatten)]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDuration {
    #[serde(flatten)]
    pub felling_right_duration: FellingRightDurationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub offer_working_site_wood_trade_info: OfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub offer_working_site_silviculture_info: OfferWorkingSiteSilvicultureInfoType,
}

