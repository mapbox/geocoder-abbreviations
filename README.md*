# geocoder-abbreviations

Geocoder Abbreviations divided into language groups. These are lossy word
replacements that are useful for geocoding.

In the `tokens` directory, each JSON file contains a list of word equivalencies
for one language and is named by language code.

Each group contains a list of tokens that are considered semantically equivalent
to one another -- they have the same meaning, but some might be abbreviations,
alternate spellings, etc. Different groups might overlap with one another in the
tokens they contain, if the same abbreviation can be used to represent
semantically distinct concepts; for example, in English, 'Cl' is short for both
'Clinic' and 'Close', but 'Clinic' and 'Close' are not equivalent, so the
`en.json` file contains two different groups that both contain the 'Cl' token.

Each group also contains additional properties that might be of use in deciding
how to employ the token list in question. They are:

## Required properties
* **tokens (array of strings):** the tokens included in the group
* **full (string):** given a mix of abbreviations and full words in the token
    list, the preferred full word
* **canonical (string):** given a mix of abbreviations and full words in the
    token list, the preferred abbreviation

## Optional properties
* **note (string):** a human-readable note as to the purpose of the group;
    sometimes contains an English translation of a non-English word
* **onlyCountries (array of strings):** a list of ISO country codes to which the
    usage of the replacement is restricted
* **onlyLayers (array of strings):** a list of kinds of data to which the use of
    the equivalency should be restricted (currently always `address` if present)
* **preferFull (boolean):** an indication that the abbreviation is uncommon and
    the full form should be preserved if present (for example, 'college' can be
    abbreviated 'coll' but this is atypical); absence should be interpreted as
    `false`.
* **regex (boolean):** an indication that the replacement contains a regular
    expression. Absence should be interpreted as `false`. If false, **words should
    be assumed not to have been escaped for use in regular expression** (e.g.,
    periods are just periods), and consumers who want to use them in regular
    expressions should perform their own escaping before doing so.
* **reduceRelevance (boolean):** an indication that the replacement will be indexed
    with a reduced relevance.
* **skipBoundaries (boolean):** an indication that the replacement shouldn't
    have to match at a word boundary. Absence should be interpreted as `false`.
* **skipDiacriticStripping (boolean):** an indication that the replacement
    shouldn't be applied with diacritical marks ignored. Absence should be
    interpreted as `false`.
* **spanBoundaries (number):** if present, indicates that the suggested
    replacement must span tokenization boundaries to be performed, and specifies
    how many boundaries are spanned.
* **type (string):** an indication of the semantic class of the word group (for
    example, a kind of street or road, a number, a cardinal direction, etc.).
    Currently allowed values:
    * **box**: a designator of a postal box or similar (e.g., 'PO Box', 'Boite')
    * **cardinal**: a cardinal direction (e.g., 'North', 'Östra')
    * **number**: a number (e.g., 'eight', '9', 'trois')
    * **ordinal**: an numerical ordinal (e.g., 'fifth', 'deuxième')
    * **unit**: a pattern indicating a unit/floor/sub-address designation (e.g.,
        'Apt [0-9]+')
    * **way**: words like 'street', 'road', 'avenue' -- travel throughways

## Usage

geocoder-abbreviations is available as a Node.js package and as a Rust crate.

### Node.js package

index.js exposes a single function with an optional `lang` arg or if null
returns a map of all the tokens separated by language. It takes an optional
second boolean argument for whether or not to include groups with a single
token in them, and a third optional boolean argument for whether or not to
return the full/advanced representation including all metadata, or a simpler
version of just tokens with no metadata that's backwards-compatible with earlier
releases of this library.

See the index.js JSDoc for more details

### Rust crate

geocoder-abbreviations isn't currently published on crates.io. To add it to your
project, add the following to your `Cargo.toml` `[dependencies]`:

```
geocoder-abbreviations = { git = "https://github.com/mapbox/geocoder-abbreviations", rev = "master" }
```
