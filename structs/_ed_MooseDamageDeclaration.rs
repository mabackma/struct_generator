#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygon {
    #[serde(flatten)]
    pub declaration_polygon: DeclarationPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousSameAreaMooseDamageCompensationYear {
    #[serde(flatten)]
    pub previous_same_area_moose_damage_compensation_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedCompensation {
    #[serde(flatten)]
    pub received_compensation: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislation {
    #[serde(flatten)]
    pub compensation_by_legislation: CompensationByLegislationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationDescription {
    #[serde(flatten)]
    pub compensation_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationDate {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceCompany {
    #[serde(flatten)]
    pub insurance_company: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstates {
    #[serde(flatten)]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceNumber {
    #[serde(flatten)]
    pub insurance_number: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamages {
    #[serde(flatten)]
    pub previous_moose_damages: PreviousMooseDamagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsurance {
    #[serde(flatten)]
    pub compensation_by_insurance: CompensationByInsuranceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDate {
    #[serde(flatten)]
    pub moose_damage_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationAmount {
    #[serde(flatten)]
    pub compensation_amount: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationMunicipality {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_municipality: CoMunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationReference {
    #[serde(flatten)]
    pub moose_damage_declaration_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationApplicant {
    #[serde(flatten)]
    pub compensation_applicant: ContactInformationBankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonReference {
    #[serde(flatten)]
    pub declaration_polygon_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MooseDamageDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygons {
    #[serde(flatten)]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamage {
    #[serde(flatten)]
    pub previous_moose_damage: PreviousMooseDamageType,
}

