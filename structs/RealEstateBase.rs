#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeCharType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "EstateRegisterUnitGroup")]
    pub estate_register_unit_group: String,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub location_municipality_number: Option<MunicipalityNumberType>,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<MunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumberType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumberType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateNameType {
    #[serde(flatten)]
    pub base: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType2 {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber")]
    pub location_municipality_number: MunicipalityNumberType,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<MunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: ParcelNumberType,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumberType {
    #[serde(flatten)]
    pub base: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitIdType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumberType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumberType {
    #[serde(flatten)]
    pub base: String,
}

