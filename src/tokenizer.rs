
#[deriving(PartialEq, Show)]
pub enum CombinatorKind {
    GreaterThan,
    Whitespace,
    Plus,
    Tilde
}

#[deriving(PartialEq, Show)]
pub enum Token {
    Hyphen,
    OpeningBracket,
    ClosingBracket,
    Equal,
    Caret,
    DollarSign,
    Star,
    Colon,
    Dot,
    HashTag,
    ClosingParen,
    OpeningParen,
    Quote,
    Colons,
    Identifier(String),
    Integer(String),
    Combinator(CombinatorKind),
}

#[deriving(PartialEq, Show)]
pub struct TokenSink {
    pub tokens: Vec<Token>
}

impl TokenSink {
    pub fn new() -> TokenSink {
        TokenSink::from_vec(Vec::new())
    }

    pub fn from_vec(v: Vec<Token>) -> TokenSink {
        TokenSink {
            tokens: v
        }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);

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

    pub fn take_while(&mut self, test: |char| -> bool) -> String {

        let mut s = String::new();

        loop {
            if self.done() || !test(self.current_char()) {
                break
            } else {
                s.push(self.current_char());
                self.advance();
            }
        }

        s
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
            let mut advanced_already = false;

            let token = if c.is_whitespace() { Combinator(Whitespace) } else {

                match c {
                    '_' | 'a' ... 'z' | 'A' ... 'Z' => {
                        advanced_already = true;
                        Identifier(tokenizer.take_while(|c| {
                            match c {
                                '0' ... '9' | '_' | 'a' ... 'z' | 'A' ... 'Z' => true,
                                _ => false
                            }
                        }))
                    },
                    '-' => Hyphen,
                    '0' ... '9' => {
                        advanced_already = true;
                        Integer(tokenizer.take_while(|c| {
                            match c {
                                '0' ... '9' => true,
                                _ => false
                            }
                        }))
                    },
                    '[' => OpeningBracket,
                    ']' => ClosingBracket,
                    '=' => Equal,
                    '^' => Caret,
                    '$' => DollarSign,
                    '*' => Star,
                    ':' => Colon,
                    '.' => Dot,
                    '#' => HashTag,
                    '(' => OpeningParen,
                    ')' => ClosingParen,
                    '"' => Quote,
                    '>' => Combinator(GreaterThan),
                    '+' => Combinator(Plus),
                    '~' => Combinator(Tilde),
                    _ => {


                        if c == ',' {
                            panic!("Found a \",\" but grouped selectors aren't supported yet!");
                        } else {
                            panic!("Unknown character \"{}\" in \"{}\"", c, tokenizer.input);

                        }
                    }
                }
            };

            tokenizer.token_sink.push(token);

            if !tokenizer.done() && !advanced_already {
                tokenizer.advance();
            }


        }

    }

    tokenizer.token_sink
}
