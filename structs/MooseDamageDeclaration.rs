#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<LocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: RealEstatesWithOwnersInformationType2,
    #[serde(rename = "DeclarationPolygons")]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamagesType {
    #[serde(rename = "PreviousMooseDamage")]
    pub previous_moose_damage: Vec<PreviousMooseDamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislationType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: YesNoType,
    #[serde(rename = "CompensationDescription", skip_serializing_if = "Option::is_none")]
    pub compensation_description: Option<string>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<Decimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "MooseDamageDeclarationReference")]
    pub moose_damage_declaration_reference: ReferenceType,
    #[serde(rename = "MooseDamageDate")]
    pub moose_damage_date: DateType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String2000Type>,
    #[serde(rename = "CompensationApplicant")]
    pub compensation_applicant: ContactInformationBankAccountType,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<String>,
    #[serde(rename = "MooseDamageDeclarationRealEstates")]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
    #[serde(rename = "CompensationByInsurance")]
    pub compensation_by_insurance: CompensationByInsuranceType,
    #[serde(rename = "CompensationByLegislation")]
    pub compensation_by_legislation: CompensationByLegislationType,
    #[serde(rename = "PreviousMooseDamages", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damages: Option<PreviousMooseDamagesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsuranceType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: YesNoType,
    #[serde(rename = "InsuranceCompany", skip_serializing_if = "Option::is_none")]
    pub insurance_company: Option<String500Type>,
    #[serde(rename = "InsuranceNumber", skip_serializing_if = "Option::is_none")]
    pub insurance_number: Option<String100Type>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<Decimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationBankAccountType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "BankAccount")]
    pub bank_account: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "DeclarationPolygonReference")]
    pub declaration_polygon_reference: ReferenceType,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonsType {
    #[serde(rename = "DeclarationPolygon")]
    pub declaration_polygon: Vec<DeclarationPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageType {
    #[serde(rename = "PreviousMooseDamageEvaluationDate", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_date: Option<DateType>,
    #[serde(rename = "PreviousMooseDamageEvaluationMunicipality", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_municipality: Option<MunicipalityNameType>,
    #[serde(rename = "PreviousSameAreaMooseDamageCompensationYear", skip_serializing_if = "Option::is_none")]
    pub previous_same_area_moose_damage_compensation_year: Option<YearType>,
}

