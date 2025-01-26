use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingHeadHydraulicsRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderLinkageHydraulicsRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderLinkageMechanicalRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnutilizedTimeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarrierElectricalRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingHeadElectricalRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherRepairReasonType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisturbanceCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineRunTimeCategoryListType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherWorkCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarrierAirRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherMachineDownTimeCodeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarrierHydraulicsRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorWorkTimeCategoryType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarrierMechanicalRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderLinkageElectricalRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingHeadMechanicalRepairReasonCodeType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

