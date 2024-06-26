#![allow(unused)]
#![allow(irrefutable_let_patterns)]     // to allow `control_flow::if_let()`

mod module_03;
mod module_04;
mod module_05;
mod module_06;
mod module_07;
mod exercise_project;
mod module_08;
mod module_09;
mod module_10;
mod module_11;
mod module_12;
mod module_13;

pub use module_03::data_types;
pub use module_04::variables;
pub use module_05::operators;
pub use module_06::control_flow;
pub use module_07::ownership_and_borrowing;
pub use exercise_project::project;
pub use module_08::functions_and_error_handling;
pub use module_08::functions_and_error_handling_more;
pub use module_09::data_structures_and_traits;
// use crate::functions_and_error_handling::functions;
pub use module_10::collections;
pub use module_11::generics;
pub use module_12::concurrency;
pub use module_13::crates_and_modules;

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
    // control_flow::if_else();
    // control_flow::if_else_logical_ops();
    // control_flow::enums();
    // control_flow::options();
    // control_flow::match_statement();
    // control_flow::match_with_enums();
    // control_flow::if_let();
    // control_flow::rust_loops();
    // control_flow::while_loops();
    // control_flow::for_loops();

    // Module 07 - Ownership and Borrowing
    // ownership_and_borrowing::ownership();
    // ownership_and_borrowing::borrowing();
    // ownership_and_borrowing::borrowing_with_scope();
    // ownership_and_borrowing::lifetimes();
    // ownership_and_borrowing::bad_ref();
    // ownership_and_borrowing::explicit_lifetime_demo();

    // Exercise Project
    // project::project_part_one();
    // project::project_part_two();

    // Module 08 - Functions and Error Handling
    // functions_and_error_handling::functions();
    // functions_and_error_handling::ownership_and_borrowing_with_functions();
    // functions_and_error_handling::closures();
    // functions_and_error_handling::error_handling();
    // functions_and_error_handling::result_enum();
    // functions_and_error_handling_more::error_propagation();

    // Module 09 - Data Structures and Traits
    // data_structures_and_traits::data_structures();
    // data_structures_and_traits::associated_methods();
    // data_structures_and_traits::traits();

    // Module 10 - Collections
    // collections::vectors();
    // collections::vector_double_ended_queue();
    // collections::maps();
    // collections::sets();

    // Module 11 - Generics
    // generics::generic_types();
    // generics:: constraints();

    // Module 12 - Concurrency
    // concurrency::creating_threads();
    // concurrency::thread_communications();

    // Module 13 - Crates and Modules
    crates_and_modules::cargo_toml();
}
