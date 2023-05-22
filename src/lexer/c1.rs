use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // Whitespace
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Whitespace,

    // KEYWORDS
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,
    #[token("while")]
    KwWhile,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("==")]
    Eq,
    #[token("=")]
    Assign,
    #[token("!=")]
    Neq,

    // Operators
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    // Other simple tokens
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    /*
        Digit: [0-9]
        Integer: [0-9]+
        Float: (([0-9]+.[0-9]+)|(.[0-9]+))
        Letter: [a-zA-Z]

        ConstFloat expanded:
            (
                (([0-9]+\\.[0-9]+)|(\\.[0-9]+))
                ([eE] ([+-])? [0-9]+)?
            )|(
                [0-9]+ [eE] ([-+])? [0-9]+
            )
    */

    // Termvariablen
    #[regex("[0-9]+")]
    ConstInt,
    #[regex("((([0-9]+\\.[0-9]+)|(\\.[0-9]+))([eE]([+-])?[0-9]+)?)|([0-9]+[eE]([-+])?[0-9]+)")]
    ConstFloat,
    #[regex("true|false")]
    ConstBoolean,
    #[regex(r#""([^"\\\n]|\\.)*""#)]
    ConstString,
    #[regex("[a-zA-Z]+([a-zA-Z0-9])*")]
    Id,

    // Skip
    #[regex(r"/\*([^*]|(\*[^/]))*\*/", logos::skip)]
    CPPComment,
    #[regex(r#"//[^\n]*"#, logos::skip)]
    CComment,
    #[error]
    Error,
}