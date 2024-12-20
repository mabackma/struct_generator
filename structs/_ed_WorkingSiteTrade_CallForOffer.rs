#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDate {
    #[serde(flatten)]
    pub offer_expiration_date: OfferExpirationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_wood_trade_info: WtcoCallForOfferWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationMode {
    #[serde(flatten)]
    pub operation_mode: OperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferDate {
    #[serde(flatten)]
    pub call_for_offer_date: CallForOfferDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_silviculture_info: CallForOfferSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publicity {
    #[serde(flatten)]
    pub publicity: PublicityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferId {
    #[serde(flatten)]
    pub related_call_for_offer_id: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferDescription {
    #[serde(flatten)]
    pub related_call_for_offer_description: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPerson {
    #[serde(flatten)]
    pub technical_contact_person: TechnicalContactPersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffer {
    #[serde(flatten)]
    pub related_call_for_offer: RelatedCallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSender {
    #[serde(flatten)]
    pub call_for_offer_business_sender: WtcoCallForOfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffer {
    #[serde(flatten)]
    pub call_for_offer: CallForOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: WtcoCallForOfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub used_pricing_method_type: UsedPricingMethodTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCode {
    #[serde(flatten)]
    pub additional_code: AdditionalCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffers {
    #[serde(flatten)]
    pub related_call_for_offers: RelatedCallForOffersType,
}

