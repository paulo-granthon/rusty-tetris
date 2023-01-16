use crate::{ clear_binary, append_binary, load_binary };
use crate::Controller;

const CONFIG_PATH: &str = "data/config";

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
    if let Err(err) = clear_binary(CONFIG_PATH) { return Err(err) }

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
                        format!("{}/{}", CONFIG_PATH, ["default", "versus1", "versus2"][i])
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
            
        // loads the binary and match result
        match load_binary(format!("{}/{}", CONFIG_PATH, ["default", "versus1", "versus2"][i]).as_str()) {

            // file is loaded successfully
            Ok(buffer) => {

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

                controllers.push(match Controller::from_vec(list) {
                    Some(controller) => controller,
                    None => [Controller::default, Controller::default_versus1, Controller::default_versus2][i]()
                });
            },

            // error loading binary file
            Err(e) => return Err(e)
        }

    }

    // return the result
    Ok(controllers.try_into().unwrap_or_else(|_| panic!("config_tracker -- Error converting Controller vec to array")))

}

