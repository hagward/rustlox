use std::mem;

use crate::{scanner::*, chunk::{Chunk, OpCode, Value}};

pub struct Compiler {
    source: String,
    chunk: Chunk,
    scanner: Scanner,
    parser: Parser,
}

struct Parser {
    current: Option<Token>,
    previous: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
enum Precedence {
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    Term,        // + -
    Factor,      // * /
    Unary,       // ! -
    Call,        // . ()
    Primary,
}

impl From<u8> for Precedence {
    fn from(precedence: u8) -> Self {
        match precedence {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            _ => panic!("Unknown precedence: {precedence}"),
        }
    }
}

struct ParseRule {
    prefix: Option<fn(&mut Compiler) -> ()>,
    infix: Option<fn(&mut Compiler) -> ()>,
    precedence: Precedence,
}

const RULES: [ParseRule; 40] = [
    ParseRule {prefix: Some(Compiler::grouping), infix: None,                   precedence: Precedence::None},   // TOKEN_LEFT_PAREN
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_RIGHT_PAREN
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_LEFT_BRACE
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_RIGHT_BRACE
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_COMMA
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_DOT
    ParseRule {prefix: Some(Compiler::unary),    infix: Some(Compiler::binary), precedence: Precedence::Term},   // TOKEN_MINUS
    ParseRule {prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Term},   // TOKEN_PLUS
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_SEMICOLON
    ParseRule {prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Factor}, // TOKEN_SLASH
    ParseRule {prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Factor}, // TOKEN_STAR
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_BANG
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_BANG_EQUAL
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_EQUAL
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_EQUAL_EQUAL
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_GREATER
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_GREATER_EQUAL
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_LESS
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_LESS_EQUAL
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_IDENTIFIER
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_STRING
    ParseRule {prefix: Some(Compiler::number),   infix: None,                   precedence: Precedence::None},   // TOKEN_NUMBER
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_AND
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_CLASS
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_ELSE
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_FALSE
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_FOR
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_FUN
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_IF
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_NIL
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_OR
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_PRINT
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_RETURN
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_SUPER
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_THIS
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_TRUE
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_VAR
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_WHILE
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_ERROR
    ParseRule {prefix: None,                     infix: None,                   precedence: Precedence::None},   // TOKEN_EOF
];

impl Compiler {
    pub fn compile(source: String) -> Result<Chunk, u8> {
        let mut compiler = Self {
            source,
            chunk: Chunk::new(),
            scanner: Scanner::new(),
            parser: Parser { current: None, previous: None, had_error: false, panic_mode: false },
        };

        compiler.advance();
        compiler.expression();
        compiler.consume(TokenType::Eof, "Expect end of expression.");
        compiler.end_compiler();

        if compiler.parser.had_error {
            return Err(0);
        }
        Ok(compiler.chunk)
    }

    fn advance(&mut self) {
        self.parser.previous = mem::take(&mut self.parser.current);

        loop {
            let source_bytes = self.source.as_bytes();
            self.parser.current = Some(self.scanner.scan_token(source_bytes));
            if self.parser.current.as_ref().unwrap().token_type != TokenType::Error {
                break;
            }

            self.error_at_current("Something was erroneous.");
        }
    }

    // These error functions really don't play well with the borrow checker. Not sure how to
    // structure it to make it happy.
    fn error_at_current(&mut self, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;

        let token = self.parser.current.as_ref().unwrap();

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            _ => eprint!(" at '{}'", self.format_token(token)),
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }

    fn error(&mut self, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;

        let token = self.parser.previous.as_ref().unwrap();

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}'", self.format_token(&token)),
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }

    fn format_token(&self, token: &Token) -> &str {
        &self.source[token.label_start..token.label_end]
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.as_ref().unwrap().token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.as_ref().unwrap().token_type;
        let rule = self.get_rule(operator_type);
        self.parse_precedence((rule.precedence as u8 + 1).into());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => (), // Unreachable.
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let token = self.parser.previous.as_ref().unwrap();
        let value: Value = self.source[token.label_start..token.label_end].parse().unwrap_or_default();
        self.emit_constant(value);
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.as_ref().unwrap().token_type;

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
          TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
          _ => (), // Unreachable.
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        if let Some(prefix_rule) = self.get_rule(self.parser.previous.as_ref().unwrap().token_type).prefix {
            prefix_rule(self);
        } else {
            self.error("Expect expression.");
            return;
        };

        while precedence <= self.get_rule(self.parser.current.as_ref().unwrap().token_type).precedence {
            self.advance();
            if let Some(infix_rule) = self.get_rule(self.parser.previous.as_ref().unwrap().token_type).infix {
                infix_rule(self);
            }
        }
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        &RULES[token_type as usize]
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        if constant > u8::MAX as usize {
            self.error("Too many constants in one chunk.");
            return 0;
        }
        constant as u8
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte, self.parser.previous.as_ref().unwrap().line);
    }
}
