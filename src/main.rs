#[macro_use]
extern crate serde_derive;

use std::error::Error;
use reqwest;
use reqwest::Response;
use std::fs;
use std::fs::File;
use std::path::Path;
use dirs;

#[derive(Deserialize, Debug)]
struct Images {
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
struct Image {
    url: String,
    startdate: String
}

const CACHE_DIR: &str = "Wallpapers-rust";
const FORMAT: &str = "js";
const INDEX: &str = "0";
const NUMBER: &str = "1";
const REGION: &str = "en-US";
const API_URL: &str = "http://www.bing.com/HPImageArchive.aspx?\
        format={format}&\
        idx={index}&\
        n={number}&\
        mkt={region}";

fn main() -> std::io::Result<()> {
    let url = API_URL.replace("{format}",FORMAT)
        .replace("{index}", INDEX)
        .replace("{number}", NUMBER)
        .replace("{region}", REGION);
    
    let images = match get_json(&url) {
       Ok(images) => images,
       Err(_) => panic!("couldn't get Bing Image")
    };
    
    let image_name = format!("image_{}.jpg",&images[0].startdate.as_str());
    let image_url = format!("http://www.bing.com{}",&images[0].url.as_str());

    let mut bar = sysbar::Sysbar::new("ʕ•̮͡•ʔ");

    bar.add_item(
        "Today",
        Box::new(move || {
            set_background(&image_name, &image_url);
        }),
    );

    bar.add_quit_item("Quit");

    bar.display();
    Ok(())
}

fn set_background(image_name: &str, image_url: &str) -> std::io::Result<()> {
    match dirs::home_dir().map(|h| h.join(CACHE_DIR)) {
        Some(cache_dir) => {
            fs::create_dir_all(&cache_dir)?;
            
            let image_path = cache_dir.join(image_name);
            
            let file = match download_image(&image_url) {
                Ok(file) => file,
                Err(e) => panic!("couldn't set background image to {}", e)
            };

            match save_image(&image_path, file) {
                Ok(_) => println!("saved"),
                Err(_) => panic!("couldn't save background image to {:?}", image_path)
            }

            match set_from_path(&image_path){
                Ok(result) => println!("{:?}", result),
                Err(_) => panic!("couldn't set background image to {:?}", image_path)
            }
            Ok(())
        },
        None => Ok(())
    }
}

// Request and parse JSON
fn get_json(url: &str) -> Result<Vec<Image>, reqwest::Error>{
    let client = reqwest::Client::new();
    let res = client.get(url).send()?.json::<Images>()?;    
    Ok(res.images)
}

// Download file
// Save image locally
// Return Image buffer to be set as background
fn download_image(url: &str) -> Result<Response, reqwest::Error>{
    let client = reqwest::Client::new();
    match client.get(url).send() {
        Ok(file) => Ok(file),
        Err(why) => panic!("couldn't download {}: {}", url, why),
    }
}

// Save file locally
fn save_image(path: &Path, mut file: Response) -> std::io::Result<()> {
    
    let mut cache_file = match File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
    };

    match std::io::copy(&mut file, &mut cache_file) {
        Ok(_) => Ok(()),
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
    }
}

fn set_from_path(path: &Path) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
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
        let e3: Box<Error> = From::from(format!(
            "{} exited with status code {}",
            command,
            output.status.code().unwrap_or(-1),
        ));
        Err(e3)
    }
}

fn run(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    get_stdout(command, args)?;
    Ok(())
}