use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOffer {
    #[serde(flatten)]
    pub call_for_offer: CallForOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsedPricingMethodType {
    #[serde(flatten)]
    pub used_pricing_method_type: UsedPricingMethodTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferDate {
    #[serde(flatten)]
    pub call_for_offer_date: CallForOfferDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationMode {
    #[serde(flatten)]
    pub operation_mode: OperationModeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Publicity {
    #[serde(flatten)]
    pub publicity: PublicityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOffer {
    #[serde(flatten)]
    pub related_call_for_offer: RelatedCallForOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalCode {
    #[serde(flatten)]
    pub additional_code: AdditionalCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_silviculture_info: CallForOfferSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOffers {
    #[serde(flatten)]
    pub related_call_for_offers: RelatedCallForOffersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOfferDescription {
    #[serde(flatten)]
    pub related_call_for_offer_description: CoString1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCallForOfferId {
    #[serde(flatten)]
    pub related_call_for_offer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
    #[serde(rename = "RelatedCallForOffers", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offers: Option<RelatedCallForOffersType>,
    #[serde(rename = "AdditionalCode", skip_serializing_if = "Option::is_none")]
    pub additional_code: Option<AdditionalCodeType>,
    #[serde(rename = "CallForOfferBusinessSender")]
    pub call_for_offer_business_sender: WtcoCallForOfferBusinessSenderType,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<SellersType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "CallForOfferDate")]
    pub call_for_offer_date: CallForOfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<WtcoCallForOfferTextType>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<WtcoCallForOfferWoodTradeInfoType>,
    #[serde(rename = "CallForOfferSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_silviculture_info: Option<CallForOfferSilvicultureInfoType>,
    #[serde(rename = "CallForOfferWorkingSites")]
    pub ws_call_for_offer_working_sites: WsCallForOfferWorkingSites,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<WtcoDocuments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferType {
    #[serde(rename = "RelatedCallForOfferId")]
    pub related_call_for_offer_id: String,
    #[serde(rename = "RelatedCallForOfferDescription", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offer_description: Option<String1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfoType {
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffersType {
    #[serde(rename = "RelatedCallForOffer")]
    pub related_call_for_offer: Vec<RelatedCallForOfferType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityType {
    #[serde(flatten)]
    pub base: CoPublicityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExpirationDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationModeType {
    #[serde(flatten)]
    pub base: CoOperationModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseModeType {
    #[serde(flatten)]
    pub base: WtcoPurchaseModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodTypeType {
    #[serde(flatten)]
    pub base: CoUsedPricingMethodTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPersonType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

