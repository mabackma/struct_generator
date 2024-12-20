#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementsType {
    #[serde(rename = "LogDiameter")]
    pub log_diameter: BdtPositiveInteger3digitsType,
    #[serde(rename = "ControlLogDiameter")]
    pub control_log_diameter: BdtPositiveInteger3digitsType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDataType {
    #[serde(rename = "LogKey")]
    pub log_key: BdtString10Type,
    #[serde(rename = "ProductKey")]
    pub product_key: WctERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: BdtPositiveInteger4digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: BdtPositiveInteger4digitsType,
    #[serde(rename = "CalibrationUseLog", skip_serializing_if = "Option::is_none")]
    pub calibration_use_log: Option<BdtYesNoType>,
    #[serde(rename = "LogDiameterClass", skip_serializing_if = "Option::is_none")]
    pub log_diameter_class: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "LogLengthClass", skip_serializing_if = "Option::is_none")]
    pub log_length_class: Option<BdtPositiveInteger4digitsType>,
    #[serde(rename = "LogMeasurements")]
    pub log_measurements: Vec<LogMeasurementsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "DiameterCalibrationDate")]
    pub diameter_calibration_date: BdtTimeStampType,
    #[serde(rename = "DiameterCalibrationReason")]
    pub diameter_calibration_reason: BdtString200Type,
    #[serde(rename = "DiameterCalibrationDescription")]
    pub diameter_calibration_description: BdtString200Type,
    #[serde(rename = "DiameterCalibrationAdjustment")]
    pub diameter_calibration_adjustment: BdtInteger3digitsType,
    #[serde(rename = "DiameterCalibrationAdjustmentButtLog", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration_adjustment_butt_log: Option<BdtInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStemSelectionType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString100Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Xsbase64Binary,
    #[serde(rename = "Stem")]
    pub stem: Vec<StemDataType>,
    #[serde(rename = "LengthCalibration", skip_serializing_if = "Option::is_none")]
    pub length_calibration: Option<Vec<LengthCalibrationType>>,
    #[serde(rename = "DiameterCalibration", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration: Option<Vec<DiameterCalibrationType>>,
    #[serde(rename = "Caliper", skip_serializing_if = "Option::is_none")]
    pub caliper: Option<CaliperType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "LengthCalibrationDate")]
    pub length_calibration_date: BdtTimeStampType,
    #[serde(rename = "LengthCalibrationReason")]
    pub length_calibration_reason: BdtString200Type,
    #[serde(rename = "LengthCalibrationDescription")]
    pub length_calibration_description: BdtString200Type,
    #[serde(rename = "LengthCalibrationAdjustment")]
    pub length_calibration_adjustment: BdtInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperType {
    #[serde(rename = "CaliperId", skip_serializing_if = "Option::is_none")]
    pub caliper_id: Option<BdtString200Type>,
    #[serde(rename = "CaliperApplication", skip_serializing_if = "Option::is_none")]
    pub caliper_application: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDataType {
    #[serde(rename = "StemId")]
    pub stem_id: BdtPositiveIntegerType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<BdtHarvestingStemTypeType>,
    #[serde(rename = "SelectionType")]
    pub selection_type: ControlStemSelectionType,
    #[serde(rename = "RandomControlStemRejectedReason", skip_serializing_if = "Option::is_none")]
    pub random_control_stem_rejected_reason: Option<BdtString100Type>,
    #[serde(rename = "StemCoordinates", skip_serializing_if = "Option::is_none")]
    pub stem_coordinates: Option<GdtPointGeometryType>,
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogDataType>>,
}

