#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReason {
    #[serde(flatten)]
    pub dis_qualification_reason: BdtString10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationSign {
    #[serde(flatten)]
    pub dis_qualification_sign: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentage {
    #[serde(flatten)]
    pub dis_qualification_percentage: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracySign {
    #[serde(flatten)]
    pub cutting_accuracy_sign: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationPercentageTotal {
    #[serde(flatten)]
    pub dis_qualification_percentage_total: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonText {
    #[serde(flatten)]
    pub dis_qualification_reason_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceIdMJ {
    #[serde(flatten)]
    pub resource_id_m_j: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingAccuracy {
    #[serde(flatten)]
    pub cutting_accuracy: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasons {
    #[serde(flatten)]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

