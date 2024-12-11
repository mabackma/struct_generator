#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CallForOffer")]
    pub call_for_offer: String,
    #[serde(rename = "CFOWorkingSite")]
    pub c_f_o_working_site: String,
    #[serde(rename = "BusinessAcceptance")]
    pub business_acceptance: String,
    #[serde(rename = "Payment")]
    pub payment: String,
    #[serde(rename = "MeasurementCertificate")]
    pub measurement_certificate: String,
    #[serde(rename = "ForestUseDeclarationReferences")]
    pub forest_use_declaration_references: String,
    #[serde(rename = "Contract")]
    pub contract: String,
    #[serde(rename = "Offer")]
    pub offer: String,
    #[serde(rename = "OWorkingSite")]
    pub o_working_site: String,
    #[serde(rename = "Operations")]
    pub operations: String,
    #[serde(rename = "ContactRequest")]
    pub contact_request: String,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: String,
    #[serde(rename = "Acknowledge")]
    pub acknowledge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

