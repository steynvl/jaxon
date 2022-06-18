#[derive(Debug, PartialEq)]
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
