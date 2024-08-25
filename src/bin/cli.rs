#[path = "../shared/mod.rs"]
mod shared;

fn main() {
    let js_source_code = r#"function add(a, b) {
    let result = a + b;
    if (result > 10) {
        return result * 2;
    } else {
        return result;
    }
}
    "#;

    println!("Source Code:\n{}\n", js_source_code);

    println!(
        "Parse Tree:\n{}\n",
        shared::get_parse_tree(js_source_code.to_string())
    );

    println!("AST:\n{}", shared::get_ast(js_source_code.to_string()));
}
