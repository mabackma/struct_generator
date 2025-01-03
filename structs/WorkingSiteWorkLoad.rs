#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialType {
    #[serde(rename = "MaterialId", skip_serializing_if = "Option::is_none")]
    pub material_id: Option<String20Type>,
    #[serde(rename = "MaterialCode")]
    pub material_code: MaterialCodeType,
    #[serde(rename = "MaterialVolume")]
    pub material_volume: Decimal2FractionDigitsType,
    #[serde(rename = "MaterialUnit")]
    pub material_unit: MaterialUnitType,
    #[serde(rename = "MaterialSupplier", skip_serializing_if = "Option::is_none")]
    pub material_supplier: Option<String50Type>,
    #[serde(rename = "MaterialProducer", skip_serializing_if = "Option::is_none")]
    pub material_producer: Option<String50Type>,
    #[serde(rename = "MaterialShipment", skip_serializing_if = "Option::is_none")]
    pub material_shipment: Option<String20Type>,
    #[serde(rename = "FreezingDate", skip_serializing_if = "Option::is_none")]
    pub freezing_date: Option<DateType>,
    #[serde(rename = "PackagingDate", skip_serializing_if = "Option::is_none")]
    pub packaging_date: Option<DateType>,
    #[serde(rename = "UnfreezingDate", skip_serializing_if = "Option::is_none")]
    pub unfreezing_date: Option<DateType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "GrainSize", skip_serializing_if = "Option::is_none")]
    pub grain_size: Option<PositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: String10Type,
    #[serde(rename = "FeeListId", skip_serializing_if = "Option::is_none")]
    pub fee_list_id: Option<PositiveIntegerType>,
    #[serde(rename = "FeeYesNo", skip_serializing_if = "Option::is_none")]
    pub fee_yes_no: Option<YesNoType>,
    #[serde(rename = "FeeValue", skip_serializing_if = "Option::is_none")]
    pub fee_value: Option<String10Type>,
    #[serde(rename = "FeeAssortment", skip_serializing_if = "Option::is_none")]
    pub fee_assortment: Option<String50Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "NeedToCheck", skip_serializing_if = "Option::is_none")]
    pub need_to_check: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMemberType {
    #[serde(rename = "ResourceId")]
    pub resource_id: ShortERPIdType,
    #[serde(rename = "UserId")]
    pub user_id: ShortERPIdType,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoadType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<String20Type>,
    #[serde(rename = "WorkLoadId")]
    pub work_load_id: u64,
    #[serde(rename = "Accepted")]
    pub accepted: YesNoType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<String20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<String50Type>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<String50Type>,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier1", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "WorkCodeQualifier2", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "WorkCodeQualifier3", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "WorkCodeQualifier4", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "WorkCodeQualifier5", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier5: Option<Vec<WorkCodeQualifierType5>>,
    #[serde(rename = "WorkLoad1")]
    pub work_load1: Decimal2FractionDigitsType,
    #[serde(rename = "WorkLoad1Unit")]
    pub work_load1_unit: WorkCodeUnitType,
    #[serde(rename = "TransportDistance", skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TransportDistanceUnit", skip_serializing_if = "Option::is_none")]
    pub transport_distance_unit: Option<DistanceUnitType>,
    #[serde(rename = "WorkLoad2", skip_serializing_if = "Option::is_none")]
    pub work_load2: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoad2Unit", skip_serializing_if = "Option::is_none")]
    pub work_load2_unit: Option<WorkCodeUnitType>,
    #[serde(rename = "WorkLoadInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_invoiced: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoadUnitInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_unit_invoiced: Option<WorkCodeUnitType>,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StandFinished", skip_serializing_if = "Option::is_none")]
    pub stand_finished: Option<YesNoType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeStampType>,
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeStampType>,
    #[serde(rename = "SavingTime", skip_serializing_if = "Option::is_none")]
    pub saving_time: Option<TimeStampType>,
    #[serde(rename = "WorkPointCount", skip_serializing_if = "Option::is_none")]
    pub work_point_count: Option<PositiveInteger5digitsType>,
    #[serde(rename = "WorkGrouMembers", skip_serializing_if = "Option::is_none")]
    pub work_grou_members: Option<WorkGrouMembersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material")]
    pub material: Vec<MaterialType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembersType {
    #[serde(rename = "WorkGrouMember")]
    pub work_grou_member: Vec<WorkGrouMemberType>,
}

