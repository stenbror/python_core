use crate::parser::trivia;

#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
    /* Special tokens */
    Eof
    {
        position: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Newline
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Indent
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Dedent
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Typecomment,

    /* Reserved keywords */
    False
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    None
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    True
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    And
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    As
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Assert
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Async
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Await
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Break
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Case
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Class
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Continue
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Def
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Del
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Elif
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Else
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Except
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Finally
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    For
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    From
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Global
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    If
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Import
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    In
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Is
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Lambda
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Match
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Nonlocal
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Not
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Or
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Pass
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Raise
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Return
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Try
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Type
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    While
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    With
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Yield
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },

    /* Operators */
    Plus
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    }, 
    Minus
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Star
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Power
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Slash
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    SlashSlash
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Modulo
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Matrice
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    ShiftLeft
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    ShiftRight
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitAnd
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitOr
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitXor
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitInvert
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    ColonAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Less
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Greater
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    LessEqual
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    GreaterEqual
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Equal
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    NotEqual
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },

    /* Delimiters */
    LeftParen
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    RightParen
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    LeftBracket
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    RightBracket
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    LeftCurly
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    RightCurly
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Comma
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Colon
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Bang
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Period
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    SemiColom
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Assign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    Arrow
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    PlusAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    MinusAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    StarAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    SlashAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    SlashSlashAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    ModuloAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    MatriceAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitAndAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitOrAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitXorAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitShiftLeft
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    BitShiftRight
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },
    PowerAssign
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>
    },

    /* Literals */
    Name
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>,
        content: String
    },
    Number
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>,
        content: String
    },
    String
    {
        start: u32,
        end: u32,
        column: u32,
        line: u32,
        trivia: Vec<Box<trivia::Trivia>>,
        content: String
    }
}