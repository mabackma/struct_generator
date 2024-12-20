#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString100Type,
    #[serde(rename = "StartDate")]
    pub start_date: BdtTimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtTimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: BdtTimeStampType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<BdtString100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Xsbase64Binary>,
    #[serde(rename = "StemTypeVolumes")]
    pub stem_type_volumes: StemTypeVolumesType,
    #[serde(rename = "AssortmentVolumes")]
    pub assortment_volumes: AssortmentVolumesType,
    #[serde(rename = "AssortmentVolumesUnclassified", skip_serializing_if = "Option::is_none")]
    pub assortment_volumes_unclassified: Option<AssortmentVolumesUnclassifiedType>,
    #[serde(rename = "AssortmentMatrixVolumes")]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
    #[serde(rename = "ProductUserIds")]
    pub product_user_ids: Vec<ProductUserIdsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "DBH")]
    pub dbh: BdtPositiveInteger3digitsType,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesType {
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: Vec<AssortmentVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassifiedType {
    #[serde(rename = "AssortmentVolumeUnclassified")]
    pub assortment_volume_unclassified: Vec<AssortmentVolumeUnclassifiedType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassifiedType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: BdtString50Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<BdtString50Type>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<BdtString5Type>,
    #[serde(rename = "ProductGroupName", skip_serializing_if = "Option::is_none")]
    pub product_group_name: Option<BdtString50Type>,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: WctStanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: BdtHarvestingStemTypeType,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: BdtString100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "Quality")]
    pub quality: BdtString5Type,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumesType {
    #[serde(rename = "StemTypeVolume")]
    pub stem_type_volume: Vec<StemTypeVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumesType {
    #[serde(rename = "AssortmentMatrixVolume")]
    pub assortment_matrix_volume: Vec<AssortmentMatrixVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumeType {
    #[serde(rename = "ProductUserId")]
    pub product_user_id: BdtString100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: BdtString50Type,
    #[serde(rename = "Quality")]
    pub quality: BdtString5Type,
    #[serde(rename = "LengthClass")]
    pub length_class: BdtPositiveInteger4digitsType,
    #[serde(rename = "DiameterClass")]
    pub diameter_class: BdtPositiveInteger4digitsType,
    #[serde(rename = "Count")]
    pub count: BdtPositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "ProductUserId")]
    pub product_user_id: BdtString100Type,
    #[serde(rename = "ModificationDate")]
    pub modification_date: BdtTimeStampType,
}

