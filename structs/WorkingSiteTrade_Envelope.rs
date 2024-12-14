#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CallForOffer")]
    pub call_for_offer: CallForOfferType,
    #[serde(rename = "CFOWorkingSite")]
    pub c_f_o_working_site: WorkingSiteType,
    #[serde(rename = "BusinessAcceptance")]
    pub business_acceptance: BusinessAcceptanceType,
    #[serde(rename = "Payment")]
    pub payment: PaymentDataType,
    #[serde(rename = "MeasurementCertificate")]
    pub measurement_certificate: MeasurementCertificateType,
    #[serde(rename = "ForestUseDeclarationReferences")]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
    #[serde(rename = "Contract")]
    pub contract: ContractType,
    #[serde(rename = "Offer")]
    pub offer: OfferType,
    #[serde(rename = "OWorkingSite")]
    pub o_working_site: WorkingSiteType,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
    #[serde(rename = "ContactRequest")]
    pub contact_request: ContactRequestType,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: MapSymbolType,
    #[serde(rename = "Acknowledge")]
    pub acknowledge: AcknowledgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

