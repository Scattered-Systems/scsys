/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description: ... summary ...
*/
pub use self::basic::*;

pub(crate) mod basic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_default() {
        let data = "\"abc\"";
        let result = parse_string::<()>(data);
        assert_eq!(result, Ok(("", String::from("abc"))))
    }

    #[test]
    fn test_other_parser() {
        let data = "\"tab:\\tafter tab, newline:\\nnew line, quote: \\\", emoji: \\u{1F602}, newline:\\nescaped whitespace: \\    abc\"";
        let result = parse_string::<()>(data);
        assert_eq!(
            result,
            Ok((
            "",
            String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc")
            ))
        )
    }
}
