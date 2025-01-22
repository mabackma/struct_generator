use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamage {
    #[serde(flatten)]
    pub previous_moose_damage: PreviousMooseDamageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamageEvaluationMunicipality {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_municipality: CoMunicipalityNameType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamages {
    #[serde(flatten)]
    pub previous_moose_damages: PreviousMooseDamagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MooseDamageDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationApplicant {
    #[serde(flatten)]
    pub compensation_applicant: ContactInformationBankAccountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDeclarationRealEstates {
    #[serde(flatten)]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationPolygon {
    #[serde(flatten)]
    pub declaration_polygon: DeclarationPolygonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivedCompensation {
    #[serde(flatten)]
    pub received_compensation: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceCompany {
    #[serde(flatten)]
    pub insurance_company: CoString500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationPolygonReference {
    #[serde(flatten)]
    pub declaration_polygon_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousMooseDamageEvaluationDate {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationByLegislation {
    #[serde(flatten)]
    pub compensation_by_legislation: CompensationByLegislationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDeclarationReference {
    #[serde(flatten)]
    pub moose_damage_declaration_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MooseDamageDate {
    #[serde(flatten)]
    pub moose_damage_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationByInsurance {
    #[serde(flatten)]
    pub compensation_by_insurance: CompensationByInsuranceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceNumber {
    #[serde(flatten)]
    pub insurance_number: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationAmount {
    #[serde(flatten)]
    pub compensation_amount: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompensationDescription {
    #[serde(flatten)]
    pub compensation_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousSameAreaMooseDamageCompensationYear {
    #[serde(flatten)]
    pub previous_same_area_moose_damage_compensation_year: CoYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationPolygons {
    #[serde(flatten)]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonsType {
    #[serde(rename = "DeclarationPolygon")]
    pub declaration_polygon: Vec<DeclarationPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: ReRealEstatesWithOwnersInformationType2,
    #[serde(rename = "DeclarationPolygons")]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationBankAccountType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "BankAccount")]
    pub co_bank_account: CoBankAccount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsuranceType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "InsuranceCompany", skip_serializing_if = "Option::is_none")]
    pub insurance_company: Option<CoString500Type>,
    #[serde(rename = "InsuranceNumber", skip_serializing_if = "Option::is_none")]
    pub insurance_number: Option<CoString100Type>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "DeclarationPolygonReference")]
    pub declaration_polygon_reference: CoReferenceType,
    #[serde(rename = "polygonProperty")]
    pub gmlpolygon_property: GmlpolygonProperty,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<FccLocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageType {
    #[serde(rename = "PreviousMooseDamageEvaluationDate", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_date: Option<CoDateType>,
    #[serde(rename = "PreviousMooseDamageEvaluationMunicipality", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_municipality: Option<CoMunicipalityNameType>,
    #[serde(rename = "PreviousSameAreaMooseDamageCompensationYear", skip_serializing_if = "Option::is_none")]
    pub previous_same_area_moose_damage_compensation_year: Option<CoYearType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamagesType {
    #[serde(rename = "PreviousMooseDamage")]
    pub previous_moose_damage: Vec<PreviousMooseDamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislationType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: CoYesNoType,
    #[serde(rename = "CompensationDescription", skip_serializing_if = "Option::is_none")]
    pub compensation_description: Option<String>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<CoDecimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "MooseDamageDeclarationReference")]
    pub moose_damage_declaration_reference: CoReferenceType,
    #[serde(rename = "MooseDamageDate")]
    pub moose_damage_date: CoDateType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<CoString2000Type>,
    #[serde(rename = "CompensationApplicant")]
    pub compensation_applicant: ContactInformationBankAccountType,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub fcc_attorney: Option<FccAttorney>,
    #[serde(rename = "MooseDamageDeclarationRealEstates")]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
    #[serde(rename = "CompensationByInsurance")]
    pub compensation_by_insurance: CompensationByInsuranceType,
    #[serde(rename = "CompensationByLegislation")]
    pub compensation_by_legislation: CompensationByLegislationType,
    #[serde(rename = "PreviousMooseDamages", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damages: Option<PreviousMooseDamagesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub fcc_documents: Option<FccDocuments>,
}

