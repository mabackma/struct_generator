#[derive(Debug, Serialize, Deserialize)]
pub struct OfferType {
    #[serde(rename = "@callForOfferId")]
    pub call_for_offer_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@versionNo")]
    pub version_no: i32,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "OfferBusinessSender")]
    pub offer_business_sender: OfferBusinessSenderType,
    #[serde(rename = "CallForOfferBusinessSender", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_business_sender: Option<WtcoCallForOfferBusinessSenderType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "OfferDate")]
    pub offer_date: OfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "OfferText", skip_serializing_if = "Option::is_none")]
    pub offer_text: Option<OfferTextType>,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<WtcoCallForOfferTextType>,
    #[serde(rename = "WsOfferWorkingSites")]
    pub ws_offer_working_sites: OfferWorkingSites,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSenderType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferTextType {
    #[serde(flatten)]
    pub base: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

