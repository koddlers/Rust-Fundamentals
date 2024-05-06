#![allow(unused)]
#![allow(irrefutable_let_patterns)]     // to allow `control_flow::if_let()`

mod module_03;
mod module_04;
mod module_05;
mod module_06;

pub use module_03::data_types;
pub use module_04::variables;
pub use module_05::operators;
pub use module_06::control_flow;

fn main() {
    // Module 03 - Data Types -> 02 - Number Types
    // data_types::number_types();
    // data_types::bool_and_char();
    // data_types::arrays_and_tuples();
    // data_types::strings_and_string_slices();
    // data_types::string_concatenation();

    // Module 04 - Variables
    // variables::vars();
    // variables::casting_data_types();
    // variables::variable_mutability();
    // variables::scope_and_shadowing();

    // Module 05 - Operators
    // operators::math_operators();
    // operators::logic_operators();
    // operators::bitwise_operators();
    // operators::project_part_one();
    // control_flow::if_else();
    // control_flow::if_else_logical_ops();
    // control_flow::enums();
    // control_flow::options();
    // control_flow::match_statement();
    // control_flow::match_with_enums();
    // control_flow::if_let();
    control_flow::rust_loops();
}
