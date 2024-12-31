use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WorkingSitePlanType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: SellerRepresentativePersonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WorkingSiteGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: RoadUsingRightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: SellersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: CallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: SceneryWorkPermissionNeededType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: VATInfoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    #[serde(flatten)]
    pub products: ProductsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOfferType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationsType {
    #[serde(rename = "SilviculturalOperation")]
    pub silvicultural_operation: Vec<SilviculturalOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "WorkingSiteName")]
    pub working_site_name: WorkingSiteNameType,
    #[serde(rename = "IncludeInOffer")]
    pub include_in_offer: IncludeInOfferType,
    #[serde(rename = "SellerRepresentativePerson")]
    pub seller_representative_person: SellerRepresentativePersonType,
    #[serde(rename = "Sellers")]
    pub sellers: SellersType,
    #[serde(rename = "VATInfo")]
    pub v_a_t_info: VATInfoType,
    #[serde(rename = "RealEstate")]
    pub real_estate: RealEstateType,
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstatesType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<RoadUsingRightType>,
    #[serde(rename = "WorkingSitePlan", skip_serializing_if = "Option::is_none")]
    pub working_site_plan: Option<WorkingSitePlanType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<CallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "WsstStands", skip_serializing_if = "Option::is_none")]
    pub wsst_stands: Option<Stands>,
    #[serde(rename = "WtcoCertificationSystems", skip_serializing_if = "Option::is_none")]
    pub wtco_certification_systems: Option<CertificationSystems>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WorkingSiteGeometriesType>,
    #[serde(rename = "CallForOfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_silviculture_info: Option<CallForOfferWorkingSiteSilvicultureInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTextType {
    #[serde(rename = "working_site_text_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionType {
    #[serde(rename = "SceneryWorkPermissionNeeded")]
    pub scenery_work_permission_needed: SceneryWorkPermissionNeededType,
    #[serde(rename = "SceneryWorkPermissionAcceptance", skip_serializing_if = "Option::is_none")]
    pub scenery_work_permission_acceptance: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "SilviculturalOperations", skip_serializing_if = "Option::is_none")]
    pub silvicultural_operations: Option<SilviculturalOperationsType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActorsType {
    #[serde(rename = "InformedActor")]
    pub informed_actor: Vec<ContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationType {
    #[serde(rename = "@id")]
    pub id: String,
}

