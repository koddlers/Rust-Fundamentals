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
        NDB = 3,
        VOR = 2,
        VORDME = 5,
        // FIX { name: String, latitude: f32, longitude: f32 },
    }

    pub fn enums() {
        println!("NDB:\t{}", NavigationAids::NDB as u8);
        println!("VOR:\t{}", NavigationAids::VOR as u8);
        println!("VORDME:\t{}", NavigationAids::VORDME as u8);
    }
}