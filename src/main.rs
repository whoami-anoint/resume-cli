mod blogs;
mod contact;
mod skills;
mod poems;

use inquire::Select;
use std::fs;
use colored::Colorize;
use contact::show_contact;
use skills::show_skills;
use blogs::show_blogs;
use poems :: show_poems;

fn main() {
    println!("");
    println!("");
    println!("Hey there! I'm {},
     {} | {} | {} | {}
     Always exploring with tech..","Abhishek Kafle".bold().bright_yellow(),"Infosec Poet".bold().bright_red(),"Developer".bold().bright_green(),"Alpha Geek".bold().bright_purple(),"DevOps Engineer".bold().yellow());

    let options = vec!["Intro","Skills","Blogs","Poems","Projects","Contact","Exit"];

    loop{
        let choice = Select::new("What would you like to know?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0] {
                    println!("");
                    println!("🔧💻 I'm a {}, always exploring with tech.","DevOps Engineer".bold().bright_yellow());
                    println!("");
                    println!("🚀🛡️ On a mission to excel in { }.","DevSecOps".bold().bright_yellow());
                    println!("");
                    println!("🔒🤖 Passionate about {} and {}.","cybersecurity".bold().bright_yellow(),"automation".bold().bright_yellow());
                    println!("");
                    println!("🕵️‍♂️🔨 Eager to tackle exciting projects, like {}.","security tests and tool development".bold().bright_yellow());
                    println!("");
                    println!("💻🚀 Digging into {}.","coding, deployment,and problem solving".bold().bright_yellow());
                    println!("");
                    println!("🎮🌐 Even my free time is a playground for {}.","infosec and infotech".bold().bright_yellow());
                    println!("");
                    println!("✍️📚 Proudly an {}, I write poems on tech and cybersecurity.","Infosec Poet".bold().bright_yellow());
                    println!("");
                    println!("🌟 I am constantly working to improve myself and my skills, always pushing to {}.","dream,dare and do".bold().bright_yellow());
                    println!("");
                    println!("🚀🛡️👾 Let's {} cool tech stuff together!","team up,learn and create".bold().bright_yellow());
                    println!("");
                }
                else if choice == options[1] {
                    let file_path = "./data/skills/skills.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_skills(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in skills.rs"),
                    }
                }
                else if choice == options[2] {
                    let file_path = "./data/blogs/blogs.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_blogs(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in blogs.rs"),
                    }
                }
                else if choice == options[3] {
                    show_poems();
                }
                else if choice == options[4] {
                    show_contact();
                }
                else if choice == options[5] {
                    println!("Bye! Have a great day!");
                    break;
                }
            },
            Err(_) => println!("You did not select a valid option"),
        }
    }
}
