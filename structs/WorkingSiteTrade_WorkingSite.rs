#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKeyType {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "CallForOfferWorkingSiteDetails")]
    pub call_for_offer_working_site_details: Vec<CallForOfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "OfferWorkingSiteDetails")]
    pub offer_working_site_details: Vec<OfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: WorkingSiteType,
}

