pub mod crates_and_modules {
    use rand::Rng;

    pub fn cargo_toml() {
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();
        println!("{}", random_number);
    }
}