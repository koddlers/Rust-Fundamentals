// Revisit this module again
//     * Visit the docs
//     * Watch YouTube videos/Tutorials

pub mod ownership_and_borrowing {
    pub fn ownership() {
        let original = String::from("original value");
        println!("\noriginal: \t\"{}\"\n", original);

        /*
        `next` and `original` are both pointers and resides on the `stack` when declared
        but the value they point to, i.e. `String` can change size and are located in the
        `heap`. When `next` is assigned the value of `original`, we are actually handing
        over the ownership of the memory allocated for the `String` ("original value") to
        `next`. The variable `original` is discarded from the `stack` and no longer exists.
        */
        // `next` takes ownership of the value of `original`
        // running the following code will produce error
        // let next = original;
        // println!("{}", original);
    }

    pub fn borrowing() {
        let mut original = String::from("original value");
        println!("\noriginal: \t\"{}\"", original);
        // original = String::from("new value");

        let next = &original;
        original = String::from("new value");
        println!("original: \t\"{}\"", original);
    }

    pub fn borrowing_with_scope() {
        let mut original = String::from("original value");
        println!("\nOuter original value: \t\"{}\"", original);
        println!("original memory address: \t{:p}", &original);    // memory

        // Introducing new scope
        {
            // `next` takes ownership of the memory address pointed to by `original`
            // look at the `// memory` comments
            let next = &mut original;
            *next = String::from("next value");

            println!("next memory address: \t\t{:p}", next);     // memory
            println!("inner scope next: \t\"{}\"", next);
            println!("inner scope original: \t\"{}\"", original);
        }

        println!("Outer original value: \t\"{}\"", original);
    }
}