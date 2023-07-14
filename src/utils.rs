use std::{error::Error, io::Read};

use reqwest;

use crate::API_KEY;

pub fn get_from_api(url: &String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client
        .get(url)
        .header("apikey", API_KEY)
        .header("locale", "en")
        .send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf).unwrap();
    Ok(buf)
}
