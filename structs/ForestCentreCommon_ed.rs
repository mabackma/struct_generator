#[derive(Debug, Serialize, Deserialize)]
pub struct ElectronicNotification {
    #[serde(flatten)]
    pub electronic_notification: ElectronicNotificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentative {
    #[serde(flatten)]
    pub working_representative: WorkingRepresentativeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometries {
    #[serde(flatten)]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationTextInformation {
    #[serde(flatten)]
    pub financing_act_application_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalVolume {
    #[serde(flatten)]
    pub small_wood_removal_volume: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliers {
    #[serde(flatten)]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: BankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubGroup {
    #[serde(flatten)]
    pub sub_group: SubGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCode {
    #[serde(flatten)]
    pub feature_code: FeatureCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClass {
    #[serde(flatten)]
    pub height_class: HeightClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActor {
    #[serde(flatten)]
    pub application_actor: ApplicationActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationNumber {
    #[serde(flatten)]
    pub forest_use_declaration_number: ForestUseDeclarationNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    #[serde(flatten)]
    pub unit_price: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreviousMessage {
    #[serde(flatten)]
    pub update_previous_message: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstates {
    #[serde(flatten)]
    pub financing_act_real_estates: FinancingActRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStand {
    #[serde(flatten)]
    pub financing_act_application_stand: FinancingActApplicationStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureInfo {
    #[serde(flatten)]
    pub feature_info: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationNumber {
    #[serde(flatten)]
    pub completion_declaration_number: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilityClass {
    #[serde(flatten)]
    pub fertility_class: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: SoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActNumber {
    #[serde(flatten)]
    pub financing_act_number: FinancingActNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCount {
    #[serde(flatten)]
    pub cutting_stem_count: CuttingStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: StandNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstate {
    #[serde(flatten)]
    pub financing_act_real_estate: FinancingActRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationTextInformation {
    #[serde(flatten)]
    pub financing_act_completion_declaration_text_information: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documents {
    #[serde(flatten)]
    pub documents: DocumentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletion {
    #[serde(flatten)]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIsAuthorizedForEstate {
    #[serde(flatten)]
    pub project_is_authorized_for_estate: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainApplier {
    #[serde(flatten)]
    pub main_applier: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandExtraInfo {
    #[serde(flatten)]
    pub stand_extra_info: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(flatten)]
    pub language: ISO639char2LanguageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjects {
    #[serde(flatten)]
    pub financing_act_application_other_subjects: FinancingActApplicationOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReferenceType {
    #[serde(flatten)]
    pub forest_centre_message_reference_type: ForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPerson {
    #[serde(flatten)]
    pub contact_person: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareOfOwnerShip {
    #[serde(flatten)]
    pub share_of_owner_ship: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDocument {
    #[serde(flatten)]
    pub power_of_attorney_document: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierId {
    #[serde(flatten)]
    pub subsidy_applier_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplier {
    #[serde(flatten)]
    pub subsidy_applier: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverallTotalSubsidy {
    #[serde(flatten)]
    pub overall_total_subsidy: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActor {
    #[serde(flatten)]
    pub completion_declaration_actor: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedEndDate {
    #[serde(flatten)]
    pub estimated_end_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateId {
    #[serde(flatten)]
    pub real_estate_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTreeSpecies {
    #[serde(flatten)]
    pub main_tree_species: TreeSpeciesConciseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStands {
    #[serde(flatten)]
    pub financing_act_completion_stands: FinancingActCompletionStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationReference {
    #[serde(flatten)]
    pub completion_declaration_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClass {
    #[serde(flatten)]
    pub removal_class: RemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjects {
    #[serde(flatten)]
    pub financing_act_completion_other_subjects: FinancingActCompletionOtherSubjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstate {
    #[serde(flatten)]
    pub payee_and_real_estate: PayeeAndRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerReference {
    #[serde(flatten)]
    pub customer_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstates {
    #[serde(flatten)]
    pub payees_and_real_estates: PayeesAndRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationPercentage {
    #[serde(flatten)]
    pub participation_percentage: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostType {
    #[serde(flatten)]
    pub cost_type: CostTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedWorkAmount {
    #[serde(flatten)]
    pub completed_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: DiameterClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometry {
    #[serde(flatten)]
    pub financing_act_application_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtension {
    #[serde(flatten)]
    pub stand_number_extension: StandNumberExtensionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeNumber {
    #[serde(flatten)]
    pub cost_type_number: CostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: Document4MBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStands {
    #[serde(flatten)]
    pub financing_act_application_stands: FinancingActApplicationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: ForestCentreDataMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestReference {
    #[serde(flatten)]
    pub request_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyReceivesPayment {
    #[serde(flatten)]
    pub attorney_receives_payment: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationReference {
    #[serde(flatten)]
    pub financing_act_application_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationType {
    #[serde(flatten)]
    pub fertilization_type: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopOperationProject {
    #[serde(flatten)]
    pub cop_operation_project: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkCode {
    #[serde(flatten)]
    pub financing_act_work_code: FinancingActWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstate {
    #[serde(flatten)]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstate {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(flatten)]
    pub authorization: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStand {
    #[serde(flatten)]
    pub financing_act_completion_stand: FinancingActCompletionStandType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNo {
    #[serde(flatten)]
    pub area_no: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    #[serde(flatten)]
    pub date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorney {
    #[serde(flatten)]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActors {
    #[serde(flatten)]
    pub application_actors: ApplicationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attorney {
    #[serde(flatten)]
    pub attorney: AttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometries {
    #[serde(flatten)]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandReference {
    #[serde(flatten)]
    pub stand_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAmount {
    #[serde(flatten)]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentDate {
    #[serde(flatten)]
    pub sent_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometry {
    #[serde(flatten)]
    pub financing_act_completion_geometry: FinancingActGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReference {
    #[serde(flatten)]
    pub subsidy_applier_reference: SubsidyApplierReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActors {
    #[serde(flatten)]
    pub completion_declaration_actors: CompletionDeclarationActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplication {
    #[serde(flatten)]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentatives {
    #[serde(flatten)]
    pub working_representatives: WorkingRepresentativesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeCode {
    #[serde(flatten)]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActWorkGroup {
    #[serde(flatten)]
    pub financing_act_work_group: FinancingActWorkGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: HeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataText {
    #[serde(flatten)]
    pub metadata_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedStartDate {
    #[serde(flatten)]
    pub estimated_start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceList {
    #[serde(flatten)]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RightToSpecifyBankAccountsOfPaymentTransactions {
    #[serde(flatten)]
    pub right_to_specify_bank_accounts_of_payment_transactions: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentReference {
    #[serde(flatten)]
    pub payment_reference: PaymentsReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedWorkAmount {
    #[serde(flatten)]
    pub planned_work_amount: AmountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeId {
    #[serde(flatten)]
    pub payee_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipType {
    #[serde(flatten)]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDate {
    #[serde(flatten)]
    pub payment_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClass {
    #[serde(flatten)]
    pub small_wood_removal_class: SmallWoodRemovalClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: ExtendedMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: StemCountType,
}

