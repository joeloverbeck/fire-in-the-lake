extern crate fire_in_the_lake;
use fire_in_the_lake::user_interface::controllers::user_interface_controller::UserInterfaceController;

extern crate termcolor;
use self::termcolor::{BufferWriter, ColorChoice};

fn main() {
    let user_interface_writer =
        UserInterfaceController::new(BufferWriter::stdout(ColorChoice::Always));

    if let Err(error) = user_interface_writer.write_announcement("Welcome to 'Fire in the Lake'") {
        println!("Error: {:?}", error);
    }

    if let Err(error) = user_interface_writer.write_announcement("Let's begin setup:") {
        println!("Error: {:?}", error);
    }
}
