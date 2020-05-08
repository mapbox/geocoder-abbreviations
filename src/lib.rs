use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use fancy_regex::Regex;

#[derive(RustEmbed)]
#[folder = "./tokens/"]
struct Tokens;
impl Tokens {
    pub fn codes() -> Vec<String> {
        let mut codes: Vec<String> = Tokens::iter().filter(|lang| {
            lang.contains(".json")
        }).map(|lang| {
            String::from(lang).replace(".json", "")
        }).collect();

        codes.sort();

        codes
    }

    pub fn import(lc: &str) -> Result<String, Error> {
        match Tokens::get(format!("./tokens/{}.json", &lc).as_str()) {
            Some(tokens) => match std::str::from_utf8(tokens.as_ref()) {
                Ok(tokens) => Ok(String::from(tokens)),
                _ => Err(Error::TokenFileImportNotSupported(lc.to_string()))
            },
            None => Err(Error::TokenFileImportNotSupported(lc.to_string()))
        }
    }
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
    Way,
    Determiner
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
            "determiner" => Ok(TokenType::Determiner),
            _ => Err(Error::TokenTypeNotSupported(s.to_string()))
        }
    }
}

pub fn config(v: Vec<String>) -> Result<HashMap<String, Vec<Token>>, Error> {
    if v.is_empty() {
        return Ok(prepare(Tokens::codes())?)
    }
    for lc in &v {
        if !Tokens::codes().contains(lc) {
            return Err(Error::LanguageCodeNotSupported(lc.to_string()))
        }
    }
    Ok(prepare(v)?)
}

fn prepare(v: Vec<String>) -> Result<HashMap<String, Vec<Token>>, Error> {
    let mut map = HashMap::new();
    for lc in &v {
        let parsed : Vec<InToken> = serde_json::from_str(Tokens::import(lc)?.as_str())
            .expect("unable to parse token JSON");
        let mut tokens = Vec::new();
        for tk in &parsed {
            tokens.push(Token::new(tk.clone())?);
        }
        map.insert(lc.clone(), tokens);
    }
    Ok(map)
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
        let every_lc = prepare(Tokens::codes()).unwrap();
        assert_eq!(empty_lc.len(), every_lc.len());
        for lc in Tokens::codes() {
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
        assert_eq!(Tokens::codes(), fs_lcs);
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
        Tokens::import("zz").unwrap();
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
