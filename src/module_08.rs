pub mod functions_and_error_handling {
    pub fn functions() {
        let greater = return_greater(10, 5);
        println!("greater: {}", greater);
    }

    fn return_greater(first: u8, second: u8) -> u8 {
        if first > second {
            first
        } else {
            second
        }
    }

    pub fn ownership_and_borrowing_with_functions() {
        let mut original = String::from("original value");
        println!("\nouter scope original: \t\"{}\"", original);

        {
            print_original(&original);
            change_orignal(&mut original);
            println!("inner scope original: \t\"{}\"", original);
        }
    }

    fn print_original(original: &String) {
        println!("fn print_original: \t\"{}\"", original);
    }

    fn change_orignal(original: &mut String) {
        let next = original;
        *next = String::from("next value");
        println!("fn change_original: \t\"{}\"", next)
    }
}