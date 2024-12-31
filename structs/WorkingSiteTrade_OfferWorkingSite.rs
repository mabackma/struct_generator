use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct OWorkingSite {
    #[serde(flatten)]
    pub o_working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureValidity {
    #[serde(flatten)]
    pub silviculture_validity: SilvicultureValidityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    #[serde(flatten)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteSilvicultureText {
    #[serde(flatten)]
    pub offer_working_site_silviculture_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FellingRightDuration {
    #[serde(flatten)]
    pub felling_right_duration: FellingRightDurationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DueDate {
    #[serde(flatten)]
    pub due_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub offer_working_site_wood_trade_info: OfferWorkingSiteWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub offer_working_site_silviculture_info: OfferWorkingSiteSilvicultureInfoType,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferWorkingSitePaymentTransactions {
    #[serde(flatten)]
    pub offer_working_site_payment_transactions: OfferWorkingSitePaymentTransactionsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: ChronoNaiveDate,
    #[serde(rename = "EndDate")]
    pub end_date: ChronoNaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    #[serde(rename = "working_site_text_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AsAssortmentClasses")]
    pub as_assortment_classes: AssortmentClasses,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedInOfferType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightDurationType {
    #[serde(flatten)]
    pub base: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FellingRightValidityDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePersonType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
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
    pub offer_working_site_payment_transactions: Option<OfferWorkingSitePaymentTransactionsType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<WorkingSiteNameType>,
    #[serde(rename = "SellerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub seller_representative_person: Option<SellerRepresentativePersonType>,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<SellersType>,
    #[serde(rename = "VATInfo", skip_serializing_if = "Option::is_none")]
    pub v_a_t_info: Option<VATInfoType>,
    #[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
    pub real_estate: Option<RealEstateType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<RoadUsingRightType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<CallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "WsstStands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<Stands>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WorkingSiteGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateType>,
    #[serde(rename = "SilvicultureValidity", skip_serializing_if = "Option::is_none")]
    pub silviculture_validity: Option<SilvicultureValidityType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureText", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_text: Option<String>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstateType>,
}

