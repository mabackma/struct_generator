use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: MunicipalityNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: MunicipalityNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ParcelNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: MunicipalityNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumberType {
    #[serde(flatten)]
    pub base: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeCharType {
    #[serde(rename = "unsepareted_parcel_type_char_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumberType {
    #[serde(rename = "area_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumberType {
    #[serde(rename = "unit_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateNameType {
    #[serde(flatten)]
    pub base: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitIdType {
    #[serde(rename = "register_unit_id_type.base")]
    pub base: String,
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
pub struct EstateRegisterUnitGroup {
    #[serde(rename = "MunicipalityName", skip_serializing_if = "Option::is_none")]
    pub municipality_name: Option<MunicipalityName>,
    #[serde(rename = "AreaNumber")]
    pub area_number: AreaNumber,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumber,
    #[serde(rename = "UnitNumber")]
    pub unit_number: UnitNumber,
    #[serde(rename = "GroupNumber")]
    pub group_number: GroupNumber,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: ParcelNumberType,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumberType {
    #[serde(rename = "unsepareted_parcel_number_type.base")]
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
    #[serde(rename = "group_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "EstateRegisterUnitGroup")]
    pub estate_register_unit_group: EstateRegisterUnitGroup,
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

