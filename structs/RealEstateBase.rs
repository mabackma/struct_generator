#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityNumber {
    #[serde(flatten)]
    pub municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumber {
    #[serde(flatten)]
    pub unsepareted_parcel_number: UnseparetedParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumber {
    #[serde(flatten)]
    pub unit_number: UnitNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MunicipalityName {
    #[serde(flatten)]
    pub municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityName {
    #[serde(flatten)]
    pub location_municipality_name: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumber {
    #[serde(flatten)]
    pub group_number: GroupNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstate {
    #[serde(flatten)]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitId {
    #[serde(flatten)]
    pub register_unit_id: RegisterUnitIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumber {
    #[serde(flatten)]
    pub parcel_number: ParcelNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcels {
    #[serde(flatten)]
    pub parcels: ParcelsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationMunicipalityNumber {
    #[serde(flatten)]
    pub location_municipality_number: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumber {
    #[serde(flatten)]
    pub area_number: AreaNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateName {
    #[serde(flatten)]
    pub real_estate_name: RealEstateNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateData {
    #[serde(flatten)]
    pub real_estate_data: RealEstateDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parcel {
    #[serde(flatten)]
    pub parcel: ParcelType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstate {
    #[serde(flatten)]
    pub real_estate: BaseRealEstateType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeChar {
    #[serde(flatten)]
    pub unsepareted_parcel_type_char: UnseparetedParcelTypeCharType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelNumberType {
    #[serde(rename = "base")]
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
    pub location_municipality_number: Option<CoMunicipalityNumberType>,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<CoMunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUnitIdType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateNameType {
    #[serde(flatten)]
    pub base: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateRegisterUnitGroup {
    #[serde(rename = "MunicipalityName", skip_serializing_if = "Option::is_none")]
    pub municipality_name: Option<MunicipalityName>,
    #[serde(rename = "GroupNumber")]
    pub group_number: GroupNumber,
    #[serde(rename = "UnitNumber")]
    pub unit_number: UnitNumber,
    #[serde(rename = "AreaNumber")]
    pub area_number: AreaNumber,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumber,
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
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNumberType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnseparetedParcelTypeCharType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: BaseRealEstateType,
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
    pub location_municipality_number: CoMunicipalityNumberType,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<CoMunicipalityNameType>,
}

