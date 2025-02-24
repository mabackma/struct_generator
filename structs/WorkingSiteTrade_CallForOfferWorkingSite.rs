use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    #[serde(flatten)]
    pub products: PrProductsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: WtcoCallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WtcoWorkingSitePlanType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: WtcoRoadUsingRightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WtcoWorkingSiteGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: WtcoSellersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: CiContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: WtcoVATInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: WtcoSellerRepresentativePersonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionType {
    #[serde(rename = "SceneryWorkPermissionNeeded")]
    pub scenery_work_permission_needed: CoSceneryWorkPermissionNeededType,
    #[serde(rename = "SceneryWorkPermissionAcceptance", skip_serializing_if = "Option::is_none")]
    pub scenery_work_permission_acceptance: Option<CoDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationsType {
    #[serde(rename = "SilviculturalOperation")]
    pub silvicultural_operation: Vec<SilviculturalOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOfferType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "SilviculturalOperations", skip_serializing_if = "Option::is_none")]
    pub silvicultural_operations: Option<SilviculturalOperationsType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<PrProductsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActorsType {
    #[serde(rename = "InformedActor")]
    pub informed_actor: Vec<CiContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
    #[serde(rename = "WorkingSiteName")]
    pub working_site_name: WtcoWorkingSiteNameType,
    #[serde(rename = "IncludeInOffer")]
    pub include_in_offer: IncludeInOfferType,
    #[serde(rename = "SellerRepresentativePerson")]
    pub seller_representative_person: WtcoSellerRepresentativePersonType,
    #[serde(rename = "Sellers")]
    pub sellers: WtcoSellersType,
    #[serde(rename = "VATInfo")]
    pub v_a_t_info: WtcoVATInfoType,
    #[serde(rename = "RealEstate")]
    pub real_estate: WtcoRealEstateType,
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstatesType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<WtcoRoadUsingRightType>,
    #[serde(rename = "WorkingSitePlan", skip_serializing_if = "Option::is_none")]
    pub working_site_plan: Option<WtcoWorkingSitePlanType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<WtcoCallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<WsstStands>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub wtco_certification_systems: Option<WtcoCertificationSystems>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WtcoWorkingSiteGeometriesType>,
    #[serde(rename = "CallForOfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_silviculture_info: Option<CallForOfferWorkingSiteSilvicultureInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<WtcoRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationType {
    #[serde(rename = "@id")]
    pub id: String,
}

