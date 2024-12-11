#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControl {
    #[serde(flatten)]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    #[serde(flatten)]
    pub user_information: UserInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotification {
    #[serde(flatten)]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(flatten)]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocations {
    #[serde(flatten)]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotification {
    #[serde(flatten)]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControl {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrder {
    #[serde(flatten)]
    pub silviculture_order: SilvicultureOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachment {
    #[serde(flatten)]
    pub quality_attachment: QualityAttachmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioning {
    #[serde(flatten)]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: WorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFile {
    #[serde(flatten)]
    pub external_file: ExternalFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilviculture {
    #[serde(flatten)]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(flatten)]
    pub envelope: EnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acknowledge {
    #[serde(flatten)]
    pub acknowledge: AcknowledgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbol {
    #[serde(flatten)]
    pub map_symbol: MapSymbolType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagement {
    #[serde(flatten)]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManual {
    #[serde(flatten)]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperational {
    #[serde(flatten)]
    pub working_site_operational: WorkingSiteOperationalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMaking {
    #[serde(flatten)]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    pub resource: ResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub image: ImageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoad {
    #[serde(flatten)]
    pub working_site_work_load: WorkingSiteWorkLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTime {
    #[serde(flatten)]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrder {
    #[serde(flatten)]
    pub harvesting_order: HarvestingOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDraining {
    #[serde(flatten)]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasis {
    #[serde(flatten)]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProduction {
    #[serde(flatten)]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvesting {
    #[serde(flatten)]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotification {
    #[serde(flatten)]
    pub forwarding_notification: ForwardingNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: PayloadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmation {
    #[serde(flatten)]
    pub order_confirmation: OrderConfirmationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSchedule {
    #[serde(flatten)]
    pub resource_schedule: ResourceScheduleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendar {
    #[serde(flatten)]
    pub week_calendar: WeekCalendarType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotification {
    #[serde(flatten)]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFile {
    #[serde(flatten)]
    pub stanford_file: StanfordFileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilviculture {
    #[serde(flatten)]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCutting {
    #[serde(flatten)]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformation {
    #[serde(flatten)]
    pub company_information: CompanyInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamic {
    #[serde(flatten)]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProduction {
    #[serde(flatten)]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingType,
}

