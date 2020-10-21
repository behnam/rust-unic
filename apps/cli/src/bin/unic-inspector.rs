// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;

#[macro_use]
extern crate prettytable;

use clap::Arg;
use prettytable::format::TableFormat;
use prettytable::Table;

use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{name_aliases_of, GeneralCategory, Name, NameAliasType};

use std::env;
use std::io::{self, Read};


fn main() {
    let app = app_from_crate!()
        .about(concat!(
            env!("CARGO_PKG_DESCRIPTION"),
            "\n\n",
            "Inspect characters and their properties",
        ))
        .arg(
            Arg::with_name("STRINGS")
                .help("Input strings (expected valid Unicode)")
                .required(false)
                .multiple(true),
        );
    let matches = app.get_matches();

    // == Read input ==
    let mut input: String = matches
        .values_of("STRINGS")
        .unwrap_or_default()
        .collect::<Vec<&str>>()
        .join(" ");

    if input.len() == 0 {
        if let Some(last_arg) = env::args().last() {
            if last_arg == "--" {
                input.clear();
                io::stdin().read_to_string(&mut input).unwrap();
            }
        }
    }

    // == Write output ==
    let mut table = Table::new();
    let mut table_format = TableFormat::new();
    table_format.padding(1, 1);
    table_format.column_separator('|');
    table.set_format(table_format);

    /* TODO: Enable with option
    table.set_titles(row![
        cu -> " Char ",
        cu -> " Code ",
        cu -> " Name ",
        cu -> " General_Category "
    ]);
    */

    input.chars().for_each(|chr| {
        let display_name = Name::of(chr).map(|n| n.to_string()).unwrap_or_else(|| {
            match name_aliases_of(chr, NameAliasType::NameAbbreviations) {
                Some(abbrs) => abbrs[0].to_owned(),
                None => "<none>".to_owned(),
            }
        });

        table.add_row(row![
            cb  -> &format!("{}", chr),
            rFr -> &format!("U+{:04X}", chr as u32),
            l   -> &display_name,
            l   -> &GeneralCategory::of(chr).long_name()
        ]);
    });

    table.printstd();
}
