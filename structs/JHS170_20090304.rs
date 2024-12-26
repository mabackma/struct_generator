#[derive(Debug, Serialize, Deserialize)]
pub struct SahkoinenAsiointiTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostinumeroKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtunimetNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuudesRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VakinainenKytkinTyyppi {
    #[serde(flatten)]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteKirjainTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoltosuhdeTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuPvmTyyppi {
    #[serde(flatten)]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuhelinnumeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaPostitoimipaikkaNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBANTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukupuoliKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuPvmTyyppi {
    #[serde(flatten)]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtuNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SahkopostiosoiteTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PankkitiliTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiviilisaatyTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusryhmaTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurvakieltoKytkinTyyppi {
    #[serde(flatten)]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KansalaisuusKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValiaikainenHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaksinumeroTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareOfTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KatuNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KolmasRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimilajiKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaNumeroTyyppi {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsimmainenRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntymaPvmTyyppi {
    #[serde(flatten)]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsoiteNumeroTyyppi {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoppuHetkiTyyppi {
    #[serde(flatten)]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UlkomaaHenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuntaKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoimassaoloKytkinTyyppi {
    #[serde(flatten)]
    pub base: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaayksikkoNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostitoimipaikkaNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdellinenSukuNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeljasRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlkuHetkiTyyppi {
    #[serde(flatten)]
    pub base: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NimiTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuvausTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlayksikkoNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuolemaPvmTyyppi {
    #[serde(flatten)]
    pub base: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YritysTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IkaluokkaTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HenkiloTunnusTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KieliKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValtiotunnusKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BICKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SukuNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmmattiKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LajiKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViidesRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToinenRiviTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteNumeroTyyppi {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaatunnusKoodiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KutsumaNimiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostilokerolyhenneTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HuoneistotunnisteJakokirjainTekstiTyyppi {
    #[serde(flatten)]
    pub base: String,
}

