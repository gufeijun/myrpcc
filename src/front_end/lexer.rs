use crate::{Token, TokenKind};
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Lexer {
    pub line: u32,
    pub column: u32,
    pub cursor: usize,
    pub src_file: String, // filename
    src_data: String,     // file data
    ahead_token: Option<Rc<Token>>,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(src_file: String) -> Result<Lexer, io::Error> {
        let mut src_data = String::new();
        File::open(&src_file)?.read_to_string(&mut src_data)?;
        Ok(Lexer {
            line: 1,
            column: 0,
            cursor: 0,
            src_file,
            src_data,
            ahead_token: None,
        })
    }
    pub fn look_ahead(&mut self) -> Option<Rc<Token>> {
        if let None = &self.ahead_token {
            self.ahead_token = self.get_next_token();
        }
        match &self.ahead_token {
            None => None,
            Some(v) => Some(Rc::clone(v)),
        }
    }
    pub fn next_token(&mut self) -> Option<Rc<Token>> {
        match &self.ahead_token {
            None => self.get_next_token(),
            Some(v) => {
                let vv = Some(Rc::clone(v));
                self.ahead_token = None;
                vv
            }
        }
    }
    fn get_next_token(&mut self) -> Option<Rc<Token>> {
        let result = loop {
            self.skip_white_spaces();
            let cur_char = self.look_ahead_n_bytes(0)?;
            break match cur_char {
                b'{' => Some(self.gen_token(TokenKind::SepLCurly, 1)),
                b'}' => Some(self.gen_token(TokenKind::SepRCurly, 1)),
                b'(' => Some(self.gen_token(TokenKind::SepLParen, 1)),
                b')' => Some(self.gen_token(TokenKind::SepRParen, 1)),
                b',' => Some(self.gen_token(TokenKind::SepComma, 1)),
                b';' => Some(self.gen_token(TokenKind::SepSemi, 1)),
                b'=' => Some(self.gen_token(TokenKind::SepEq, 1)),
                b'<' => Some(self.gen_token(TokenKind::SepLt, 1)),
                b'>' => Some(self.gen_token(TokenKind::SepGt, 1)),
                b'"' | b'`' => self.scan_string_literal(),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.scan_identifier(),
                b'0'..=b'9' => self.scan_integer(),
                b'/' => {
                    self.skip_comment();
                    continue;
                }
                _ => {
                    panic!("unexpected character: {}", cur_char);
                }
            };
        };
        self.forward(1);
        result
    }

    // TODO
    fn scan_string_literal(&mut self) -> Option<Rc<Token>> {
        None
    }

    fn scan_identifier(&mut self) -> Option<Rc<Token>> {
        let len = self
            .src_data
            .as_bytes()
            .iter()
            .skip(self.cursor)
            .enumerate()
            .find(|&(_, &ch)| !ch.is_ascii_alphabetic() && ch != b'_');
        let len = match len {
            None => self.src_data.len(),
            Some((len, _)) => len,
        };
        let identifier = self.src_data[self.cursor..len + self.cursor].to_string();
        let token = self.gen_token(TokenKind::STRING(identifier), len);
        self.forward(len - 1);
        Some(token)
    }
    fn scan_integer(&mut self) -> Option<Rc<Token>> {
        let len = self
            .src_data
            .as_bytes()
            .iter()
            .skip(self.cursor)
            .enumerate()
            .find(|&(_, &ch)| !ch.is_ascii_digit());
        let len = match len {
            None => self.src_data.len(),
            Some((len, _)) => len,
        };
        let integer = self.src_data[self.cursor..len + self.cursor]
            .parse()
            .unwrap(); // TODO
        let token = self.gen_token(TokenKind::INTEGER(integer), len);
        self.forward(len - 1);
        Some(token)
    }

    fn skip_comment(&mut self) {
        match self.look_ahead_n_bytes(1) {
            Some(b'/') => self.forward(2),
            _ => panic!("mismatched '\'"), // TODO
        }
        let mut k = 0;
        loop {
            let ahead = self.look_ahead_n_bytes(k);
            match ahead {
                None | Some(b'\n' | b'\r') => break self.forward(k),
                _ => (),
            }
            k += 1;
        }
    }
    fn forward(&mut self, n: usize) {
        self.cursor += n;
        self.column += n as u32;
    }
    fn gen_token(&self, kind: TokenKind, content_length: usize) -> Rc<Token> {
        Rc::new(Token {
            line: self.line,
            column: self.column,
            content: self.src_data[self.cursor..self.cursor + content_length].to_string(),
            kind,
        })
    }
    fn look_ahead_n_bytes(&self, n: usize) -> Option<u8> {
        let ch = self.src_data.as_bytes().get(self.cursor + n)?;
        Some(*ch)
    }
    fn skip_white_spaces(&mut self) {
        for (idx, &ch) in self
            .src_data
            .as_bytes()
            .iter()
            .skip(self.cursor)
            .enumerate()
        {
            match ch {
                b'\n' => {
                    self.column = 0;
                    self.line += 1;
                    self.cursor += 1;
                }
                b'\r' => {
                    if let Some(next_ch) = self.look_ahead_n_bytes(1) {
                        let mut steps = 1usize;
                        if next_ch == b'\n' {
                            steps += 1;
                        }
                        self.column = 0;
                        self.line += 1;
                        self.cursor = idx + steps;
                    } else {
                        // eof
                        self.cursor = idx + 1;
                    }
                }
                b' ' | b'\t' => {
                    self.column += 1;
                    self.cursor += 1;
                }
                _ => return,
            }
        }
    }
}

// cargo test -- --nocapture   可以输出终端信息
#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    pub fn new_lexer() {
        let res = Lexer::new("doc/bnf.txt".to_string()).unwrap();
        println!("{:?}", res)
    }
    #[test]
    pub fn show_tokens() {
        let mut lex = Lexer::new("./test.txt".to_string()).unwrap();
        while let Some(v) = lex.get_next_token() {
            println!("{:?}", *v);
        }
    }
}
