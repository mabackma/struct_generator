use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationNumber {
    #[serde(flatten)]
    pub forest_use_declaration_number: ForestUseDeclarationNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingRepresentative {
    #[serde(flatten)]
    pub working_representative: WorkingRepresentativeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActRealEstate {
    #[serde(flatten)]
    pub financing_act_real_estate: FinancingActRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightClass {
    #[serde(flatten)]
    pub height_class: HeightClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyAmount {
    #[serde(flatten)]
    pub subsidy_amount: MoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallWoodRemovalClass {
    #[serde(flatten)]
    pub small_wood_removal_class: SmallWoodRemovalClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandExtraInfo {
    #[serde(flatten)]
    pub stand_extra_info: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipationPercentage {
    #[serde(flatten)]
    pub participation_percentage: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePreviousMessage {
    #[serde(flatten)]
    pub update_previous_message: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentDate {
    #[serde(flatten)]
    pub sent_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerOfAttorneyDocument {
    #[serde(flatten)]
    pub power_of_attorney_document: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionStands {
    #[serde(flatten)]
    pub financing_act_completion_stands: FinancingActCompletionStandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerShipType {
    #[serde(flatten)]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    #[serde(flatten)]
    pub date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentReference {
    #[serde(flatten)]
    pub payment_reference: PaymentsReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationActors {
    #[serde(flatten)]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedEndDate {
    #[serde(flatten)]
    pub estimated_end_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaNo {
    #[serde(flatten)]
    pub area_no: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlannedWorkAmount {
    #[serde(flatten)]
    pub planned_work_amount: AmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: AttorneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedStartDate {
    #[serde(flatten)]
    pub estimated_start_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplierId {
    #[serde(flatten)]
    pub subsidy_applier_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkCompletion {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionGeometries {
    #[serde(flatten)]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationReference {
    #[serde(flatten)]
    pub completion_declaration_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataText {
    #[serde(flatten)]
    pub metadata_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: CostTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareOfOwnerShip {
    #[serde(flatten)]
    pub share_of_owner_ship: PercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerReference {
    #[serde(flatten)]
    pub customer_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingStemCount {
    #[serde(flatten)]
    pub cutting_stem_count: CuttingStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationOtherSubjects {
    #[serde(flatten)]
    pub financing_act_application_other_subjects: FinancingActApplicationOtherSubjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationActor {
    #[serde(flatten)]
    pub application_actor: ApplicationActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIsAuthorizedForEstate {
    #[serde(flatten)]
    pub project_is_authorized_for_estate: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionOtherSubjects {
    #[serde(flatten)]
    pub financing_act_completion_other_subjects: FinancingActCompletionOtherSubjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub contact_person: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationActors {
    #[serde(flatten)]
    pub application_actors: ApplicationActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplierReferenceList {
    #[serde(flatten)]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: PayeeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayeesAndRealEstates {
    #[serde(flatten)]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationGeometries {
    #[serde(flatten)]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeNumber {
    #[serde(flatten)]
    pub cost_type_number: CostTypeNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActWorkGroup {
    #[serde(flatten)]
    pub financing_act_work_group: FinancingActWorkGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallWoodRemovalVolume {
    #[serde(flatten)]
    pub small_wood_removal_volume: PositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubsidyApplierReference {
    #[serde(flatten)]
    pub subsidy_applier_reference: SubsidyApplierReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingRepresentatives {
    #[serde(flatten)]
    pub working_representatives: WorkingRepresentativesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkApplicationRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkCompletionRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElectronicNotification {
    #[serde(flatten)]
    pub electronic_notification: ElectronicNotificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationActor {
    #[serde(flatten)]
    pub completion_declaration_actor: PayeeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionGeometry {
    #[serde(flatten)]
    pub financing_act_completion_geometry: FinancingActGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionStand {
    #[serde(flatten)]
    pub financing_act_completion_stand: FinancingActCompletionStandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedWorkAmount {
    #[serde(flatten)]
    pub completed_work_amount: AmountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopOperationProject {
    #[serde(flatten)]
    pub cop_operation_project: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizationType {
    #[serde(flatten)]
    pub fertilization_type: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RightToSpecifyBankAccountsOfPaymentTransactions {
    #[serde(flatten)]
    pub right_to_specify_bank_accounts_of_payment_transactions: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerShipTypeCode {
    #[serde(flatten)]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActRealEstates {
    #[serde(flatten)]
    pub financing_act_real_estates: FinancingActRealEstatesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorization {
    #[serde(flatten)]
    pub authorization: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(flatten)]
    pub message: ForestCentreDataMessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OverallTotalSubsidy {
    #[serde(flatten)]
    pub overall_total_subsidy: MoneyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayeeId {
    #[serde(flatten)]
    pub payee_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestReference {
    #[serde(flatten)]
    pub request_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationStand {
    #[serde(flatten)]
    pub financing_act_application_stand: FinancingActApplicationStandType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub financing_act_application_text_information: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationGeometry {
    #[serde(flatten)]
    pub financing_act_application_geometry: FinancingActGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: ContactInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationStands {
    #[serde(flatten)]
    pub financing_act_application_stands: FinancingActApplicationStandsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: DiameterClassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActNumber {
    #[serde(flatten)]
    pub financing_act_number: FinancingActNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActApplicationReference {
    #[serde(flatten)]
    pub financing_act_application_reference: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostTypeAndCompletedWorkApplication {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayeeAndRealEstate {
    #[serde(flatten)]
    pub payee_and_real_estate: PayeeAndRealEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttorneyReceivesPayment {
    #[serde(flatten)]
    pub attorney_receives_payment: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActWorkCode {
    #[serde(flatten)]
    pub financing_act_work_code: FinancingActWorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub financing_act_completion_declaration_text_information: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionDeclarationNumber {
    #[serde(flatten)]
    pub completion_declaration_number: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainApplier {
    #[serde(flatten)]
    pub main_applier: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    #[serde(flatten)]
    pub language: ISO639char2LanguageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: UnitPriceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: LocationEstateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub forest_centre_message_reference_type: ForestCentreMessageReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemovalClass {
    #[serde(flatten)]
    pub removal_class: RemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringNotEmptyType,
    #[serde(rename = "PayeeId")]
    pub payee_id: IdStringNotEmptyType,
    #[serde(rename = "ParticipationPercentage", skip_serializing_if = "Option::is_none")]
    pub participation_percentage: Option<PercentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "SubsidyApplierReferenceList")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandsType {
    #[serde(rename = "FinancingActCompletionStand")]
    pub financing_act_completion_stand: Vec<FinancingActCompletionStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "FinancingActApplicationGeometry")]
    pub financing_act_application_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "SubsidyApplierReference")]
    pub subsidy_applier_reference: Vec<SubsidyApplierReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate")]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
    #[serde(rename = "FinancingActApplicationGeometries")]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountType {
    #[serde(rename = "amount_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActorsType {
    #[serde(rename = "CompletionDeclarationActor")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstatesType {
    #[serde(rename = "PayeeAndRealEstate")]
    pub payee_and_real_estate: Vec<PayeeAndRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate")]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
    #[serde(rename = "FinancingActCompletionGeometries")]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandsType {
    #[serde(rename = "FinancingActApplicationStand")]
    pub financing_act_application_stand: Vec<FinancingActApplicationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceType {
    #[serde(rename = "SubsidyApplierId")]
    pub subsidy_applier_id: IdStringNotEmptyType,
    #[serde(rename = "MainApplier")]
    pub main_applier: YesNoType,
    #[serde(rename = "ShareOfOwnerShip", skip_serializing_if = "Option::is_none")]
    pub share_of_owner_ship: Option<PercentType>,
    #[serde(rename = "OwnerShipType")]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeType {
    #[serde(rename = "OwnerShipTypeCode")]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType {
    #[serde(rename = "CostTypeNumber")]
    pub cost_type_number: CostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsReferenceType {
    #[serde(rename = "payments_reference_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationOfAttorneyType {
    #[serde(rename = "Authorization")]
    pub authorization: YesNoType,
    #[serde(rename = "Date")]
    pub date: DateType,
    #[serde(rename = "AttorneyReceivesPayment")]
    pub attorney_receives_payment: YesNoType,
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: String2000Type,
    #[serde(rename = "PowerOfAttorneyDocument", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_document: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    #[serde(rename = "cutting_stem_count_type.base")]
    pub base: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: ReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<ExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesConciseType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<HeightClassType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<HeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<DiameterClassType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<DiameterType>,
    #[serde(rename = "RemovalClass", skip_serializing_if = "Option::is_none")]
    pub removal_class: Option<RemovalClassType>,
    #[serde(rename = "CuttingStemCount", skip_serializing_if = "Option::is_none")]
    pub cutting_stem_count: Option<CuttingStemCountType>,
    #[serde(rename = "SmallWoodRemovalClass", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_class: Option<SmallWoodRemovalClassType>,
    #[serde(rename = "SmallWoodRemovalVolume", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_volume: Option<PositiveInteger4digitsType>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletion")]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<String2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: PowerOfAttorneyType,
    #[serde(rename = "RightToSpecifyBankAccountsOfPaymentTransactions")]
    pub right_to_specify_bank_accounts_of_payment_transactions: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumberType {
    #[serde(rename = "financing_act_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumberType {
    #[serde(rename = "forest_use_declaration_number_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<Document4MBType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativeType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount")]
    pub completed_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "PaymentReference", skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<PaymentsReferenceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: ForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<ReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDescriptionType {
    #[serde(flatten)]
    pub base: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountType {
    #[serde(flatten)]
    pub base: Decimal7And2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedTotalSubsidyType {
    #[serde(flatten)]
    pub base: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType2 {
    #[serde(flatten)]
    pub base: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliersType {
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorsType {
    #[serde(rename = "ApplicationActor")]
    pub application_actor: Vec<ApplicationActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedAmountUnitType {
    #[serde(flatten)]
    pub base: ForestCentreUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstatesType {
    #[serde(rename = "FinancingActRealEstate")]
    pub financing_act_real_estate: Vec<FinancingActRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureInfo")]
    pub feature_info: String500Type,
    #[serde(rename = "GdtPointAndLineGeometriesGroup")]
    pub gdt_point_and_line_geometries_group: PointAndLineGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativesType {
    #[serde(rename = "WorkingRepresentative")]
    pub working_representative: Vec<WorkingRepresentativeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPriceType {
    #[serde(rename = "unit_price_type.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstateType {
    #[serde(flatten)]
    pub base: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: ReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<ExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<HeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<DiameterClassType>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication")]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<String2000Type>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: polygonProperty,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "FinancingActCompletionGeometry")]
    pub financing_act_completion_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount")]
    pub planned_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecidedUnitPriceType {
    #[serde(flatten)]
    pub base: MoneyType,
}

