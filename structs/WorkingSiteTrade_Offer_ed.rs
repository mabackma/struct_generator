#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferText {
    #[serde(flatten)]
    pub call_for_offer_text: CallForOfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferText {
    #[serde(flatten)]
    pub offer_text: OfferTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDate {
    #[serde(flatten)]
    pub offer_date: OfferDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSender {
    #[serde(flatten)]
    pub offer_business_sender: OfferBusinessSenderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_wood_trade_info: CallForOfferWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    #[serde(flatten)]
    pub offer: OfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPerson {
    #[serde(flatten)]
    pub technical_contact_person: TechnicalContactPersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDate {
    #[serde(flatten)]
    pub offer_expiration_date: OfferExpirationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSender {
    #[serde(flatten)]
    pub call_for_offer_business_sender: CallForOfferBusinessSenderType,
}

