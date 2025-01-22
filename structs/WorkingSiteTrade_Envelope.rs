use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CallForOffer")]
    pub cfo_call_for_offer: CfoCallForOffer,
    #[serde(rename = "CFOWorkingSite")]
    pub cfows_c_f_o_working_site: CfowsCFOWorkingSite,
    #[serde(rename = "BusinessAcceptance")]
    pub ba_business_acceptance: BaBusinessAcceptance,
    #[serde(rename = "Payment")]
    pub wpp_payment: WppPayment,
    #[serde(rename = "MeasurementCertificate")]
    pub wpmc_measurement_certificate: WpmcMeasurementCertificate,
    #[serde(rename = "ForestUseDeclarationReferences")]
    pub fudr_forest_use_declaration_references: FudrForestUseDeclarationReferences,
    #[serde(rename = "Contract")]
    pub wstcr_contract: WstcrContract,
    #[serde(rename = "Offer")]
    pub wsto_offer: WstoOffer,
    #[serde(rename = "OWorkingSite")]
    pub ows_o_working_site: OwsOWorkingSite,
    #[serde(rename = "Operations")]
    pub osu_operations: OsuOperations,
    #[serde(rename = "ContactRequest")]
    pub cr_contact_request: CrContactRequest,
    #[serde(rename = "MapSymbol")]
    pub ms_map_symbol: MsMapSymbol,
    #[serde(rename = "Acknowledge")]
    pub ack_acknowledge: AckAcknowledge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EbEnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

