use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub offer_working_site_payment_transactions: WtcoOfferWorkingSitePaymentTransactionsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub offer_working_site_silviculture_info: OfferWorkingSiteSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureValidity {
    #[serde(flatten)]
    pub silviculture_validity: SilvicultureValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FellingRightDuration {
    #[serde(flatten)]
    pub felling_right_duration: FellingRightDurationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub offer_working_site_wood_trade_info: OfferWorkingSiteWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OWorkingSite {
    #[serde(flatten)]
    pub o_working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteSilvicultureText {
    #[serde(flatten)]
    pub offer_working_site_silviculture_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    #[serde(flatten)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaserRepresentativePerson {
    #[serde(flatten)]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludedInOffer {
    #[serde(flatten)]
    pub included_in_offer: IncludedInOfferType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
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
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<WtcoDocuments>,
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
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<WsstStands>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WtcoWorkingSiteGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AssortmentClasses")]
    pub as_assortment_classes: AsAssortmentClasses,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<WtcoDocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: CoDateYYYYMMOrYYYYMMDDType,
    #[serde(rename = "EndDate")]
    pub end_date: CoDateYYYYMMOrYYYYMMDDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDurationType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
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
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<WtcoRealEstateType>,
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
    pub offer_working_site_silviculture_text: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<WtcoDocuments>,
}

