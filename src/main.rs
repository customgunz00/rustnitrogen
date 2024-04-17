use reqwest::blocking::Client;
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};

fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

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

    let rn = generate_random_number(13, 24);
    let rs = generate_random_string(rn as usize);

    let url = "https://discord.com/api/v8/entitlements/gift-codes/";

    let finished_url = format!("{}{}", url, rs);

    let response = client.get(finished_url).send()?;

    println!("Status: {}", response.status());
    println!("Code: {}", rs);

    Ok(())
}
