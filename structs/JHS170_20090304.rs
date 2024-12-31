use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: UlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: HuoltosuhdeTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: HenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: SahkoinenAsiointiTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: EtuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: IBANTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: KolmasRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: LajiTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: KieliKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: HuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: LoppuPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: IkaluokkaTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: PaayksikkoNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: EtunimetNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: YritysTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: NeljasRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: SukuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: PuhelinnumeroTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: SiviilisaatyTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: PostilokerolyhenneTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: LajiKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: KansalaisuusKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: FaksinumeroTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: KuntaKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: AmmattiKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: TurvakieltoKytkinTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: VakinainenKytkinTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: NimilajiKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: AlayksikkoNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: ToinenRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: CareOfTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: ValtiotunnusKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: KatuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: KutsumaNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: MaatunnusKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: KuudesRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: VoimassaoloKytkinTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: UlkomaaHenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: KuolemaPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: SukupuoliKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: PostilokeroTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: OsoiteNumeroTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: EdellinenSukuNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: NimiTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: BICKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: EnsimmainenRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: ViidesRiviTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: ValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: KuvausTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: HuoneistotunnisteNumeroTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: AlkuHetkiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: KuntaNumeroTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: SahkopostiosoiteTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: StatusryhmaTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: PostitoimipaikkaNimiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: AlkuPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: PostinumeroKoodiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: SyntymaPvmTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: HuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: LoppuHetkiTyyppi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: PankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTekstiTyyppi {
    #[serde(rename = "laji_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimiTyyppi {
    #[serde(rename = "ulkomaa_postitoimipaikka_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTekstiTyyppi {
    #[serde(rename = "neljas_rivi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnusTyyppi {
    #[serde(rename = "sahkoinen_asiointi_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodiTyyppi {
    #[serde(rename = "b_i_c_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodiTyyppi {
    #[serde(rename = "maatunnus_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTekstiTyyppi {
    #[serde(rename = "viides_rivi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTekstiTyyppi {
    #[serde(rename = "postilokero_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodiTyyppi {
    #[serde(rename = "kansalaisuus_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodiTyyppi {
    #[serde(rename = "nimilaji_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodiTyyppi {
    #[serde(rename = "kieli_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimiTyyppi {
    #[serde(rename = "alayksikko_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTekstiTyyppi {
    #[serde(rename = "nimi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTekstiTyyppi {
    #[serde(rename = "faksinumero_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumeroTyyppi {
    #[serde(rename = "osoite_numero_tyyppi.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumeroTyyppi {
    #[serde(rename = "huoneistotunniste_numero_tyyppi.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTekstiTyyppi {
    #[serde(rename = "sahkopostiosoite_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTekstiTyyppi {
    #[serde(rename = "statusryhma_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetkiTyyppi {
    #[serde(rename = "loppu_hetki_tyyppi.base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimiTyyppi {
    #[serde(rename = "postitoimipaikka_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkinTyyppi {
    #[serde(rename = "voimassaolo_kytkin_tyyppi.base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodiTyyppi {
    #[serde(rename = "postinumero_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodiTyyppi {
    #[serde(rename = "laji_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnusTyyppi {
    #[serde(rename = "valiaikainen_henkilo_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumeroTyyppi {
    #[serde(rename = "kunta_numero_tyyppi.base")]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimiTyyppi {
    #[serde(rename = "etu_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvmTyyppi {
    #[serde(rename = "alku_pvm_tyyppi.base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimiTyyppi {
    #[serde(rename = "etunimet_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvmTyyppi {
    #[serde(rename = "kuolema_pvm_tyyppi.base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTekstiTyyppi {
    #[serde(rename = "care_of_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnusTyyppi {
    #[serde(rename = "i_b_a_n_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimiTyyppi {
    #[serde(rename = "katu_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnusTyyppi {
    #[serde(rename = "henkilo_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTekstiTyyppi {
    #[serde(rename = "kolmas_rivi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTekstiTyyppi {
    #[serde(rename = "kuudes_rivi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTekstiTyyppi {
    #[serde(rename = "kuvaus_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTekstiTyyppi {
    #[serde(rename = "siviilisaaty_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkinTyyppi {
    #[serde(rename = "vakinainen_kytkin_tyyppi.base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodiTyyppi {
    #[serde(rename = "valtiotunnus_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTekstiTyyppi {
    #[serde(rename = "ensimmainen_rivi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvmTyyppi {
    #[serde(rename = "loppu_pvm_tyyppi.base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTekstiTyyppi {
    #[serde(rename = "huoltosuhde_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimiTyyppi {
    #[serde(rename = "edellinen_suku_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetkiTyyppi {
    #[serde(rename = "alku_hetki_tyyppi.base")]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTekstiTyyppi {
    #[serde(rename = "huoneistotunniste_jakokirjain_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodiTyyppi {
    #[serde(rename = "sukupuoli_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnusTyyppi {
    #[serde(rename = "yritys_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTekstiTyyppi {
    #[serde(rename = "ikaluokka_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTekstiTyyppi {
    #[serde(rename = "postilokerolyhenne_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTekstiTyyppi {
    #[serde(rename = "puhelinnumero_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimiTyyppi {
    #[serde(rename = "suku_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimiTyyppi {
    #[serde(rename = "kutsuma_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkinTyyppi {
    #[serde(rename = "turvakielto_kytkin_tyyppi.base")]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnusTyyppi {
    #[serde(rename = "ulkomaa_henkilo_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTekstiTyyppi {
    #[serde(rename = "huoneistotunniste_kirjain_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimiTyyppi {
    #[serde(rename = "paayksikko_nimi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvmTyyppi {
    #[serde(rename = "syntyma_pvm_tyyppi.base")]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTekstiTyyppi {
    #[serde(rename = "toinen_rivi_teksti_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnusTyyppi {
    #[serde(rename = "pankkitili_tunnus_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodiTyyppi {
    #[serde(rename = "kunta_koodi_tyyppi.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodiTyyppi {
    #[serde(rename = "ammatti_koodi_tyyppi.base")]
    pub base: String,
}

