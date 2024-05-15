pub mod functions_and_error_handling {
    use std::fmt::Error;
    use std::fs::File;
    use std::io::ErrorKind;

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

    pub fn result_enum() {
        // Rust way of error handling
        let filename = "C:\\Temp\\handbrake-user-presets.jsonp";

        // `match` is equivalent to `try`
        // the `Ok()` block is executed if the `try` succeeds
        // the `Err()` block is equivalent to `catch` blocks
        match File::open(filename) {
            Ok(file) => {
                println!("{:#?}", file);
            }
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        match File::create(filename) {
                            Ok(file) => {
                                println!("File created");
                            }
                            Err(error) => {
                                println!("{:#?}", error);
                            }
                        }
                    }
                    _ => {
                        println!("{:#?}", error);
                    }
                }
            }
        }
    }
}

pub mod functions_and_error_handling_more {
    use std::io::{Error, Read};
    use std::fs::File;

    pub fn error_propagation() {
        let filename = "C:\\Temp\\handbrake-user-presets.json";
        let file_data = read_file(filename);

        match file_data {
            Ok(data) => {
                println!("{}", data);
            }
            Err(_) => {
                // nothing to do
            }
        }
    }

    fn read_file(filename: &str) -> Result<String, Error> {
        let mut file_handle = File::open(filename)?;
        let mut file_data = String::new();

        file_handle.read_to_string(&mut file_data)?;
        Ok(file_data)
    }
}