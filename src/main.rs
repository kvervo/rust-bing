#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::io;
use curl::easy::Easy;
use rustc_serialize::json::Json;
use reqwest;
// use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Debug)]
struct Images {
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
struct Image {
    url: String,
    startdate: String
}

fn main() {

    let format = "js";
    let index = "0";
    let number = "1";
    let region = "en-US";
    let api_url = "http://www.bing.com/HPImageArchive.aspx?\
        format={format}&\
        idx={index}&\
        n={number}&\
        mkt={region}";
    
    let debug = api_url.replace("{format}",format)
        .replace("{index}", index)
        .replace("{number}", number)
        .replace("{region}", region);
    
    match get_json(debug) {
        Ok(value) => println!("{:?}",value[0]),
        Err(_) => println!("Fuck!"),
    };
    
    
}

// Request and parse JSON
fn get_json(url: String) -> Result<Vec<Image>, reqwest::Error>{
    
    let client = reqwest::Client::new();
    let res = client.get(url.as_str()).send()?.json::<Images>()?;
    Ok(res.images)

}

// Download file
fn download_image(){

}

// Save file locally
fn save_image(){

}