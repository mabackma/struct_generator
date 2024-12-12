#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationReference {
    #[serde(flatten)]
    pub moose_damage_declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstates {
    #[serde(flatten)]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationDescription {
    #[serde(flatten)]
    pub compensation_description: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonReference {
    #[serde(flatten)]
    pub declaration_polygon_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygon {
    #[serde(flatten)]
    pub declaration_polygon: DeclarationPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsurance {
    #[serde(flatten)]
    pub compensation_by_insurance: CompensationByInsuranceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MooseDamageDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstates {
    #[serde(flatten)]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceNumber {
    #[serde(flatten)]
    pub insurance_number: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationApplicant {
    #[serde(flatten)]
    pub compensation_applicant: ContactInformationBankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislation {
    #[serde(flatten)]
    pub compensation_by_legislation: CompensationByLegislationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDate {
    #[serde(flatten)]
    pub moose_damage_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceCompany {
    #[serde(flatten)]
    pub insurance_company: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygons {
    #[serde(flatten)]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationMunicipality {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_municipality: MunicipalityNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamages {
    #[serde(flatten)]
    pub previous_moose_damages: PreviousMooseDamagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamage {
    #[serde(flatten)]
    pub previous_moose_damage: PreviousMooseDamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationAmount {
    #[serde(flatten)]
    pub compensation_amount: Decimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedCompensation {
    #[serde(flatten)]
    pub received_compensation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageEvaluationDate {
    #[serde(flatten)]
    pub previous_moose_damage_evaluation_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousSameAreaMooseDamageCompensationYear {
    #[serde(flatten)]
    pub previous_same_area_moose_damage_compensation_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: ContactInformationType,
}

