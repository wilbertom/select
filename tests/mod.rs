#![feature(globs)]

extern crate select;

use std::collections::HashMap;

use select::tokenizer::*;
use select::select_tree::SelectTree;

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
    let expected = vec![Combinator(Whitespace)];

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
    let expected = vec![Universal,
                        OpeningBracket,
                        ClosingBracket,
                        Equal,
                        Combinator(Tilde),
                        Caret,
                        DollarSign,
                        Colon,
                        Dot,
                        HashTag,
                        Combinator(GreaterThan),
                        Combinator(Plus),
                        OpeningParen,
                        ClosingParen,
                        Quote,
                        Hyphen];

    assert_eq!(result, expected);

}

#[test]
fn test_tokenize_examples() {

    // testing with real world examples

    let examples = vec![
      String::from_str(".location a:first-child"),
      String::from_str("a"),
      String::from_str(".block a.current.trait"),
      String::from_str("._5bsm ._50bm ._po-"),
      String::from_str("#bootloader_WO4pn"),
      String::from_str(".avatar-change:hover"),
      String::from_str("ul li:nth-child(2)"),
      String::from_str("span[lang~=\"en-us\"]")
    ];

    let results = vec![
      vec![
        Dot, Identifier(String::from_str("location")),
        Combinator(Whitespace), Identifier(String::from_str("a")),
        Colon, Identifier(String::from_str("first")),
        Hyphen, Identifier(String::from_str("child"))
      ],

      vec![Identifier(String::from_str("a"))],

      vec![
        Dot, Identifier(String::from_str("block")), Combinator(Whitespace),
        Identifier(String::from_str("a")), Dot,
        Identifier(String::from_str("current")), Dot,
        Identifier(String::from_str("trait"))
      ],

      vec![
        Dot, Identifier(String::from_str("_5bsm")),
        Combinator(Whitespace), Dot, Identifier(String::from_str("_50bm")),
        Combinator(Whitespace), Dot, Identifier(String::from_str("_po")),
        Hyphen
      ],

      vec![
        HashTag, Identifier(String::from_str("bootloader_WO4pn"))
      ],

      vec![
        Dot, Identifier(String::from_str("avatar")), Hyphen,
        Identifier(String::from_str("change")), Colon,
        Identifier(String::from_str("hover"))
      ],

      vec![
        Identifier(String::from_str("ul")), Combinator(Whitespace),
        Identifier(String::from_str("li")),
        Colon, Identifier(String::from_str("nth")), Hyphen,
        Identifier(String::from_str("child")), OpeningParen,
        Integer(String::from_str("2")), ClosingParen
      ],

      vec![
        Identifier(String::from_str("span")), OpeningBracket,
        Identifier(String::from_str("lang")), Combinator(Tilde), Equal,
        Quote, Identifier(String::from_str("en")), Hyphen,
        Identifier(String::from_str("us")), Quote, ClosingBracket
      ]

    ];

    for (example, result) in examples.iter().zip(results.iter()) {
        assert_eq!(tokenize(example.clone()).tokens, *result);
    }
}

#[test]
fn test_seleting_element() {

    let select_data = SelectTree {
        id: None,
        kind: "root",
        children: vec![
          SelectTree {id: None, kind: "person", children: vec![], attributes: HashMap::new()},
          SelectTree {id: None, kind: "person", children: vec![], attributes: HashMap::new()},
          SelectTree {id: None, kind: "person", children: vec![], attributes: HashMap::new()},
          SelectTree {id: None, kind: "person", children: vec![], attributes: HashMap::new()},
        ],
        attributes: HashMap::new()
    };


}
