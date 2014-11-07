
#[deriving(PartialEq, Show)]
pub enum TokenKind {
    Whitespace,
    Letter,
    Hyphen,
    Digit,
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
    Error
}

#[deriving(PartialEq, Show)]
pub struct Token {
    pub kind: TokenKind,
    pub value: char
}

impl Token {
    pub fn new(kind: TokenKind, value: char) -> Token {
        Token {
            kind: kind, value: value
        }
    }

    pub fn letter(c: char) -> Token {
        Token::new(Letter, c)
    }

    pub fn whitespace() -> Token {
        Token::new(Whitespace, ' ')
    }

    pub fn digit(c: char) -> Token {
        Token::new(Digit, c)
    }

    pub fn tilda() -> Token {
        Token::new(Tilda, '~')
    }

    pub fn star() -> Token {
        Token::new(Star, '*')
    }

    pub fn dollar() -> Token {
        Token::new(DollarSign, '$')
    }

    pub fn opening_bracket() -> Token {
        Token::new(OpeningBracket, '[')
    }

    pub fn closing_bracket() -> Token {
        Token::new(ClosingBracket, ']')
    }

    pub fn equal() -> Token {
        Token::new(Equal, '=')
    }

    pub fn caret() -> Token {
        Token::new(Caret, '^')
    }

    pub fn colon() -> Token {
        Token::new(Colon, ':')
    }

    pub fn dot() -> Token {
        Token::new(Dot, '.')
    }

    pub fn hash_tag() -> Token {
        Token::new(HashTag, '#')
    }

    pub fn greater_than() -> Token {
        Token::new(GreaterThan, '>')
    }

    pub fn plus() -> Token {
        Token::new(Plus, '+')
    }


}

#[deriving(PartialEq, Show)]
pub struct TokenSink {
    pub tokens: Vec<Token>
}

impl TokenSink {
    pub fn new() -> TokenSink {
        TokenSink {
            tokens: Vec::new()
        }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
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

            println!("Processing {}", c);

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
                    _ => Error
                }
            };

            tokenizer.token_sink.push_k_v(token_kind, c);
            tokenizer.advance();
        }

    }

    tokenizer.token_sink
}
