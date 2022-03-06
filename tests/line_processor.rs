#[cfg(test)]
mod tests {
    use gcode_converter::configuration::Configuration;
    use gcode_converter::line_processor;
    use gcode_converter::replace_movement_cmd::ReplaceMovementCmd;

    #[test]
    // This will be used for integration tests
    fn g0x_to_gx() {
        let conf = Configuration::new(true, "test".to_string(), None);
        let line_processor = line_processor::LineProcessor::new(conf);
        assert_eq!(
            line_processor.process_line(&mut String::from("G00 Z1.000000")),
            "G0 Z1.000000"
        );
        assert_eq!(
            line_processor.process_line(&mut String::from(
                "G03 X598.985205 Y244.724713 Z1.000000 I5.835383 J-53.018202"
            )),
            "G3 X598.985205 Y244.724713 Z1.000000 I5.835383 J-53.018202"
        );
    }

    #[test]
    fn g0x_to_gx_will_not_run_if_convert_gcode_is_false() {
        let conf = Configuration::new(false, "test".to_string(), None);
        let line_processor = line_processor::LineProcessor::new(conf);
        assert_eq!(
            line_processor.process_line(&mut String::from("G00 Z1.000000")),
            "G00 Z1.000000"
        );
        assert_eq!(
            line_processor.process_line(&mut String::from(
                "G03 X598.985205 Y244.724713 Z1.000000 I5.835383 J-53.018202"
            )),
            "G03 X598.985205 Y244.724713 Z1.000000 I5.835383 J-53.018202"
        );
    }

    #[test]
    fn replace_movement_value() {
        let mut conf = Configuration::new(true, "test".to_string(), None);
        let cmd = ReplaceMovementCmd::new(vec!["Z", "1.234", "1.000"]).unwrap();
        conf.add_replace_cmd(cmd);
        let line_processor = line_processor::LineProcessor::new(conf);
        assert_eq!(
            line_processor.process_line(&mut String::from("G1 Z1.234000")),
            "G1 Z1.000000"
        );
    }
}
