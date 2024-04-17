use reqwest::blocking::Client;
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::Write; 

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

fn makereq(cli: Client, filename: &str) -> Result<(), reqwest::Error> {
    let rn = generate_random_number(13, 24);
    let rs = generate_random_string(rn as usize);

    let mut file = File::open(filename).unwrap();

    let url = "https://discord.com/api/v8/entitlements/gift-codes/";

    let finished_url = format!("{}{}", url, rs);

    let response = cli.get(finished_url).send()?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        println!("{} | CODE INVALID", rs);
    } else if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!("Waiting for 7.5 seconds... | RATE LIMITED");
    } else if response.status() == reqwest::StatusCode::OK {
        println!("{} | CODE VALID", rs);
        file.write(rs.as_bytes());
    }
    
    Ok(())
}


fn main() {
    let client: Client = Client::new();

    loop {
        makereq(client.clone(), "codes.txt");
    }
}