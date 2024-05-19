#![allow(unused)]

pub mod data_structures_and_traits {
    struct Waypoint {
        name: String,
        latitude: f64,
        longitude: f64,
    }

    struct Segment {
        start: Waypoint,
        end: Waypoint,
    }

    impl Segment {
        fn new(start: Waypoint, end: Waypoint) -> Self {
            Self {
                start,
                end,
            }
        }

        fn distance(&self) -> f32 {
            const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
            let start_radians = self.start.latitude.to_radians();
            let end_radians = self.end.latitude.to_radians();
            let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
            let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();
            let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                + start_radians.cos() * end_radians.cos()
                * f64::powi((delta_longitude / 2.0).sin(), 2);
            let central_angle = 2.0 * inner_central_angle.sqrt().asin();
            let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

            distance as f32
        }
    }

    pub fn data_structures() {
        const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

        let kcle = Waypoint {
            name: "KCLE".to_string(),
            latitude: 41.4075,
            longitude: -81.85111,
        };

        println!("name: {}, lat: {}, lon: {}", kcle.name, kcle.latitude, kcle.longitude);

        let kslc = Waypoint {
            name: "KSLC".to_string(),
            ..kcle
        };

        println!("name: {}, lat: {}, lon: {}", kslc.name, kslc.latitude, kslc.longitude);
    }

    pub fn associated_methods() {
        let kcle = Waypoint {
            name: "KCLE".to_string(),
            latitude: 41.4075,
            longitude: -81.85111,
        };

        let kslc = Waypoint {
            name: "KSLC".to_string(),
            latitude: 40.7861,
            longitude: -111.9822,
        };

        let kcle_kslc = Segment::new(kcle, kslc);
        let distance = kcle_kslc.distance();
        println!("{:.1}", distance);
    }

    struct Boeing {
        required_crew: u8,
        range: u16,
    }

    struct Airbus {
        required_crew: u8,
        range: u16,
    }

    trait Flight {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
    }

    impl Flight for Boeing {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
            if (available_crew >= required_crew) && (range + 150 > distance) {
                true
            } else {
                false
            }
        }
    }

    impl Flight for Airbus {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
            if (available_crew >= required_crew) && (range + 280 > distance) {
                true
            } else {
                false
            }
        }
    }

    pub fn traits() {
        let boeing = Boeing {
            required_crew: 4,
            range: 7370,
        };

        let airbus = Airbus {
            required_crew: 7,
            range: 5280,
        };

        let boeing_is_legal = boeing.is_legal(
            boeing.required_crew, 18,
            boeing.range, 2385,
        );

        let airbus_is_legal = airbus.is_legal(
            airbus.required_crew, 3,
            airbus.range, 2200,
        );

        println!("Is the Boeing flight legal? {}\nIs the Airbus flight legal? {}\n",
                 boeing_is_legal, airbus_is_legal);
    }
}