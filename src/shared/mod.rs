use parser::build_ast_node;
use tree_sitter::{Language, Parser, Tree};

mod ast;
mod parser;

pub fn get_parse_tree(source_code: String) -> String {
    let tree = tree_sitter_parse_javascript(&source_code);
    let root_node = tree.root_node();
    root_node.to_sexp()
}

pub fn get_ast(source_code: String) -> String {
    let tree = tree_sitter_parse_javascript(&source_code);
    let root_node = tree.root_node();
    let ast = build_ast_node(&root_node, &source_code);
    format!("{}", ast)
}

fn tree_sitter_parse_javascript(source_code: &str) -> Tree {
    let mut parser = Parser::new();
    let language: Language = tree_sitter_javascript::LANGUAGE.into();
    parser
        .set_language(&language)
        .expect("Error loading JavaScript grammar");
    let parsed = parser
        .parse(&source_code, None)
        .expect("Error parsing JavaScript code");
    parsed
}
