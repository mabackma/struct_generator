#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaysType {
    #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
    pub day: Option<Vec<DayType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayType {
    #[serde(rename = "CalendarDay")]
    pub calendar_day: BdtDateType,
    #[serde(rename = "Hours")]
    pub hours: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDataType {
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<BdtString100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "ResourceName")]
    pub resource_name: BdtString50Type,
    #[serde(rename = "WorkingHoursBusinessDay")]
    pub working_hours_business_day: BdtPositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSaturday")]
    pub working_hours_saturday: BdtPositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSunday")]
    pub working_hours_sunday: BdtPositiveInteger2digitsType,
    #[serde(rename = "Days")]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendarType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

