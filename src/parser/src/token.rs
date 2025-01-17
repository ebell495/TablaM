use corelib::derive_more::Display;
use corelib::prelude::*;

use corelib::text_size::TextRange;
use logos::{Lexer, Logos};

#[derive(Debug, Clone, Copy)]
pub struct ExtrasLexer {
    pub current_line: usize,
    pub current_initial_column: usize,
}

impl Default for ExtrasLexer {
    fn default() -> Self {
        ExtrasLexer {
            current_line: 1,
            current_initial_column: 0,
        }
    }
}

fn increase_current_line(lexer: &mut Lexer<Syntax>) {
    //When a line-feed happens, it reset the position of the "column"
    lexer.extras.current_line += 1;
    lexer.extras.current_initial_column = lexer.span().end;
}

/// Classify the kind of syntax for the parse, so it knows when to apply precedence...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    Infix,
    Prefix,
    Postfix,
    Open,
    Close,
    Trivia,
    Kw,
    Atom,
    Ident,
    Err,
    Root,
    Eof,
}

// *IMPORTANT*: Coordinate with the syntax definitions on /tools
//TODO: For ideas for the lexer: https://github.com/YoloDev/yolodev-jsonnet/blob/master/crates/lex/src/lib.rs
//
//TODO: Support count lines collapsing many CR?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Logos)]
#[logos(extras = ExtrasLexer)]
#[logos(subpattern decimal = r"\d+_\d+|\d+")]
#[repr(u16)]
pub enum Syntax {
    //trivia
    #[token("\n", increase_current_line)]
    Cr,
    #[regex(r"[ \t\f]+")]
    Whitespace,
    #[regex(r"--.*")]
    Comment,

    //literals
    #[regex("true|false")]
    Bool,
    #[regex(r"(?&decimal)")]
    Integer,
    #[regex(r"(?&decimal)\.(?&decimal)f")]
    Float,
    #[regex(r"(?&decimal)\.(?&decimal)d")]
    #[regex(r"(?&decimal)\.(?&decimal)")]
    Decimal,
    #[regex(r#"d"[^"]*""#)]
    #[regex(r#"d'[^']*'"#)]
    Date,
    #[regex(r#"t"[^"]*""#)]
    #[regex(r#"t'[^']*'"#)]
    Time,
    #[regex(r#"dt"[^"]*""#)]
    #[regex(r#"dt'[^']*'"#)]
    DateTime,
    // Strings, capture with both single and double quote
    #[regex(r#""[^"]*""#)]
    #[regex(r#"'[^']*'"#)]
    String,

    //keywords
    #[display(fmt = "fun")]
    #[token("fun")]
    FnKw,

    #[display(fmt = "let")]
    #[token("let")]
    LetKw,

    #[display(fmt = "var")]
    #[token("var")]
    VarKw,

    #[display(fmt = "if")]
    #[token("if")]
    IfKw,

    #[display(fmt = "else")]
    #[token("else")]
    ElseKw,

    #[display(fmt = "do")]
    #[token("do")]
    DoKw,

    #[display(fmt = "end")]
    #[token("end")]
    EndKw,

    //idents
    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    //OPS

    //Punctuation
    #[display(fmt = ".")]
    #[token(".")]
    Point,

    #[display(fmt = ",")]
    #[token(",")]
    Comma,

    #[display(fmt = ";")]
    #[token(";")]
    Semicolon,

    #[display(fmt = "?")]
    #[token("?")]
    Question,

    //Math
    #[display(fmt = "+")]
    #[token("+")]
    Plus,
    Add, // Plus get turned Add only in parser

    #[display(fmt = "-")]
    #[token("-")]
    Minus,
    Neg, // Minus get turned Neg only in parser

    #[display(fmt = "*")]
    #[token("*")]
    Star,

    #[display(fmt = "/")]
    #[token("/")]
    Slash,

    //Logic
    #[display(fmt = ":=")]
    #[token(":=")]
    Assign,

    #[display(fmt = "=")]
    #[token("=")]
    Equals,

    #[display(fmt = "!=")]
    #[token("!=")]
    NotEquals,

    #[display(fmt = "<")]
    #[token("<")]
    Less,
    #[display(fmt = "<=")]
    #[token("<=")]
    LessThan,

    #[display(fmt = ">")]
    #[token(">")]
    Greater,
    #[display(fmt = ">=")]
    #[token(">=")]
    GreaterThan,

    #[display(fmt = "and")]
    #[token("and")]
    AndKw,

    #[display(fmt = "or")]
    #[token("or")]
    OrKw,

    #[display(fmt = "not")]
    #[token("not")]
    NotKw,

    //Grouping
    #[display(fmt = "(")]
    #[token("(")]
    LParen,

    #[display(fmt = ")")]
    #[token(")")]
    RParen,

    #[display(fmt = "{{")]
    #[token("{")]
    LBrace,

    #[display(fmt = "}}")]
    #[token("}")]
    RBrace,

    #[display(fmt = "[")]
    #[token("[")]
    LSquare,

    #[display(fmt = "]")]
    #[token("]")]
    RSquare,

    //Markers, not represent code!
    Root,
    Eof,
    #[error]
    Error,
}

impl Syntax {
    pub fn is(self) -> SyntaxKind {
        match self {
            Syntax::Cr | Syntax::Whitespace | Syntax::Comment => SyntaxKind::Trivia,
            Syntax::Bool
            | Syntax::Integer
            | Syntax::Float
            | Syntax::Decimal
            | Syntax::String
            | Syntax::Date
            | Syntax::Time
            | Syntax::DateTime => SyntaxKind::Atom,
            Syntax::Ident => SyntaxKind::Atom,
            Syntax::FnKw
            | Syntax::LetKw
            | Syntax::VarKw
            | Syntax::IfKw
            | Syntax::ElseKw
            | Syntax::DoKw
            | Syntax::EndKw => SyntaxKind::Kw,
            Syntax::Point
            | Syntax::Assign
            | Syntax::Question
            | Syntax::Plus
            | Syntax::Minus
            | Syntax::Star
            | Syntax::Slash
            | Syntax::Comma
            | Syntax::Semicolon
            | Syntax::Equals
            | Syntax::NotEquals
            | Syntax::Less
            | Syntax::LessThan
            | Syntax::Greater
            | Syntax::GreaterThan
            | Syntax::AndKw
            | Syntax::OrKw
            | Syntax::NotKw => SyntaxKind::Infix,
            Syntax::Neg | Syntax::Add => SyntaxKind::Prefix,
            Syntax::LParen | Syntax::LBrace | Syntax::LSquare => SyntaxKind::Open,
            Syntax::RParen | Syntax::RBrace | Syntax::RSquare => SyntaxKind::Close,
            Syntax::Error => SyntaxKind::Err,
            Syntax::Eof => SyntaxKind::Eof,
            Syntax::Root => SyntaxKind::Root,
        }
    }
    pub fn is_head_tree(self) -> bool {
        self.is() != SyntaxKind::Atom
    }

    pub fn to_bin_op(self) -> Option<BinaryOp> {
        let res = match self {
            Syntax::Plus => BinaryOp::Add,
            Syntax::Minus => BinaryOp::Sub,
            Syntax::Star => BinaryOp::Mul,
            Syntax::Slash => BinaryOp::Div,
            _ => return None,
        };
        Some(res)
    }
    pub fn to_unary_op(self) -> Option<UnaryOp> {
        let res = match self {
            Syntax::Neg => UnaryOp::Neg,
            _ => return None,
        };
        Some(res)
    }

    pub fn is_op(self) -> bool {
        self.to_unary_op().is_some() || self.to_bin_op().is_some() || matches!(self, Self::Point)
    }

    pub fn to_separator(self) -> SepOp {
        match self {
            Syntax::Comma => SepOp::Comma,
            Syntax::Semicolon => SepOp::Semicolon,
            _ => unreachable!("{}", self),
        }
    }
    pub fn is_var_let(self) -> bool {
        matches!(self, Self::LetKw | Self::VarKw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Equals,
    NotEquals,
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SepOp {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub file_id: FileId,
    pub id: TokenId,
    pub kind: Syntax,
    pub range: TextRange,
    pub line: u32,
    pub col: u32,
}

impl Token {
    pub fn range_tokens(tokens: &[Token]) -> TextRange {
        let min = tokens.first().map(|x| x.range.start());
        let max = tokens.last().map(|x| x.range.start() + x.range.len());

        TextRange::new(min.unwrap_or_default(), max.unwrap_or_default())
    }
}

//#[cfg(test)]
pub(crate) fn token_eof() -> Token {
    Token {
        file_id: FileId::from_index(0),
        id: TokenId(0),
        kind: Syntax::Eof,
        range: Default::default(),
        line: 0,
        col: 0,
    }
}

#[cfg(test)]
pub(crate) fn token_test() -> Token {
    Token {
        file_id: FileId::from_index(0),
        id: TokenId(0),
        kind: Syntax::Error,
        range: Default::default(),
        line: 0,
        col: 0,
    }
}
