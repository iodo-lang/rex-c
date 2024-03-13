/*
 * @author: ruka-lang
 * @created: 2024-02-28
 */

use crate::prelude::*;

use std::sync::Arc;

/// Contains a token's type and position, and file it belongs to
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub file: Arc<str>,
    pub pos: Position
}

impl Token {
    /// Creates a new Token
    ///
    /// # Arguments
    /// * `kind` -
    /// * `file` -
    /// * `pos` -
    ///
    /// # Returns
    /// * A token, of type kind, belonging to file, at pos
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(kind: Kind, file: Arc<str>, pos: Position) -> Self {
        Self {
            kind,
            file,
            pos
        }
    }
}

/// Represents the type of a token
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Tag(Arc<str>),
    Keyword(Keyword),
    Mode(Mode),
    String(Box<str>),
    Char(char),
    Regex(Box<str>),
    Integer(Box<str>),
    Float(Box<str>),
    // Assignment
    Assign,               // =
    AssignExp,            // :=
    // Punctuation
    Dot,                  // .
    Comma,                // ,
    LeftParen,            // (
    RightParen,           // )
    LeftBracket,          // [
    RightBracket,         // ]
    LeftSquirly,          // {
    RightSquirly,         // }
    SingleQuote,          // '
    DoubleQuote,          // "
    Backtick,             // `
    Backslash,            // \
    Colon,                // :
    Semicolon,            // ;
    Arrow,                // ->
    WideArrow,            // =>
    // Operators
    Address,              // @
    Cash,                 // $
    Pound,                // #
    Bang,                 // !
    Question,             // ?
    RangeExc,             // ..
    RangeInc,             // ..=
    ForwardApp,           // |>
    ReverseApp,           // <|
    Concat,               // <>
    // Arithmetic
    Plus,                 // +
    Minus,                // -
    Asterisk,             // *
    Slash,                // /
    Percent,              // %
    Increment,            // ++
    Decrement,            // --
    Power,                // **
    // Bitwise
    Ampersand,            // &
    Pipe,                 // |
    Caret,                // ^
    Tilde,                // ~
    LeftShift,            // <<
    RightShift,           // >>
    // Comparators
    Lesser,               // <
    LesserEq,             // <=
    Greater,              // >
    GreaterEq,            // >=
    Equal,                // ==
    NotEqual,             // !=
    // Others
    Newline,              // '\n'
    Illegal,
    Eof
}

impl Kind {
    /// Converts a char into it's corresponding Kind
    ///
    /// # Arguments
    /// * `ch` -
    ///
    /// # Returns
    /// * A Kind, whose value will be Illegal if the char is not supported
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn from_char(ch: char) -> Kind {
        match ch {
            '=' => Kind::Assign,

            '.'  => Kind::Dot,
            ','  => Kind::Comma,
            '('  => Kind::LeftParen,
            ')'  => Kind::RightParen,
            '['  => Kind::LeftBracket,
            ']'  => Kind::RightBracket,
            '{'  => Kind::LeftSquirly,
            '}'  => Kind::RightSquirly,
            '\'' => Kind::SingleQuote,
            '"'  => Kind::DoubleQuote,
            '`'  => Kind::Backtick,
            '\\' => Kind::Backslash,
            ':'  => Kind::Colon,
            ';'  => Kind::Semicolon,

            '@'  => Kind::Address,
            '$'  => Kind::Cash,
            '#'  => Kind::Pound,
            '!'  => Kind::Bang,
            '?'  => Kind::Question,

            '+'  => Kind::Plus,
            '-'  => Kind::Minus,
            '*'  => Kind::Asterisk,
            '/'  => Kind::Slash,
            '%'  => Kind::Percent,

            '&'  => Kind::Ampersand,
            '|'  => Kind::Pipe,
            '^'  => Kind::Caret,
            '~'  => Kind::Tilde,

            '<'  => Kind::Lesser,
            '>'  => Kind::Greater,

            '\n' => Kind::Newline,
            '\0' => Kind::Eof,
            _    => Kind::Illegal
        }
    }

    /// Tries to create a Kind from a string representing a compound operator
    ///
    /// # Arguments
    /// * `str` -
    ///
    /// # Returns
    /// * An optional Kind
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn try_from_str(str: &str) -> Option<Kind> {
        match str {
            ":="  => Some(Kind::AssignExp),

            "->"  => Some(Kind::Arrow),
            "=>"  => Some(Kind::WideArrow),

            ".."  => Some(Kind::RangeExc),
            "..=" => Some(Kind::RangeInc),
            "|>"  => Some(Kind::ReverseApp),
            "<|"  => Some(Kind::ForwardApp),
            "<>"  => Some(Kind::Concat),

            "++"  => Some(Kind::Increment),
            "--"  => Some(Kind::Decrement),
            "**"  => Some(Kind::Power),

            "<<"  => Some(Kind::LeftShift),
            ">>"  => Some(Kind::RightShift),

            "<="  => Some(Kind::LesserEq),
            ">="  => Some(Kind::GreaterEq),
            "=="  => Some(Kind::NotEqual),
            "!="  => Some(Kind::Equal),

            _     => None
        }

    }

    /// Tries to create a Kind from a string representing a keyword
    ///
    /// # Arguments
    /// * `str` -
    ///
    /// # Returns
    /// * An optional Kind
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn try_keyword(str: &str) -> Option<Kind> {
        use Keyword::*;
        match str {
            "const"  => Some(Kind::Keyword(Const)),
            "let"  => Some(Kind::Keyword(Let)),
            "pub"  => Some(Kind::Keyword(Pub)),
            "return" => Some(Kind::Keyword(Return)),
            "do"  => Some(Kind::Keyword(Do)),
            "end"  => Some(Kind::Keyword(End)),
            "record"  => Some(Kind::Keyword(Record)),
            "variant"  => Some(Kind::Keyword(Variant)),
            "use"  => Some(Kind::Keyword(Use)),
            "interface"  => Some(Kind::Keyword(Interface)),
            "module"  => Some(Kind::Keyword(Module)),
            "defer"  => Some(Kind::Keyword(Defer)),
            "true"  => Some(Kind::Keyword(True)),
            "false"  => Some(Kind::Keyword(False)),
            "for"  => Some(Kind::Keyword(For)),
            "while"  => Some(Kind::Keyword(While)),
            "break"  => Some(Kind::Keyword(Break)),
            "continue"  => Some(Kind::Keyword(Continue)),
            "match"  => Some(Kind::Keyword(Match)),
            "if"  => Some(Kind::Keyword(If)),
            "else"  => Some(Kind::Keyword(Else)),
            "and"  => Some(Kind::Keyword(And)),
            "or"  => Some(Kind::Keyword(Or)),
            "not"  => Some(Kind::Keyword(Not)),
            "inline"  => Some(Kind::Keyword(Inline)),
            "test"  => Some(Kind::Keyword(Test)),
            "fn"  => Some(Kind::Keyword(Fn)),
            "in"  => Some(Kind::Keyword(In)),
            // Reserved
            "private"  => Some(Kind::Keyword(Private)),
            "derive"  => Some(Kind::Keyword(Derive)),
            "static"  => Some(Kind::Keyword(Static)),
            "macro"  => Some(Kind::Keyword(Macro)),
            "from"  => Some(Kind::Keyword(From)),
            "impl"  => Some(Kind::Keyword(Impl)),
            "when"  => Some(Kind::Keyword(When)),
            "any"  => Some(Kind::Keyword(Any)),
            "as"  => Some(Kind::Keyword(As)),

            _     => None
        }
    }

    /// Tries to create a Kind from a string representing a mode
    ///
    /// # Arguments
    /// * `str` -
    ///
    /// # Returns
    /// * An optional Kind
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn try_mode(str: &str) -> Option<Kind> {
        use Mode::*;
        match str {
            "comptime"  => Some(Kind::Mode(Comptime)),
            "mut"  => Some(Kind::Mode(Mut)),
            "mov"  => Some(Kind::Mode(Mov)),
            "loc"  => Some(Kind::Mode(Loc)),

            _     => None
        }
    }

    /// Converts a Kind to a string slice
    ///
    /// # Arguments
    ///
    /// # Returns
    /// * A string slice
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn to_str(&self) -> &str {
        todo!()
    }
}

/// Enumeration of the keywords supported
#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Const,
    Let,
    Pub,
    Return,
    Do,
    Begin,
    End,
    Record,
    Variant,
    Interface,
    Module,
    Defer,
    True,
    False,
    For,
    While,
    Break,
    Continue,
    Match,
    If,
    Else,
    And,
    Or,
    Not,
    Inline,
    Test,
    Fn,
    In,
    // Reserved
    Private,
    Derive,
    Static,
    Macro,
    From,
    Impl,
    When,
    Any,
    Use,
    As
}

impl Keyword {
    /// Converts a Keyword to a string slice
    ///
    /// # Arguments
    ///
    /// # Returns
    /// * A string slice
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn to_str(&self) -> &str {
        todo!()
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Comptime,
    Mut,
    Mov,
    Loc
}

impl Mode {
    /// Converts a Mode to a string slice
    ///
    /// # Arguments
    ///
    /// # Returns
    /// * A string slice
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn to_str(&self) -> &str {
        todo!()
    }
}
