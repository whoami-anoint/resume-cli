use serde_json::{Result, Value};
use colored::Colorize;

pub fn show_skills(json_data: &str) -> Result<()>{
    let v: Value = serde_json::from_str(json_data)?;

    println!("");
    println!("{}: {}","Languages".bold(),  v["Languages"].as_str().unwrap().bright_green());
    println!("{}: {}","Web Technologies".bold(),  v["Web Technologies"].as_str().unwrap().bright_green());
    println!("{}: {}","Hosting".bold(),  v["Hosting"].as_str().unwrap().bright_green());
    println!("{}: {}","Server".bold(),  v["Server"].as_str().unwrap().bright_green());
    println!("{}: {}","Languages".bold(),  v["Languages"].as_str().unwrap().bright_green());
    println!("{}: {}","AI/ML".bold(),  v["AI/ML"].as_str().unwrap().bright_green());
    println!("{}: {}","Pentesting".bold(),  v["Pentesting"].as_str().unwrap().bright_green());
    println!("{}: {}","DevOps".bold(),  v["DevOps"].as_str().unwrap().bright_green());
    Ok(())
}
