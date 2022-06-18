#[derive(Debug, PartialEq)]
pub enum Token {
    // end-of-file
    Eof,
    // identifier
    Id(String),
    // number literal
    Number(i32),
    // string literal
    StringLiteral(String),

    // keywords
    Array,
    Begin,
    Boolean,
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

// the maximum length of an identifier
pub const MAX_ID_LENGTH: usize = 32;

// an array of reserved keywords and its corresponding Token. The array should
// be sorted since Lexer::process_word performs binary search on the array.
pub const RESERVED_WORDS: &'static [(&str, Token)] = &[
    ("and", Token::And),
    ("array", Token::Array),
    ("begin", Token::Begin),
    ("boolean", Token::Boolean),
    ("call", Token::Call),
    ("do", Token::Do),
    ("else", Token::Else),
    ("elsif", Token::Elsif),
    ("end", Token::End),
    ("false", Token::False),
    ("function", Token::Function),
    ("get", Token::Get),
    ("if", Token::If),
    ("integer", Token::Integer),
    ("leave", Token::Leave),
    ("not", Token::Not),
    ("or", Token::Or),
    ("put", Token::Put),
    ("relax", Token::Relax),
    ("rem", Token::Remainder),
    ("source", Token::Source),
    ("then", Token::Then),
    ("to", Token::To),
    ("true", Token::True),
    ("while", Token::While),
];
