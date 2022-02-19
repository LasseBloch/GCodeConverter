#[cfg(test)]
mod tests {
    use gcode_converter::line_processor;

    #[test]
    // This will be used for integration tests
    fn g0x_to_gx() {
        assert_eq!(
            line_processor::process_line(&mut String::from("G00 Z1.000000")),
            "G0 Z1.000000"
        );
        assert_eq!(
            line_processor::process_line(&mut String::from(
                "G03 X598.985205 Y244.724713 Z1.000000 I5.835383 J-53.018202"
            )),
            "G3 X598.985205 Y244.724713 Z1.000000 I5.835383 J-53.018202"
        );
    }
}
