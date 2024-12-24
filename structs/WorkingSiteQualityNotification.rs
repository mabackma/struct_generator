#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonsType {
    #[serde(rename = "DisQualificationReason", skip_serializing_if = "Option::is_none")]
    pub dis_qualification_reason: Option<Vec<DisQualificationReasonDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceIdMJ", skip_serializing_if = "Option::is_none")]
    pub resource_id_m_j: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: BdtString20Type,
    #[serde(rename = "ForestOwner", skip_serializing_if = "Option::is_none")]
    pub forest_owner: Option<BdtString100Type>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "DisQualificationPercentageTotal")]
    pub dis_qualification_percentage_total: BdtDecimal2FractionDigitsType,
    #[serde(rename = "CuttingAccuracy")]
    pub cutting_accuracy: BdtDecimal2FractionDigitsType,
    #[serde(rename = "DisQualificationSign")]
    pub dis_qualification_sign: BdtString5Type,
    #[serde(rename = "CuttingAccuracySign")]
    pub cutting_accuracy_sign: BdtString5Type,
    #[serde(rename = "Document", skip_serializing_if = "Option::is_none")]
    pub document: Option<base64Binary>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<base64Binary>,
    #[serde(rename = "DisQualificationReasons")]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonDataType {
    #[serde(rename = "DisQualificationReason")]
    pub dis_qualification_reason: BdtString10Type,
    #[serde(rename = "DisQualificationReasonText")]
    pub dis_qualification_reason_text: BdtString200Type,
    #[serde(rename = "DisQualificationPercentage")]
    pub dis_qualification_percentage: BdtDecimal2FractionDigitsType,
}

