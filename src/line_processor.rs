use lazy_static::lazy_static;
use regex::Regex;

pub fn process_line(s: &str) -> String {
    reformat_gcode(s)
}

fn reformat_gcode(s: &str) -> String {
    g0x_to_gx(s)
}

fn g0x_to_gx(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[G][0]").unwrap();
    }
    String::from(RE.replace(s, "G"))
}

// fn replace_movement_value(s: &str) -> String
// {
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_g0x_to_gx() {
        assert_eq!(g0x_to_gx(&mut String::from("G01")), "G1");
        assert_eq!(g0x_to_gx(&mut String::from("G01")), "G1");
        assert_eq!(g0x_to_gx(&mut String::from("G02")), "G2");
    }

    #[test]
    fn test_g0x_to_gx_line_without_gcode() {
        assert_eq!(g0x_to_gx(&mut String::from("Hello world")), "Hello world");
    }
}
