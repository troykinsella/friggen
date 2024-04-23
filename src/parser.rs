use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{AstNode, AstTaskDef, AstTaskDep, AstTaskHeader, AstTaskScript, AstVarAssignment};

#[derive(Parser)]
#[grammar = "friggenfile.pest"]
struct FriggenfileParser;

pub fn parse_friggenfile(buf: &str) -> Result<AstNode, Box<Error<Rule>>> {
    let friggenfile = match FriggenfileParser::parse(Rule::friggenfile, buf) {
        Ok(mut ff) => ff.next().unwrap(),
        Err(e) => return Err(Box::new(e)),
    };

    Ok(parse_ast(friggenfile))
}

fn parse_ast(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::root => AstNode::Root(pair.into_inner().map(parse_ast).collect()),
        Rule::var_assignment => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap().as_str();
            let value = if let Some(next) = pairs.next() {
                parse_ast(next)
            } else {
                AstNode::VarValue("")
            };
            AstNode::VarAssignment(AstVarAssignment {
                name,
                value: Box::new(value),
            })
        }
        Rule::plain_value
        | Rule::single_quoted_value
        | Rule::double_quoted_value
        | Rule::triple_quoted_value => {
            let value = pair.as_str();
            AstNode::VarValue(value)
        }
        Rule::command_sub_command => {
            let command = pair.as_str();
            AstNode::CommandSubstitution(command)
        }
        Rule::task_def => {
            let mut pairs = pair.into_inner();
            let docs = if pairs.len() == 3 {
                Some(Box::new(parse_ast(pairs.next().unwrap())))
            } else {
                None
            };
            let header = parse_ast(pairs.next().unwrap());
            let script = parse_ast(pairs.next().unwrap());
            AstNode::TaskDef(AstTaskDef {
                docs,
                header: Box::new(header),
                script: Box::new(script),
            })
        }
        Rule::task_header => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap().as_str();
            let deps = if let Some(pair) = pairs.next() {
                pair.into_inner().map(parse_ast).collect()
            } else {
                vec![]
            };
            AstNode::TaskHeader(AstTaskHeader { name, deps })
        }
        Rule::task_docs => AstNode::TaskDocs(
            pair.into_inner()
                .map(|p| p.into_inner().next().unwrap().as_str())
                .collect(),
        ),
        Rule::task_dep_list => AstNode::TaskDepList(pair.into_inner().map(parse_ast).collect()),
        Rule::task_dep => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap().as_str();
            let run_always = if let Some(pair) = pairs.next() {
                pair.as_str() == "!"
            } else {
                false
            };
            AstNode::TaskDep(AstTaskDep { name, run_always })
        }
        Rule::task_script => {
            let pairs = pair.into_inner();

            let hash_bang: Option<Vec<_>> = pairs
                .find_first_tagged("hash_bang")
                .map(|pair| pair.into_inner().map(|p| p.as_str()).collect());

            let mut lines: Vec<_> = pairs.map(|p| p.as_str()).collect();
            process_script(&mut lines);
            AstNode::TaskScript(AstTaskScript { hash_bang, lines })
        }
        _ => {
            unreachable!("noap {:?}", pair)
        }
    }
}

fn process_script(script: &mut [&str]) {
    let indent = find_indent(script);
    if indent > 0 {
        for line in script {
            if line.len() >= indent {
                *line = &line[indent..];
            }
        }
    }
}

fn find_indent(script: &[&str]) -> usize {
    let first_line = script.iter().find(|line| !is_whitespace(line)).unwrap(); // TODO
    leading_whitespace(first_line)
}

#[inline]
fn is_whitespace(line: &str) -> bool {
    for c in line.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}

fn leading_whitespace(line: &str) -> usize {
    let mut ws = 0;
    for c in line.chars() {
        if c.is_whitespace() {
            ws += 1;
        } else {
            break;
        }
    }
    ws
}

#[cfg(test)]
mod tests {
    use crate::ast::{AstNode, AstTaskDef, AstTaskHeader, AstTaskScript, AstVarAssignment};
    use crate::parser::parse_friggenfile;

    #[test]
    fn test_empty() {
        let ast = parse_friggenfile("").unwrap();
        assert_eq!(ast, AstNode::Root(vec![]));
    }

    #[test]
    fn test_empty_lines() {
        let ast = parse_friggenfile("\n\r\n\r").unwrap();
        assert_eq!(ast, AstNode::Root(vec![]));
    }

    #[test]
    fn test_single_comment() {
        let ast = parse_friggenfile("#hey").unwrap();
        assert_eq!(ast, AstNode::Root(vec![]));

        let ast = parse_friggenfile("#hey\n").unwrap();
        assert_eq!(ast, AstNode::Root(vec![]));
    }

    #[test]
    fn test_multiple_comment() {
        let ast = parse_friggenfile("#foo\n# bar\n#  baz\n").unwrap();
        assert_eq!(ast, AstNode::Root(vec![]));

        let ast = parse_friggenfile("#  baz\n# bar\n#foo\n").unwrap();
        assert_eq!(ast, AstNode::Root(vec![]));
    }

    #[test]
    fn test_simple_task() {
        let ff = r#"
foo:
  echo hi
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::TaskDef(AstTaskDef {
                docs: None,
                header: Box::new(AstNode::TaskHeader(AstTaskHeader {
                    name: "foo",
                    deps: vec![],
                })),
                script: Box::new(AstNode::TaskScript(AstTaskScript {
                    hash_bang: None,
                    lines: vec!["echo hi\n"],
                })),
            })])
        );
    }

    #[test]
    fn test_two_simple_tasks() {
        let ff = r#"
foo:
  echo hi

bar:
  echo hello
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![
                AstNode::TaskDef(AstTaskDef {
                    docs: None,
                    header: Box::new(AstNode::TaskHeader(AstTaskHeader {
                        name: "foo",
                        deps: vec![],
                    })),
                    script: Box::new(AstNode::TaskScript(AstTaskScript {
                        hash_bang: None,
                        lines: vec!["echo hi\n", "\n"],
                    })),
                }),
                AstNode::TaskDef(AstTaskDef {
                    docs: None,
                    header: Box::new(AstNode::TaskHeader(AstTaskHeader {
                        name: "bar",
                        deps: vec![],
                    })),
                    script: Box::new(AstNode::TaskScript(AstTaskScript {
                        hash_bang: None,
                        lines: vec!["echo hello\n"],
                    })),
                }),
            ])
        );
    }

    #[test]
    fn test_task_docs_single_line() {
        let ff = r#"
## foo kicks ass
foo:
  echo hi
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::TaskDef(AstTaskDef {
                docs: Some(Box::new(AstNode::TaskDocs(vec!["foo kicks ass\n"]))),
                header: Box::new(AstNode::TaskHeader(AstTaskHeader {
                    name: "foo",
                    deps: vec![],
                })),
                script: Box::new(AstNode::TaskScript(AstTaskScript {
                    hash_bang: None,
                    lines: vec!["echo hi\n"],
                })),
            })])
        );
    }

    #[test]
    fn test_task_docs_multiple_lines() {
        let ff = r#"
## foo kicks ass
## no seriously
foo:
  echo hi
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::TaskDef(AstTaskDef {
                docs: Some(Box::new(AstNode::TaskDocs(vec![
                    "foo kicks ass\n",
                    "no seriously\n"
                ]))),
                header: Box::new(AstNode::TaskHeader(AstTaskHeader {
                    name: "foo",
                    deps: vec![],
                })),
                script: Box::new(AstNode::TaskScript(AstTaskScript {
                    hash_bang: None,
                    lines: vec!["echo hi\n"],
                })),
            })])
        );
    }

    #[test]
    fn test_task_docs_multiple_lines_with_ws_gap() {
        let ff = r#"
## foo kicks ass
## no seriously

foo:
  echo hi
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::TaskDef(AstTaskDef {
                docs: Some(Box::new(AstNode::TaskDocs(vec![
                    "foo kicks ass\n",
                    "no seriously\n"
                ]))),
                header: Box::new(AstNode::TaskHeader(AstTaskHeader {
                    name: "foo",
                    deps: vec![],
                })),
                script: Box::new(AstNode::TaskScript(AstTaskScript {
                    hash_bang: None,
                    lines: vec!["echo hi\n"],
                })),
            })])
        );
    }

    #[test]
    fn test_var_assignment_empty() {
        let ff = r#"
foo =
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::VarValue("")),
            })])
        );
    }

    #[test]
    fn test_var_assignment_plain() {
        let ff = r#"
foo = bar
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::VarValue("bar")),
            })])
        );
    }

    #[test]
    fn test_var_assignment_plain_no_eol() {
        let ff = r#"
foo = bar"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::VarValue("bar")),
            })])
        );
    }

    #[test]
    fn test_var_assignment_single_quote() {
        let ff = r#"
foo = 'bar'
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::VarValue("bar")),
            })])
        );
    }

    #[test]
    fn test_var_assignment_double_quote() {
        let ff = r#"
foo = "bar"
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::VarValue("bar")),
            })])
        );
    }

    #[test]
    fn test_var_assignment_triple_quote() {
        let ff = r#"
foo = """bar"""
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::VarValue("bar")),
            })])
        );
    }

    #[test]
    fn test_var_assignment_command_substitution() {
        let ff = r#"
foo = $(echo "bar")
"#;

        let ast = parse_friggenfile(ff).unwrap();
        assert_eq!(
            ast,
            AstNode::Root(vec![AstNode::VarAssignment(AstVarAssignment {
                name: "foo",
                value: Box::new(AstNode::CommandSubstitution("echo \"bar\"")),
            })])
        );
    }
}
