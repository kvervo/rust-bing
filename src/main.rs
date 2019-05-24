#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate serde;
// #[macro_use]
// extern crate serde_json;

use std::io;
use std::error::Error;
use rustc_serialize::json::Json;
use reqwest;
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::path::Path;

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
        Ok(images) => {
            download_image(&images[0]);
            
            let image_path = format!("./image_{}.jpg",images[0].startdate.as_str());
            match set_from_path(&image_path){
                Ok(result) => {
                    println!("{:?}", result);
                },
                Err(_) => panic!("couldn't set background image to {}", image_path),
            }
        },
        Err(_) => panic!("couldn't get Bing Image")
    };
}

// Request and parse JSON
fn get_json(url: String) -> Result<Vec<Image>, reqwest::Error>{
    
    let client = reqwest::Client::new();
    let res = client.get(url.as_str()).send()?.json::<Images>()?;    
    Ok(res.images)

}

// Download file
// Save image locally
// Return Image buffer to be set as background
fn download_image(img: &Image) -> Result<(), reqwest::Error>{
    let url = format!("http://www.bing.com{}",img.url.as_str());
    let image_path = format!("./image_{}.jpg",img.startdate.as_str());

    let mut client = reqwest::Client::new();
    let mut image_file = client
        .get(&url)
        .send()?;

    save_image(image_path, image_file);
    Ok(())
}


// Save file locally
fn save_image(image_path: String, mut image_file: Response){
    let path = Path::new(&image_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match std::io::copy(&mut image_file, &mut file) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display)
    }
    
}

fn set_from_path(path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&path);
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "Finder" to set desktop picture to POSIX file {}"#,
                enquote::enquote('"', path.canonicalize().unwrap().to_str().unwrap()),
            ),
        ],
    )
}

fn get_stdout(command: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        let e3: Box<Error> = From::from((format!(
            "{} exited with status code {}",
            command,
            output.status.code().unwrap_or(-1),
        )));
        Err(e3)
    }
}

fn run(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    get_stdout(command, args)?;
    Ok(())
}