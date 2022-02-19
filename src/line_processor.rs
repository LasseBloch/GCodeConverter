use regex::Regex;

pub fn hello_world() {
    println!("blah")
}

pub fn process_line(s: &mut String) -> String {
    let s = reformat_gcode(s);
    return s;
}

fn reformat_gcode(s: &mut String) -> String {
    return g0x_to_gx(s);
}

fn g0x_to_gx(s: &mut String) -> String {
    let re = Regex::new(r"^[G][0]").unwrap();
    return String::from(re.replace(s, "G"));
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
}
