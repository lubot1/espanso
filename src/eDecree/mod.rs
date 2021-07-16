#![allow(unused_variables)]

use std::io;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,Debug, Clone, PartialEq)]
struct MyData {
    page: usize,
    per_page: usize,
    total: usize,
    total_pages: usize,
    data: Vec<dataObject>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct dataObject {
    id: usize,
    email: String,
    first_name: String,
    last_name: String,
    avatar: String
}

// fn main() {
//     let url = "https://reqres.in/api/users?page=2".to_string();
//     let path = "result.txt".to_string();

//     convert_to_yaml(url, path).expect("Oops something went wrong");
// }
// Claudio project
pub fn download_config(url: &String, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to a url and convert it to a string
    let res = reqwest::blocking::get(url)?.text()?;
    // Create a new file (or open) to copy data to.
    let mut file = File::create(path)?;

    // Convert to deserialized yaml using the JSON file directly
    let deserialized_json = serde_json::from_str::<serde_yaml::Value>(&res)?;
    // Serialize it into YAML form
    let serialized_yaml = serde_yaml::to_string(&deserialized_json)?;
    // Convert YAML form to bytes and print it to the new file
    io::copy(&mut serialized_yaml.as_bytes(), &mut file)?;

    //println!("{:?}", serialized_yaml); // This is for debugging purposes

    Ok(())
}