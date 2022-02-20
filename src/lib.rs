pub mod configuration;
pub mod line_processor;
use std::error::Error;
use std::fs;
use std::process;

// TODO: Refactor or validate sanity
fn read_file(input_file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(input_file)?;
    let lines = content.lines();
    // We cannot return lines directly because lines only references content
    // and therefor we cannot return it since content will go out of scope
    // We are mapping the str& to Strings and collecting them into a vector
    let lines = lines.map(String::from).collect();

    Ok(lines)
}

pub fn convert_file(conf: configuration::Configuration) {
    let mut lines = read_file(&conf.input_file()).unwrap_or_else(|err| {
        println!("Could not read file {}", err);
        process::exit(1);
    });

    println!("{}", lines.len());

    for line in lines {
        println!("{}", line);
    }
}
