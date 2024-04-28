pub mod data_types {
    use std::mem::size_of_val;

    pub fn number_types() {
        println!("Rust number types example\n");

        // max & min of 8-bit signed integer
        const SIGNED_8_BIT_MIN: i8 = -128;
        const SIGNED_8_BIT_MAX: i8 = 127;
        println!("8-bit signed integer min: {}", SIGNED_8_BIT_MIN);
        println!("8-bit signed integer max: {}\n", SIGNED_8_BIT_MAX);

        // max & min of 8-bit un-signed integer
        const UNSIGNED_8_BIT_MIN: u8 = 0;
        const UNSIGNED_8_BIT_MAX: u8 = 255;
        println!("8-bit un-signed integer min: {}", UNSIGNED_8_BIT_MIN);
        println!("8-bit un-signed integer max: {}\n", UNSIGNED_8_BIT_MAX);

        // max & min of 16-bit signed integer
        const SIGNED_16_BIT_MIN: i16 = -32768;
        const SIGNED_16_BIT_MAX: i16 = 32767;
        println!("16-bit signed integer min: {}", SIGNED_16_BIT_MIN);
        println!("16-bit signed integer max: {}\n", SIGNED_16_BIT_MAX);

        // max & min of 16-bit un-signed integer
        const UNSIGNED_16_BIT_MIN: u16 = 0;
        const UNSIGNED_16_BIT_MAX: u16 = 65535;
        println!("16-bit un-signed integer min: {}", UNSIGNED_16_BIT_MIN);
        println!("16-bit un-signed integer max: {}\n", UNSIGNED_16_BIT_MAX);

        // max & min of 32-bit signed integer
        const SIGNED_32_BIT_MIN: i32 = -2147483648;
        const SIGNED_32_BIT_MAX: i32 = 2147483647;
        println!("32-bit signed integer min: {}", SIGNED_32_BIT_MIN);
        println!("32-bit signed integer max: {}\n", SIGNED_32_BIT_MAX);

        // max & min of 32-bit un-signed integer
        const UNSIGNED_32_BIT_MIN: u32 = 0;
        const UNSIGNED_32_BIT_MAX: u32 = 4294967295;
        println!("32-bit un-signed integer min: {}", UNSIGNED_32_BIT_MIN);
        println!("32-bit un-signed integer max: {}\n", UNSIGNED_32_BIT_MAX);

        // max & min of 64-bit signed integer
        const SIGNED_64_BIT_MIN: i64 = -9223372036854775808;
        const SIGNED_64_BIT_MAX: i64 = 9223372036854775807;
        println!("64-bit signed integer min: {}", SIGNED_64_BIT_MIN);
        println!("64-bit signed integer max: {}\n", SIGNED_64_BIT_MAX);

        // max & min of 64-bit un-signed integer
        const UNSIGNED_64_BIT_MIN: u64 = 0;
        const UNSIGNED_64_BIT_MAX: u64 = 18446744073709551615;
        println!("64-bit un-signed integer min: {}", UNSIGNED_64_BIT_MIN);
        println!("64-bit un-signed integer max: {}\n", UNSIGNED_64_BIT_MAX);

        // max & min of 128-bit signed integer
        const SIGNED_128_BIT_MIN: i128 = -170141183460469231731687303715884105728;
        const SIGNED_128_BIT_MAX: i128 = 170141183460469231731687303715884105727;
        println!("128-bit signed integer min: {}", SIGNED_128_BIT_MIN);
        println!("128-bit signed integer max: {}\n", SIGNED_128_BIT_MAX);

        // max & min of 128-bit un-signed integer
        const UNSIGNED_128_BIT_MIN: u128 = 0;
        const UNSIGNED_128_BIT_MAX: u128 = 340282366920938463463374607431768211455;
        println!("128-bit un-signed integer min: {}", UNSIGNED_128_BIT_MIN);
        println!("128-bit un-signed integer max: {}\n", UNSIGNED_128_BIT_MAX);
    }

    pub fn bool_and_char() {
        let truth: bool = true;
        let untrue: bool = false;
        println!("`{}` and `{}` are booleans", truth, untrue);

        let character = 'a';
        println!("`{}` is a character type and it is {} Bytes in size",
                 character, size_of_val(&character));
    }
}