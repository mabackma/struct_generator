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
pub struct Days {
    #[serde(flatten)]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursSaturday {
    #[serde(flatten)]
    pub working_hours_saturday: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDay {
    #[serde(flatten)]
    pub calendar_day: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingHoursBusinessDay {
    #[serde(flatten)]
    pub working_hours_business_day: BdtPositiveInteger2digitsType,
}

