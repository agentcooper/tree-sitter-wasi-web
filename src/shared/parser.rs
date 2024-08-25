use tree_sitter::Node;

use super::ast::ASTNode;

pub fn build_ast_node<'a>(node: &Node<'a>, source: &'a str) -> ASTNode {
    match node.kind() {
        "program" => {
            let statements = parse_statement_list(node, source);
            ASTNode::Program(statements)
        }
        "function_declaration" => {
            let cursor = &mut node.walk();
            let mut children = node.named_children(cursor);
            let name = children
                .next()
                .unwrap()
                .utf8_text(source.as_bytes())
                .unwrap()
                .to_string();
            let params = children
                .next()
                .unwrap()
                .named_children(&mut node.walk())
                .map(|param| param.utf8_text(source.as_bytes()).unwrap().to_string())
                .collect();
            let body_node = children.next().unwrap();
            let body = parse_statement_list(&body_node, source);
            ASTNode::Function { name, params, body }
        }
        "lexical_declaration" => {
            let variable_declarator = node.named_child(0).unwrap();
            let name = variable_declarator
                .child_by_field_name("name")
                .unwrap()
                .utf8_text(source.as_bytes())
                .unwrap()
                .to_string();
            let value = Box::new(build_ast_node(
                &variable_declarator.child_by_field_name("value").unwrap(),
                source,
            ));
            ASTNode::LetAssignment { name, value }
        }
        "if_statement" => {
            let condition = Box::new(build_ast_node(
                &node.child_by_field_name("condition").unwrap(),
                source,
            ));
            let consequence =
                parse_statement_list(&node.child_by_field_name("consequence").unwrap(), source);
            let alternative = node
                .child_by_field_name("alternative")
                .map(|alt| parse_statement_list(&alt.named_child(0).unwrap(), source));
            ASTNode::IfStatement {
                condition,
                consequence,
                alternative,
            }
        }
        "parenthesized_expression" => build_ast_node(&node.named_child(0).unwrap(), source),
        "assignment_expression" => {
            let left = node.child_by_field_name("left").unwrap();
            let name = left.utf8_text(source.as_bytes()).unwrap().to_string();
            let value = Box::new(build_ast_node(
                &node.child_by_field_name("right").unwrap(),
                source,
            ));
            ASTNode::Assignment { name, value }
        }
        "binary_expression" => {
            let left = Box::new(build_ast_node(
                &node.child_by_field_name("left").unwrap(),
                source,
            ));
            let operator = node
                .child_by_field_name("operator")
                .unwrap()
                .utf8_text(source.as_bytes())
                .unwrap()
                .to_string();
            let right = Box::new(build_ast_node(
                &node.child_by_field_name("right").unwrap(),
                source,
            ));
            ASTNode::BinaryOperation {
                left,
                operator,
                right,
            }
        }
        "return_statement" => {
            let value = Box::new(build_ast_node(&node.named_child(0).unwrap(), source));
            ASTNode::ReturnStatement(value)
        }
        "identifier" => ASTNode::Identifier(node.utf8_text(source.as_bytes()).unwrap().to_string()),
        "number" => {
            let value = node.utf8_text(source.as_bytes()).unwrap().parse().unwrap();
            ASTNode::Number(value)
        }
        _ => {
            eprintln!("Unhandled node type: {}", node.kind());
            ASTNode::Program(vec![]) // Return an empty program as a fallback
        }
    }
}

fn parse_statement_list<'a>(node: &Node<'a>, source: &'a str) -> Vec<ASTNode> {
    node.named_children(&mut node.walk())
        .map(|child| build_ast_node(&child, source))
        .collect()
}
