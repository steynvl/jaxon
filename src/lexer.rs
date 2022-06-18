use crate::error::SourcePosition;
use crate::token::{Token, MAX_ID_LENGTH, RESERVED_WORDS};

pub struct Lexer<'a> {
    // the source file contents
    bytes: &'a [u8],

    // the next character in the source
    ch: u8,

    // the last character that was read
    last_read: u8,

    // the current index in the source
    index: usize,

    // the current column number
    column_number: usize,

    // a place (or position) in the source file
    source_position: SourcePosition,
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Lexer {
            bytes: bytes,
            ch: bytes[0],
            last_read: bytes[0],
            index: 0,
            column_number: 0,
            source_position: SourcePosition::default(),
        }
    }

    pub fn get_token(&mut self, token: &mut Token) {
        self.skip_whitespace();

        // remember token start
        self.source_position.col = self.column_number;

        if !self.has_next_char() {
            *token = Token::Eof;
            return;
        }

        if self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.process_word(token);
        } else if self.ch.is_ascii_digit() {
            todo!("process_number()")
        } else {
        }
    }

    fn next_char(&mut self) {
        self.ch = self.bytes[self.index];
        self.index += 1;

        self.column_number += 1;
        if self.last_read == b'\n' {
            self.source_position.line += 1;
            self.column_number = 1;
        }
        self.last_read = self.ch;
    }

    fn has_next_char(&self) -> bool {
        self.index < self.bytes.len()
    }

    fn process_word(&mut self, token: &mut Token) {
        self.source_position.col = self.column_number;

        let start = self.index;
        let mut i = 0;

        let is_alphanum_or_lodash = |ch: &u8| -> bool { ch.is_ascii_alphanumeric() || *ch == b'_' };

        while is_alphanum_or_lodash(&self.ch) && i < MAX_ID_LENGTH {
            i += 1;
            self.next_char();

            if !self.has_next_char() {
                break;
            }
        }

        if is_alphanum_or_lodash(&self.ch) && i == MAX_ID_LENGTH {
            // TODO: abort compile
            panic!("identifier too long");
        }

        let lexeme = match std::str::from_utf8(&self.bytes[start..start + i]) {
            Ok(value) => value,
            Err(err) => panic!("Invalid UTF-8 sequence {}", err),
        };

        match RESERVED_WORDS.binary_search_by_key(&lexeme, |(raw_str, _)| raw_str) {
            // TODO: way to index in RESERVED_WORDS and create the specific token
            Ok(index) => *token = Token::Function,
            Err(_) => *token = Token::Id(String::from(lexeme)),
        };
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.bytes.len() {
            if self.bytes[self.index] == b' ' {
                self.next_char();
            } else {
                break;
            }
        }
    }
}
