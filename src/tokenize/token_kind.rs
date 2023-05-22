#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Number(usize),
    Punc(PuncToken),
    Return,
    If,
    Else,
    For,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PuncToken {
    /// '[' '<:'
    OpenSquare,
    /// ']' ':>'
    CloseSquare,
    /// '('
    OpenRound,
    /// ')'
    CloseRound,
    /// '{' '<%'
    OpenCurly,
    /// '}' '%>'
    CloseCurly,
    /// '.'
    Dot,
    /// '->'
    MinusGt,
    /// '++'
    PlusPlus,
    /// '--'
    MinusMinus,
    /// '&'
    And,
    /// '*'
    Asterisk,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '~'
    Tilde,
    /// '!'
    Excl,
    /// '/'
    Slash,
    /// '%'
    Percent,
    /// '<<'
    LtLt,
    /// '>>'
    GtGt,
    /// '<'
    Lt,
    /// '>'
    Gt,
    /// '<='
    LtEqual,
    /// '>='
    GtEqual,
    /// '=='
    EqualEqual,
    /// '!='
    ExclEqual,
    /// '^'
    Hat,
    /// '|'
    Vert,
    /// '&&'
    AndAnd,
    /// '||'
    VertVert,
    /// '?'
    Question,
    /// ':'
    Colon,
    /// ';'
    Semicolon,
    /// '...'
    DotDotDot,
    /// '='
    Equal,
    /// '*='
    AsteriskEqual,
    /// '/='
    SlashEqual,
    /// '%='
    PercentEqual,
    /// '+='
    PlusEqual,
    /// '-='
    MinusEqual,
    /// '<<='
    LtLtEqual,
    /// '>>='
    GtGtEqual,
    /// '&='
    AndEqual,
    /// '^='
    HatEqual,
    /// '|='
    VertEqual,
    /// ','
    Comma,
}
