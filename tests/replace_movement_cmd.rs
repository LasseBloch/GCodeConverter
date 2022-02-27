#[cfg(test)]
mod tests {
    use gcode_converter::replace_movement_cmd::ReplaceMovementCmd;

    #[test]
    fn parse_vec_test() {
        let v = vec!["Z", "10", "20"];
        let cmd = ReplaceMovementCmd::new(v);
        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();
        assert_eq!(cmd.axis(), "Z");
        assert_eq!(cmd.old_val(), "10");
        assert_eq!(cmd.new_val(), "20");
    }

    #[test]
    fn input_size_is_off_test() {
        let v = vec!["Z", "10"];
        let cmd = ReplaceMovementCmd::new(v);
        assert!(cmd.is_err());
        let v = vec!["Z", "10", "20", "20"];
        let cmd = ReplaceMovementCmd::new(v);
        assert!(cmd.is_err());
    }

    #[test]
    fn replace_strings_from_cmd() {
        let v = vec!["Z", "10", "20"];
        let cmd = ReplaceMovementCmd::new(v).unwrap();
        let (to_be_replaced, replacement) = cmd.gen_replace_strings();

        assert_eq!(to_be_replaced, "Z10");
        assert_eq!(replacement, "Z20")
    }
}
