#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContactorId")]
    pub contactor_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "Interrupted")]
    pub interrupted: BdtYesNoType,
    #[serde(rename = "FulfilledArea", skip_serializing_if = "Option::is_none")]
    pub fulfilled_area: Option<FulfilledAreasType>,
    #[serde(rename = "UnfulfilledArea", skip_serializing_if = "Option::is_none")]
    pub unfulfilled_area: Option<GdtPolygonOrMultiPolygon2Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<BdtString1000Type>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: BdtDecimal3FractionDigitsType,
    #[serde(rename = "AmountLeft")]
    pub amount_left: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "Name")]
    pub name: BdtString50Type,
    #[serde(rename = "CodeGroup")]
    pub code_group: BdtAssortmentGroupType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal6TotalDigitsType,
    #[serde(rename = "VolumeLeft")]
    pub volume_left: BdtDecimal6TotalDigitsType,
}

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
pub struct FulfilledAreaType {
    #[serde(rename = "Geometry")]
    pub geometry: GdtPolygonOrMultiPolygon2Type,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<BdtYesNoType>,
}

