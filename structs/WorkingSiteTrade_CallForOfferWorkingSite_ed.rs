#[derive(Debug, Serialize, Deserialize)]
pub struct VATStatus {
    #[serde(flatten)]
    pub v_a_t_status: VATStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadUsingRight {
    #[serde(flatten)]
    pub road_using_right: RoadUsingRightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionNeeded {
    #[serde(flatten)]
    pub scenery_work_permission_needed: SceneryWorkPermissionNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_silviculture_info: CallForOfferWorkingSiteSilvicultureInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperations {
    #[serde(flatten)]
    pub silvicultural_operations: SilviculturalOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlan {
    #[serde(flatten)]
    pub working_site_plan: WorkingSitePlanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystems {
    #[serde(flatten)]
    pub certification_systems: CertificationSystemsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfo {
    #[serde(flatten)]
    pub call_for_offer_working_site_wood_trade_info: CallForOfferWorkingSiteWoodTradeInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstates {
    #[serde(flatten)]
    pub real_estates: RealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfo {
    #[serde(flatten)]
    pub v_a_t_info: VATInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometries {
    #[serde(flatten)]
    pub working_site_geometries: WorkingSiteGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActor {
    #[serde(flatten)]
    pub informed_actor: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteText {
    #[serde(flatten)]
    pub working_site_text: WorkingSiteTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystem {
    #[serde(flatten)]
    pub certification_system: CertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFOWorkingSite {
    #[serde(flatten)]
    pub c_f_o_working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sellers {
    #[serde(flatten)]
    pub sellers: SellersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteName {
    #[serde(flatten)]
    pub working_site_name: WorkingSiteNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePerson {
    #[serde(flatten)]
    pub seller_representative_person: SellerRepresentativePersonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperation {
    #[serde(flatten)]
    pub silvicultural_operation: SilviculturalOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATRegistrationDate {
    #[serde(flatten)]
    pub v_a_t_registration_date: VATRegistrationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: RealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionAcceptance {
    #[serde(flatten)]
    pub scenery_work_permission_acceptance: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    #[serde(flatten)]
    pub products: ProductsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludeInOffer {
    #[serde(flatten)]
    pub include_in_offer: IncludeInOfferType,
}

