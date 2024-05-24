pub mod collections {
    fn print_dashes(flights: &Vec<&str>) {
        let mut length = 0;
        for flight in flights {
            if (flight.len() > length) {
                length = flight.len() + 2;
            }
        }

        // let length = flights.get(0).unwrap().len();
        (0..length).for_each(
            |i|
                if (i < length - 1) {
                    print!("-")
                } else {
                    print!("-\n")
                }
        );
    }

    pub fn vectors() {
        let mut flights: Vec<&str> = Vec::new();

        // alternative: declaration and initialization at the same time
        // let vec_macro = vec![1, 2, 3, 4, 5];

        flights.push("DA113\tto Boston departs at 06:20");
        flights.push("DA98\tto London departs at 09:43");
        flights.push("DA428\tto Salt Lake City departs at 12:05");
        flights.push("DA41\tto Berlin departs at 15:30");
        flights.push("DA2815\tto Nashville departs at 17:11");

        // accessing elements by index: ERROR PRONE
        let third = flights[2];
        println!("\nThe third entry in the vector is: {}\n", third);

        // accessing elements by index: SAFE OPTION
        let fourth = flights.get(3);
        match fourth {
            None => {}
            // _ => {}
            Some(flight) => {
                println!("The fourth entry in the vector is: {}\n", flight);
            }
        }

        // `if let` is beter than `match` if we are going to do something only if any value exists
        if let Some(flight_value) = flights.get(4) {
            println!("The fifth entry in the vector is: {}\n", flight_value);
        }

        print_dashes(&flights);
        println!("Flights: Unchanged");
        print_dashes(&flights);
        for flight in flights.iter() {
            println!("{}", flight);
        }

        print_dashes(&flights);
        println!("Flights: After INSERTION of data at index: 2");
        print_dashes(&flights);

        flights.insert(2, "DA918\tto Orlando departs at 11:12");
        for flight in flights.iter() {
            println!("{}", flight);
        }

        print_dashes(&flights);
        println!("Flights: After REMOVAL of data from index: 1");
        print_dashes(&flights);

        flights.remove(1);
        for flight in flights.iter() {
            println!("{}", flight);
        }
    }
}