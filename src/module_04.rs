pub mod variables {
    pub fn vars() {
        const ROOT_TWO: f32 = std::f32::consts::SQRT_2;
        println!("Root(2) as 32-bit floating point: {:.32}", ROOT_TWO);

        const PI: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;
        println!("PI as 64-bit floating point: {:.64}", PI);

        let inferred_var = 55_i8;
        println!("Inferred variable: {}", inferred_var);

        let _unused = 0;
    }

    pub fn casting_data_types() {
        let float_number = 17.2;
        let unsigned_number = 5;
        // let unsigned_number_cast = 5f32; // alternative
        let cast_unsigned_number = unsigned_number as f32;

        // ERROR: type mismatch
        // let result = float_number / unsigned_number;
        let result = float_number / cast_unsigned_number;
        println!("Result: {}", result);

        let number: u8 = 65;
        // let number: f32 = 65.0;     // ERROR ON CONVERSION
        let letter: char = number as char;

        println!("Number as char: {}", letter);
    }

    pub fn variable_mutability() {
        // let var = 8;
        // var = 5;    // ERROR: CANNOT CHANGE IMMUTABLE VARIABLES AFTER INITIALIZATION

        let mut mutable = 55;
        println!("Mutable value before change: {}", mutable);

        mutable = 32;
        println!("Mutable value after change: {}", mutable);
    }

    pub fn scope_and_shadowing() {
        let scope_test = "outer scope";
        println!("{}", scope_test);
        {
            let scope_test = "inner scope";
            println!("{}", scope_test);
        }
        println!("{}", scope_test);
    }
}