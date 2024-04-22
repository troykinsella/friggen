#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<'src> {
    Root(Vec<Self>),
    TaskDef(AstTaskDef<'src>),
    TaskDocs(Vec<&'src str>),
    TaskHeader(AstTaskHeader<'src>),
    TaskScript(AstTaskScript<'src>),
    TaskDepList(Vec<Self>),
    TaskDep(AstTaskDep<'src>),
    VarAssignment(AstVarAssignment<'src>),
    VarValue(&'src str),
    CommandSubstitution(&'src str),
}

impl<'src> AstNode<'src> {
    #[inline]
    pub fn as_task_header(&self) -> &AstTaskHeader {
        match self {
            Self::TaskHeader(h) => h,
            _ => panic!("expected task header"),
        }
    }

    #[inline]
    pub fn as_task_docs(&self) -> &Vec<&'src str> {
        match self {
            Self::TaskDocs(d) => d,
            _ => panic!("expected task docs"),
        }
    }

    #[inline]
    pub fn as_task_script(&self) -> &AstTaskScript {
        match self {
            Self::TaskScript(s) => s,
            _ => panic!("expected task script"),
        }
    }

    #[inline]
    pub fn as_task_dep(&self) -> &AstTaskDep {
        match self {
            Self::TaskDep(d) => d,
            _ => panic!("expected task dep"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstTaskDef<'src> {
    pub docs: Option<Box<AstNode<'src>>>,
    pub header: Box<AstNode<'src>>,
    pub script: Box<AstNode<'src>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstTaskHeader<'src> {
    pub name: &'src str,
    pub deps: Vec<AstNode<'src>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstTaskDep<'src> {
    pub name: &'src str,
    pub run_always: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstTaskScript<'src> {
    pub hash_bang: Option<Vec<&'src str>>,
    pub lines: Vec<&'src str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstVarAssignment<'src> {
    pub name: &'src str,
    pub value: Box<AstNode<'src>>,
}
