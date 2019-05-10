use std::io;

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
    
    get_json(debug);
}

// Request and parse JSON
fn get_json(url: String){
    println!("API {}", url);
}

// Download file
fn download_image(){

}

// Save file locally
fn save_image(){

}