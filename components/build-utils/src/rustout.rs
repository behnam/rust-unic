const INDENT: &'static str = "    ";

use std::io;
use std::io::Write;
use std::fs::File;
use std::fmt::Display;

fn escape_char(codepoint: char) -> String {
    format!("\\u{{{:X}}}", codepoint as u32)
}

pub fn emit_preamble(script: &str, output: &mut File) -> io::Result<()> {
    writeln!(
        output,
        "// WARNING: Auto-generated by {}. DO NOT EDIT MANUALLY!",
        script
    )
}

pub fn emit_value<T, F, D>(
    script: &str,
    output: &mut File,
    value: T,
    formatter: F,
) -> io::Result<()>
where
    F: Fn(T) -> D,
    D: Display,
{
    emit_preamble(script, output)?;
    writeln!(output, "{}", formatter(value))
}

pub fn emit_strings<I, II>(script: &str, output: &mut File, strings: I) -> io::Result<()>
where
    I: IntoIterator<Item = II>,
    II: IntoIterator<Item = char>,
{
    let codepoints = strings
        .into_iter()
        .map(|string| {
            string
                .into_iter()
                .map(|char| escape_char(char))
                .fold(String::new(), |acc, ref str| acc + str)
        })
        .fold(String::new(), |acc, ref str| acc + "\n" + str);
    emit_value(
        script,
        output,
        codepoints,
        |str| format!("\"\\\n{}\\\n\"", str),
    )
}

fn write_rows<I, T, F, D>(output: &mut File, values: I, formatter: F) -> io::Result<()>
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> D,
    D: Display,
{
    for value in values.into_iter() {
        writeln!(output, "{}{},", INDENT, formatter(value))?;
    }
    Ok(())
}

pub fn emit_table<I, T, F, D>(
    script: &str,
    output: &mut File,
    data: I,
    formatter: F,
) -> io::Result<()>
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> D,
    D: Display,
{
    emit_preamble(script, output)?;
    writeln!(output, "&[")?;
    write_rows(output, data, formatter)?;
    writeln!(output, "]")
}

pub fn emit_range_bsearch_table<I>(script: &str, output: &mut File, data: I) -> io::Result<()>
where
    I: IntoIterator<Item = (u32, u32)>,
{
    emit_table(script, output, data, |datum| {
        format!("(0x{:X},0x{:X})", datum.0, datum.1)
    })
}

pub fn emit_value_range_bsearch_table<I, D>(
    script: &str,
    output: &mut File,
    data: I,
) -> io::Result<()>
where
    I: IntoIterator<Item = (u32, u32, D)>,
    D: Display,
{
    emit_table(script, output, data, |datum| {
        format!("(0x{:X},0x{:X},\"{}\")", datum.0, datum.1, datum.2)
    })
}

/// TODO
pub fn emit_lookup_tables() -> ! {
    unimplemented!()
}
