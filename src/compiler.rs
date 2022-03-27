use crate::scanner::*;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:?} '{}'", token.token_type, &source[token.label_start..token.label_end]);

        if token.token_type == TokenType::Eof {
            break;
        }
    }
}