use serde_json::{Result, Value};
use colored::Colorize;

pub fn show_blogs(json_data: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_data)?;

    // Display feed information
    let feed = &v["feed"];
    println!("Feed Information:");
    println!("URL: {}", feed["url"].as_str().unwrap().bright_green());
    println!("Title: {}", feed["title"].as_str().unwrap().bright_green());
    println!("Link: {}", feed["link"].as_str().unwrap().bright_green());
    println!("Author: {}", feed["author"].as_str().unwrap().bright_green());
    println!("Description: {}", feed["description"].as_str().unwrap().bright_green());
    println!("Image: {}", feed["image"].as_str().unwrap().bright_green());
    println!();

    // Display blog items
    let items = &v["items"];
    for item in items.as_array().unwrap() {
        println!("Title: {}", item["title"].as_str().unwrap().bright_green());
        println!("Publication Date: {}", item["pubDate"].as_str().unwrap().bright_green());
        println!("Link: {}", item["link"].as_str().unwrap().bright_green());
        println!("GUID: {}", item["guid"].as_str().unwrap().bright_green());
        println!("Author: {}", item["author"].as_str().unwrap().bright_green());
        println!("Thumbnail: {}", item["thumbnail"].as_str().unwrap().bright_green());
        println!("Description: {}", item["description"].as_str().unwrap().bright_green());
        println!();
    }

    Ok(())
}
