use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref LANGUAGE_CODES: Vec<String> = vec![
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
        String::from("sv")
    ];
}

#[derive(Debug, PartialEq)]
pub enum Error {
    LanguageCodeNotSupported(String),
    TokenFileImportNotSupported(String),
    TokenTypeNotSupported(String),
    RegexError(String)
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::RegexError(error.to_string())
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

#[derive(Debug, Clone)]
pub struct Token {
    pub tokens: Vec<String>,
    pub full: String,
    pub regex: Option<Regex>,
    pub canonical: String,
    pub note: Option<String>,
    pub only_countries: Option<Vec<String>>,
    pub only_layers: Option<Vec<String>>,
    pub prefer_full: bool,
    pub skip_boundaries: bool,
    pub skip_diacritic_stripping: bool,
    pub span_boundaries: Option<u8>,
    pub token_type: Option<TokenType>,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {

        // do not check that self.regex == other.regex
        // can't derive PartialEq trait on regex::Regex
        // these values are created from the full property which is checked
        let self_regex = match &self.regex {
            Some(r) => Some(r.as_str()),
            None => None
        };
        let other_regex = match &other.regex {
            Some(r) => Some(r.as_str()),
            None => None
        };

        self_regex == other_regex &&
        self.tokens == other.tokens &&
        self.full == other.full &&
        self.canonical == other.canonical &&
        self.note == other.note &&
        self.only_countries == other.only_countries &&
        self.only_layers == other.only_layers &&
        self.prefer_full == other.prefer_full &&
        self.skip_boundaries == other.skip_boundaries &&
        self.skip_diacritic_stripping == other.skip_diacritic_stripping &&
        self.span_boundaries == other.span_boundaries &&
        self.token_type == other.token_type
    }
}

impl Token {
    pub fn new(full: String, canonical: String, token_type: Option<TokenType>, regex: bool) -> Result<Self, Error> {
        Ok(Token {
            regex: match regex {
                true => Some(Regex::new(&full)?),
                false => None
            },
            tokens: vec![canonical.clone(), full.clone()],
            full: full,
            canonical: canonical,
            note: None,
            only_countries: None,
            only_layers: None,
            prefer_full: false,
            skip_boundaries: false,
            skip_diacritic_stripping: false,
            span_boundaries: None,
            token_type: token_type,
        })
    }

    fn from_input(input: InToken) -> Result<Self, Error> {
        Ok(Token {
            regex: match input.regex {
                Some(true) => Some(Regex::new(&input.full)?),
                Some(false) | None => None,
            },
            tokens: input.tokens,
            full: input.full,
            canonical: input.canonical,
            note: input.note,
            only_countries: input.only_countries,
            only_layers: input.only_layers,
            prefer_full: input.prefer_full.unwrap_or(false),
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
            let out = Token::from_input(tk.clone());
            match out {
                Ok(o) => tokens.push(o),
                Err(err) => {
                    match err {
                        Error::RegexError(ref e) => {
                            if e.contains("look-around, including look-ahead and look-behind, is not supported") {
                                println!("warn - filtered unsupported lookaround regex {}", tk.full);
                                continue;
                            } else {
                                return Err(err)
                            }
                        },
                        _ => return Err(err)
                    }
                },
            }
        }
        map.insert(lc.clone(), tokens);
    }
    Ok(map)
}

fn import(lc: &str) -> Result<&str, Error> {
    match lc {
        "de" => Ok(include_str!("../tokens/de.json")),
        "en" => Ok(include_str!("../tokens/en.json")),
        "es" => Ok(include_str!("../tokens/es.json")),
        "et" => Ok(include_str!("../tokens/et.json")),
        "fi" => Ok(include_str!("../tokens/fi.json")),
        "fr" => Ok(include_str!("../tokens/fr.json")),
        "he" => Ok(include_str!("../tokens/he.json")),
        "id" => Ok(include_str!("../tokens/id.json")),
        "it" => Ok(include_str!("../tokens/it.json")),
        "ja" => Ok(include_str!("../tokens/ja.json")),
        "nl" => Ok(include_str!("../tokens/nl.json")),
        "no" => Ok(include_str!("../tokens/no.json")),
        "pl" => Ok(include_str!("../tokens/pl.json")),
        "pt" => Ok(include_str!("../tokens/pt.json")),
        "ro" => Ok(include_str!("../tokens/ro.json")),
        "ru" => Ok(include_str!("../tokens/ru.json")),
        "sv" => Ok(include_str!("../tokens/sv.json")),
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
