pub mod configuration;
pub mod line_processor;
pub mod replace_movement_cmd;
use std::error::Error;
use std::fs;
use std::io::Write;
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

fn write_output_file(output_file: &str, content: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::create(output_file)?;

    for line in content {
        writeln!(f, "{}", line)?;
    }

    Ok(())
}

pub fn convert_file(conf: configuration::Configuration) {
    let lines = read_file(conf.input_file()).unwrap_or_else(|err| {
        println!("Could not read file {}", err);
        process::exit(1);
    });

    let line_processor = line_processor::LineProcessor::new(conf);

    let modified_lines = lines
        .iter()
        .map(|line| line_processor.process_line(line))
        .collect();

    write_output_file(line_processor.conf().output_file(), modified_lines).unwrap_or_else(|err| {
        println!("Could not write file {}", err);
        process::exit(1);
    });
}
