use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use fancy_regex::Regex;

lazy_static! {
    static ref LANGUAGE_CODES: Vec<String> = vec![
        String::from("cs"),
        String::from("de"),
        String::from("en"),
        String::from("es"),
        String::from("et"),
        String::from("fi"),
        String::from("fr"),
        String::from("he"),
        String::from("id"),
        String::from("it"),
        String::from("ja"),
        String::from("nl"),
        String::from("no"),
        String::from("pl"),
        String::from("pt"),
        String::from("ro"),
        String::from("ru"),
        String::from("sv"),
        String::from("sq"),
        String::from("cat"),
        String::from("be"),
        String::from("bs"),
        String::from("bg"),
        String::from("hr"),
        String::from("da"),
        String::from("el"),
        String::from("hu"),
        String::from("is"),
        String::from("lv"),
        String::from("lt"),
        String::from("sr"),
        String::from("sk"),
        String::from("tr"),
        String::from("uk"),
    ];
}

#[derive(Debug, PartialEq)]
pub enum Error {
    LanguageCodeNotSupported(String),
    TokenFileImportNotSupported(String),
    TokenTypeNotSupported(String),
    FancyRegexError
}

impl From<fancy_regex::Error> for Error {
    fn from(_error: fancy_regex::Error) -> Self {
        Error::FancyRegexError
    }
}

#[derive(Deserialize, Debug, Clone)]
struct InToken {
    tokens: Vec<String>,
    full: String,
    canonical: String,
    note: Option<String>,
    #[serde(rename = "onlyCountries")]
    only_countries: Option<Vec<String>>,
    #[serde(rename = "onlyLayers")]
    only_layers: Option<Vec<String>>,
    #[serde(rename = "preferFull")]
    prefer_full: Option<bool>,
    regex: Option<bool>,
    #[serde(rename = "skipBoundaries")]
    skip_boundaries: Option<bool>,
    #[serde(rename = "skipDiacriticStripping")]
    skip_diacritic_stripping: Option<bool>,
    #[serde(rename = "spanBoundaries")]
    span_boundaries: Option<u8>,
    #[serde(rename = "type")]
    token_type: Option<String>,
}

pub struct Token {
    pub tokens: Vec<String>,
    pub full: Replacer,
    pub canonical: String,
    pub note: Option<String>,
    pub only_countries: Option<Vec<String>>,
    pub only_layers: Option<Vec<String>>,
    pub prefer_full: bool,
    pub regex: bool,
    pub skip_boundaries: bool,
    pub skip_diacritic_stripping: bool,
    pub span_boundaries: Option<u8>,
    pub token_type: Option<TokenType>,
}

impl Token {
    fn new(input: InToken) -> Result<Self, Error> {
        Ok(Token {
            tokens: input.tokens,
            full: match input.regex {
                Some(true) => Replacer::Regex(Regex::new(&input.full)?),
                Some(false) | None => Replacer::String(input.full),
            },
            canonical: input.canonical,
            note: input.note,
            only_countries: input.only_countries,
            only_layers: input.only_layers,
            prefer_full: input.prefer_full.unwrap_or(false),
            regex: input.regex.unwrap_or(false),
            skip_boundaries: input.skip_boundaries.unwrap_or(false),
            skip_diacritic_stripping: input.skip_diacritic_stripping.unwrap_or(false),
            span_boundaries: input.span_boundaries,
            token_type: match input.token_type {
                None => None,
                Some(t) => match TokenType::from_str(&t) {
                    Ok(t) => Some(t),
                    Err(e) => return Err(e)
                }
            }
        })
    }
}

pub enum Replacer {
   String(String),
   Regex(Regex)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TokenType {
    PostalBox,
    Cardinal,
    Number,
    Ordinal,
    Unit,
    Way
}

impl TokenType {
    fn from_str(s: &str) -> Result<TokenType, Error> {
        match s {
            "box" => Ok(TokenType::PostalBox),
            "cardinal" => Ok(TokenType::Cardinal),
            "number" => Ok(TokenType::Number),
            "ordinal" => Ok(TokenType::Ordinal),
            "unit" => Ok(TokenType::Unit),
            "way" => Ok(TokenType::Way),
            _ => Err(Error::TokenTypeNotSupported(s.to_string()))
        }
    }
}

pub fn config(v: Vec<String>) -> Result<HashMap<String, Vec<Token>>, Error> {
    if v.is_empty() {
        return Ok(prepare(LANGUAGE_CODES.to_vec())?)
    }
    for lc in &v {
        if !LANGUAGE_CODES.contains(lc) {
            return Err(Error::LanguageCodeNotSupported(lc.to_string()))
        }
    }
    Ok(prepare(v)?)
}

fn prepare(v: Vec<String>) -> Result<HashMap<String, Vec<Token>>, Error> {
    let mut map = HashMap::new();
    for lc in &v {
        let parsed : Vec<InToken> = serde_json::from_str(import(lc)?)
            .expect("unable to parse token JSON");
        let mut tokens = Vec::new();
        for tk in &parsed {
            tokens.push(Token::new(tk.clone())?);
        }
        map.insert(lc.clone(), tokens);
    }
    Ok(map)
}

fn import(lc: &str) -> Result<&str, Error> {
    match lc {
        "be" => Ok(include_str!("../tokens/be.json")),
        "bg" => Ok(include_str!("../tokens/bg.json")),
        "bs" => Ok(include_str!("../tokens/bs.json")),
        "cat" => Ok(include_str!("../tokens/cat.json")),
        "cs" => Ok(include_str!("../tokens/cs.json")),
        "da" => Ok(include_str!("../tokens/da.json")),
        "de" => Ok(include_str!("../tokens/de.json")),
        "el" => Ok(include_str!("../tokens/el.json")),
        "en" => Ok(include_str!("../tokens/en.json")),
        "es" => Ok(include_str!("../tokens/es.json")),
        "et" => Ok(include_str!("../tokens/et.json")),
        "fi" => Ok(include_str!("../tokens/fi.json")),
        "fr" => Ok(include_str!("../tokens/fr.json")),
        "he" => Ok(include_str!("../tokens/he.json")),
        "hr" => Ok(include_str!("../tokens/hr.json")),
        "hu" => Ok(include_str!("../tokens/hu.json")),
        "id" => Ok(include_str!("../tokens/id.json")),
        "is" => Ok(include_str!("../tokens/is.json")),
        "it" => Ok(include_str!("../tokens/it.json")),
        "ja" => Ok(include_str!("../tokens/ja.json")),
        "lt" => Ok(include_str!("../tokens/lt.json")),
        "lv" => Ok(include_str!("../tokens/lv.json")),
        "nl" => Ok(include_str!("../tokens/nl.json")),
        "no" => Ok(include_str!("../tokens/no.json")),
        "pl" => Ok(include_str!("../tokens/pl.json")),
        "pt" => Ok(include_str!("../tokens/pt.json")),
        "ro" => Ok(include_str!("../tokens/ro.json")),
        "ru" => Ok(include_str!("../tokens/ru.json")),
        "sk" => Ok(include_str!("../tokens/sk.json")),
        "sq" => Ok(include_str!("../tokens/sq.json")),
        "sr" => Ok(include_str!("../tokens/sr.json")),
        "sv" => Ok(include_str!("../tokens/sv.json")),
        "tr" => Ok(include_str!("../tokens/tr.json")),
        "uk" => Ok(include_str!("../tokens/uk.json")),
        _ => Err(Error::TokenFileImportNotSupported(lc.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_config() {
        let lcs = config(vec![String::from("de"), String::from("en")]).unwrap();
        assert_eq!(lcs.len(), 2);
        assert!(lcs.contains_key("de"));
        assert!(lcs.contains_key("en"));

        let empty_lc = config(Vec::new()).unwrap();
        let every_lc = prepare(LANGUAGE_CODES.to_vec()).unwrap();
        assert_eq!(empty_lc.len(), every_lc.len());
        for lc in LANGUAGE_CODES.to_vec() {
            assert!(empty_lc.contains_key(&lc));
        }
    }

    #[test]
    #[should_panic(expected = "LanguageCodeNotSupported(\"zz\")")]
    fn fail_config() {
        config(vec![String::from("zz")]).unwrap();
    }

    #[test]
    fn test_all_lcs() {
        let mut fs_lcs = read_files();
        alphanumeric_sort::sort_str_slice(&mut fs_lcs);
        assert_eq!(LANGUAGE_CODES.to_vec(), fs_lcs);
    }

    #[test]
    fn test_prepare() {
        let lcs = prepare(vec![String::from("de"), String::from("en")]).unwrap();
        assert_eq!(lcs.len(), 2);
        assert!(lcs.contains_key("de"));
        assert!(lcs.contains_key("en"));
    }

    #[test]
    #[should_panic(expected = "TokenFileImportNotSupported(\"zz\")")]
    fn fail_import() {
        import("zz").unwrap();
    }

    #[test]
    fn test_token_values() {
        let map = config(Vec::new()).unwrap();

        for lc in map.values() {
            for tk in lc {
                assert!(tk.tokens.len() > 0);
                match &tk.only_layers {
                    Some(l) => {
                        assert_eq!(l[0], "address");
                        assert_eq!(l.len(), 1);
                    },
                    _ => (),
                }
            }
        }
    }

    fn read_files() -> Vec<String> {
        let mut lcs = Vec::new();
        for entry in fs::read_dir("./tokens").unwrap() {
            let file_name = entry.unwrap().file_name().into_string().unwrap();
            let file_components: Vec<&str> = file_name.split(".").collect();
            if file_components[1] == "json" {
                lcs.push(file_components[0].to_owned());
            }
        }
        lcs
    }
}
