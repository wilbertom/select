extern crate csspie;

use csspie::tokenizer::{Token, TokenSink, tokenize};

#[test]
fn test_tokenizer_letters() {
    let result = tokenize(String::from_str("abc"));
    let expected = TokenSink {
        tokens: vec![Token::letter('a'),
                     Token::letter('b'),
                     Token::letter('c')]
    };

    assert_eq!(result, expected);

}

#[test]
fn test_tokenize_whitespace() {
    let result = tokenize(String::from_str(" "));
    let expected = TokenSink {
        tokens: vec![Token::whitespace()]
    };

    assert_eq!(result, expected);

}

#[test]
fn test_tokenize_digits() {
    let result = tokenize(String::from_str("0123456789"));
    let expected = TokenSink {
        tokens: vec![Token::digit('0'),
                     Token::digit('1'),
                     Token::digit('2'),
                     Token::digit('3'),
                     Token::digit('4'),
                     Token::digit('5'),
                     Token::digit('6'),
                     Token::digit('7'),
                     Token::digit('8'),
                     Token::digit('9')]
    };

    assert_eq!(result, expected);

}

#[test]
fn test_tokenize_specials() {
    let result = tokenize(String::from_str("*[]=~^$:.#>+()"));
    let expected = TokenSink {
        tokens: vec![Token::star(),
                     Token::opening_bracket(),
                     Token::closing_bracket(),
                     Token::equal(),
                     Token::tilda(),
                     Token::caret(),
                     Token::dollar(),
                     Token::colon(),
                     Token::dot(),
                     Token::hash_tag(),
                     Token::greater_than(),
                     Token::plus(),
                     Token::opening_paren(),
                     Token::closing_paren()]
    };

    assert_eq!(result, expected);

}
