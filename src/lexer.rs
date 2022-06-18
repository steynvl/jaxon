pub enum Token {
    // end-of-file
    Eof,
    // identifier
    Id(String),
    // number literal
    Number(i32),
    // string literal
    String(String),

    // keywords
    Array,
    Begin,
    Call,
    Do,
    Else,
    Elsif,
    End,
    False,
    Function,
    Get,
    If,
    Integer,
    Leave,
    Not,
    Put,
    Relax,
    Source,
    Then,
    To,
    True,
    While,

    // relational operators
    Equal,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,
    NotEqual,

    // additive operators
    Minus,
    Or,
    Plus,

    // multiplicative operators
    And,
    Divide,
    Multiply,
    Remainder,

    /* other non-alphabetic operators */
    CloseBracket,
    CloseParenthesis,
    Comma,
    Concatenate,
    Gets,
    OpenBracket,
    OpenParenthesis,
    Semicolon,
}

pub struct Lexer {
    // the next source character
    ch: char,

    // the current index in the source
    index: usize,

    // the current column number
    column_number: usize,
}

impl Lexer {
    pub fn new(bytes: &[u8]) -> Self {
        Lexer {
            ch: ' ',
            index: 0,
            column_number: 0,
        }
    }

    pub fn get_token(&self, token: &mut Token) {}
}
