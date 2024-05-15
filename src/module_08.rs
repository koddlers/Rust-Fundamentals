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

    pub fn closures() {
        let name = "Duck Airlines";

        let write_message = |slogan: String| {
            // print `name` and `slogan`
            // println!("{}. {}.", name, slogan);

            // return `name` and `slogan`
            // notice there is no trailing `;` in the return value
            String::from(format!("{}. {}.", name, slogan))
        };

        let slogan = String::from("We hit the ground everytime");
        let phrase = write_message(slogan);
        println!("{}", phrase);
    }

    // fn write_message() {
    //     let name = "Duck Airlines";
    //     let slogan = "We hit the ground everytime";
    //     println!("Welcome to {}. {}.", name, slogan);
    // }

    pub fn error_handling() {
        panic!("The following function call will panic");
        // panic_vector();
    }

    // fn panic_vector() {
    //     let vector = vec![1, 2, 3, 4, 5];
    //     println!("{}", vector[10]);
    // }
}