#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: String,
    #[serde(rename = "CfowsCFOWorkingSite")]
    pub cfows_c_f_o_working_site: String,
    #[serde(rename = "BaBusinessAcceptance")]
    pub ba_business_acceptance: String,
    #[serde(rename = "WppPayment")]
    pub wpp_payment: String,
    #[serde(rename = "WpmcMeasurementCertificate")]
    pub wpmc_measurement_certificate: String,
    #[serde(rename = "FudrForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: String,
    #[serde(rename = "WstcrContract")]
    pub wstcr_contract: String,
    #[serde(rename = "WstoOffer")]
    pub wsto_offer: String,
    #[serde(rename = "OwsOWorkingSite")]
    pub ows_o_working_site: String,
    #[serde(rename = "OsuOperations")]
    pub osu_operations: String,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: String,
    #[serde(rename = "MsMapSymbol")]
    pub ms_map_symbol: String,
    #[serde(rename = "AckAcknowledge")]
    pub ack_acknowledge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EbEnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

