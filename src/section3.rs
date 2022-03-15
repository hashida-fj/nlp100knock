#[macro_use]
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use nom::{
    bytes::complete::{is_a, is_not, tag, take_while_m_n},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult,
};
extern crate nom;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Template {
    pub template_name: String,
    pub template_parameter: String,
    pub keyvalue: Vec<KeyValue>,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn load_articles(path: &Path) -> std::io::Result<HashMap<String, String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    #[derive(Deserialize)]
    struct Article {
        title: String,
        text: Option<String>,
    }
    let mut result: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        let article: Article = serde_json::from_str(line)?;
        result.insert(article.title, article.text.unwrap_or("".to_string()));
    }
    return Ok(result);
}

fn category(body: &String) {
    let re = Regex::new(r"\[\[Category:.*\]\]").unwrap();
    for line in body.lines().filter(|l| re.is_match(l)) {
        println!("{}", line);
    }
}

fn category_name(body: &String) {
    let re = Regex::new(r"\[\[Category:([^|\n]*)(|.*)?\]\]").unwrap();
    for cap in body.lines().filter_map(|l| re.captures(l)) {
        println!("{:?}", cap.get(1).map_or("", |m| m.as_str()));
    }
}

fn section(body: &String) {
    let re = Regex::new(r"(?P<level>==+)([^=]*)=+").unwrap();
    for cap in body.lines().filter_map(|l| re.captures(l)) {
        println!(
            "{}: level{}  ({})",
            cap.get(2).map_or("", |m| m.as_str()),
            cap.get(1).map_or("", |m| m.as_str()).len(),
            cap.get(0).map_or("", |m| m.as_str())
        );
    }
}

fn regexp_test() {
    let re = Regex::new(r"aaa.*(\|(key=(\w+))).*").unwrap();
    let text = "aaa|key=hoge|key=fuga";

    for f in re.find_iter(text) {
        eprintln!(" = {:?}", f);
    }
}

struct BasicInfomation {}

fn kv_parser(s: &str) -> IResult<&str, KeyValue> {
    let (unused, _) = tag("|")(s)?;
    let (unused, key) = is_not("=")(unused)?;
    let (unused, _) = tag("=")(unused)?;
    let (unused, val) = is_not("|}")(unused)?;

    let kv = KeyValue {
        key: key.to_string(),
        value: val.to_string(),
    };
    Ok((unused, kv))
}

fn template_perser(s: &str) -> IResult<&str, Template> {
    let (unused, name) = is_not(" \n")(s)?;
    let (unused, _) = is_a(" \n")(unused)?;
    let (unused, param) = is_not(" \n")(unused)?;
    let (unused, _) = is_a(" \n")(unused)?;
    let (unused, kv) = many0(kv_parser)(unused)?;
    Ok((
        unused,
        Template {
            template_name: name.to_string(),
            template_parameter: param.to_string(),
            keyvalue: kv,
        },
    ))
}

fn basic_infomation(body: &str) -> IResult<&str, Template> {
    let (unused, _) = tag("{{")(body)?;
    let (unused, tmp) = template_perser(unused)?;
    let (unused, _) = tag("}}")(unused)?;

    Ok((unused, tmp))
}

pub fn run_section3() {
    let articles = load_articles(Path::new("./assets/jawiki-country.json")).unwrap();
    // println!("{}", articles["イギリス"]);
    // category(&articles["イギリス"]);
    // category_name(&articles["イギリス"]);
    // section(&articles["イギリス"]);

    // basic_infomation(&articles["イギリス"]);
    // regexp_test();

    let target = r"{{基礎情報 国
|key1=hoge
|key2=fuga
}}";

    let ret = basic_infomation(&target[..]).unwrap();
    eprintln!(" = {:#?}", ret);
}

#[test]
fn regexp() {
    let target = r"{{基礎情報 国
|key1=hoge
|key2=fuga
}}"
    .to_string();
    let re = Regex::new(r"(?ms)^\{\{\s*基礎情報\s*国\s*(\|.*?=.*?)+\}\}").unwrap();
    let cap = re.captures(&target).unwrap();
    let keyvalues = cap.get(1).map_or("", |m| m.as_str());
    assert_eq!(keyvalues, ".");
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}
