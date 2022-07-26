use crate::env::Env;
use corelib::chrono::format::Parsed;
use corelib::prelude::{Scalar, Span};
use parser::ast::Ast;
use tablam_parser::parser::Parsed;

pub type CodeEx = Box<dyn FnMut(&Env) -> Code>;

/// Encode the executable code for the language using closures,
/// equivalent to bytecode
pub enum Code {
    Root,
    Scalar { val: Scalar, span: Span },
    If { code: CodeEx, span: Span },
    Eof,
}

pub fn compile(ast: &Parsed) -> Result<Code, ()> {
    // Only compile valid code!
    if !ast.errors.is_empty() {
        return Err(());
    }
    // Moving forward this MUST be correct code!

    Ok(Code::Eof)
}