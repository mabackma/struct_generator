#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetki {
    #[serde(flatten)]
    pub loppu_hetki: JhsLoppuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTeksti {
    #[serde(flatten)]
    pub laji_teksti: JhsLajiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodi {
    #[serde(flatten)]
    pub b_i_c_koodi: JhsBICKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvm {
    #[serde(flatten)]
    pub syntyma_pvm: JhsSyntymaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetki {
    #[serde(flatten)]
    pub alku_hetki: JhsAlkuHetkiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTeksti {
    #[serde(flatten)]
    pub neljas_rivi_teksti: JhsNeljasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodi {
    #[serde(flatten)]
    pub kunta_koodi: JhsKuntaKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimi {
    #[serde(flatten)]
    pub etunimet_nimi: JhsEtunimetNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimi {
    #[serde(flatten)]
    pub katu_nimi: JhsKatuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTeksti {
    #[serde(flatten)]
    pub huoltosuhde_teksti: JhsHuoltosuhdeTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodi {
    #[serde(flatten)]
    pub maatunnus_koodi: JhsMaatunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_kirjain_teksti: JhsHuoneistotunnisteKirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvm {
    #[serde(flatten)]
    pub alku_pvm: JhsAlkuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTeksti {
    #[serde(flatten)]
    pub toinen_rivi_teksti: JhsToinenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodi {
    #[serde(flatten)]
    pub valtiotunnus_koodi: JhsValtiotunnusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkin {
    #[serde(flatten)]
    pub voimassaolo_kytkin: JhsVoimassaoloKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnus {
    #[serde(flatten)]
    pub henkilo_tunnus: JhsHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodi {
    #[serde(flatten)]
    pub ammatti_koodi: JhsAmmattiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTeksti {
    #[serde(flatten)]
    pub huoneistotunniste_jakokirjain_teksti: JhsHuoneistotunnisteJakokirjainTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvm {
    #[serde(flatten)]
    pub loppu_pvm: JhsLoppuPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTeksti {
    #[serde(flatten)]
    pub nimi_teksti: JhsNimiTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTeksti {
    #[serde(flatten)]
    pub kuudes_rivi_teksti: JhsKuudesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTeksti {
    #[serde(flatten)]
    pub puhelinnumero_teksti: JhsPuhelinnumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTeksti {
    #[serde(flatten)]
    pub care_of_teksti: JhsCareOfTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodi {
    #[serde(flatten)]
    pub kieli_koodi: JhsKieliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTeksti {
    #[serde(flatten)]
    pub postilokerolyhenne_teksti: JhsPostilokerolyhenneTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimi {
    #[serde(flatten)]
    pub paayksikko_nimi: JhsPaayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkin {
    #[serde(flatten)]
    pub turvakielto_kytkin: JhsTurvakieltoKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimi {
    #[serde(flatten)]
    pub edellinen_suku_nimi: JhsEdellinenSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodi {
    #[serde(flatten)]
    pub kansalaisuus_koodi: JhsKansalaisuusKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTeksti {
    #[serde(flatten)]
    pub ikaluokka_teksti: JhsIkaluokkaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvm {
    #[serde(flatten)]
    pub kuolema_pvm: JhsKuolemaPvmTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTeksti {
    #[serde(flatten)]
    pub viides_rivi_teksti: JhsViidesRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnus {
    #[serde(flatten)]
    pub ulkomaa_henkilo_tunnus: JhsUlkomaaHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodi {
    #[serde(flatten)]
    pub postinumero_koodi: JhsPostinumeroKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTeksti {
    #[serde(flatten)]
    pub siviilisaaty_teksti: JhsSiviilisaatyTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimi {
    #[serde(flatten)]
    pub kutsuma_nimi: JhsKutsumaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTeksti {
    #[serde(flatten)]
    pub faksinumero_teksti: JhsFaksinumeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimi {
    #[serde(flatten)]
    pub etu_nimi: JhsEtuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimi {
    #[serde(flatten)]
    pub postitoimipaikka_nimi: JhsPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTeksti {
    #[serde(flatten)]
    pub kuvaus_teksti: JhsKuvausTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumero {
    #[serde(flatten)]
    pub kunta_numero: JhsKuntaNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTeksti {
    #[serde(flatten)]
    pub statusryhma_teksti: JhsStatusryhmaTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimi {
    #[serde(flatten)]
    pub suku_nimi: JhsSukuNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnus {
    #[serde(flatten)]
    pub yritys_tunnus: JhsYritysTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTeksti {
    #[serde(flatten)]
    pub ensimmainen_rivi_teksti: JhsEnsimmainenRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumero {
    #[serde(flatten)]
    pub osoite_numero: JhsOsoiteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTeksti {
    #[serde(flatten)]
    pub kolmas_rivi_teksti: JhsKolmasRiviTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnus {
    #[serde(flatten)]
    pub valiaikainen_henkilo_tunnus: JhsValiaikainenHenkiloTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkin {
    #[serde(flatten)]
    pub vakinainen_kytkin: JhsVakinainenKytkinTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodi {
    #[serde(flatten)]
    pub laji_koodi: JhsLajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodi {
    #[serde(flatten)]
    pub nimilaji_koodi: JhsNimilajiKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumero {
    #[serde(flatten)]
    pub huoneistotunniste_numero: JhsHuoneistotunnisteNumeroTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnus {
    #[serde(flatten)]
    pub sahkoinen_asiointi_tunnus: JhsSahkoinenAsiointiTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimi {
    #[serde(flatten)]
    pub ulkomaa_postitoimipaikka_nimi: JhsUlkomaaPostitoimipaikkaNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodi {
    #[serde(flatten)]
    pub sukupuoli_koodi: JhsSukupuoliKoodiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimi {
    #[serde(flatten)]
    pub alayksikko_nimi: JhsAlayksikkoNimiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTeksti {
    #[serde(flatten)]
    pub sahkopostiosoite_teksti: JhsSahkopostiosoiteTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTeksti {
    #[serde(flatten)]
    pub postilokero_teksti: JhsPostilokeroTekstiTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnus {
    #[serde(flatten)]
    pub pankkitili_tunnus: JhsPankkitiliTunnusTyyppi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnus {
    #[serde(flatten)]
    pub i_b_a_n_tunnus: JhsIBANTunnusTyyppi,
}

