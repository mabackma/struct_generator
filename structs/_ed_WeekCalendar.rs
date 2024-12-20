#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDay {
    #[serde(flatten)]
    pub calendar_day: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    #[serde(flatten)]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    #[serde(flatten)]
    pub day: DayType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursBusinessDay {
    #[serde(flatten)]
    pub working_hours_business_day: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    pub resource: ResourceDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceId {
    #[serde(flatten)]
    pub service_buyer_resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordResourceId {
    #[serde(flatten)]
    pub stanford_resource_id: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(flatten)]
    pub resource_type: BdtResourceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hours {
    #[serde(flatten)]
    pub hours: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Days {
    #[serde(flatten)]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendar {
    #[serde(flatten)]
    pub week_calendar: WeekCalendarType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursSunday {
    #[serde(flatten)]
    pub working_hours_sunday: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursSaturday {
    #[serde(flatten)]
    pub working_hours_saturday: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceName {
    #[serde(flatten)]
    pub resource_name: BdtString50Type,
}

