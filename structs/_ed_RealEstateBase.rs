#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: RegisterUnitIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: UnitNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: RealEstateNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: GroupNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: BaseRealEstateType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: AreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

