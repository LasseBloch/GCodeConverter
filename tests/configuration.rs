#[cfg(test)]
mod tests {
    use gcode_converter::{configuration::Configuration, replace_movement_cmd::ReplaceMovementCmd};

    #[test]
    fn cmds_is_empty_by_default() {
        let conf = Configuration::new(false, "test".to_string(), None);
        assert!(conf.replace_cmds().is_empty());
    }

    #[test]
    fn cmds_can_be_added() {
        let mut conf = Configuration::new(false, "test".to_string(), None);
        let cmd = ReplaceMovementCmd::new(vec!["X", "1", "2"]).unwrap();
        conf.add_replace_cmd(cmd);
        assert_eq!(conf.replace_cmds().len(), 1);

        let cmd = ReplaceMovementCmd::new(vec!["Z", "3", "4"]).unwrap();
        conf.add_replace_cmd(cmd);
        assert_eq!(conf.replace_cmds().len(), 2);
    }

    #[test]
    fn give_me_the_cmd_back() {
        let mut conf = Configuration::new(false, "test".to_string(), None);
        let cmd = ReplaceMovementCmd::new(vec!["X", "1", "2"]).unwrap();
        let cmd_copy = cmd.clone();
        conf.add_replace_cmd(cmd);

        // Get cmd back
        let cmd = conf.replace_cmds()[0].clone();
        assert_eq!(cmd, cmd_copy);
    }

    #[test]
    fn output_filename_if_none_given() {
        let conf = Configuration::new(false, "inputFile.gcode".to_string(), None);
        // The input file_ name is used to generated output filename
        assert_eq!(conf.output_file(), "inputFile_converted.gcode");
    }

    #[test]
    fn output_filename_if_none_given_input_name_without_gcode_ending() {
        let conf = Configuration::new(false, "inputFile.txt".to_string(), None);
        // The input file_ name is used to generated output filename
        assert_eq!(conf.output_file(), "inputFile.txt_converted.gcode");
    }

    #[test]
    fn output_filame_is_not_changed_if_provided() {
        let output_file_name: &str = "outputFile.gcode";
        let conf = Configuration::new(false, "inputFile.gcode".to_string(), Some(&output_file_name));
        // The input file_ name is used to generated output filename
        assert_eq!(conf.output_file(), String::from(output_file_name));
    }
}
