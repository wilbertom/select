#![feature(globs)]

extern crate csspie;

use csspie::tokenizer::*;

#[test]
fn test_tokenizer_letters() {
    let s = String::from_str("abc23");
    let result = tokenize(s.clone()).tokens;
    let expected = vec![Identifier(s)];

    assert_eq!(result, expected);
}

#[test]
fn test_tokenizer_underscore_start() {
    let s = String::from_str("_abc_12_");
    let result = tokenize(s.clone()).tokens;
    let expected = vec![Identifier(s)];

    assert_eq!(result, expected);
}


#[test]
fn test_tokenizer_underscore_in() {
    let s = String::from_str("abc_12");
    let result = tokenize(s.clone()).tokens;
    let expected = vec![Identifier(s)];

    assert_eq!(result, expected);
}

#[test]
fn test_tokenize_whitespace() {
    let result = tokenize(String::from_str(" ")).tokens;
    let expected = vec![Whitespace];

    assert_eq!(result, expected);

}

#[test]
fn test_tokenize_digits() {
    let s = String::from_str("0123456789");
    let result = tokenize(s.clone()).tokens;
    let expected = vec![Integer(s)];

    assert_eq!(result, expected);

}

#[test]
fn test_tokenize_specials() {
    let result = tokenize(String::from_str("*[]=~^$:.#>+()\"-")).tokens;
    let expected = vec![Star,
                        OpeningBracket,
                        ClosingBracket,
                        Equal,
                        Tilda,
                        Caret,
                        DollarSign,
                        Colon,
                        Dot,
                        HashTag,
                        GreaterThan,
                        Plus,
                        OpeningParen,
                        ClosingParen,
                        Quote,
                        Hyphen];

    assert_eq!(result, expected);

}
