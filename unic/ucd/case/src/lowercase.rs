// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Lowercase` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Lowercase`](https://www.unicode.org/reports/tr44/#Lowercase).
    ///
    /// The value is `true` for lowercase characters, `false` otherwise.
    pub struct Lowercase(bool) {
        abbr => "Lower";
        long => "Lowercase";
        human => "Lowercase";

        data_table_path => "../tables/lowercase.rsv";
    }

    /// Return `true` for lowercase character, `false` otherwise.
    pub fn is_lowercase(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_lowercase;

        // ASCII
        assert_eq!(is_lowercase('\u{0020}'), false);
        assert_eq!(is_lowercase('\u{0021}'), false);
        assert_eq!(is_lowercase('\u{0022}'), false);

        assert_eq!(is_lowercase('\u{0030}'), false);
        assert_eq!(is_lowercase('\u{0031}'), false);
        assert_eq!(is_lowercase('\u{0032}'), false);

        assert_eq!(is_lowercase('\u{0040}'), false);
        assert_eq!(is_lowercase('\u{0041}'), false);
        assert_eq!(is_lowercase('\u{0042}'), false);

        assert_eq!(is_lowercase('\u{0060}'), false);
        assert_eq!(is_lowercase('\u{0061}'), true);
        assert_eq!(is_lowercase('\u{0062}'), true);

        assert_eq!(is_lowercase('\u{007e}'), false);
        assert_eq!(is_lowercase('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_lowercase('\u{061b}'), false);
        assert_eq!(is_lowercase('\u{061c}'), false);
        assert_eq!(is_lowercase('\u{061d}'), false);

        assert_eq!(is_lowercase('\u{200d}'), false);
        assert_eq!(is_lowercase('\u{200e}'), false);
        assert_eq!(is_lowercase('\u{200f}'), false);
        assert_eq!(is_lowercase('\u{2010}'), false);

        assert_eq!(is_lowercase('\u{2029}'), false);
        assert_eq!(is_lowercase('\u{202a}'), false);
        assert_eq!(is_lowercase('\u{202e}'), false);
        assert_eq!(is_lowercase('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_lowercase('\u{10000}'), false);
        assert_eq!(is_lowercase('\u{10001}'), false);

        assert_eq!(is_lowercase('\u{20000}'), false);
        assert_eq!(is_lowercase('\u{30000}'), false);
        assert_eq!(is_lowercase('\u{40000}'), false);
        assert_eq!(is_lowercase('\u{50000}'), false);
        assert_eq!(is_lowercase('\u{60000}'), false);
        assert_eq!(is_lowercase('\u{70000}'), false);
        assert_eq!(is_lowercase('\u{80000}'), false);
        assert_eq!(is_lowercase('\u{90000}'), false);
        assert_eq!(is_lowercase('\u{a0000}'), false);
        assert_eq!(is_lowercase('\u{b0000}'), false);
        assert_eq!(is_lowercase('\u{c0000}'), false);
        assert_eq!(is_lowercase('\u{d0000}'), false);
        assert_eq!(is_lowercase('\u{e0000}'), false);

        assert_eq!(is_lowercase('\u{efffe}'), false);
        assert_eq!(is_lowercase('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_lowercase('\u{f0000}'), false);
        assert_eq!(is_lowercase('\u{f0001}'), false);
        assert_eq!(is_lowercase('\u{ffffe}'), false);
        assert_eq!(is_lowercase('\u{fffff}'), false);
        assert_eq!(is_lowercase('\u{100000}'), false);
        assert_eq!(is_lowercase('\u{100001}'), false);
        assert_eq!(is_lowercase('\u{10fffe}'), false);
        assert_eq!(is_lowercase('\u{10ffff}'), false);
    }
}
