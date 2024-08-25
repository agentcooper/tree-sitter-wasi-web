#[allow(warnings)]
mod bindings;

mod shared;

use bindings::Guest;
struct Component;

impl Guest for Component {
    fn get_parse_tree(source_code: String) -> String {
        shared::get_parse_tree(source_code)
    }
    fn get_ast(source_code: String) -> String {
        shared::get_ast(source_code)
    }
}

bindings::export!(Component with_types_in bindings);
