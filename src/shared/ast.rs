use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum ASTNode {
    Program(Vec<ASTNode>),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<ASTNode>,
    },
    LetAssignment {
        name: String,
        value: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        consequence: Vec<ASTNode>,
        alternative: Option<Vec<ASTNode>>,
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    BinaryOperation {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    ReturnStatement(Box<ASTNode>),
    Identifier(String),
    Number(f64),
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl ASTNode {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let indent_str = " ".repeat(indent);
        match self {
            ASTNode::Program(statements) => {
                writeln!(f, "{}Program:", indent_str)?;
                for stmt in statements {
                    stmt.fmt_with_indent(f, indent + 2)?;
                }
            }
            ASTNode::Function { name, params, body } => {
                writeln!(
                    f,
                    "{}Function '{}' with parameters {:?}:",
                    indent_str, name, params
                )?;
                for stmt in body {
                    stmt.fmt_with_indent(f, indent + 2)?;
                }
            }
            ASTNode::LetAssignment { name, value } => {
                writeln!(f, "{}Let Assignment:", indent_str)?;
                writeln!(f, "{}  Name: {}", indent_str, name)?;
                writeln!(f, "{}  Value:", indent_str)?;
                value.fmt_with_indent(f, indent + 4)?;
            }
            ASTNode::IfStatement {
                condition,
                consequence,
                alternative,
            } => {
                writeln!(f, "{}If Statement:", indent_str)?;
                writeln!(f, "{}  Condition:", indent_str)?;
                condition.fmt_with_indent(f, indent + 4)?;
                writeln!(f, "{}  Consequence:", indent_str)?;
                for stmt in consequence {
                    stmt.fmt_with_indent(f, indent + 4)?;
                }
                if let Some(alt) = alternative {
                    writeln!(f, "{}  Alternative:", indent_str)?;
                    for stmt in alt {
                        stmt.fmt_with_indent(f, indent + 4)?;
                    }
                }
            }
            ASTNode::Assignment { name, value } => {
                writeln!(f, "{}Assignment:", indent_str)?;
                writeln!(f, "{}  Name: {}", indent_str, name)?;
                writeln!(f, "{}  Value:", indent_str)?;
                value.fmt_with_indent(f, indent + 4)?;
            }
            ASTNode::BinaryOperation {
                left,
                operator,
                right,
            } => {
                writeln!(f, "{}Binary Operation:", indent_str)?;
                writeln!(f, "{}  Operator: {}", indent_str, operator)?;
                writeln!(f, "{}  Left:", indent_str)?;
                left.fmt_with_indent(f, indent + 4)?;
                writeln!(f, "{}  Right:", indent_str)?;
                right.fmt_with_indent(f, indent + 4)?;
            }
            ASTNode::ReturnStatement(value) => {
                writeln!(f, "{}Return Statement:", indent_str)?;
                value.fmt_with_indent(f, indent + 2)?;
            }
            ASTNode::Identifier(name) => {
                writeln!(f, "{}Identifier: {}", indent_str, name)?;
            }
            ASTNode::Number(value) => {
                writeln!(f, "{}Number: {}", indent_str, value)?;
            }
        }
        Ok(())
    }
}
