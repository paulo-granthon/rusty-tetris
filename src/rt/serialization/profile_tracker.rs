use crate::{ write_binary, append_binary, load_binary };


// path to the profiles file
const PROFILES_PATH: &str = "data/profiles/profiles";

// maximum number of profiles 
const MAX_PROFILES: usize = 15;

// formats the given profile to binary
fn to_bytes (name: &str) -> Result<[u8; 16], std::io::Error> {

    assert!(name.len() <= 16, "profile_tracker.to_bytes() -- Error: Expected name of size <= 16 but got {} instead", name.len());

    // get the name as bytes
    let name_bytes = name.to_owned().into_bytes();

    // initialize a 16 length u8 array
    let mut bytes: [u8; 16] = [0; 16];

    // loop trough the string bytes
    for i in 0..name_bytes.len() {
        bytes[16 - name_bytes.len() + i] = name_bytes[i];
    }

    // return the result
    Ok(bytes)
}

pub fn save_profile (name: &str) -> Result<(), std::io::Error> {
    match get_profiles() {
        Ok(profiles) => {
            assert!(profiles.len() <= MAX_PROFILES, "profile_tracker.save_profile() -- Error: Max number of profiles reached ({})", MAX_PROFILES);
            // let mut id = 0;
            // for profile in profiles {
            //     if profile.0 <= id { continue; }
            //     id = profile.0 + 1;
            // }
            match to_bytes(name) {
                Ok(name_bytes) => append_binary(PROFILES_PATH, name_bytes),
                Err(e) => Err(e)
            }

        },
        Err(_) => {
            match to_bytes(name) {
                Ok(name_bytes) => write_binary(PROFILES_PATH, name_bytes),
                Err(e) => Err(e)
            }
        }
    }
}

pub fn get_profiles () -> Result<Vec<String>, std::io::Error> {

    // loads the binary and match result
    match load_binary(PROFILES_PATH) {

        // file is loaded successfully
        Ok(buffer) => {

            // create a vec of tuple for player, gamemode and score
            let mut list = vec![]; 

            // loop through the loaded buffer with increments of 16 u8
            for i in 0..buffer.len() / 16 {

                // add the following tuple to the list
                list.push(

                    // id: first u8
                    // buffer[i * 16],

                    // score: the 3 remaining u8
                    // i32::from_be_bytes([0, buffer[(i * 4) + 1], buffer[(i * 4) + 2], buffer[(i * 4) + 3]]) * 10
                    match String::from_utf8(buffer[(i * 16) .. (i * 16) + 16].to_vec()) {
                        Ok(name) => name.trim_matches(char::from(0)).to_string(),
                        Err(_) => "!!invalid name!!".to_string()
                    }
                )
            }

            // return the result
            Ok(list)
        },

        // error loading binary file
        Err(e) => Err(e)
    }
}

pub fn set_profile (profile: usize) {

}

pub fn load_profile () -> Option<usize> {
    None
}