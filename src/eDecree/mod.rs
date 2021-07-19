#![allow(unused_variables)]

use std::io;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,Debug, Clone, PartialEq)]
struct MatchesObject {
    matches: Vec<TriggerObject>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct TriggerObject {
    trigger: String,
    replace: String,
    vars: Option<Vec<VarsObject>>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct VarsObject {
    name: String,
    r#type: String,
    params: ParamsObject
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct ParamsObject {
    format: Option<String>,
    cmd: Option<String>,
}
// Claudio project
pub fn download_config(url: &String, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to a url and convert it to a string
    let res = reqwest::blocking::get(url)?.text()?;
    // Create a new file (or open) to copy data to.
    let mut file = File::create(path)?;

    // Convert to deserialized yaml using the JSON file directly
    //let deserialized_json = serde_json::from_str::<serde_yaml::Value>(&res)?;
    
    // Serialize it into YAML form
    //let serialized_yaml = serde_yaml::to_string(&deserialized_json)?;
    //println!("{:?}", serialized_yaml);
    // Convert YAML form to bytes and print it to the new file
    //io::copy(&mut serialized_yaml.as_bytes(), &mut file)?;
    io::copy(&mut res.as_bytes(), &mut file)?;

    //println!("{:?}", deserialized_json); // This is for debugging purposes

    Ok(())
}