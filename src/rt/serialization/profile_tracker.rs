use crate::{ clear_binary, append_binary, load_binary };


// path to the profiles file
const PROFILES_PATH: &str = "data/profiles/profiles";

// maximum number of profiles 
pub const MAX_PROFILES: usize = 15;

// formats the given profile to binary
fn to_bytes (name: String) -> Result<[u8; 16], std::io::Error> {

    // make sure that the length of the name is at most 16  chars
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

// saves the given vec of profiles, overwriting any previous profiles
pub fn save_profiles (profiles: &Vec<String>) -> Result<(), std::io::Error> {

    // clear binary while catching error
    if let Err(err) = clear_binary(PROFILES_PATH) { return Err(err) }

    // loop through given profiles
    for i in 0..profiles.len() {

        // get bytes of profile and match result
        match to_bytes(profiles[i].to_owned()) {

            // Ok: append to the file and atch if error
            Ok(bytes) => if let Err(err) = append_binary(PROFILES_PATH, bytes) { return Err(err) },
            
            // Error: return the error
            Err(err) => return Err(err)
        }
    }
    
    // successfull operation
    Ok(())
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

pub fn profile_name (profile: usize) -> String {
    match get_profiles() {
        Ok(profiles) => profiles[profile - 1].to_string(),
        Err(_) => "Error".to_string()
    }
}

pub fn set_profile (profile: usize) {

}

pub fn load_profile () -> Option<usize> {
    None
}