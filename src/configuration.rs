#[derive(Debug)]
pub struct Configuration {
    convert_gcode: bool,
    input_file: String,
}

impl Configuration {
    pub fn new(convert_gcode: bool, input_file: String) -> Configuration {
        Configuration {
            convert_gcode: convert_gcode,
            input_file: input_file,
        }
    }

    pub fn input_file(&self) -> &str {
        return &self.input_file;
    }
}
