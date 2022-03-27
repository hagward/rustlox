pub struct Scanner<'a> {
    source_str: &'a str,
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source_str: source,
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let token_type = if self.accept('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.make_token(token_type)
            },
            '=' => {
                let token_type = if self.accept('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.make_token(token_type)
            },
            '<' => {
                let token_type = if self.accept('=') { TokenType::LessEqual } else { TokenType::Less };
                self.make_token(token_type)
            },
            '>' => {
                let token_type = if self.accept('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.make_token(token_type)
            },
            '"' => self.string(),
            c if Scanner::is_alpha(c) => self.identifier(),
            c if c.is_ascii_digit() => self.number(),
            _ => self.error_token(),
        }
    }
    
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {
                    self.advance();
                },
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                },
                Some('/') if self.peek_next() == Some('/') => {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                },
                _ => {
                    return;
                }
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().filter(|c| Scanner::is_alpha(*c) || c.is_ascii_digit()).is_some() {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_keyword(1, "nd", TokenType::And),
            'c' => self.check_keyword(1, "lass", TokenType::Class),
            'e' => self.check_keyword(1, "lse", TokenType::Else),
            'f' if self.current - self.start > 1 => match self.source[self.start + 1] {
                'a' => self.check_keyword(2, "lse", TokenType::False),
                'o' => self.check_keyword(2, "r", TokenType::For),
                'u' => self.check_keyword(2, "n", TokenType::Fun),
                _ => TokenType::Identifier
            }
            'i' => self.check_keyword(1, "f", TokenType::If),
            'n' => self.check_keyword(1, "il", TokenType::Nil),
            'o' => self.check_keyword(1, "r", TokenType::Or),
            'p' => self.check_keyword(1, "rint", TokenType::Print),
            'r' => self.check_keyword(1, "eturn", TokenType::Return),
            's' => self.check_keyword(1, "uper", TokenType::Super),
            't' if self.current - self.start > 1 => match self.source[self.start + 1] {
                'h' => self.check_keyword(2, "is", TokenType::This),
                'r' => self.check_keyword(2, "ue", TokenType::True),
                _ => TokenType::Identifier
            }
            'v' => self.check_keyword(1, "ar", TokenType::Var),
            'w' => self.check_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, rest: &'static str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + rest.len() && &self.source_str[self.start + start..self.start + start + rest.len()] == rest {
            return token_type;
        }

        TokenType::Identifier
    }

    fn string(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c == '"' { break; }
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token();
        }

        // The closing quote.
        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while self.peek().filter(|c| c.is_ascii_digit()).is_some() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == Some('.') && self.peek_next().filter(|c| c.is_ascii_digit()).is_some() {
            // Consume the ".".
            self.advance();

            while self.peek().filter(|c| c.is_ascii_digit()).is_some() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn is_alpha(c: char) -> bool {
        c == '_' || c.is_ascii_alphabetic()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source[self.current])
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        Some(self.source[self.current + 1])
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn accept(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
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

#[derive(Debug, PartialEq, Eq)]
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