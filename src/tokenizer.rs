use std::option::Option;

#[deriving(PartialEq, Show)]
pub enum Token {
    Whitespace,
    Hyphen,
    OpeningBracket,
    ClosingBracket,
    Equal,
    Tilda,
    Caret,
    DollarSign,
    Star,
    Colon,
    Dot,
    HashTag,
    GreaterThan,
    Plus,
    ClosingParen,
    OpeningParen,
    Quote,
    Colons,
    Word(&'static str),
    Integer(&'static str),
}


#[deriving(PartialEq, Show)]
pub struct TokenSink {
    pub tokens: Vec<Token>,
    last_sunk: Option<Token>
}

impl TokenSink {
    pub fn new() -> TokenSink {
        TokenSink::from_vec(Vec::new())
    }

    pub fn from_vec(v: Vec<Token>) -> TokenSink {
        TokenSink {
            tokens: v,
            last_sunk: None
        }
    }

    pub fn push(&mut self, token: Token) {

        self.last_sunk = Some(new_token);
        self.tokens.push(new_token);

    }

    pub fn push_k_v(&mut self, k: TokenKind, v: char) {
        self.push(Token::new(k, v));
    }
}

pub struct Tokenizer {
    token_sink: TokenSink,
    position: uint,
    input: String
}

impl Tokenizer {

    pub fn current_char(&self) -> char {
        self.input.as_slice().char_at(self.position)
    }

    pub fn advance(&mut self) {
        let range = self.input.as_slice().char_range_at(self.position);
        self.position = range.next;
    }

    pub fn done(&self) -> bool {
        self.position >= self.input.len()
    }

}


pub fn tokenize(input: String) -> TokenSink {

    let mut tokenizer = Tokenizer {
        token_sink: TokenSink::new(),
        position: 0,
        input: input
    };

    loop {

        if tokenizer.done() {
            break;
        } else {

            let c = tokenizer.current_char();

            let token_kind = if c.is_whitespace() { Whitespace } else {

                match c {
                    'a' ... 'z' | 'A' ... 'Z' => Letter,
                    '-' => Hyphen,
                    '0' ... '9' => Digit,
                    '[' => OpeningBracket,
                    ']' => ClosingBracket,
                    '=' => Equal,
                    '~' => Tilda,
                    '^' => Caret,
                    '$' => DollarSign,
                    '*' => Star,
                    ':' => Colon,
                    '.' => Dot,
                    '#' => HashTag,
                    '>' => GreaterThan,
                    '+' => Plus,
                    '(' => OpeningParen,
                    ')' => ClosingParen,
                    _ => panic!("Unknown character {} in {}", c, tokenizer.input)
                }
            };

            tokenizer.token_sink.push_k_v(token_kind, c);
            tokenizer.advance();
        }

    }

    tokenizer.token_sink
}
