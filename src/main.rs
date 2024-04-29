mod module_03;
mod module_04;

pub use module_03::data_types;
pub use module_04::variables;

fn main() {
    // 03 - Data Types -> 02 - Number Types
    // data_types::number_types();
    // data_types::bool_and_char();
    // data_types::arrays_and_tuples();
    // data_types::strings_and_string_slices();
    // data_types::string_concatenation();
    // variables::vars();
    // variables::casting_data_types();
    // variables::variable_mutability();
    variables::scope_and_shadowing();
}
