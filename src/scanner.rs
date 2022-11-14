pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self, source: &[u8]) -> Token {
        self.skip_whitespace(source);

        self.start = self.current;

        if self.is_at_end(source) {
            return self.make_token(TokenType::Eof);
        }

        match self.advance(source) {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'!' => {
                let token_type = if self.accept(source, b'=') { TokenType::BangEqual } else { TokenType::Bang };
                self.make_token(token_type)
            },
            b'=' => {
                let token_type = if self.accept(source, b'=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.make_token(token_type)
            },
            b'<' => {
                let token_type = if self.accept(source, b'=') { TokenType::LessEqual } else { TokenType::Less };
                self.make_token(token_type)
            },
            b'>' => {
                let token_type = if self.accept(source, b'=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.make_token(token_type)
            },
            b'"' => self.string(source),
            c if Scanner::is_alpha(c) => self.identifier(source),
            c if c.is_ascii_digit() => self.number(source),
            _ => self.error_token(),
        }
    }

    fn skip_whitespace(&mut self, source: &[u8]) {
        loop {
            match self.peek(source) {
                Some(b' ' | b'\r' | b'\t') => {
                    self.advance(source);
                },
                Some(b'\n') => {
                    self.line += 1;
                    self.advance(source);
                },
                Some(b'/') if self.peek_next(source) == Some(b'/') => {
                    while self.peek(source) != Some(b'\n') && !self.is_at_end(source) {
                        self.advance(source);
                    }
                },
                _ => {
                    return;
                }
            }
        }
    }

    fn identifier(&mut self, source: &[u8]) -> Token {
        while self.peek(source).filter(|c| Scanner::is_alpha(*c) || c.is_ascii_digit()).is_some() {
            self.advance(source);
        }

        self.make_token(self.identifier_type(source))
    }

    fn identifier_type(&self, source: &[u8]) -> TokenType {
        match source[self.start] {
            b'a' => self.check_keyword(source, 1, "nd", TokenType::And),
            b'c' => self.check_keyword(source, 1, "lass", TokenType::Class),
            b'e' => self.check_keyword(source, 1, "lse", TokenType::Else),
            b'f' if self.current - self.start > 1 => match source[self.start + 1] {
                b'a' => self.check_keyword(source, 2, "lse", TokenType::False),
                b'o' => self.check_keyword(source, 2, "r", TokenType::For),
                b'u' => self.check_keyword(source, 2, "n", TokenType::Fun),
                _ => TokenType::Identifier
            }
            b'i' => self.check_keyword(source, 1, "f", TokenType::If),
            b'n' => self.check_keyword(source, 1, "il", TokenType::Nil),
            b'o' => self.check_keyword(source, 1, "r", TokenType::Or),
            b'p' => self.check_keyword(source, 1, "rint", TokenType::Print),
            b'r' => self.check_keyword(source, 1, "eturn", TokenType::Return),
            b's' => self.check_keyword(source, 1, "uper", TokenType::Super),
            b't' if self.current - self.start > 1 => match source[self.start + 1] {
                b'h' => self.check_keyword(source, 2, "is", TokenType::This),
                b'r' => self.check_keyword(source, 2, "ue", TokenType::True),
                _ => TokenType::Identifier
            }
            b'v' => self.check_keyword(source, 1, "ar", TokenType::Var),
            b'w' => self.check_keyword(source, 1, "hile", TokenType::While),
            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, source: &[u8], start: usize, rest: &'static str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + rest.len() && &source[self.start + start..self.start + start + rest.len()] == rest.as_bytes() {
            return token_type;
        }

        TokenType::Identifier
    }

    fn string(&mut self, source: &[u8]) -> Token {
        while let Some(c) = self.peek(source) {
            if c == b'"' { break; }
            if c == b'\n' {
                self.line += 1;
            }
            self.advance(source);
        }

        if self.is_at_end(source) {
            return self.error_token();
        }

        // The closing quote.
        self.advance(source);
        self.make_token(TokenType::String)
    }

    fn number(&mut self, source: &[u8]) -> Token {
        while self.peek(source).filter(|c| c.is_ascii_digit()).is_some() {
            self.advance(source);
        }

        // Look for a fractional part.
        if self.peek(source) == Some(b'.') && self.peek_next(source).filter(|c| c.is_ascii_digit()).is_some() {
            // Consume the ".".
            self.advance(source);

            while self.peek(source).filter(|c| c.is_ascii_digit()).is_some() {
                self.advance(source);
            }
        }

        self.make_token(TokenType::Number)
    }

    fn advance(&mut self, source: &[u8]) -> u8 {
        self.current += 1;
        source[self.current - 1]
    }

    fn is_alpha(c: u8) -> bool {
        c == b'_' || c.is_ascii_alphabetic()
    }

    fn peek(&self, source: &[u8]) -> Option<u8> {
        if self.is_at_end(source) {
            return None;
        }
        Some(source[self.current])
    }

    fn peek_next(&self, source: &[u8]) -> Option<u8> {
        if self.current + 1 >= source.len() {
            return None;
        }
        Some(source[self.current + 1])
    }

    fn accept(&mut self, source: &[u8], expected: u8) -> bool {
        if self.is_at_end(source) {
            return false;
        }
        if source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self, source: &[u8]) -> bool {
        self.current >= source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            label_start: self.start,
            label_end: self.current,
            line: self.line,
        }
    }

    fn error_token(&self) -> Token {
        Token {
            token_type: TokenType::Error,
            label_start: 0,
            label_end: 0,
            line: self.line,
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub label_start: usize,
    pub label_end: usize,
    pub line: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
    
    Error, Eof
}