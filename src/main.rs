#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;
use serde_json::{Value};
use serde::Deserialize;

#[derive(Deserialize)]
struct Simple {
   key: String
}

#[derive(Deserialize, Debug)]
struct User {
    context: String,
    types: String, 
    geo: String,
    properties: String
}

fn main() -> Result<(), Error> {
    let request_url = format!("https://api.weather.gov/gridpoints/{office}/{lat},{long}/forecast",
                              office = "FFC",
                              lat = "51",
                              long = "83");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    //let users: Vec<User> = response.json()?;
    let data = response.text()?;
    let s_slice: &str = data.as_str();
    let v: Value = serde_json::from_str(s_slice);
    println!("{:?}", v);
    Ok(())
}
