use regex::Regex;
use lazy_static::lazy_static;

pub fn process_line(s: &String) -> String {
    let s = reformat_gcode(s);
    return s;
}

fn reformat_gcode(s: &String) -> String {
    return g0x_to_gx(s);
}

fn g0x_to_gx(s: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[G][0]").unwrap();
    }
    return String::from(RE.replace(s, "G"));
}

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
