#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    #[serde(flatten)]
    pub extra_info: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: SahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: VoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: StoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolume {
    #[serde(flatten)]
    pub harvested_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolume {
    #[serde(flatten)]
    pub forwarded_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCount {
    #[serde(flatten)]
    pub stem_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionCode {
    #[serde(flatten)]
    pub restriction_code: RestrictionCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: PostilokeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: UlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diameter {
    #[serde(flatten)]
    pub diameter: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: YritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAccounting {
    #[serde(flatten)]
    pub final_accounting: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: LajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFileName {
    #[serde(flatten)]
    pub document_file_name: DocumentFileNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: SukupuoliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: AmmattiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: HuoneistotunnisteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDescription {
    #[serde(flatten)]
    pub document_description: DocumentDescriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode {
    #[serde(flatten)]
    pub postal_code: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumNumber {
    #[serde(flatten)]
    pub stratum_number: StratumNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: EtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: NeljasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: AlayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTrees {
    #[serde(flatten)]
    pub spare_trees: SpareTreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionType {
    #[serde(flatten)]
    pub restriction_type: RestrictionTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(flatten)]
    pub identifier_type: IdentifierTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub document: DocumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBAN {
    #[serde(flatten)]
    pub iban: IBANType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: FaksinumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOfSpareTrees {
    #[serde(flatten)]
    pub mean_height_of_spare_trees: HeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogVolume {
    #[serde(flatten)]
    pub saw_log_volume: SawLogVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemBiomass {
    #[serde(flatten)]
    pub stem_biomass: StemBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: PaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOfSpareTrees {
    #[serde(flatten)]
    pub mean_diameter_of_spare_trees: DiameterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: ValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawLogPercent {
    #[serde(flatten)]
    pub saw_log_percent: SawLogPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionTreeTarget {
    #[serde(flatten)]
    pub retention_tree_target: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub identifier: IdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: UlkomaaHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: SiviilisaatyTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: ViidesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: KansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: CareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolumeAccounted {
    #[serde(flatten)]
    pub harvested_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: EnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeCategory {
    #[serde(flatten)]
    pub spare_tree_category: SpareTreeCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: AlkuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: OsoiteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Age {
    #[serde(flatten)]
    pub age: AgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpBiomass {
    #[serde(flatten)]
    pub stump_biomass: StumpBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: KatuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: KolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BIC {
    #[serde(flatten)]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(flatten)]
    pub email_address: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: ToinenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: KuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedVolume {
    #[serde(flatten)]
    pub planned_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGrowth {
    #[serde(flatten)]
    pub volume_growth: VolumeGrowthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountingDate {
    #[serde(flatten)]
    pub accounting_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: IkaluokkaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTime {
    #[serde(flatten)]
    pub change_time: ChangeTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: KuudesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: HenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: SyntymaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    #[serde(flatten)]
    pub file_type: FileTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: ValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpWoodVolume {
    #[serde(flatten)]
    pub pulp_wood_volume: PulpWoodVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankReferenceNumber {
    #[serde(flatten)]
    pub bank_reference_number: BankReferenceNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WideCertificationSystem {
    #[serde(flatten)]
    pub wide_certification_system: WideCertificationSystemType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: NimilajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: KuolemaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: TurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchBiomass {
    #[serde(flatten)]
    pub branch_biomass: BranchBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: IBANTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinary {
    #[serde(flatten)]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOfSpareTrees {
    #[serde(flatten)]
    pub volume_of_spare_trees: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Height {
    #[serde(flatten)]
    pub height: HeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: LajiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolumeAccounted {
    #[serde(flatten)]
    pub forwarded_volume_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierValue {
    #[serde(flatten)]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: SahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: EtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: VakinainenKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biomass {
    #[serde(flatten)]
    pub biomass: BiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(flatten)]
    pub bank_account: BankAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageCode {
    #[serde(flatten)]
    pub language_code: LanguageCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: VolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(flatten)]
    pub address: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDate {
    #[serde(flatten)]
    pub document_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: EdellinenSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: KuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: HuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeState {
    #[serde(flatten)]
    pub change_state: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: PostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: HuoltosuhdeTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: SukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: LoppuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: LoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: PostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: KuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafBiomass {
    #[serde(flatten)]
    pub leaf_biomass: LeafBiomassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: KieliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContarctorId {
    #[serde(flatten)]
    pub contarctor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: BICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: HuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountAccounted {
    #[serde(flatten)]
    pub amount_accounted: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: MaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountOfSpareTrees {
    #[serde(flatten)]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: PankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: StemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: PuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountNotified {
    #[serde(flatten)]
    pub amount_notified: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: PostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: KutsumaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentClass {
    #[serde(flatten)]
    pub document_class: DocumentClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: StatusryhmaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOffice {
    #[serde(flatten)]
    pub post_office: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClassOfSpareTrees {
    #[serde(flatten)]
    pub diameter_class_of_spare_trees: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalArea {
    #[serde(flatten)]
    pub basal_area: BasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(flatten)]
    pub identifiers: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: AlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub data_source: DataSourceType,
}

