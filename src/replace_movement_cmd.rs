#[derive(Debug, Clone, PartialEq)]
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

    pub fn axis(&self) -> &str {
        &self.axis
    }

    pub fn old_val(&self) -> &str {
        &self.old_val
    }

    pub fn new_val(&self) -> &str {
        &self.new_val
    }

    pub fn gen_replace_strings(&self) -> (String, String) {
        (
            (self.axis.clone() + &self.old_val),
            self.axis.clone() + &self.new_val,
        )
    }
}
