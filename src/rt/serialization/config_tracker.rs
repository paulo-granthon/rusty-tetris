use crate::{ clear_binary, append_binary, load_binary };
use crate::Controller;

const CONFIG_PATH: &str = "data/config";
const CONTROLLERS: [&str; 3] = ["default", "versus1", "versus2"];

// toggle for runtime debbuging
const DEBUG: bool = false;

/// converts given index InputID and corresponding key str to a singgle array of bytes
fn to_bytes (input_id: u8, key: String) -> Result<[u8; 16], std::io::Error> {
    match crate::file_handler::to_bytes::<16>(key, true) {
        Ok(mut bytes) => {
            bytes[0] = input_id;
            Ok(bytes)
        },
        Err(err) => Err(err)
    }
}

/// saves the given controller
pub fn save_controllers (controllers: &mut [Controller; 3]) -> Result<(), std::io::Error> {

    // clear binary while catching error
    for controller in CONTROLLERS {
        if let Err(err) = clear_binary(format!("{}/{}", CONFIG_PATH, controller).as_str()) { return Err(err) }
    }

    // loop through the controllers
    for i in 0..controllers.len() {

        // get the keys of this controller
        let keys = controllers[i].get_all();

        // loop through the keys
        for j in 0..keys.len() {

            // convert to bytes and match result
            match to_bytes(j as u8, keys[j].to_string()) {

                // Ok: append to the file and catch if error
                Ok(bytes) => {
                    if let Err(err) = append_binary(
                        format!("{}/{}", CONFIG_PATH, CONTROLLERS[i])
                        .as_str(), bytes) { return Err(err) }
                },
                // Error: return the error
                Err(err) => return Err(err)
            }
        }
    }
    
    // successfull operation
    Ok(())

}

/// returns the saved controller configuration
pub fn get_controllers () -> Result<[Controller; 3], std::io::Error> {

    let mut controllers = vec![];

    // loop through the loaded buffer with increments of 128 u8 -- controller
    for i in 0..3 {

        match get_controller(i) {
            Ok(controller) => controllers.push(controller),
            Err(err) => return Err(err)
        }
    }

    // return the result
    Ok(controllers.try_into().unwrap_or_else(|_| panic!("config_tracker -- Error converting Controller vec to array")))

}

/// returns the configured controller for the given category.
/// 0: singleplayer;    1: versus1;     2: versus2;
pub fn get_controller (player: usize) -> Result<Controller, std::io::Error> {

    assert!(player <= 2, "config_tracker::get_controller({}) -- Error: expected one of (0, 1, 2) player values but got {} instead!", player, player);

    // loads the binary and match result
    match load_binary(format!("{}/{}", CONFIG_PATH, CONTROLLERS[player]).as_str()) {

        // file is loaded successfully
        Ok(buffer) => {

            if DEBUG { println!("config_tracker::get_controller() -- Buffer read successfull for {}", CONTROLLERS[player]); }

            // create a vec of tuple for player, gamemode and score
            let mut list = vec![]; 

            // loop through the keys
            for j in 0..buffer.len() / 16 {

                // add the following tuple to the list
                list.push(
                    match String::from_utf8(buffer[(j * 16) + 1 .. (j * 16) + 16].to_vec()) {
                        Ok(key) => key.trim_matches(char::from(0)).to_string(),
                        Err(_) => "!!invalid key!!".to_string()
                    }
                )
            }

            Ok(match Controller::from_vec(list) {
                Some(controller) => {
                    if DEBUG { println!("config_tracker::get_controller() -- Controller loaded successfully: {:?}", controller); }
                    controller
                },
                None => {
                    let controller = [Controller::default, Controller::default_versus1, Controller::default_versus2][player]();
                    if DEBUG { println!("config_tracker::get_controller() -- No controller found, loading default: {:?}", controller); }
                    controller
                }
            })
        },

        // error loading binary file
        Err(e) => return Err(e)
    }
}
