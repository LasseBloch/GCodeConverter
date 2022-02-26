#[derive(Debug)]
pub struct ReplaceMovementCmd {
    axis: String,
    old_val: String,
    new_val: String,
}

impl ReplaceMovementCmd {
    // TODO: add more handling of input vars
    pub fn new(values: Vec<&str>) -> Result<ReplaceMovementCmd, &'static str> {
        if values.len() != 3 {
            return Err("invalid length of values expected 3");
        }
        Ok(ReplaceMovementCmd {
            axis: values[0].to_string(),
            old_val: values[1].to_string(),
            new_val: values[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_vec_test() {
        let v = vec!["Z", "10", "20"];
        let cmd = ReplaceMovementCmd::new(v);
        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();
        assert_eq!(cmd.axis, "Z");
        assert_eq!(cmd.old_val, "10");
        assert_eq!(cmd.new_val, "20");
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
}
