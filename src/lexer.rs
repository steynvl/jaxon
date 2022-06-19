use crate::error::SourcePosition;
use crate::token::{create_token_from_reserved_words_index, Token, MAX_ID_LENGTH, RESERVED_WORDS};

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

        println!(
            "{} / {} [{}]",
            self.index,
            self.bytes.len(),
            self.bytes[self.index] as char
        );
        if self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.process_word(token);
        } else if self.ch.is_ascii_digit() {
            self.process_number(token);
        } else {
        }
    }

    fn next_char(&mut self) {
        self.index += 1;
        self.ch = self.bytes[self.index];

        self.column_number += 1;
        if self.last_read == b'\n' {
            self.source_position.line += 1;
            self.column_number = 1;
        }
        self.last_read = self.ch;
    }

    fn has_next_char(&self) -> bool {
        self.index + 1 < self.bytes.len()
    }

    fn process_number(&mut self, token: &mut Token) {
        let mut final_value = 0;

        self.source_position.col = self.column_number;

        while self.ch.is_ascii_digit() {
            let digit = (self.ch as char)
                .to_digit(10)
                .expect("Could not convert to a digit") as i32;

            if final_value <= ((i32::MAX - digit) / 10) {
                final_value = final_value * 10 + digit;

                if !self.has_next_char() {
                    break;
                }

                self.next_char();
            } else {
                // TODO: abort compile
                panic!("number too large");
            }
        }

        *token = Token::Number(final_value);
    }

    fn process_word(&mut self, token: &mut Token) {
        self.source_position.col = self.column_number;

        let start = self.index;
        let mut id_length = 0;

        let is_alphanum_or_lodash = |ch: &u8| ch.is_ascii_alphanumeric() || *ch == b'_';

        loop {
            self.next_char();
            id_length += 1;

            if !(is_alphanum_or_lodash(&self.ch) && id_length < MAX_ID_LENGTH) {
                break;
            }

            if !self.has_next_char() {
                break;
            }
        }

        if is_alphanum_or_lodash(&self.ch) && id_length == MAX_ID_LENGTH {
            // TODO: abort compile
            panic!("identifier too long");
        }

        let lexeme = match std::str::from_utf8(&self.bytes[start..start + id_length]) {
            Ok(value) => value,
            Err(err) => panic!("Invalid UTF-8 sequence {}", err),
        };

        match RESERVED_WORDS.binary_search_by_key(&lexeme, |(raw_str, _)| raw_str) {
            Ok(index) => *token = create_token_from_reserved_words_index(index),
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
        self.ch = self.bytes[self.index];
    }
}
