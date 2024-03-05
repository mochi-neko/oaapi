use std::fmt::{Display, Formatter};

/// ISO 639-1 Language Codes.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Iso639_1 {
    // A
    /// Afar
    Aa,
    /// Abkhazian
    Ab,
    /// Avestan
    Ae,
    /// Afrikaans
    Af,
    /// Akan
    Ak,
    /// Amharic
    Am,
    /// Aragonese
    An,
    /// Arabic
    Ar,
    /// Assamese
    As,
    /// Avaric
    Av,
    /// Aymara
    Ay,
    /// Azerbaijani
    Az,

    // B
    /// Bashkir
    Ba,
    /// Belarusian
    Be,
    /// Bulgarian
    Bg,
    /// Bihari languages
    Bh,
    /// Bislama
    Bi,
    /// Bambara
    Bm,
    /// Bengali
    Bn,
    /// Tibetan
    Bo,
    /// Breton
    Br,
    /// Bosnian
    Bs,

    // C
    /// Catalan; Valencian
    Ca,
    /// Chechen
    Ce,
    /// Chamorro
    Ch,
    /// Corsican
    Co,
    /// Cree
    Cr,
    /// Czech
    Cs,
    /// Church Slavic; Old Slavonic; Church Slavonic; Old Bulgarian; Old Church Slavonic
    Cu,
    /// Chuvash
    Cv,
    /// Welsh
    Cy,
    // D
    /// Danish
    Da,
    /// German
    De,
    /// Divehi; Dhivehi; Maldivian
    Dv,
    /// Dzongkha
    Dz,

    // E
    /// Greek, Modern (1453-)
    El,
    /// English
    En,
    /// Esperanto
    Eo,
    /// Spanish; Castilian
    Es,
    /// Estonian
    Et,
    /// Basque
    Eu,
    /// Persian
    Fa,
    /// Fulah
    Ff,
    /// Finnish
    Fi,
    /// Fijian
    Fj,
    /// Faroese
    Fo,
    /// French
    Fr,
    /// Western Frisian
    Fy,

    // G
    /// Irish
    Ga,
    /// Gaelic; Scottish Gaelic
    Gd,
    /// Galician
    Gl,
    /// Guarani
    Gn,
    /// Gujarati
    Gu,

    // H
    /// Manx
    Hv,
    /// Hausa
    Ha,
    /// Hebrew
    He,
    /// Hindi
    Hi,
    /// Hiri Motu
    Ho,
    /// Croatian
    Hr,
    /// Haitian; Haitian Creole
    Ht,
    /// Hungarian
    Hu,

    // I
    /// Armenian
    Hy,
    /// Interlingua (International Auxiliary Language Association)
    Ia,
    /// Indonesian
    Id,
    /// Interlingue; Occidental
    Ie,
    /// Igbo
    Ig,
    /// Sichuan Yi; Nuosu
    Ii,
    /// Inupiaq
    Ik,
    /// Ido
    Io,
    /// Icelandic
    Is,
    /// Italian
    It,
    /// Inuktitut
    Iu,

    // J
    /// Japanese
    Ja,
    /// Javanese
    Jv,

    // K
    /// Georgian
    Ka,
    /// Kongo
    Kg,
    /// Kikuyu; Gikuyu
    Ki,
    /// Kuanyama; Kwanyama
    Kj,
    /// Kazakh
    Kk,
    /// Kalaallisut; Greenlandic
    Kl,
    /// Central Khmer
    Km,
    /// Kannada
    Kn,
    /// Korean
    Ko,
    /// Kanuri
    Kr,
    /// Kashmiri
    Ks,
    /// Kurdish
    Ku,
    /// Komi
    Kv,
    /// Cornish
    Kw,
    /// Kirghiz; Kyrgyz
    Ky,

    // L
    /// Latin
    La,
    /// Luxembourgish; Letzeburgesch
    Lb,
    /// Ganda
    Lg,
    /// Limburgan; Limburger; Limburgish
    Li,
    /// Lingala
    Ln,
    /// Lao
    Lo,
    /// Lithuanian
    Lt,
    /// Luba-Katanga
    Lu,
    /// Latvian
    Lv,
    // M
    /// Malagasy
    Mg,
    /// Marshallese
    Mh,
    /// Maori
    Mi,
    /// Macedonian
    Mk,
    /// Malayalam
    Ml,
    /// Mongolian
    Mn,
    /// Marathi
    Mr,
    /// Malay
    Ms,
    /// Maltese
    Mt,
    /// Burmese
    My,

    // N
    /// Nauru
    Na,
    /// Bokmål, Norwegian; Norwegian Bokmål
    Nb,
    /// Ndebele, North; North Ndebele
    Nd,
    /// Nepali
    Ne,
    /// Ndonga
    Ng,
    /// Dutch; Flemish
    Nl,
    /// Norwegian Nynorsk; Nynorsk, Norwegian
    Nn,
    /// Norwegian
    No,
    /// Ndebele, South; South Ndebele
    Nr,
    /// Navajo; Navaho
    Nv,
    /// Chichewa; Chewa; Nyanja
    Ny,

    // O
    /// Occitan (post 1500)
    Oc,
    /// Ojibwa
    Oj,
    /// Oromo
    Om,
    /// Oriya
    Or,
    /// Ossetian; Ossetic
    Os,

    // P
    /// Panjabi; Punjabi
    Pa,
    /// Pali
    Pi,
    /// Polish
    Pl,
    /// Pushto; Pashto
    Ps,
    /// Portuguese
    Pt,

    // Q
    /// Quechua
    Qu,

    // R
    /// Romansh
    Rm,
    /// Rundi
    Rn,
    /// Romanian; Moldavian; Moldovan
    Ro,
    /// Russian
    Ru,
    /// Kinyarwanda
    Rw,

    // S
    /// Sanskrit
    Sa,
    /// Sardinian
    Sc,
    /// Sindhi
    Sd,
    /// Northern Sami
    Se,
    /// Sango
    Sg,
    /// Sinhala; Sinhalese
    Si,
    /// Slovak
    Sk,
    /// Slovenian
    Sl,
    /// Samoan
    Sm,
    /// Shona
    Sn,
    /// Somali
    So,
    /// Albanian
    Sq,
    /// Serbian
    Sr,
    /// Swati
    Ss,
    /// Sotho, Southern
    St,
    /// Sundanese
    Su,
    /// Swedish
    Sv,
    /// Swahili
    Sw,

    // T
    /// Tamil
    Ta,
    /// Telugu
    Te,
    /// Tajik
    Tg,
    /// Thai
    Th,
    /// Tigrinya
    Ti,
    /// Turkmen
    Tk,
    /// Tagalog
    Tl,
    /// Tswana
    Tn,
    /// Tonga (Tonga Islands)
    To,
    /// Turkish
    Tr,
    /// Tsonga
    Ts,
    /// Tatar
    Tt,
    /// Twi
    Tw,

    // U
    /// Uighur; Uyghur
    Ug,
    /// Ukrainian
    Uk,
    /// Urdu
    Ur,
    /// Uzbek
    Uz,

    // V
    /// Venda
    Ve,
    /// Vietnamese
    Vi,
    /// Volapük
    Vo,

    // W
    /// Walloon
    Wa,
    /// Wolof
    Wo,

    // X
    /// Xhosa
    Xh,

    // Y
    /// Yiddish
    Yi,
    /// Yoruba
    Yo,

    // Z
    /// Zhuang; Chuang
    Za,
    /// Chinese
    Zh,
    /// Zulu
    Zu,
}

impl Default for Iso639_1 {
    fn default() -> Self {
        Self::En
    }
}

impl Display for Iso639_1 {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            // A
            | Iso639_1::Aa => write!(f, "aa"),
            | Iso639_1::Ab => write!(f, "ab"),
            | Iso639_1::Ae => write!(f, "ae"),
            | Iso639_1::Af => write!(f, "af"),
            | Iso639_1::Ak => write!(f, "ak"),
            | Iso639_1::Am => write!(f, "am"),
            | Iso639_1::An => write!(f, "an"),
            | Iso639_1::Ar => write!(f, "ar"),
            | Iso639_1::As => write!(f, "as"),
            | Iso639_1::Av => write!(f, "av"),
            | Iso639_1::Ay => write!(f, "ay"),
            | Iso639_1::Az => write!(f, "az"),

            // B
            | Iso639_1::Ba => write!(f, "ba"),
            | Iso639_1::Be => write!(f, "be"),
            | Iso639_1::Bg => write!(f, "bg"),
            | Iso639_1::Bh => write!(f, "bh"),
            | Iso639_1::Bi => write!(f, "bi"),
            | Iso639_1::Bm => write!(f, "bm"),
            | Iso639_1::Bn => write!(f, "bn"),
            | Iso639_1::Bo => write!(f, "bo"),
            | Iso639_1::Br => write!(f, "br"),
            | Iso639_1::Bs => write!(f, "bs"),

            // C
            | Iso639_1::Ca => write!(f, "ca"),
            | Iso639_1::Ce => write!(f, "ce"),
            | Iso639_1::Ch => write!(f, "ch"),
            | Iso639_1::Co => write!(f, "co"),
            | Iso639_1::Cr => write!(f, "cr"),
            | Iso639_1::Cs => write!(f, "cs"),
            | Iso639_1::Cu => write!(f, "cu"),
            | Iso639_1::Cv => write!(f, "cv"),
            | Iso639_1::Cy => write!(f, "cy"),

            // D
            | Iso639_1::Da => write!(f, "da"),
            | Iso639_1::De => write!(f, "de"),
            | Iso639_1::Dv => write!(f, "dv"),
            | Iso639_1::Dz => write!(f, "dz"),

            // E
            | Iso639_1::El => write!(f, "el"),
            | Iso639_1::En => write!(f, "en"),
            | Iso639_1::Eo => write!(f, "eo"),
            | Iso639_1::Es => write!(f, "es"),
            | Iso639_1::Et => write!(f, "et"),
            | Iso639_1::Eu => write!(f, "eu"),

            // F
            | Iso639_1::Fa => write!(f, "fa"),
            | Iso639_1::Ff => write!(f, "ff"),
            | Iso639_1::Fi => write!(f, "fi"),
            | Iso639_1::Fj => write!(f, "fj"),
            | Iso639_1::Fo => write!(f, "fo"),
            | Iso639_1::Fr => write!(f, "fr"),
            | Iso639_1::Fy => write!(f, "fy"),

            // G
            | Iso639_1::Ga => write!(f, "ga"),
            | Iso639_1::Gd => write!(f, "gd"),
            | Iso639_1::Gl => write!(f, "gl"),
            | Iso639_1::Gn => write!(f, "gn"),
            | Iso639_1::Gu => write!(f, "gu"),

            // H
            | Iso639_1::Hv => write!(f, "hv"),
            | Iso639_1::Ha => write!(f, "ha"),
            | Iso639_1::He => write!(f, "he"),
            | Iso639_1::Hi => write!(f, "hi"),
            | Iso639_1::Ho => write!(f, "ho"),
            | Iso639_1::Hr => write!(f, "hr"),
            | Iso639_1::Ht => write!(f, "ht"),
            | Iso639_1::Hu => write!(f, "hu"),

            // I
            | Iso639_1::Hy => write!(f, "hy"),
            | Iso639_1::Ia => write!(f, "ia"),
            | Iso639_1::Id => write!(f, "id"),
            | Iso639_1::Ie => write!(f, "ie"),
            | Iso639_1::Ig => write!(f, "ig"),
            | Iso639_1::Ii => write!(f, "ii"),
            | Iso639_1::Ik => write!(f, "ik"),
            | Iso639_1::Io => write!(f, "io"),
            | Iso639_1::Is => write!(f, "is"),
            | Iso639_1::It => write!(f, "it"),
            | Iso639_1::Iu => write!(f, "iu"),

            // J
            | Iso639_1::Ja => write!(f, "ja"),
            | Iso639_1::Jv => write!(f, "jv"),

            // K
            | Iso639_1::Ka => write!(f, "ka"),
            | Iso639_1::Kg => write!(f, "kg"),
            | Iso639_1::Ki => write!(f, "ki"),
            | Iso639_1::Kj => write!(f, "kj"),
            | Iso639_1::Kk => write!(f, "kk"),
            | Iso639_1::Kl => write!(f, "kl"),
            | Iso639_1::Km => write!(f, "km"),
            | Iso639_1::Kn => write!(f, "kn"),
            | Iso639_1::Ko => write!(f, "ko"),
            | Iso639_1::Kr => write!(f, "kr"),
            | Iso639_1::Ks => write!(f, "ks"),
            | Iso639_1::Ku => write!(f, "ku"),
            | Iso639_1::Kv => write!(f, "kv"),
            | Iso639_1::Kw => write!(f, "kw"),
            | Iso639_1::Ky => write!(f, "ky"),

            // L
            | Iso639_1::La => write!(f, "la"),
            | Iso639_1::Lb => write!(f, "lb"),
            | Iso639_1::Lg => write!(f, "lg"),
            | Iso639_1::Li => write!(f, "li"),
            | Iso639_1::Ln => write!(f, "ln"),
            | Iso639_1::Lo => write!(f, "lo"),
            | Iso639_1::Lt => write!(f, "lt"),
            | Iso639_1::Lu => write!(f, "lu"),
            | Iso639_1::Lv => write!(f, "lv"),

            // M
            | Iso639_1::Mg => write!(f, "mg"),
            | Iso639_1::Mh => write!(f, "mh"),
            | Iso639_1::Mi => write!(f, "mi"),
            | Iso639_1::Mk => write!(f, "mk"),
            | Iso639_1::Ml => write!(f, "ml"),
            | Iso639_1::Mn => write!(f, "mn"),
            | Iso639_1::Mr => write!(f, "mr"),
            | Iso639_1::Ms => write!(f, "ms"),
            | Iso639_1::Mt => write!(f, "mt"),
            | Iso639_1::My => write!(f, "my"),

            // N
            | Iso639_1::Na => write!(f, "na"),
            | Iso639_1::Nb => write!(f, "nb"),
            | Iso639_1::Nd => write!(f, "nd"),
            | Iso639_1::Ne => write!(f, "ne"),
            | Iso639_1::Ng => write!(f, "ng"),
            | Iso639_1::Nl => write!(f, "nl"),
            | Iso639_1::Nn => write!(f, "nn"),
            | Iso639_1::No => write!(f, "no"),
            | Iso639_1::Nr => write!(f, "nr"),
            | Iso639_1::Nv => write!(f, "nv"),
            | Iso639_1::Ny => write!(f, "ny"),

            // O
            | Iso639_1::Oc => write!(f, "oc"),
            | Iso639_1::Oj => write!(f, "oj"),
            | Iso639_1::Om => write!(f, "om"),
            | Iso639_1::Or => write!(f, "or"),
            | Iso639_1::Os => write!(f, "os"),

            // P
            | Iso639_1::Pa => write!(f, "pa"),
            | Iso639_1::Pi => write!(f, "pi"),
            | Iso639_1::Pl => write!(f, "pl"),
            | Iso639_1::Ps => write!(f, "ps"),
            | Iso639_1::Pt => write!(f, "pt"),

            // Q
            | Iso639_1::Qu => write!(f, "qu"),

            // R
            | Iso639_1::Rm => write!(f, "rm"),
            | Iso639_1::Rn => write!(f, "rn"),
            | Iso639_1::Ro => write!(f, "ro"),
            | Iso639_1::Ru => write!(f, "ru"),
            | Iso639_1::Rw => write!(f, "rw"),

            // S
            | Iso639_1::Sa => write!(f, "sa"),
            | Iso639_1::Sc => write!(f, "sc"),
            | Iso639_1::Sd => write!(f, "sd"),
            | Iso639_1::Se => write!(f, "se"),
            | Iso639_1::Sg => write!(f, "sg"),
            | Iso639_1::Si => write!(f, "si"),
            | Iso639_1::Sk => write!(f, "sk"),
            | Iso639_1::Sl => write!(f, "sl"),
            | Iso639_1::Sm => write!(f, "sm"),
            | Iso639_1::Sn => write!(f, "sn"),
            | Iso639_1::So => write!(f, "so"),
            | Iso639_1::Sq => write!(f, "sq"),
            | Iso639_1::Sr => write!(f, "sr"),
            | Iso639_1::Ss => write!(f, "ss"),
            | Iso639_1::St => write!(f, "st"),
            | Iso639_1::Su => write!(f, "su"),
            | Iso639_1::Sv => write!(f, "sv"),
            | Iso639_1::Sw => write!(f, "sw"),

            // T
            | Iso639_1::Ta => write!(f, "ta"),
            | Iso639_1::Te => write!(f, "te"),
            | Iso639_1::Tg => write!(f, "tg"),
            | Iso639_1::Th => write!(f, "th"),
            | Iso639_1::Ti => write!(f, "ti"),
            | Iso639_1::Tk => write!(f, "tk"),
            | Iso639_1::Tl => write!(f, "tl"),
            | Iso639_1::Tn => write!(f, "tn"),
            | Iso639_1::To => write!(f, "to"),
            | Iso639_1::Tr => write!(f, "tr"),
            | Iso639_1::Ts => write!(f, "ts"),
            | Iso639_1::Tt => write!(f, "tt"),
            | Iso639_1::Tw => write!(f, "tw"),

            // U
            | Iso639_1::Ug => write!(f, "ug"),
            | Iso639_1::Uk => write!(f, "uk"),
            | Iso639_1::Ur => write!(f, "ur"),
            | Iso639_1::Uz => write!(f, "uz"),

            // V
            | Iso639_1::Ve => write!(f, "ve"),
            | Iso639_1::Vi => write!(f, "vi"),
            | Iso639_1::Vo => write!(f, "vo"),

            // W
            | Iso639_1::Wa => write!(f, "wa"),
            | Iso639_1::Wo => write!(f, "wo"),

            // X
            | Iso639_1::Xh => write!(f, "xh"),

            // Y
            | Iso639_1::Yi => write!(f, "yi"),
            | Iso639_1::Yo => write!(f, "yo"),

            // Z
            | Iso639_1::Za => write!(f, "za"),
            | Iso639_1::Zh => write!(f, "zh"),
            | Iso639_1::Zu => write!(f, "zu"),
        }
    }
}
