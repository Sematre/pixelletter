use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pixelletter")]
pub struct Pixelletter {
    #[serde(rename = "@version")]
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costumer_id: Option<String>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costumer_data: Option<CostumerData>,

    #[serde(rename = "credit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costumer_credit: Option<CostumerCredit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auth {
    pub email: String,
    pub password: String,

    #[serde(
        serialize_with = "serialize_bool_to_str_yes_no_german",
        deserialize_with = "deserialize_bool_from_str_yes_no_german"
    )]
    pub agb: bool,

    #[serde(
        serialize_with = "serialize_bool_to_str_yes_no_german",
        deserialize_with = "deserialize_bool_from_str_yes_no_german"
    )]
    pub widerrufsverzicht: bool,

    pub testmodus: bool,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_ref: Option<Ref>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ref {
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "@type")]
    pub content_type: String,

    pub options: Options,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub action: ActionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,

    pub control: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    #[serde(
        serialize_with = "serialize_addoption_list_to_str",
        deserialize_with = "deserialize_addoption_list_from_str"
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addoption: Vec<Addoption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    pub returnaddress: String,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ActionType {
    Letter = 1,
    Fax = 2,
    LetterAndFax = 3,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::Letter
    }
}

impl TryFrom<u8> for ActionType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Letter),
            2 => Ok(Self::Fax),
            3 => Ok(Self::LetterAndFax),
            _ => Err("Invalid value!"),
        }
    }
}

impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}

impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Location {
    Munich = 1,
    Hausleiten = 2,
    Hamburg = 3,
}

impl Default for Location {
    fn default() -> Self {
        Self::Munich
    }
}

impl TryFrom<u8> for Location {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Munich),
            2 => Ok(Self::Hausleiten),
            3 => Ok(Self::Hamburg),
            _ => Err("Invalid value!"),
        }
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Addoption {
    Einschreiben = 27,
    Rückschein = 28,
    Eigenhändig = 29,
    EinschreibenEinwurf = 30,
    Color = 33,
    Green = 44,
}

impl TryFrom<u8> for Addoption {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            27 => Ok(Self::Einschreiben),
            28 => Ok(Self::Rückschein),
            29 => Ok(Self::Eigenhändig),
            30 => Ok(Self::EinschreibenEinwurf),
            33 => Ok(Self::Color),
            44 => Ok(Self::Green),
            _ => Err("Invalid value!"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub address: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    // This is workaround for the missing prefix inplementation in quick-xml
    #[serde(rename(serialize = "account:info", deserialize = "info"))]
    pub account_info: AccountInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "@type")]
    pub account_info_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub result: ResponseResult,
    pub transaction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseResult {
    #[serde(rename = "@code")]
    pub code: i32,

    pub msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostumerData {
    pub company: Company,
    pub sex: String,
    pub title: Title,
    pub firstname: String,
    pub lastname: String,
    pub street: String,
    pub pcode: String,
    pub city: String,
    pub country: String,

    #[serde(rename = "prefix")]
    pub tel_prefix: String,

    pub tel: String,

    #[serde(rename = "prefix")]
    pub fax_prefix: String,

    pub fax: String,

    #[serde(rename = "prefix")]
    pub mobil_prefix: String,

    pub mobil: String,
    pub email: String,

    #[serde(rename = "type")]
    pub payment_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Title {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostumerCredit {
    #[serde(rename = "@currency")]
    pub currency: String,
}

fn deserialize_bool_from_str_yes_no_german<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    match value.as_str() {
        "ja" => Ok(true),
        "nein" => Ok(false),

        _ => Err(serde::de::Error::custom(format!(
            "Unkown value, expected 'ja' or 'nein', but got: {value}"
        ))),
    }
}

fn serialize_bool_to_str_yes_no_german<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(match value {
        true => "ja",
        false => "nein",
    })
}

fn deserialize_addoption_list_from_str<'de, D>(deserializer: D) -> Result<Vec<Addoption>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?
        .split(',')
        .filter_map(|value: &str| value.parse().ok())
        .filter_map(|value: u8| Addoption::try_from(value).ok())
        .collect())
}

fn serialize_addoption_list_to_str<S>(
    values: &[Addoption],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(
        values
            .iter()
            .cloned()
            .map(|value| format!("{}", value as u8))
            .collect::<Vec<String>>()
            .join(",")
            .as_str(),
    )
}

#[derive(Error, Debug)]
pub enum PixelletterErrorCode {
    #[error("Die Datei konnte nicht erzeugt werden. Bitte versuchen Sie es noch einmal.")]
    Code1,

    #[error("Unbekannter Fehler. Bitte versuchen Sie es noch einmal.")]
    Code2,

    #[error("Unbekannter Fehler. Bitte versuchen Sie es noch einmal.")]
    Code3,

    #[error("Die angegebene e-mail-Adresse oder das Passwort sind nicht korrekt.")]
    Code4,

    #[error("Unberechtigter Seiten-Aufruf. Bitte beginnen Sie von vorne.")]
    Code5,

    #[error("Dieser Auftrag wurde bereits erteilt.")]
    Code6,

    #[error("Ihr Account ist gesperrt. Bitte wenden Sie sich an uns.")]
    Code7,

    #[error("Es wurden keine korrekten XML-Daten übermittelt.")]
    Code8,

    #[error("Es wurde kein Wert im Feld type angeben. Bitte wählen Sie zwischen den Values text oder upload.")]
    Code9,

    #[error(
        "Der Datei-Typ ist nicht korrekt. Uploads sind nur mit korrekter Datei-Endung möglich."
    )]
    Code10,

    #[error("Die Konvertierung des Dokuments ist fehlgeschlagen, bitte versuchen Sie es mit einer anderen Datei-Endung.")]
    Code11,

    #[error("Die Datei konnte nicht übertragen werden.")]
    Code12,

    #[error("Bitte bestätigen Sie die Allgemeinen Geschäftsbedingungen mit \"ja\".")]
    Code13,

    #[error("Bitte geben Sie an, ob Sie auf das Widerrufsrecht verzichten möchten.")]
    Code14,

    #[error("Es wurde keine Adresse für den Empfänger angegeben oder es wurde kein Dateianhang mitgesendet.")]
    Code15,

    #[error("Es wurde kein Text für den Briefinhalt angegeben.")]
    Code16,

    #[error("Sind Sie sicher, dass Sie vom Widerrufsrecht gebrauch machen möchten? Dies führt zu einer Verzögerung von 2 Wochen beim Versand Ihres Auftrags. Bitte treffen Sie eine Auswahl und klicken Sie erneut auf den Bestellknopf.")]
    Code17,

    #[error("Es wurde eine Faxzustellung gewünscht, aber keine gültige Faxnummer angegeben.")]
    Code18,

    #[error("Es wurde keine action definiert. Bitte geben Sie 1 (für Briefversand), 2 (für Fax-Versand) oder 3 (für Brief- und Faxversand) an.")]
    Code19,

    #[error("Die Datei darf max. 50 MB groß sein.")]
    Code20,

    #[error("Ihr Guthaben reicht nicht aus. Bitte loggen Sie sich im Kundenbereich ein und laden Sie Ihr Guthaben auf.")]
    Code21,

    #[error("Falsche Angabe von Zusatzleistungen.")]
    Code22,

    #[error("Die gewählte Zusatzleistung ist bis zum 14.03.2009 nicht verfügbar.")]
    Code23,

    #[error("Der Versandort München ist bis zum 04.09.2006 nicht verfügbar.")]
    Code24,

    #[error("Kein oder ein falsches Empfängerland angegeben. Diese Angabe ist obligatorisch. Im e-mail-Template geben Sie bitte die Zeile # destination: DE bzw. das entsprechende Länderkürzel ein. Wenn Sie die HTTPS-Schnittstelle nutzen finden Sie Infos in der aktuellen Doku.")]
    Code25,

    #[error("Die ausgewählte Zusatzleistung kann in das gewählte Empfängerland nicht versendet werden. Bitte wählen Sie, sofern zutreffend, Deutschland als Zielland aus.")]
    Code26,

    #[error("Die ausgewählte Zusatzleistung kann über das gewählte Briefzentrum nicht versendet werden. Bitte wählen Sie München als Versandort aus.")]
    Code27,

    #[error("Die ausgewählte Zusatzleistung kann über das gewählte Briefzentrum nicht versendet werden. Bitte wählen Sie Hausleiten/Wien als Versandort aus.")]
    Code28,

    #[error("Die ausgewählte Zusatzleistung kann in das gewählte Empfängerland nicht versendet werden. Bitte wählen Sie ein Zielland innerhalb Europas aus.")]
    Code29,

    #[error("Der angegebene Name des Begünstigten ist fehlerhaft. Geben Sie mind. 1 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen.")]
    Code30,

    #[error("Der angegebene Name der Bank des Begünstigten ist fehlerhaft. Geben Sie mind. 1 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen.")]
    Code31,

    #[error("Die erste Zeile des angegebenen Verwendungszwecks ist fehlerhaft. Geben Sie mind. 0 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen.")]
    Code32,

    #[error("Die zweite Zeile des angegebenen Verwendungszwecks ist fehlerhaft. Geben Sie mind. 1 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen.")]
    Code33,

    #[error("Der angegebene Betrag der Nachnahme ist fehlerhaft. Geben Sie den Betrag ohne tausender Trennzeichen im Format XXXX,XX an.")]
    Code34,

    #[error("Der angegebene Betrag der Nachnahme ist zu hoch oder zu niedrig. Geben Sie einen Euro-Betrag von min. 3,00 EUR und max. 1600,00 EUR an.")]
    Code35,

    #[error("Die angegebene Kontonummer des Begünstigten ist fehlerhaft. Geben Sie zwischen 6 und 10 Ziffern an.")]
    Code36,

    #[error("Die angegebene Bankleitzahl des Begünstigten ist fehlerhaft. Geben Sie genau 8 Ziffern an.")]
    Code37,

    #[error("Farbdrucke können derzeit nicht über das ausgewählt Briefzentrum verschickt werden. Bitte wählen Sie ein anderes Briefzentrum.")]
    Code38,

    #[error("Es wurde keine e-mailadresse für den Absender der Signaturbenachrichtung an den Kunden definiert.")]
    Code39,

    #[error("Die Länge der e-mailadresse für den Absender der Signaturbenachrichtung an den Kunden ist zu groß. Maximal können 255 Zeichen angeben werden.")]
    Code40,

    #[error("Es wurde keine e-mailadresse für den Empfänger der Signaturbenachrichtung an den Kunden definiert.")]
    Code41,

    #[error("Die Länge der e-mailadresse für den Empfänger der Signaturbenachrichtung an den Kunden ist zu groß. Maximal können 255 Zeichen angeben werden.")]
    Code42,

    #[error("Es wurde kein e-mail-Betreff für die Signaturbenachrichtung definiert.")]
    Code43,

    #[error("Der Betreff der e-mail für die Signaturbenachrichtung ist zu lang.  Maximal können 255 Zeichen angeben werden.")]
    Code44,

    #[error("Es wurde kein e-mail-Text für die Signaturbenachrichtung definiert.")]
    Code45,

    #[error("Um elektronische Signaturen zu beauftragen, müssen Sie sich einmalig kostenfrei im Kundenbereich unter dem Menüpunkt \"Elektr. Signaturen\" freischalten.")]
    Code46,

    #[error("Die PDF-Datei ist verschlüsselt. Bitte laden Sie eine unverschlüsselte PDF-Datei hoch die keine Bearbeitungseinschränkungen beinhaltet.")]
    Code47,

    #[error(
        "Die verwendete Transaction-ID wurde bereits verwendet. (Spezielle Kundeneinstellung)"
    )]
    Code48,

    #[error("Für den Auftrag zur digitalen Signatur wurde keine Datei übermittelt.")]
    Code49,

    #[error("In der PDF-Datei wurde kein Zeiger auf die xref-Tabelle gefunden.")]
    Code50,

    #[error("Es wurde kein Upload-Template mit dieser Nummer gefunden.")]
    Code51,

    #[error("Beim Wert Template wurde keine Template-Nummer definiert.")]
    Code52,

    #[error("Derzeit sind aus technischen Gründen nur Uploads von PDF-Dateien möglich.")]
    Code53,

    #[error("Es ist ein Fehler bei der PGP-Entschlüsselung aufgetreten.")]
    Code54,

    #[error("Der Auftrag kann nicht gefunden werden.")]
    Code55,

    #[error("Die Nummerierung der Bulkaufträge ist nicht fortlaufend.")]
    Code56,

    #[error("Bulkaufträge können nur mit dem type-Wert template übermittelt werden.")]
    Code57,

    #[error("Es wurde eine für Bulkaufträge ungültige action angegben. Die action muss 5 sein.")]
    Code58,

    #[error("Die Angabe von Werten für den Tag control ist bei Bulkaufträge nicht möglich.")]
    Code59,

    #[error("Die Angabe von Werten für den Tag addoption ist bei Bulkaufträge nicht möglich.")]
    Code60,

    #[error("Die Angabe location muss bei Bulkaufträge immer 1 sein.")]
    Code61,

    #[error("Es wurde ein ungültiger Ländercode als Zielland (destination) angeben.")]
    Code62,

    #[error("Es wurde eine ungültige Nr für die Absenderzeile (sendernr) angeben.")]
    Code63,

    #[error("Es wurde eine ungültige Angabe für das Geschlecht (gender) angeben.")]
    Code64,

    #[error("Es wurde kein Vornamen angeben.")]
    Code65,

    #[error("Es wurde kein Nachname angeben.")]
    Code66,

    #[error("Es wurde keine Strasse angeben.")]
    Code67,

    #[error("Es wurde keine Postleitzahl angeben.")]
    Code68,

    #[error("Es wurde keine gültige Postleitzahl angeben.")]
    Code69,

    #[error("Es wurde kein Ort angeben.")]
    Code70,

    #[error("Der Auftrag konnte nicht in die Datenbank geschrieben werden.")]
    Code71,

    #[error("Das Foto muss im JPG-Format übermittelt werden.")]
    Code72,

    #[error("Die Anschrift bei Postkarten darf max. 6 Zeilen haben.")]
    Code73,

    #[error("Der Text für diese Postkarte ist zu lang.")]
    Code74,

    #[error("Das Foto darf nicht größer als 6 MB sein.")]
    Code75,

    #[error("Es wurde kein Foto übermittelt.")]
    Code76,

    #[error("Der Gutscheincode ist ungültig.")]
    Code77,

    #[error("Die angegebene URL ist ungültig.")]
    Code78,

    #[error("Die Unterstützung von Cookies ist deaktiviert. Bitte aktivieren Sie diese über die Einstellungen in Ihrem Browser.")]
    Code79,

    #[error("Der Gutscheincode ist nicht für diese Dienstleistung einlösbar.")]
    Code80,

    #[error("Die Session ist abgelaufen, bitte loggen Sie sich erneut ein.")]
    Code81,

    #[error("Die gewählte Dienstleistung kann nicht mehr angeboten werden.")]
    Code82,

    #[error("Bitte geben Sie mindestens einen Betrag von 5,00 EUR ein.")]
    Code83,

    #[error("Bitte geben Sie mindestens einen Betrag von 10,00 EUR ein.")]
    Code84,

    #[error("Bitte geben Sie mindestens einen Betrag von 25,00 EUR ein.")]
    Code85,

    #[error("Bitte geben Sie mindestens einen Betrag von 1,00 EUR ein.")]
    Code86,

    #[error("Bitte geben Sie einen gültigen Betrag (z.B. 10,00) ein.")]
    Code87,

    #[error("Um Premiumadress nutzen zu können, müssen Sie einen Premiumadress-Zugang bei der Post haben und unser Support muss diesen für Sie in den Kundeneinstellungen hinterlegt haben.")]
    Code88,

    #[error("Allgemeiner Fehler bei der PDF-Verarbeitung.")]
    Code89,

    #[error("Es können nur maximal 1000,00 EUR aufgeladen werden. Bitte nutzen Sie alternativ z.B. eine Banküberweisung.")]
    Code90,

    #[error("Ihre e-mailadresse war nicht erreichbar und wurde deshalb deaktiviert. Für weitere Instruktionen loggen Sie sich bitte im Kundenbereich ein.")]
    Code91,

    #[error("Das von Ihnen gesetzte Transaktions-Limit wurde erreicht. Bitte kontaktieren Sie ggf. den Support.")]
    Code92,

    #[error("Die PDF-Datei ist defekt.")]
    Code93,

    #[error("Bitte geben Sie mindestens einen Betrag von 0,01 EUR ein.")]
    Code94,

    #[error("Sie haben Ihre e-mailadresse noch nicht bestätigt. Bitte klicken Sie auf den Link unserer e-mail. Anschliessend können Sie sich an dieser Stelle anmelden.")]
    Code95,

    #[error("Bitte wählen Sie Ihr Geschlecht (Herr/Frau) aus.")]
    Code201,

    #[error("Es wurde kein Vornamen angeben.")]
    Code202,

    #[error("Es wurde kein Nachname angeben.")]
    Code203,

    #[error("Es wurde keine Strasse angeben.")]
    Code204,

    #[error(
        "Es wurde keine gültige Strasse angeben. Bitte verwenden Sie keine Postfach-Anschriften."
    )]
    Code205,

    #[error("Es wurde keine Postleitzahl (PLZ) angeben.")]
    Code206,

    #[error("Die PLZ ist nicht korrekt. Für eine Anschrift in Deutschland müssen Sie eine 5-stellige Postleitzahl angeben.")]
    Code207,

    #[error("Die PLZ ist nicht korrekt. Für eine Anschrift in Österreich oder der Schweiz müssen Sie eine 4-stellige Postleitzahl angeben.")]
    Code208,

    #[error("Es wurde kein Ort angeben.")]
    Code209,

    #[error("Es wurde keine gültige Vorwahl für die Telefonnummer angeben.")]
    Code210,

    #[error("Es wurde keine gültige Durchwahl für die Telefonnummer angeben.")]
    Code211,

    #[error("Es wurde keine gültige Vorwahl für die Faxnummer angeben. Falls Sie kein Fax besitzen lassen Sie das Feld komplett leer.")]
    Code212,

    #[error("Es wurde keine gültige Durchwahl für die Faxnummer angeben. Falls Sie kein Fax besitzen lassen Sie das Feld komplett leer.")]
    Code213,

    #[error("Die angegebene Faxnummer ist nicht vollständig. Falls Sie kein Fax besitzen lassen Sie bitte das Vorwahl- und Durchwahl-Feld komplett leer.")]
    Code214,

    #[error("Es wurde keine gültige Vorwahl für die Handynummer angeben. Falls Sie kein Handy besitzen lassen Sie das Feld komplett leer.")]
    Code215,

    #[error("Es wurde keine gültige Durchwahl für die Handynummer angeben. Falls Sie kein Handy besitzen lassen Sie das Feld komplett leer.")]
    Code216,

    #[error("Die angegebene Handynummer ist nicht vollständig. Bitte geben Sie eine Vorwahl ein.")]
    Code217,

    #[error("Es wurde keine e-mail-Adresse angeben.")]
    Code218,

    #[error("Es wurde keine gültige e-mail-Adresse angeben.")]
    Code219,

    #[error("Es wurde keine gültige e-mail-Adresse für den Rechnungsempfänger angeben.")]
    Code220,

    #[error("Es wurde keine gültige e-mail-Adresse für die Statusbenachrichtungen angeben.")]
    Code221,

    #[error("Es existiert bereits ein Kunde mit dieser e-mailadresse")]
    Code222,

    #[error("Es ist ein Fehler aufgetreten. Bitte beginnen Sie erneut von der Startseite")]
    Code223,

    #[error("Es wurden keine zu bestellenden Dokumente gefunden. Bitte beginnen Sie erneut von der Startseite")]
    Code224,

    #[error("Es wurde keine Änderung durchgeführt, da alle geänderten Daten mit denen identisch sind, die wir gespeichert haben.")]
    Code225,

    #[error(
        "Es wurde keine Änderung durchgeführt, da dieser Account lediglich ein Demo-Account ist."
    )]
    Code226,

    #[error("Die angegebene Bankverbindung (Kontonummer und BLZ) ist nicht korrekt.")]
    Code227,

    #[error("Die angegebene Bankverbindung (Kontonummer und BLZ) ist nicht korrekt. Sie haben wahrscheinlich die Felder vertauscht. Ändern Sie das und versuchen Sie es erneut.")]
    Code228,

    #[error(
        "Es wurde keine Zahlungsart ausgewählt. Bitte klicken Sie Bankeinzug oder Guthaben an."
    )]
    Code229,

    #[error("Es wurde keine Kontonummer angegeben.")]
    Code230,

    #[error("Es wurde keine Bankleitzahl (BLZ) angegeben.")]
    Code231,

    #[error("Es wurde kein Konto-Inhaber für die Bankverbindung angegeben.")]
    Code232,

    #[error("Sie müssen die AGB akzeptieren, um sich als Neukunde anmelden zu können.")]
    Code233,

    #[error("Es wurde kein Land angeben.")]
    Code234,

    #[error("Es wurde kein gültiger Ländercode angeben.")]
    Code235,

    #[error("Es wurde kein gültiges Geburtsdatum angeben.")]
    Code236,

    #[error("Bitte geben Sie als Titel nicht Herr/Frau oder ähnliches an, sondern nur Titel wie Prof. Dr.")]
    Code237,

    #[error("Ihr Account weist zu viele fehlgeschlagene Faxe auf und wurde für den Faxversand gesperrt. Bitte wenden Sie sich ggf. an unseren Support.")]
    Code238,

    #[error("Es ist ein unbekannter Fehler aufgetreten.")]
    Code239,

    #[error("Unknown error code {code}: {message}")]
    Other { code: u32, message: String },
}

pub fn error_code_to_msg(code: u8) -> Option<&'static str> {
    match code {
        1	=> Some("Die Datei konnte nicht erzeugt werden. Bitte versuchen Sie es noch einmal."),
        2	=> Some("Unbekannter Fehler. Bitte versuchen Sie es noch einmal."),
        3	=> Some("Unbekannter Fehler. Bitte versuchen Sie es noch einmal."),
        4	=> Some("Die angegebene e-mail-Adresse oder das Passwort sind nicht korrekt."),
        5	=> Some("Unberechtigter Seiten-Aufruf. Bitte beginnen Sie von vorne."),
        6	=> Some("Dieser Auftrag wurde bereits erteilt."),
        7	=> Some("Ihr Account ist gesperrt. Bitte wenden Sie sich an uns."),
        8	=> Some("Es wurden keine korrekten XML-Daten übermittelt."),
        9	=> Some("Es wurde kein Wert im Feld type angeben. Bitte wählen Sie zwischen den Values text oder upload."),
        10	=> Some("Der Datei-Typ ist nicht korrekt. Uploads sind nur mit korrekter Datei-Endung möglich."),
        11	=> Some("Die Konvertierung des Dokuments ist fehlgeschlagen, bitte versuchen Sie es mit einer anderen Datei-Endung."),
        12	=> Some("Die Datei konnte nicht übertragen werden."),
        13	=> Some("Bitte bestätigen Sie die Allgemeinen Geschäftsbedingungen mit \"ja\"."),
        14	=> Some("Bitte geben Sie an, ob Sie auf das Widerrufsrecht verzichten möchten."),
        15	=> Some("Es wurde keine Adresse für den Empfänger angegeben oder es wurde kein Dateianhang mitgesendet."),
        16	=> Some("Es wurde kein Text für den Briefinhalt angegeben."),
        17	=> Some("Sind Sie sicher, dass Sie vom Widerrufsrecht gebrauch machen möchten? Dies führt zu einer Verzögerung von 2 Wochen beim Versand Ihres Auftrags. Bitte treffen Sie eine Auswahl und klicken Sie erneut auf den Bestellknopf."),
        18	=> Some("Es wurde eine Faxzustellung gewünscht, aber keine gültige Faxnummer angegeben."),
        19	=> Some("Es wurde keine action definiert. Bitte geben Sie 1 (für Briefversand), 2 (für Fax-Versand) oder 3 (für Brief- und Faxversand) an."),
        20	=> Some("Die Datei darf max. 50 MB groß sein."),
        21	=> Some("Ihr Guthaben reicht nicht aus. Bitte loggen Sie sich im Kundenbereich ein und laden Sie Ihr Guthaben auf."),
        22	=> Some("Falsche Angabe von Zusatzleistungen."),
        23	=> Some("Die gewählte Zusatzleistung ist bis zum 14.03.2009 nicht verfügbar."),
        24	=> Some("Der Versandort München ist bis zum 04.09.2006 nicht verfügbar."),
        25	=> Some("Kein oder ein falsches Empfängerland angegeben. Diese Angabe ist obligatorisch. Im e-mail-Template geben Sie bitte die Zeile # destination: DE bzw. das entsprechende Länderkürzel ein. Wenn Sie die HTTPS-Schnittstelle nutzen finden Sie Infos in der aktuellen Doku."),
        26	=> Some("Die ausgewählte Zusatzleistung kann in das gewählte Empfängerland nicht versendet werden. Bitte wählen Sie, sofern zutreffend, Deutschland als Zielland aus."),
        27	=> Some("Die ausgewählte Zusatzleistung kann über das gewählte Briefzentrum nicht versendet werden. Bitte wählen Sie München als Versandort aus."),
        28	=> Some("Die ausgewählte Zusatzleistung kann über das gewählte Briefzentrum nicht versendet werden. Bitte wählen Sie Hausleiten/Wien als Versandort aus."),
        29	=> Some("Die ausgewählte Zusatzleistung kann in das gewählte Empfängerland nicht versendet werden. Bitte wählen Sie ein Zielland innerhalb Europas aus."),
        30	=> Some("Der angegebene Name des Begünstigten ist fehlerhaft. Geben Sie mind. 1 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen."),
        31	=> Some("Der angegebene Name der Bank des Begünstigten ist fehlerhaft. Geben Sie mind. 1 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen."),
        32	=> Some("Die erste Zeile des angegebenen Verwendungszwecks ist fehlerhaft. Geben Sie mind. 0 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen."),
        33	=> Some("Die zweite Zeile des angegebenen Verwendungszwecks ist fehlerhaft. Geben Sie mind. 1 und max. 27 Zeichen an und verwenden Sie nur diese Zeichen 0-9 A-Z äöüß.,&-/+*$% und Leerzeichen."),
        34	=> Some("Der angegebene Betrag der Nachnahme ist fehlerhaft. Geben Sie den Betrag ohne tausender Trennzeichen im Format XXXX,XX an."),
        35	=> Some("Der angegebene Betrag der Nachnahme ist zu hoch oder zu niedrig. Geben Sie einen Euro-Betrag von min. 3,00 EUR und max. 1600,00 EUR an."),
        36	=> Some("Die angegebene Kontonummer des Begünstigten ist fehlerhaft. Geben Sie zwischen 6 und 10 Ziffern an."),
        37	=> Some("Die angegebene Bankleitzahl des Begünstigten ist fehlerhaft. Geben Sie genau 8 Ziffern an."),
        38	=> Some("Farbdrucke können derzeit nicht über das ausgewählt Briefzentrum verschickt werden. Bitte wählen Sie ein anderes Briefzentrum."),
        39	=> Some("Es wurde keine e-mailadresse für den Absender der Signaturbenachrichtung an den Kunden definiert."),
        40	=> Some("Die Länge der e-mailadresse für den Absender der Signaturbenachrichtung an den Kunden ist zu groß. Maximal können 255 Zeichen angeben werden."),
        41	=> Some("Es wurde keine e-mailadresse für den Empfänger der Signaturbenachrichtung an den Kunden definiert."),
        42	=> Some("Die Länge der e-mailadresse für den Empfänger der Signaturbenachrichtung an den Kunden ist zu groß. Maximal können 255 Zeichen angeben werden."),
        43	=> Some("Es wurde kein e-mail-Betreff für die Signaturbenachrichtung definiert."),
        44	=> Some("Der Betreff der e-mail für die Signaturbenachrichtung ist zu lang.  Maximal können 255 Zeichen angeben werden."),
        45	=> Some("Es wurde kein e-mail-Text für die Signaturbenachrichtung definiert."),
        46	=> Some("Um elektronische Signaturen zu beauftragen, müssen Sie sich einmalig kostenfrei im Kundenbereich unter dem Menüpunkt \"Elektr. Signaturen\" freischalten."),
        47	=> Some("Die PDF-Datei ist verschlüsselt. Bitte laden Sie eine unverschlüsselte PDF-Datei hoch die keine Bearbeitungseinschränkungen beinhaltet."),
        48	=> Some("Die verwendete Transaction-ID wurde bereits verwendet. (Spezielle Kundeneinstellung)"),
        49	=> Some("Für den Auftrag zur digitalen Signatur wurde keine Datei übermittelt."),
        50	=> Some("In der PDF-Datei wurde kein Zeiger auf die xref-Tabelle gefunden."),
        51	=> Some("Es wurde kein Upload-Template mit dieser Nummer gefunden."),
        52	=> Some("Beim Wert Template wurde keine Template-Nummer definiert."),
        53	=> Some("Derzeit sind aus technischen Gründen nur Uploads von PDF-Dateien möglich."),
        54	=> Some("Es ist ein Fehler bei der PGP-Entschlüsselung aufgetreten."),
        55	=> Some("Der Auftrag kann nicht gefunden werden."),
        56	=> Some("Die Nummerierung der Bulkaufträge ist nicht fortlaufend."),
        57	=> Some("Bulkaufträge können nur mit dem type-Wert template übermittelt werden."),
        58	=> Some("Es wurde eine für Bulkaufträge ungültige action angegben. Die action muss 5 sein."),
        59	=> Some("Die Angabe von Werten für den Tag control ist bei Bulkaufträge nicht möglich."),
        60	=> Some("Die Angabe von Werten für den Tag addoption ist bei Bulkaufträge nicht möglich."),
        61	=> Some("Die Angabe location muss bei Bulkaufträge immer 1 sein."),
        62	=> Some("Es wurde ein ungültiger Ländercode als Zielland (destination) angeben."),
        63	=> Some("Es wurde eine ungültige Nr für die Absenderzeile (sendernr) angeben."),
        64	=> Some("Es wurde eine ungültige Angabe für das Geschlecht (gender) angeben."),
        65	=> Some("Es wurde kein Vornamen angeben."),
        66	=> Some("Es wurde kein Nachname angeben."),
        67	=> Some("Es wurde keine Strasse angeben."),
        68	=> Some("Es wurde keine Postleitzahl angeben."),
        69	=> Some("Es wurde keine gültige Postleitzahl angeben."),
        70	=> Some("Es wurde kein Ort angeben."),
        71	=> Some("Der Auftrag konnte nicht in die Datenbank geschrieben werden."),
        72	=> Some("Das Foto muss im JPG-Format übermittelt werden."),
        73	=> Some("Die Anschrift bei Postkarten darf max. 6 Zeilen haben."),
        74	=> Some("Der Text für diese Postkarte ist zu lang."),
        75	=> Some("Das Foto darf nicht größer als 6 MB sein."),
        76	=> Some("Es wurde kein Foto übermittelt."),
        77	=> Some("Der Gutscheincode ist ungültig."),
        78	=> Some("Die angegebene URL ist ungültig."),
        79	=> Some("Die Unterstützung von Cookies ist deaktiviert. Bitte aktivieren Sie diese über die Einstellungen in Ihrem Browser."),
        80	=> Some("Der Gutscheincode ist nicht für diese Dienstleistung einlösbar."),
        81	=> Some("Die Session ist abgelaufen, bitte loggen Sie sich erneut ein."),
        82	=> Some("Die gewählte Dienstleistung kann nicht mehr angeboten werden."),
        83	=> Some("Bitte geben Sie mindestens einen Betrag von 5,00 EUR ein."),
        84	=> Some("Bitte geben Sie mindestens einen Betrag von 10,00 EUR ein."),
        85	=> Some("Bitte geben Sie mindestens einen Betrag von 25,00 EUR ein."),
        86	=> Some("Bitte geben Sie mindestens einen Betrag von 1,00 EUR ein."),
        87	=> Some("Bitte geben Sie einen gültigen Betrag (z.B. 10,00) ein."),
        88	=> Some("Um Premiumadress nutzen zu können, müssen Sie einen Premiumadress-Zugang bei der Post haben und unser Support muss diesen für Sie in den Kundeneinstellungen hinterlegt haben."),
        89	=> Some("Allgemeiner Fehler bei der PDF-Verarbeitung."),
        90	=> Some("Es können nur maximal 1000,00 EUR aufgeladen werden. Bitte nutzen Sie alternativ z.B. eine Banküberweisung."),
        91	=> Some("Ihre e-mailadresse war nicht erreichbar und wurde deshalb deaktiviert. Für weitere Instruktionen loggen Sie sich bitte im Kundenbereich ein."),
        92	=> Some("Das von Ihnen gesetzte Transaktions-Limit wurde erreicht. Bitte kontaktieren Sie ggf. den Support."),
        93	=> Some("Die PDF-Datei ist defekt."),
        94	=> Some("Bitte geben Sie mindestens einen Betrag von 0,01 EUR ein."),
        95	=> Some("Sie haben Ihre e-mailadresse noch nicht bestätigt. Bitte klicken Sie auf den Link unserer e-mail. Anschliessend können Sie sich an dieser Stelle anmelden."),
        201	=> Some("Bitte wählen Sie Ihr Geschlecht (Herr/Frau) aus."),
        202	=> Some("Es wurde kein Vornamen angeben."),
        203	=> Some("Es wurde kein Nachname angeben."),
        204	=> Some("Es wurde keine Strasse angeben."),
        205	=> Some("Es wurde keine gültige Strasse angeben. Bitte verwenden Sie keine Postfach-Anschriften."),
        206	=> Some("Es wurde keine Postleitzahl (PLZ) angeben."),
        207	=> Some("Die PLZ ist nicht korrekt. Für eine Anschrift in Deutschland müssen Sie eine 5-stellige Postleitzahl angeben."),
        208	=> Some("Die PLZ ist nicht korrekt. Für eine Anschrift in Österreich oder der Schweiz müssen Sie eine 4-stellige Postleitzahl angeben."),
        209	=> Some("Es wurde kein Ort angeben."),
        210	=> Some("Es wurde keine gültige Vorwahl für die Telefonnummer angeben."),
        211	=> Some("Es wurde keine gültige Durchwahl für die Telefonnummer angeben."),
        212	=> Some("Es wurde keine gültige Vorwahl für die Faxnummer angeben. Falls Sie kein Fax besitzen lassen Sie das Feld komplett leer."),
        213	=> Some("Es wurde keine gültige Durchwahl für die Faxnummer angeben. Falls Sie kein Fax besitzen lassen Sie das Feld komplett leer."),
        214	=> Some("Die angegebene Faxnummer ist nicht vollständig. Falls Sie kein Fax besitzen lassen Sie bitte das Vorwahl- und Durchwahl-Feld komplett leer."),
        215	=> Some("Es wurde keine gültige Vorwahl für die Handynummer angeben. Falls Sie kein Handy besitzen lassen Sie das Feld komplett leer."),
        216	=> Some("Es wurde keine gültige Durchwahl für die Handynummer angeben. Falls Sie kein Handy besitzen lassen Sie das Feld komplett leer."),
        217	=> Some("Die angegebene Handynummer ist nicht vollständig. Bitte geben Sie eine Vorwahl ein."),
        218	=> Some("Es wurde keine e-mail-Adresse angeben."),
        219	=> Some("Es wurde keine gültige e-mail-Adresse angeben."),
        220	=> Some("Es wurde keine gültige e-mail-Adresse für den Rechnungsempfänger angeben."),
        221	=> Some("Es wurde keine gültige e-mail-Adresse für die Statusbenachrichtungen angeben."),
        222	=> Some("Es existiert bereits ein Kunde mit dieser e-mailadresse"),
        223	=> Some("Es ist ein Fehler aufgetreten. Bitte beginnen Sie erneut von der Startseite"),
        224	=> Some("Es wurden keine zu bestellenden Dokumente gefunden. Bitte beginnen Sie erneut von der Startseite"),
        225	=> Some("Es wurde keine Änderung durchgeführt, da alle geänderten Daten mit denen identisch sind, die wir gespeichert haben."),
        226	=> Some("Es wurde keine Änderung durchgeführt, da dieser Account lediglich ein Demo-Account ist."),
        227	=> Some("Die angegebene Bankverbindung (Kontonummer und BLZ) ist nicht korrekt."),
        228	=> Some("Die angegebene Bankverbindung (Kontonummer und BLZ) ist nicht korrekt. Sie haben wahrscheinlich die Felder vertauscht. Ändern Sie das und versuchen Sie es erneut."),
        229	=> Some("Es wurde keine Zahlungsart ausgewählt. Bitte klicken Sie Bankeinzug oder Guthaben an."),
        230	=> Some("Es wurde keine Kontonummer angegeben."),
        231	=> Some("Es wurde keine Bankleitzahl (BLZ) angegeben."),
        232	=> Some("Es wurde kein Konto-Inhaber für die Bankverbindung angegeben."),
        233	=> Some("Sie müssen die AGB akzeptieren, um sich als Neukunde anmelden zu können."),
        234	=> Some("Es wurde kein Land angeben."),
        235	=> Some("Es wurde kein gültiger Ländercode angeben."),
        236	=> Some("Es wurde kein gültiges Geburtsdatum angeben."),
        237	=> Some("Bitte geben Sie als Titel nicht Herr/Frau oder ähnliches an, sondern nur Titel wie Prof. Dr."),
        238	=> Some("Ihr Account weist zu viele fehlgeschlagene Faxe auf und wurde für den Faxversand gesperrt. Bitte wenden Sie sich ggf. an unseren Support."),
        239	=> Some("Es ist ein unbekannter Fehler aufgetreten."),
        _ => None,
    }
}
