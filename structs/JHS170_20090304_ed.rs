#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: CareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: MaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: EtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: SukupuoliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: KieliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: EnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: LoppuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: ToinenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: SyntymaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: LajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: SahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: LajiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: UlkomaaHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: PostilokeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: VakinainenKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: EdellinenSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: HenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: PaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: PankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: KansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: UlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: HuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: AlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: EtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: OsoiteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: PostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: NimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: AmmattiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: PostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: ValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: NeljasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: NimilajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: BICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: HuoltosuhdeTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: KatuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: KuudesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: LoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: SiviilisaatyTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: ValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: HuoneistotunnisteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: KuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: SahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: HuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: KuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: IkaluokkaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: PuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: FaksinumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: PostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: AlayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: AlkuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: SukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: VoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: KutsumaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: KuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: KolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: TurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: StatusryhmaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: ViidesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: IBANTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: KuolemaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: YritysTunnusTyyppi,
}

