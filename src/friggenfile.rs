use crate::ast::AstNode;
use crate::error::{FriggenError, Result};
use crate::parser::parse_friggenfile;

#[derive(Debug, Clone)]
pub struct Task<'src> {
    pub name: &'src str,
    pub docs: Option<Vec<&'src str>>,
    pub deps: Vec<TaskDep<'src>>,
    pub hash_bang: Option<Vec<&'src str>>,
    pub script: Vec<&'src str>,
}

#[derive(Debug, Clone)]
pub struct TaskDep<'src> {
    pub name: &'src str,
    pub run_always: bool,
}

#[derive(Debug, Clone)]
pub struct Friggenfile<'src> {
    ast: AstNode<'src>,
}

impl<'src> Friggenfile<'src> {
    pub fn from(buf: &'src str) -> Result<Self> {
        let ast = parse_friggenfile(buf).map_err(FriggenError::FriggenfileSyntaxError)?;
        Ok(Self { ast })
    }

    #[inline]
    pub fn ast(&self) -> &AstNode<'src> {
        &self.ast
    }
}
