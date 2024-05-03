pub mod operators {
    pub fn math_operators() {
        let modulus = 18 % 7;
        println!("18 % 7 = {}", modulus);

        let squared = i32::pow(8, 2);
        println!("8 ^ 2 = {}", squared);

        let float_integer = f32::powi(6.5, 3);
        println!("6.5 ^ 3 = {}", float_integer);

        let float_float = f32::powf(6.5, 3.14);
        println!("6.5 ^ 3.14 = {}", float_float);

        let order_ops = 8 + 4 * 2 - (12 / 3 + 7) + 4;
        // 8 + 4 * 2 - (12 / 3 + 7) + 4 = 8 + 4 * 2 - (4 + 7) + 4
        // 8 + 4 * 2 - (4 + 7) + 4 = 8 + 4 * 2 - 11 + 4
        // 8 + 4 * 2 - 11 + 4 = 8 + 8 - 11 + 4
        // 8 + 8 - 11 + 4 = 16 - 11 + 4 = 20 - 11 = 9
        println!("Order of operators = {}", order_ops);
    }

    pub fn logic_operators() {
        let are_equal_is_true = 1 == 1;
        println!("1 == 1 is {}", are_equal_is_true);

        let are_equal_is_false = 1 == 2;
        println!("1 == 2 is {}", are_equal_is_false);

        let are_not_equal = 1 != 2;
        println!("1 != 2 is {}", are_not_equal);

        let is_true = true;
        let is_false = !is_true;
        println!("is_true: {}, is_false: {}", is_true, is_false);


        let have_drivers_license = false;
        let have_passport = true;
        let have_proof = have_drivers_license || have_passport;

        let have_boarding_pass = true;
        let have_id = have_proof;
        let can_board = have_boarding_pass && have_id;
        println!("Have Boarding Pass: {}, Have ID: {}", have_boarding_pass, have_id);
        println!("Can board plane: {}", can_board);

        let first_value = 10;
        let second_value = 15;
        let result = first_value < second_value;
        println!("{} < {} : {}", first_value, second_value, result);
    }

    pub fn bitwise_operators() {
        let bitwise_and = 86 & 27;
        println!("bitwise and: {}", bitwise_and);

        let bitwise_or = 86 | 27;
        println!("bitwise or: {}", bitwise_or);

        let bitwise_xor = 86 ^ 27;
        println!("bitwise xor: {}", bitwise_xor);

        let left_shift = 86 << 1;
        println!("left shift: {}", left_shift);

        let right_shift = 86 >> 1;
        println!("right shift: {}", right_shift);
    }

    pub fn project_part_one() {
        const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

        let kcle_latitude_degrees:f64 = 41.4075;
        let kcle_longitude_degrees:f64 = -81.851111;

        let kslc_latitude_degrees:f64 = 40.7861;
        let kslc_longitude_degrees:f64 = -111.9822;

        let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
        let kslc_latitude_radians = kslc_latitude_degrees.to_radians();

        let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
        let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + kcle_latitude_radians.cos()
            * kslc_latitude_radians.cos()
            * f64::powi((delta_longitude / 2.0).sin(), 2);
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();

        let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
        println!("The distance between the two points is {:.1} kilometers", distance);
    }
}