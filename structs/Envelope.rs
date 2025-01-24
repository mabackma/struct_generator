use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Envelope {
    #[serde(flatten)]
    pub envelope: EnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
    #[serde(rename = "Message")]
    pub message: PayloadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadType {
    #[serde(rename = "Acknowledge")]
    pub acknowledge: AcknowledgeType,
    #[serde(rename = "CompanyInformation")]
    pub company_information: CompanyInformationType,
    #[serde(rename = "Contract")]
    pub contract: ContractType,
    #[serde(rename = "ExternalFile")]
    pub external_file: ExternalFileType,
    #[serde(rename = "ForwardingEstimate")]
    pub forwarding_estimate: ForwardingEstimateType,
    #[serde(rename = "ForwardingNotification")]
    pub forwarding_notification: ForwardingNotificationType,
    #[serde(rename = "HarvestingOrder")]
    pub harvesting_order: HarvestingOrderType,
    #[serde(rename = "Image")]
    pub image: ImageType,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: MapSymbolType,
    #[serde(rename = "OrderConfirmation")]
    pub order_confirmation: OrderConfirmationType,
    #[serde(rename = "ProductInstruction")]
    pub product_instruction_product_instruction: ProductInstructionProductInstruction,
    #[serde(rename = "QualityAttachment")]
    pub quality_attachment: QualityAttachmentType,
    #[serde(rename = "Resource")]
    pub resource: ResourceType,
    #[serde(rename = "ResourceSchedule")]
    pub resource_schedule: ResourceScheduleType,
    #[serde(rename = "SilvicultureOrder")]
    pub silviculture_order: SilvicultureOrderType,
    #[serde(rename = "ServiceBuyerResourceLocations")]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
    #[serde(rename = "SmsOperatorStatus")]
    pub sms_operator_status: SmsOperatorStatusType,
    #[serde(rename = "StanfordFile")]
    pub stanford_file: StanfordFileType,
    #[serde(rename = "UserInformation")]
    pub user_information: UserInformationType,
    #[serde(rename = "WeekCalendar")]
    pub week_calendar: WeekCalendarType,
    #[serde(rename = "WorkingSiteAccounting")]
    pub working_site_accounting: WorkingSiteAccountingType,
    #[serde(rename = "WorkingSiteEndNotification")]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
    #[serde(rename = "WorkingSiteFeeBasis")]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
    #[serde(rename = "WorkingSiteFinalAuditBioMassForwarding")]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditDraining")]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
    #[serde(rename = "WorkingSiteFinalAuditFertilization")]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
    #[serde(rename = "WorkingSiteFinalAuditHarvesting")]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement")]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
    #[serde(rename = "WorkingSiteFinalAuditRoadMaking")]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture")]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning")]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
    #[serde(rename = "WorkingSiteFinalAuditStumpForwarding")]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditStumpLifting")]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
    #[serde(rename = "WorkingSiteFinalAuditDynamic")]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
    #[serde(rename = "WorkingSiteForwardedProduction")]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionType,
    #[serde(rename = "WorkingSiteForwardingQualityControl")]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestedProduction")]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
    #[serde(rename = "WorkingSiteHarvestingQualityControl")]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestingQualityControlManual")]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
    #[serde(rename = "WorkingSiteOperational")]
    pub working_site_operational: WorkingSiteOperationalType,
    #[serde(rename = "WorkingSiteOperationalUpdate")]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
    #[serde(rename = "WorkingSiteQualityControlCutting")]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
    #[serde(rename = "WorkingSiteQualityControlFertilization")]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement")]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
    #[serde(rename = "WorkingSiteQualityControlSilviculture")]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning")]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
    #[serde(rename = "WorkingSiteQualityNotification")]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
    #[serde(rename = "WorkingSiteStatus")]
    pub working_site_status: WorkingSiteStatusType,
    #[serde(rename = "WorkingSiteTravelNotification")]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
    #[serde(rename = "WorkingSiteWorkLoad")]
    pub working_site_work_load: WorkingSiteWorkLoadType,
    #[serde(rename = "WorkingSiteWorkTime")]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

