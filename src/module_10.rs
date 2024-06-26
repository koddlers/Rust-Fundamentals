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

        // `if let` is better than `match` if we are going to do something only if any value exists
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

    use std::collections::VecDeque;

    pub fn vector_double_ended_queue() {
        let mut flights: VecDeque<&str> = VecDeque::new();

        flights.push_front("DA918\tto Orlando departs at 11:12");
        flights.push_back("DA428\tto Salt Lake City departs at 12:05");
        flights.push_front("DA98\tto London departs at 09:43");
        flights.push_front("DA113\tto Boston departs at 06:20");
        flights.push_back("DA41\tto Berlin departs at 15:30");
        flights.push_back("DA2815\tto Nashville departs at 17:11");

        let length = flights.len();
        println!("There are {} flights scheduled.\n", length);

        for flight in flights.iter() {
            println!("{}", flight);
        }

        println!("\nChecking if an element exists with the `contains()` method");
        let exists = flights.contains(&"DA98\tto London departs at 09:43");
        println!("Element exists: {}", exists);

        println!("\nCanceling all flights");
        flights.clear();

        let length = flights.len();
        if (length > 0) {
            println!("There are {} flights scheduled.", length);
        } else {
            println!("All the flights have been cancelled.");
        }
    }

    use std::collections::HashMap;

    pub fn maps() {
        let mut flights = HashMap::new();

        flights.insert("DA918", ("11:12", "Orlando"));
        flights.insert("DA428", ("12:05", "Salt Lake City"));
        flights.insert("DA98", ("09:43", "London"));
        flights.insert("DA113", ("06:20", "Boston"));
        flights.insert("DA41", ("15:30", "Berlin"));
        flights.insert("DA2815", ("17:11", "Nashville"));

        let mut flight_number = "DA2815";
        let option = flights.get(flight_number);
        let (time, destination) = option.unwrap();
        println!("Flight {} starts at {}, for {}", flight_number, time, destination);

        // flight_number = "DA2815";    // else branch
        flight_number = "DA531";
        if !flights.contains_key(flight_number) {
            flights.insert(flight_number, ("12:00", "Puerto Rico"));
        } else {
            println!("Flight {} is already entered", flight_number);
        }

        for flight in flights.iter() {
            println!("{:?}", flight);
        }
    }

    use std::collections::HashSet;

    pub fn sets() {
        let mut flights = HashSet::new();

        flights.insert(("DA918", "11:12", "Orlando"));
        flights.insert(("DA428", "12:05", "Salt Lake City"));
        flights.insert(("DA98", "09:43", "London"));
        flights.insert(("DA113", "06:20", "Boston"));
        flights.insert(("DA918", "11:12", "Orlando"));      // Duplicate value

        // HashSets don't maintain insertion order
        // the values are printed in different order for each iteration
        for flight in flights.iter() {
            println!("{:?}", flight);
        }
    }
}