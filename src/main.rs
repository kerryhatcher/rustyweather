extern crate reqwest;
extern crate json;

use reqwest::Error;

fn main() -> Result<(), Error> {
    let request_url = format!("https://api.weather.gov/gridpoints/{office}/{lat},{long}/forecast",
                              office = "FFC",
                              lat = "51",
                              long = "83");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let data = response.text()?;
    let parsed = json::parse(&data);

    println!("{:?}", parsed);

    Ok(())
}
