module.exports = () => {
    let tokens = {};

    //Prepend Standard word boundaries + ^
    //Postpend Standard word boundaries + $
    let stdBoundaries = {
        "P\\.?\\ ?O\\.? Box [0-9]+": " ",
        "(.+)(strasse|str|straße)": " $1 str ",
        "Suite [0-9]+": " ",
        "Suite [0-9]+-[0-9]+": " ",
        "Suite [0-9]+[a-z]": " ",
        "Suite [a-z]": " ",
        "STE [0-9]+": " ",
        "STE [a-z]": " ",
        "Post Office": " Po ",
        "Railway Station": " Rs ",
        "Village Post Office": " Vpo ",
        "Camino hondo": " CH ",
        "Camino nuevo": " CN ",
        "Camino viejo": " CV ",
        "Gran Vía": " GV ",
        "Pgind": " PI ",
        "Polígono industrial": " PI ",
        "Punto kilométrico": " pk ",
        "Paseo marítimo": " Psmar ",
        "Boîte Postale": " BP ",
        "Centre Commercial": " Ccal ",
        "Route Européenne": " E ",
        "Route Nationale": " RN ",
        "Zone d'activité": " Za ",
        "N.T.": " NT ",
        "New Territories": " NT ",
        "Strada Comunale": " SC ",
        "Strada Provinciale": " SP ",
        "Strada Regionale": " SR ",
        "Strada Statale": " SS ",
        "Van De": " vd ",
        "Van Den": " vd ",
        "Van Der": " vd ",
        "Câmara Municipal": " CM ",
        "&": "and"
    };

    const BOUNDARIES = "[\\s\\u2000-\\u206F\\u2E00-\\u2E7F\\\\'!\"#$%&()*+,\\-.\\/:;<=>?@\\[\\]^_`{|}~]"

    for (let find in stdBoundaries) {
        tokens[`(?:${BOUNDARIES}|^)${find}(?:${BOUNDARIES}|$)`] = stdBoundaries[find];
    }

    let custBoundaries = {
        "\\b1丁目\\b": " 一丁目 ",
        "\\b2丁目\\b": " 二丁目 ",
        "\\b3丁目\\b": " 三丁目 ",
        "\\b4丁目\\b": " 四丁目 ",
        "\\b5丁目\\b": " 五丁目 ",
        "\\b6丁目\\b": " 六丁目 ",
        "\\b7丁目\\b": " 七丁目 ",
        "\\b8丁目\\b": " 八丁目 ",
        "\\b9丁目\\b": " 九丁目 ",
        "\\b10丁目\\b": " 十丁目 ",
        "^([\\u1100-\\u11FF\\u2E80-\\u2EFF\\u3000-\\u303F\\u3040-\\u309F\\u30A0-\\u30FF\\u3100-\\u312F\\u3130-\\u318F\\u31C0-\\u31EF\\u31F0-\\u31FF\\u3200-\\u32FF\\u3300-\\u33FF\\u3400-\\u4DBF\\u4E00-\\u9FFF\\uF900-\\uFAFF\\uFE30-\\uFE4F\\s]+)(\\s*)([0-9]+)-([0-9]+)(.*)$": "$1$5 $3"
    }

    for (let find in custBoundaries) {
        tokens[find] = custBoundaries[find];
    }

    return tokens;
}
