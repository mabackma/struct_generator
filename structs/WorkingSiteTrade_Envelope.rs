use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CfoCallForOffer")]
    pub cfo_call_for_offer: CallForOffer,
    #[serde(rename = "CfowsCFOWorkingSite")]
    pub cfows_c_f_o_working_site: CFOWorkingSite,
    #[serde(rename = "BaBusinessAcceptance")]
    pub ba_business_acceptance: BusinessAcceptance,
    #[serde(rename = "WppPayment")]
    pub wpp_payment: Payment,
    #[serde(rename = "WpmcMeasurementCertificate")]
    pub wpmc_measurement_certificate: MeasurementCertificate,
    #[serde(rename = "FudrForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: ForestUseDeclarationReferences,
    #[serde(rename = "WstcrContract")]
    pub wstcr_contract: Contract,
    #[serde(rename = "WstoOffer")]
    pub wsto_offer: Offer,
    #[serde(rename = "OwsOWorkingSite")]
    pub ows_o_working_site: OWorkingSite,
    #[serde(rename = "OsuOperations")]
    pub osu_operations: Operations,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: ContactRequest,
    #[serde(rename = "MsMapSymbol")]
    pub ms_map_symbol: MapSymbol,
    #[serde(rename = "AckAcknowledge")]
    pub ack_acknowledge: Acknowledge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

