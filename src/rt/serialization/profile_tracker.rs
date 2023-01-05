use super::super::{write_binary, append_binary, load_binary};

// path to the profiles file
const PROFILES_PATH: &str = "data/profiles/profiles";

// maximum number of profiles 
const MAX_PROFILES: usize = 15;

// formats the given profile to binary
fn to_bytes (name: &str) -> [u8; 16] {

    // get the name as bytes
    let name_bytes = name.to_owned().into_bytes();

    // initialize a 16 length u8 array
    let mut bytes: [u8; 16] = [0; 16];

    // loop trough the string bytes
    for i in 0..name_bytes.len() {
        bytes[16 - name_bytes.len() + i] = name_bytes[i];
    }

    // return the result
    bytes
}

pub fn save_profile (name: &str) -> Result<(), std::io::Error> {
    append_binary(PROFILES_PATH, to_bytes(name))
    // match get_profiles() {
    //     Ok(profiles) => {
    //         let mut id = 0;
    //         for profile in profiles {
    //             if profile.0 <= id { continue; }
    //             id = profile.0 + 1;
    //         }
    //         append_binary(PROFILES_PATH, to_bytes(name))

    //     },
    //     Err(_) => {
    //         write_binary(PROFILES_PATH, to_bytes(name))
    //     }
    // }
}

pub fn get_profiles () -> Result<Vec<(u8, String)>, std::io::Error>{

    // loads the binary and match result
    match load_binary(PROFILES_PATH) {

        // file is loaded successfully
        Ok(buffer) => {

            // create a vec of tuple for player, gamemode and score
            let mut list = vec![]; 

            // loop through the loaded buffer with increments of 16 u8
            for i in 0..buffer.len() / 16 {

                // add the following tuple to the list
                list.push((

                    // id: first u8
                    buffer[i * 16],

                    // score: the 3 remaining u8
                    // i32::from_be_bytes([0, buffer[(i * 4) + 1], buffer[(i * 4) + 2], buffer[(i * 4) + 3]]) * 10
                    match String::from_utf8(buffer[(i * 16) + 1 .. (i * 16) + 16].to_vec()) {
                        Ok(name) => name.trim_matches(char::from(0)).to_string(),
                        Err(_) => "".to_string()
                    }
                ))
            }

            // return the result
            Ok(list)
        },

        // error loading binary file
        Err(e) => Err(e)
    }
}