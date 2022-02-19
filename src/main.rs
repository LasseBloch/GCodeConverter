use clap::{arg, Command};
use gcode_converter::line_processor;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
struct Configuration {
    convert_gcode: bool,
    input_file: String,
}

// This must 
fn read_file(input_file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(input_file)?;
    let lines = content.lines();
    let lines = lines.map(String::from).collect();

    Ok(lines)
}

fn main() {
    let matches = Command::new("GCode converter")
        .version("0.0")
        .author("Lasse Bloch. <lassebloch@gmail.com>")
        .about("Converts gcode generated from Incscape into something that Klipper will accept")
        .arg(
            arg!(-r --replace_gcode ... "Replace all G-codes with the format G0X with GX")
                .required(false),
        )
        .arg(arg!(-f --input_file <FILE> "Original G-code file"))
        .get_matches();

    let replace_gcode = matches.is_present("replace_gcode");

    let input_file = matches.value_of("input_file").expect("required");
    let input_file = String::from(input_file);

    let conf = Configuration {
        convert_gcode: replace_gcode,
        input_file: input_file,
    };
    println!("{:?}", conf);

    let mut lines = read_file(&conf.input_file).unwrap_or_else(|err| {
        println!("Could not read file {}", err);
        process::exit(1);
    });

    println!("{}", lines.len());

    for line in lines {
        println!("{}", line);
    }
}
