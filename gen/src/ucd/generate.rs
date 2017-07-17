// TODO: Break this file up

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::{char, u32};

use super::ucd_data_dir;
use utils::{common, rustout};

use regex::Regex;

const SCRIPT: &'static str = "gen/src/ucd/generate.rs";

lazy_static! {
    static ref CORE_TABLES: &'static Path = Path::new("components/ucd/core/src/tables");
    static ref AGE_TABLES: &'static Path = Path::new("components/ucd/age/src/tables");
    static ref BIDI_TABLES: &'static Path = Path::new("components/ucd/bidi/src/tables");
    static ref NORMAL_TABLES: &'static Path = Path::new("components/ucd/normal/src/tables");
    static ref NORMAL_TEST_TABLES: &'static Path = Path::new("components/normal/test/tables");
}

// == VERSION == //

lazy_static! {
    static ref UNICODE_VERSION: (u16, u16, u16) = {
        let mut readme = File::open(ucd_data_dir().join("ReadMe.txt")).unwrap();
        let mut buffer = String::new();
        readme.read_to_string(&mut buffer).unwrap();
        let pattern = Regex::new(r"for Version (\d+)\.(\d+)\.(\d+)").unwrap();
        let captures = pattern.captures(&buffer).unwrap();
        (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().parse().unwrap(),
        )
    };
}

fn emit_unicode_version(dir: &Path) {
    let mut file = File::create(dir.join("unicode_version.rsv")).unwrap();
    rustout::emit_value(SCRIPT, &mut file, &UNICODE_VERSION, |triple| {
        format!(
            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            triple.0,
            triple.1,
            triple.2
        )
    }).unwrap();
}

// == Shared == //

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UnicodeDataEntry {
    pub codepoint: u32,
    pub name: String,
    pub general_category: String,
    pub canonical_combining_class: String,
    pub bidi_class: String,
    pub decomposition: String,
    pub deci: String,
    pub digit: String,
    pub num: String,
    pub bidi_mirrored: String,
    pub unicode_1_name: String,
    pub iso_comment: String,
    pub simple_uppercase_mapping: String,
    pub simple_lowercase_mapping: String,
    pub simple_titlecase_mapping: String,
}

impl UnicodeDataEntry {
    fn from<I: IntoIterator<Item = String>>(iterator: I) -> Self {
        let mut iterator = iterator.into_iter();
        UnicodeDataEntry {
            codepoint: u32::from_str_radix(&iterator.next().unwrap(), 16).unwrap(),
            name: iterator.next().unwrap(),
            general_category: iterator.next().unwrap(),
            canonical_combining_class: iterator.next().unwrap(),
            bidi_class: iterator.next().unwrap(),
            decomposition: iterator.next().unwrap(),
            deci: iterator.next().unwrap(),
            digit: iterator.next().unwrap(),
            num: iterator.next().unwrap(),
            bidi_mirrored: iterator.next().unwrap(),
            unicode_1_name: iterator.next().unwrap(),
            iso_comment: iterator.next().unwrap(),
            simple_uppercase_mapping: iterator.next().unwrap(),
            simple_lowercase_mapping: iterator.next().unwrap(),
            simple_titlecase_mapping: iterator.next().unwrap(),
        }
    }
}

lazy_static! {
    static ref UNICODE_DATA: HashMap<u32, UnicodeDataEntry> = {
        let mut unicode_data = HashMap::default();
        let mut range_start: Option<u32> = None;
        let file = File::open(ucd_data_dir().join("UnicodeData.txt")).unwrap();
        for line in BufReader::new(file).lines() {
            let data: Vec<_> = line.unwrap().split(';').map(String::from).collect();
            if data.len() != 15 { continue }
            let mut data = UnicodeDataEntry::from(data);
            // skip surrogates
            if let Some(_) = char::from_u32(data.codepoint) {
                if let Some(start) = range_start {
                    data.name = String::from("");
                    let end = data.codepoint+1;
                    for i in start..end {
                        data.codepoint = i;
                        unicode_data.insert(data.codepoint, data.clone());
                    }
                    range_start = None;
                }
                if data.name.ends_with(", First>") {
                    range_start = Some(data.codepoint);
                } else {
                    unicode_data.insert(data.codepoint, data);
                }
            }
        }
        unicode_data
    };
}

// == MAIN == //

pub fn run() {
    for &dir in &[*CORE_TABLES, *AGE_TABLES, *BIDI_TABLES, *NORMAL_TABLES, *NORMAL_TEST_TABLES] {
        common::cleanup_rsv(dir).unwrap();
    }

    // Core
    emit_unicode_version(*CORE_TABLES);
}
