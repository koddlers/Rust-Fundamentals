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
}