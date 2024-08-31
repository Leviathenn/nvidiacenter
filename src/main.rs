/**
 * @author Leviathenn
 */


use std::{fs::File, io::Read, path::Path, process::exit};
mod structs;

// make main a tokio task
#[tokio::main]
async fn main() {
    println!("Checking for updates...");
    let res = reqwest::blocking::get("https://raw.githubusercontent.com/Leviathenn/nvidiacenter/main/res/build.json");
    if res.is_err() {
        println!("Failed to check for updates.");
        return;
    }
    //check to see if destrealizing the json is successful
    let build: structs::build::Build = match res.unwrap().json() {
        Ok(build) => build,
        Err(_e) => {
            update().await;
            return;
        }
    };
    if Path::new("res/build.json").exists() {
        let mut file = File::open("res/build.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let old_build: structs::build::Build = match serde_json::from_str(&contents) {
            Ok(build) => build,
            Err(_) => {
                println!("Failed to deserialize build.json");
                exit(1);
                
            }
        };
        if old_build.version != build.version {
            println!("Update found! Downloading...");
            update().await;
            
        }
    };
    
    home().await;
}
async fn update() {
    println!("Downloading...");
    let res = reqwest::blocking::get("https://raw.githubusercontent.com/Leviathenn/nvidiacenter/main/res/build.json");
    if res.is_err() {
        println!("Failed to download build.json");
        exit(1);
    }
    let build: structs::build::Build = match res.unwrap().json() {
        Ok(build) => build,
        Err(_e) => {
            println!("Failed to deserialize build.json");
            exit(1);
        }
    };
    println!("Updating to latest version {}", build.version);
    // check for releases
    let res = reqwest::blocking::get("https://api.github.com/repos/Leviathenn/nvidiacenter/releases/latest");
    if res.is_err() {
        println!("Failed to get latest release");
        exit(1);
    }
    let release: structs::release::Release = match res.unwrap().json() {
        Ok(release) => release,
        Err(_e) => {
            println!("Failed to deserialize release.json");
            exit(1);
        }
    };
    // check user guid to see if they are root
    // if not, ask for sudo
    
    // download assets but create a temp directory

}
async fn home() {
    // Your code here
}
