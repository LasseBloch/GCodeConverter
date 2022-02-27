use crate::configuration::Configuration;
use lazy_static::lazy_static;
use regex::Regex;

pub struct LineProcessor {
    conf: Configuration,
}

impl LineProcessor {
    pub fn new(conf: Configuration) -> LineProcessor {
        LineProcessor { conf }
    }

    pub fn conf(&self) -> &Configuration {
        &self.conf
    }

    pub fn process_line(&self, s: &str) -> String {
        if self.conf.convert_gcode() {
            let mut res = self.reformat_gcode(s);
            if !self.conf.replace_cmds().is_empty() {
                res = self.replace_movement_value(&res);
            }

            return res;
        }
        String::from(s)
    }

    fn reformat_gcode(&self, s: &str) -> String {
        self.g0x_to_gx(s)
    }

    fn g0x_to_gx(&self, s: &str) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[G][0]").unwrap();
        }
        String::from(RE.replace(s, "G"))
    }

    fn replace_movement_value(&self, s: &str) -> String {
        let mut res = String::from(s);
        let replace_pairs: Vec<(String, String)> = self
            .conf
            .replace_cmds()
            .iter()
            .map(|cmd| cmd.gen_replace_strings())
            .collect();

        for replace_pair in replace_pairs {
            res = res.replace(&replace_pair.0, &replace_pair.1);
        }
        res
    }
}
