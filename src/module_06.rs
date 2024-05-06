pub mod control_flow {
    pub fn if_else() {
        let word = "Dog";
        if word == "Duck" {
            println!("Quack");
        } else if word == "Dog" {
            println!("Bark");
        } else {
            println!("All quiet out here");
        }
    }

    pub fn if_else_logical_ops() {
        let available_aircraft = "Boeing";
        let minimum_crew = 7;
        let available_crew = 4;

        if (available_aircraft == "Boeing" || available_aircraft == "Airbus")
            && available_crew >= minimum_crew {
            println!("Okay");
        } else {
            println!("Something's not right!!!")
        }
    }

    enum NavigationAids {
        NDB(u16),
        VOR(String, f32),
        VORDME(String, f32),
        FIX { name: String, latitude: f32, longitude: f32 },
    }

    pub fn enums() {
        println!("NDB:\t{}", NavigationAids::NDB as u8);
        println!("VOR:\t{}", NavigationAids::VOR as u8);
        println!("VORDME:\t{}", NavigationAids::VORDME as u8);
    }

    fn print_nav_aid(navaid: &NavigationAids) {
        match navaid {
            NavigationAids::NDB(khz) => {
                println!("NDB frequency is {} kilohertz", khz);
            }
            NavigationAids::VOR(id, freq) => {
                println!("VOR identifier is {} and its frequency is {} kilohertz", id, freq);
            }
            NavigationAids::VORDME(id, freq) => {
                println!("VORDME identifier is {} and its frequency is {} kilohertz", id, freq);
            }
            NavigationAids::FIX { name, latitude, longitude } => {
                println!("FIX {} is at {} latitude and {} longitude",
                         name, latitude, longitude);
            }
        }
    }

    pub fn match_with_enums() {
        let ndb_uwl = NavigationAids::NDB(385);
        let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
        let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
        let fix_tarry = NavigationAids::FIX {
            name: String::from("TARRY"),
            latitude: 40.05333,
            longitude: -83.91367,
        };

        print_nav_aid(&ndb_uwl);
        print_nav_aid(&vor_dqn);
        print_nav_aid(&vor_dme_sgh);
        print_nav_aid(&fix_tarry);
    }


    pub fn options() {
        let phrase = String::from("Duck Airlines");
        // let letter = phrase.chars().nth(5);
        let letter = phrase.chars().nth(15);

        let value = match letter {
            Some(character) => character.to_string(),
            None => String::from("No Value")
        };
        println!("{}", value);
    }

    pub fn match_statement() {
        let animal = "Duck";
        match animal {
            "Duck" => println!("Quack"),
            "Dog" => println!("Bark"),
            _ => println!("All quiet out here")
        }

        let ndb_freq: u16 = 384;

        match ndb_freq {
            // match any number between 200 and 500 inclusive
            200..=500 => {
                println!("NDB frequency is valid");
            }

            _ => {
                println!("NDB frequency is not valid");
            }
        }

        match ndb_freq {
            ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
                println!("NDB frequency is valid");
            }
            _ => {
                println!("NDB frequency is not valid");
            }
        }
    }

    // Issues `warning: irrefutable `if let` pattern`
    // TODO: research more ob this
    pub fn if_let() {
        let animal = "Duck";

        if let animal = "Duck" {
            println!("Quack");
        }
    }

    pub fn rust_loops() {
        // Infinite loop
        // loop {
        //     println!("Hello");
        // }

        let mut counter = 0;
        loop {
            counter += 1;
            // skip printing 5
            if counter == 5 {
                continue;
            }

            println!("{}", counter);
            if counter == 10 {
                break;
            }
        }
    }
}