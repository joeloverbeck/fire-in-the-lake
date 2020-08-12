pub fn transform_typed_digit_into_integer(typed_input_command: &str) -> u8 {
    if typed_input_command == "1" {
        1
    } else if typed_input_command == "2" {
        2
    } else if typed_input_command == "3" {
        3
    } else if typed_input_command == "6" {
        6
    } else {
        panic!("Passed: {:?}", typed_input_command);
    }
}
