#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreasType {
    #[serde(rename = "FulfilledArea")]
    pub fulfilled_area: Vec<FulfilledAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Name")]
    pub name: String50Type,
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Volume")]
    pub volume: Decimal6TotalDigitsType,
    #[serde(rename = "VolumeLeft")]
    pub volume_left: Decimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreaType {
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: Decimal3FractionDigitsType,
    #[serde(rename = "AmountLeft")]
    pub amount_left: Decimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContactorId")]
    pub contactor_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Interrupted")]
    pub interrupted: YesNoType,
    #[serde(rename = "FulfilledArea", skip_serializing_if = "Option::is_none")]
    pub fulfilled_area: Option<FulfilledAreasType>,
    #[serde(rename = "UnfulfilledArea", skip_serializing_if = "Option::is_none")]
    pub unfulfilled_area: Option<PolygonOrMultiPolygon2Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}
