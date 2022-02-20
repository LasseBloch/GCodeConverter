use regex::Regex;
use std::ops::Add;

#[derive(Debug)]
pub struct Configuration {
    convert_gcode: bool,
    input_file: String,
    output_file: String,
}

impl Configuration {
    pub fn new(
        convert_gcode: bool,
        input_file: String,
        output_file: std::option::Option<&str>,
    ) -> Configuration {
        Configuration {
            convert_gcode: convert_gcode,
            input_file: input_file.clone(),
            output_file: match output_file {
                Some(file_name) => String::from(file_name),
                // TODO: add logic to configure output name
                None => Configuration::gen_output_filename(&input_file),
            },
        }
    }

    fn gen_output_filename(input_filename: &str) -> String {
        // look for gcode file extension
        let re = Regex::new(r"\.(GCODE|gcode)$").unwrap();
        if re.is_match(input_filename) {
            return String::from(re.replace(input_filename, "_converted.gcode"));
        } else {
            return String::from(input_filename).add("_converted.gcode");
        }
    }

    pub fn input_file(&self) -> &str {
        return &self.input_file;
    }

    pub fn output_file(&self) -> &str {
        return &self.output_file;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_output_file_gen_filename_with_gcode_extension() {
        assert_eq!(
            Configuration::gen_output_filename("thing.gcode"),
            "thing_converted.gcode"
        );
        assert_eq!(
            Configuration::gen_output_filename("thing.GCODE"),
            "thing_converted.gcode"
        );
    }

    #[test]
    fn test_output_file_gen_filename_without_gcode_extension() {
        assert_eq!(
            Configuration::gen_output_filename("thing.exe"),
            "thing.exe_converted.gcode"
        );
    }
}
