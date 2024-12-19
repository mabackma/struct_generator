#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(flatten)]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertDate {
    #[serde(flatten)]
    pub insert_date: InsertDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidValue {
    #[serde(flatten)]
    pub paid_value: PaidValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificate {
    #[serde(flatten)]
    pub measurement_certificate: MeasurementCertificateType,
}

