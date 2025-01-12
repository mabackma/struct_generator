use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EbEnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: CallForOffer,
    #[serde(rename = "CFOWorkingSite")]
    pub cfows_c_f_o_working_site: CFOWorkingSite,
    #[serde(rename = "BusinessAcceptance")]
    pub ba_business_acceptance: BusinessAcceptance,
    #[serde(rename = "Payment")]
    pub wpp_payment: Payment,
    #[serde(rename = "MeasurementCertificate")]
    pub wpmc_measurement_certificate: MeasurementCertificate,
    #[serde(rename = "ForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: ForestUseDeclarationReferences,
    #[serde(rename = "Contract")]
    pub wstcr_contract: Contract,
    #[serde(rename = "Offer")]
    pub wsto_offer: Offer,
    #[serde(rename = "OWorkingSite")]
    pub ows_o_working_site: OWorkingSite,
    #[serde(rename = "Operations")]
    pub osu_operations: Operations,
    #[serde(rename = "ContactRequest")]
    pub cr_contact_request: ContactRequest,
    #[serde(rename = "MapSymbol")]
    pub ms_map_symbol: MapSymbol,
    #[serde(rename = "Acknowledge")]
    pub ack_acknowledge: Acknowledge,
}

