#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumes {
    #[serde(flatten)]
    pub assortment_volumes: AssortmentVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthClass {
    #[serde(flatten)]
    pub length_class: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumes {
    #[serde(flatten)]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolume {
    #[serde(flatten)]
    pub assortment_matrix_volume: AssortmentMatrixVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassified {
    #[serde(flatten)]
    pub assortment_volume_unclassified: AssortmentVolumeUnclassifiedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBH {
    #[serde(flatten)]
    pub dbh: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolume {
    #[serde(flatten)]
    pub stem_type_volume: StemTypeVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningMeters {
    #[serde(flatten)]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProduction {
    #[serde(flatten)]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(flatten)]
    pub count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductGroupName {
    #[serde(flatten)]
    pub product_group_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumes {
    #[serde(flatten)]
    pub stem_type_volumes: StemTypeVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassified {
    #[serde(flatten)]
    pub assortment_volumes_unclassified: AssortmentVolumesUnclassifiedType,
}

