use reqwest::blocking::Client;
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};

fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let mut response = client.get("https://example.com").send()?;

    fn generate_random_string(length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    fn generate_random_number(min: i32, max: i32) -> i32 {
        let num: i32 = thread_rng().gen_range(min..max);
        return num;
    }

    let rn = generate_random_number(1, 100);
    let rs = generate_random_string(rn as usize);

    println!("{}", rn);
    println!("{}", rs);

    Ok(())
}
